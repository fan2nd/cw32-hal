#![deny(unsafe_code)]
//! 显式传入算术后端；板上由 ADC1 独占 EAU，主机检查使用同一整数定义。
//! 除法向零截断，余数与被除数同号；平方根向下取整，不近似控制算法。
pub trait Arithmetic {
    fn failed(&self) -> bool {
        false
    }
    fn div(&mut self, numerator: i32, denominator: i32) -> i32;
    fn div_rem(&mut self, numerator: i32, denominator: i32) -> (i32, i32);
    fn sqrt(&mut self, value: u32) -> u32;
    fn rem_unsigned(&mut self, n: u32, d: u32) -> u32;
}

#[cfg(not(target_arch = "arm"))]
pub struct Software;
#[cfg(not(target_arch = "arm"))]
impl Arithmetic for Software {
    fn div(&mut self, n: i32, d: i32) -> i32 {
        n / d
    }
    fn div_rem(&mut self, n: i32, d: i32) -> (i32, i32) {
        (n / d, n % d)
    }
    fn rem_unsigned(&mut self, n: u32, d: u32) -> u32 {
        n % d
    }
    fn sqrt(&mut self, mut n: u32) -> u32 {
        let mut root = 0;
        let mut bit = 1 << 30;
        while bit > n {
            bit >>= 2;
        }
        while bit != 0 {
            if n >= root + bit {
                n -= root + bit;
                root = (root >> 1) + bit;
            } else {
                root >>= 1;
            }
            bit >>= 2;
        }
        root
    }
}
