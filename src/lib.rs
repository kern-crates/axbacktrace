#![no_std]

#[macro_use]
extern crate log;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
        pub struct Test;
    } else if #[cfg(target_arch = "riscv64")] {
        mod riscv;
        pub use self::riscv::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    }
}

pub struct StackInfo {
    low: usize,
    high: usize,
}

impl StackInfo {
    pub fn new(low: usize, high: usize) -> Self {
        Self {low, high}
    }

    pub(crate) fn contains(&self, fp: usize) -> bool {
         fp < self.high && fp > self.low
    }
}

pub trait UnwindIf {
    fn new_from_cur_ctx(stack_info: StackInfo) -> Self;
    fn new(pc:usize, fp:usize, stack_info: StackInfo) -> Self;
    fn unwind(&mut self);
}

pub fn dump_backtrace(unwind: &mut Unwind) {
    error!("Call trace: ");
    unwind.unwind();
}
