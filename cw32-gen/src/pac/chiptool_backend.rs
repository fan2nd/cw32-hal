//! The real upstream backend plus the CW32 contracts that its IR cannot express.
//!
//! chiptool owns blocks, addresses, arrays, fieldsets, fields and enums. Its
//! external-common option preserves our access policy. A checked Rust AST pass
//! only adds reset specialization, side-effect types and existing raw-bit API
//! spellings; it never regenerates field masks, array offsets or enum encodings.
use super::{instance_reset_registers, read_marker, value_name, ResetOverrides};
use crate::schema::{self, Result};
use chiptool::{generate, ir};
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{format_ident, quote};
use std::collections::{BTreeMap, BTreeSet};
use syn::{parse_quote, visit_mut::VisitMut, Item};

type Blocks = BTreeMap<(String, String), schema::Block>;
const UNKNOWN_RESET: u64 = 0x1_0000_0000;
const KEYWORD_PREFIX: &str = "__cw32_kw_";

fn check_backend_ident(name: &str) -> Result<()> {
    if name.starts_with(KEYWORD_PREFIX) || name.starts_with(&format!("set_{KEYWORD_PREFIX}")) {
        return Err(format!("reserved chiptool adapter identifier prefix: {name}").into());
    }
    Ok(())
}

fn method(name: &str) -> Result<String> {
    let name = name.to_ascii_lowercase();
    check_backend_ident(&name)?;
    Ok(if super::rust_ident(&name).starts_with("r#") {
        format!("{KEYWORD_PREFIX}{name}")
    } else {
        name
    })
}

// chiptool constructs plain Idents, so temporarily encode keyword accessors.
// Restore tokens before parsing Rust, including their generated Debug calls.
fn restore_keywords(tokens: TokenStream) -> TokenStream {
    tokens
        .into_iter()
        .map(|token| match token {
            TokenTree::Group(group) => {
                let mut mapped =
                    proc_macro2::Group::new(group.delimiter(), restore_keywords(group.stream()));
                mapped.set_span(group.span());
                TokenTree::Group(mapped)
            }
            TokenTree::Ident(ident) => {
                let name = ident.to_string();
                if let Some(name) = name.strip_prefix(KEYWORD_PREFIX) {
                    TokenTree::Ident(Ident::new_raw(name, ident.span()))
                } else if let Some(name) = name.strip_prefix(&format!("set_{KEYWORD_PREFIX}")) {
                    TokenTree::Ident(Ident::new(&format!("set_{name}"), ident.span()))
                } else {
                    TokenTree::Ident(ident)
                }
            }
            other => other,
        })
        .collect()
}

fn array(array: Option<&schema::Array>) -> Result<Option<ir::Array>> {
    array
        .map(|array| -> Result<_> {
            Ok(match array {
                schema::Array::Regular(a) => ir::Array::Regular(ir::RegularArray {
                    len: a.len.try_into()?,
                    stride: a.stride.try_into()?,
                }),
                schema::Array::Explicit(a) => ir::Array::Cursed(ir::CursedArray {
                    offsets: a
                        .offsets
                        .iter()
                        .map(|&n| n.try_into())
                        .collect::<std::result::Result<_, _>>()?,
                }),
            })
        })
        .transpose()
}

fn lower(blocks: &Blocks) -> Result<ir::IR> {
    let mut ir = ir::IR::new();
    for ((kind, version), block) in blocks {
        let module = format!("{kind}_{version}");
        check_backend_ident(&module)?;
        let mut items = Vec::new();
        for register in &block.registers {
            let fieldset = format!("{module}::regs::{}", value_name(&register.name));
            let mut fields = Vec::new();
            for field in &register.fields {
                let enumm = if field.kind == "enum" {
                    for value in &field.values {
                        check_backend_ident(&value.name)?;
                    }
                    let name = format!(
                        "{module}::vals::{}{}",
                        value_name(&register.name),
                        value_name(&field.name)
                    );
                    ir.enums.insert(
                        name.clone(),
                        ir::Enum {
                            description: Some(format!(
                                "{} Source: {}",
                                field.description,
                                field
                                    .values_source
                                    .as_deref()
                                    .ok_or("missing enum evidence")?
                            )),
                            bit_size: field.bit_size.into(),
                            variants: field
                                .values
                                .iter()
                                .map(|value| ir::EnumVariant {
                                    name: value.name.clone(),
                                    description: None,
                                    value: value.value.into(),
                                })
                                .collect(),
                        },
                    );
                    Some(name)
                } else {
                    None
                };
                fields.push(ir::Field {
                    name: method(&field.name)?,
                    description: Some(field.description.clone()),
                    bit_offset: ir::BitOffset::Regular(field.bit_offset.into()),
                    bit_size: field.bit_size.into(),
                    array: array(field.array.as_ref())?,
                    enumm,
                });
            }
            // Never deduplicate fieldsets by layout: reset belongs to a register.
            ir.fieldsets.insert(
                fieldset.clone(),
                ir::FieldSet {
                    extends: None,
                    description: Some(register.description.clone()),
                    bit_size: register.bit_size.into(),
                    fields,
                },
            );
            items.push(ir::BlockItem {
                name: method(&register.name)?,
                description: Some(register.description.clone()),
                array: array(register.array.as_ref())?,
                byte_offset: register.offset.try_into()?,
                inner: ir::BlockItemInner::Register(ir::Register {
                    access: match register.access.as_str() {
                        "ro" => ir::Access::Read,
                        "wo" => ir::Access::Write,
                        "rw" => ir::Access::ReadWrite,
                        _ => return Err("invalid register access".into()),
                    },
                    bit_size: register.bit_size.into(),
                    fieldset: Some(fieldset),
                }),
            });
        }
        for child in &block.blocks {
            items.push(ir::BlockItem {
                name: method(&child.name)?,
                description: Some(child.description.clone()),
                array: array(child.array.as_ref())?,
                byte_offset: child.offset.try_into()?,
                inner: ir::BlockItemInner::Block(ir::BlockItemBlock {
                    block: format!(
                        "{}_{}::{}",
                        child.block,
                        child.version,
                        value_name(&child.block)
                    ),
                }),
            });
        }
        ir.blocks.insert(
            format!("{module}::{}", value_name(kind)),
            ir::Block {
                extends: None,
                description: None,
                items,
            },
        );
    }
    // CW32 validation already checks expanded byte/bit ranges and aliases.
    // chiptool's base-only checks reject legal narrow register aliases and
    // explicit field arrays whose first actual offset is nonzero.
    let errors = chiptool::validate::validate(
        &ir,
        chiptool::validate::Options {
            allow_register_overlap: true,
            allow_field_overlap: true,
            allow_enum_dup_value: false,
            allow_unused_enums: false,
            allow_unused_fieldsets: false,
        },
    );
    if !errors.is_empty() {
        return Err(format!("chiptool IR validation: {}", errors.join("; ")).into());
    }
    Ok(ir)
}

fn type_ident(ty: &syn::Type) -> Result<String> {
    let syn::Type::Path(path) = ty else {
        return Err("unexpected chiptool self type".into());
    };
    if path.qself.is_some() || path.path.segments.len() != 1 {
        return Err("unexpected chiptool qualified self type".into());
    }
    Ok(path.path.segments[0].ident.to_string())
}

fn specialize(items: &mut [Item], name: &str, parameters: &[(Ident, u64)]) -> Result<()> {
    if parameters.is_empty() {
        return Ok(());
    }
    let names = parameters.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let defaults = parameters
        .iter()
        .map(|(_, value)| value)
        .collect::<Vec<_>>();
    let ident = Ident::new(name, Span::call_site());
    let mut structures = 0;
    let mut implementations = 0;
    for item in items {
        match item {
            Item::Struct(item) if item.ident == name => {
                if !item.generics.params.is_empty() {
                    return Err("unexpected chiptool struct generics".into());
                }
                item.generics = parse_quote!(<#(const #names: u64 = #defaults),*>);
                structures += 1;
            }
            Item::Impl(item) if type_ident(&item.self_ty)? == name => {
                if !item.generics.params.is_empty() {
                    return Err("unexpected chiptool impl generics".into());
                }
                item.generics = parse_quote!(<#(const #names: u64),*>);
                item.self_ty = Box::new(parse_quote!(#ident<#(#names),*>));
                implementations += 1;
            }
            _ => {}
        }
    }
    if structures != 1 || implementations == 0 {
        return Err(format!("chiptool specialization shape changed for {name}").into());
    }
    Ok(())
}

struct SharedPaths<'a>(&'a BTreeMap<String, String>);
impl VisitMut for SharedPaths<'_> {
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        // At an IP module root, upstream links other IP modules with super::.
        // The chip root selects versioned files under the stable kind name.
        if path.segments.len() >= 2 && path.segments[0].ident == "super" {
            if let Some(kind) = self.0.get(&path.segments[1].ident.to_string()) {
                path.segments[1].ident = Ident::new(kind, Span::call_site());
            }
        }
        syn::visit_mut::visit_path_mut(self, path);
    }

    fn visit_expr_method_call_mut(&mut self, call: &mut syn::ExprMethodCall) {
        // Restore only generated Debug field labels, never arbitrary doc text.
        if call.method == "field" {
            if let Some(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(label),
                ..
            })) = call.args.first_mut()
            {
                if let Some(name) = label.value().strip_prefix(KEYWORD_PREFIX) {
                    *label = syn::LitStr::new(name, label.span());
                }
            }
        }
        syn::visit_mut::visit_expr_method_call_mut(self, call);
    }
}

/// Hardware descriptions are plain text, not Rust intra-doc links. Normalize
/// documentation attributes only; preserve strings used as metadata/evidence.
struct Documentation;
impl VisitMut for Documentation {
    fn visit_attribute_mut(&mut self, attribute: &mut syn::Attribute) {
        if attribute.path().is_ident("doc") {
            if let syn::Meta::NameValue(value) = &mut attribute.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(text),
                    ..
                }) = &mut value.value
                {
                    let escaped = chiptool::util::escape_brackets(&text.value());
                    let mut out = String::new();
                    let mut rest = escaped.as_str();
                    while let Some(start) = [rest.find("https://"), rest.find("http://")]
                        .into_iter()
                        .flatten()
                        .min()
                    {
                        out.push_str(&rest[..start]);
                        rest = &rest[start..];
                        let length = rest
                            .find(|c: char| c.is_whitespace() || c == '>')
                            .unwrap_or(rest.len());
                        let url = rest[..length].trim_end_matches(['.', ',', ';', ')']);
                        let already_linked = out.ends_with('<');
                        if !already_linked {
                            out.push('<');
                        }
                        out.push_str(url);
                        if !already_linked {
                            out.push('>');
                        }
                        rest = &rest[url.len()..];
                    }
                    out.push_str(rest);
                    *text = syn::LitStr::new(&out, text.span());
                }
            }
        }
    }
}

struct ElementReset<'a> {
    value: &'a syn::Type,
    values: Vec<u32>,
    calls: usize,
}
impl VisitMut for ElementReset<'_> {
    fn visit_expr_call_mut(&mut self, call: &mut syn::ExprCall) {
        syn::visit_mut::visit_expr_call_mut(self, call);
        if let syn::Expr::Path(path) = call.func.as_ref() {
            if path.path == parse_quote!(crate::common::Reg::from_ptr) && call.args.len() == 1 {
                let value = self.value;
                let values = self
                    .values
                    .iter()
                    .map(|&value| proc_macro2::Literal::u32_unsuffixed(value))
                    .collect::<Vec<_>>();
                call.func = Box::new(parse_quote!(crate::common::Reg::from_ptr_with_reset));
                call.args
                    .push(parse_quote!(<#value>::from_bits([#(#values),*][n])));
                self.calls += 1;
            }
        }
    }
}

fn extend_block(
    items: &mut [Item],
    block: &schema::Block,
    overrides: &[schema::RegisterReset],
) -> Result<()> {
    let parameters = instance_reset_registers(overrides);
    let block_name = value_name(&block.name);
    let generics = parameters
        .iter()
        .map(|name| {
            let r = block.registers.iter().find(|r| &r.name == name).unwrap();
            (
                format_ident!("RESET_{name}"),
                r.reset_value.map(u64::from).unwrap_or(UNKNOWN_RESET),
            )
        })
        .collect::<Vec<_>>();
    specialize(items, &block_name, &generics)?;
    let mut methods = BTreeSet::new();
    for item in items {
        let Item::Impl(item) = item else {
            continue;
        };
        if item.trait_.is_some() || type_ident(&item.self_ty)? != block_name {
            continue;
        }
        for item in &mut item.items {
            let syn::ImplItem::Fn(method) = item else {
                return Err("unexpected chiptool block item".into());
            };
            let name = method.sig.ident.to_string();
            let name = name.strip_prefix("r#").unwrap_or(&name);
            if name == "from_ptr" {
                method.attrs.push(parse_quote!(#[doc = "# Safety\nThe pointer and instance reset parameters must identify this mapped register block. Respect hardware clocks, power, ownership and access side effects."]));
            }
            let Some(register) = block
                .registers
                .iter()
                .find(|r| r.name.to_ascii_lowercase() == name)
            else {
                continue;
            };
            if !methods.insert(register.name.clone()) {
                return Err("duplicate chiptool accessor".into());
            }
            let value_name = Ident::new(&value_name(&register.name), Span::call_site());
            let original: syn::ReturnType = match register.access.as_str() {
                "ro" => parse_quote!(-> crate::common::Reg<regs::#value_name, crate::common::R>),
                "wo" => parse_quote!(-> crate::common::Reg<regs::#value_name, crate::common::W>),
                _ => parse_quote!(-> crate::common::Reg<regs::#value_name, crate::common::RW>),
            };
            // Exact structural check prevents a backend upgrade silently losing policy.
            if method.sig.output != original {
                return Err(format!(
                    "chiptool accessor signature changed: {}.{}",
                    block.name, register.name
                )
                .into());
            }
            let value: syn::Type = if parameters.contains(&register.name) {
                let parameter = format_ident!("RESET_{}", register.name);
                parse_quote!(regs::#value_name<#parameter>)
            } else {
                parse_quote!(regs::#value_name)
            };
            let access = format_ident!(
                "{}",
                match register.access.as_str() {
                    "ro" => "R",
                    "wo" => "W",
                    _ => "RW",
                }
            );
            let write = Ident::new(&format!("{:?}", register.write_behavior), Span::call_site());
            let read = Ident::new(read_marker(register.read_behavior), Span::call_site());
            let varying = register.array.is_some()
                && register.reset_value.is_none()
                && register.elements.iter().all(|e| e.reset_value.is_some());
            method.sig.output = if varying {
                let mut patch = ElementReset {
                    value: &value,
                    values: register
                        .elements
                        .iter()
                        .map(|e| e.reset_value.unwrap())
                        .collect(),
                    calls: 0,
                };
                patch.visit_block_mut(&mut method.block);
                if patch.calls != 1 {
                    return Err("chiptool register constructor shape changed".into());
                }
                parse_quote!(-> crate::common::Reg<#value, crate::common::#access, crate::common::#write, crate::common::#read, crate::common::GivenReset<#value>>)
            } else {
                parse_quote!(-> crate::common::Reg<#value, crate::common::#access, crate::common::#write, crate::common::#read>)
            };
        }
    }
    if methods.len() != block.registers.len() {
        return Err("chiptool omitted register accessors".into());
    }
    Ok(())
}

fn extend_fieldsets(
    items: &mut Vec<Item>,
    block: &schema::Block,
    overrides: &[schema::RegisterReset],
) -> Result<()> {
    // Upstream defaults every fieldset to zero. Remove *all* such impls first,
    // requiring one per register, then add only CW32's evidence-backed defaults.
    let mut removed = BTreeSet::new();
    let mut kept = Vec::new();
    for item in std::mem::take(items) {
        if let Item::Impl(implementation) = &item {
            if implementation
                .trait_
                .as_ref()
                .is_some_and(|(_, path, _)| path.is_ident("Default"))
            {
                if !removed.insert(type_ident(&implementation.self_ty)?) {
                    return Err("duplicate upstream Default".into());
                }
                continue;
            }
        }
        kept.push(item);
    }
    *items = kept;
    let expected = block
        .registers
        .iter()
        .map(|r| value_name(&r.name))
        .collect::<BTreeSet<_>>();
    if removed != expected {
        return Err("chiptool fieldset Default shape changed".into());
    }
    let mut extensions = String::new();
    for register in &block.registers {
        let name = value_name(&register.name);
        let resets = overrides
            .iter()
            .filter(|r| r.register == register.name)
            .collect::<Vec<_>>();
        if !resets.is_empty() {
            specialize(
                items,
                &name,
                &[(
                    format_ident!("RESET"),
                    register.reset_value.map(u64::from).unwrap_or(UNKNOWN_RESET),
                )],
            )?;
        }
        // chiptool interprets every one-bit non-enum as bool. Keep explicitly
        // raw one-bit CW32 fields as u8 without touching its mask/offset logic.
        for item in items.iter_mut() {
            let Item::Impl(item) = item else {
                continue;
            };
            if item.trait_.is_some() || type_ident(&item.self_ty)? != name {
                continue;
            }
            for field in register
                .fields
                .iter()
                .filter(|f| f.kind == "raw" && f.bit_size == 1)
            {
                let getter = super::rust_ident(&field.name.to_ascii_lowercase());
                let setter = format!("set_{}", field.name.to_ascii_lowercase());
                let mut found = 0;
                for item in &mut item.items {
                    let syn::ImplItem::Fn(method) = item else {
                        continue;
                    };
                    if method.sig.ident == getter {
                        method.sig.output = parse_quote!(-> u8);
                        let Some(syn::Stmt::Expr(expr, None)) = method.block.stmts.last_mut()
                        else {
                            return Err("chiptool raw-bit getter shape changed".into());
                        };
                        if *expr != parse_quote!(val != 0) {
                            return Err("chiptool raw-bit conversion changed".into());
                        }
                        *expr = parse_quote!(val as u8);
                        found += 1;
                    } else if method.sig.ident == setter {
                        let Some(syn::FnArg::Typed(argument)) = method.sig.inputs.last_mut() else {
                            return Err("chiptool raw-bit setter shape changed".into());
                        };
                        argument.ty = Box::new(parse_quote!(u8));
                        found += 1;
                    }
                }
                if found != 2 {
                    return Err("chiptool omitted raw-bit field accessors".into());
                }
            }
        }
        super::render_reset_extensions(&mut extensions, register, &resets);
    }
    items.extend(syn::parse_file(&extensions)?.items);
    Ok(())
}

pub(super) fn render_blocks(
    blocks: &Blocks,
    overrides: &ResetOverrides,
) -> Result<BTreeMap<(String, String), String>> {
    let ir = lower(blocks)?;
    let options = generate::Options::new()
        .with_common_module(generate::CommonModule::External(quote!(crate::common)))
        .with_skip_no_std(true)
        .with_defmt(generate::DefmtOption::Disabled);
    let tokens = generate::render(&ir, &options)?;
    let mut file = syn::parse2::<syn::File>(restore_keywords(tokens))?;
    let paths = blocks
        .keys()
        .map(|(kind, version)| (format!("{kind}_{version}"), kind.clone()))
        .collect();
    SharedPaths(&paths).visit_file_mut(&mut file);
    let mut modules = BTreeMap::new();
    for item in file.items {
        let Item::Mod(module) = item else {
            return Err("unexpected chiptool root item".into());
        };
        let (_, contents) = module.content.ok_or("missing chiptool module body")?;
        modules.insert(module.ident.to_string(), contents);
    }
    let mut output = BTreeMap::new();
    for (key, block) in blocks {
        let mut items = modules
            .remove(&format!("{}_{}", key.0, key.1))
            .ok_or("missing chiptool IP module")?;
        extend_block(&mut items, block, &overrides[key])?;
        let mut fieldsets = 0;
        for item in &mut items {
            if let Item::Mod(module) = item {
                if module.ident == "regs" {
                    extend_fieldsets(
                        &mut module
                            .content
                            .as_mut()
                            .ok_or("missing chiptool regs body")?
                            .1,
                        block,
                        &overrides[key],
                    )?;
                    fieldsets += 1;
                }
            }
        }
        if fieldsets != usize::from(!block.registers.is_empty()) {
            return Err("chiptool regs module shape changed".into());
        }
        for register in &block.registers {
            let name = Ident::new(&register.name, Span::call_site());
            let value = register.offset;
            let doc = &register.description;
            items.push(parse_quote!(#[doc = #doc] pub const #name: usize = #value;));
        }
        for constant in &block.constants {
            let name = Ident::new(&constant.name, Span::call_site());
            let value = constant.value;
            let doc = &constant.description;
            items.push(parse_quote!(#[doc = #doc] pub const #name: u32 = #value;));
        }
        let mut file = syn::File {
            shebang: None,
            attrs: vec![parse_quote!(#![allow(non_camel_case_types)])],
            items,
        };
        Documentation.visit_file_mut(&mut file);
        output.insert(key.clone(), format!("// Generated by cw32-gen using chiptool be1bff3e9e1b27b090e69bd9ac753c66fdcce678.\n// CW32 audited reset/side-effect extensions; edit YAML, not this file.\n{}", prettyplease::unparse(&file)));
    }
    if !modules.is_empty() {
        return Err("unexpected extra chiptool IP module".into());
    }
    Ok(output)
}
