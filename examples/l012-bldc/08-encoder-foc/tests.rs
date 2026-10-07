//! 主机上直接测试同一份无硬件控制代码：rustc --edition=2021 --test tests.rs。
#![allow(dead_code)]
#[path = "src/arithmetic.rs"]
mod arithmetic;
#[path = "src/config.rs"]
mod config;
#[path = "src/control.rs"]
mod control;
#[path = "src/sampling.rs"]
mod sampling;

#[path = "src/button.rs"]
mod button;
