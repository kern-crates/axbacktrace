use core::arch::asm;
use crate::{StackInfo, UnwindIf};

pub struct Unwind {
    pc: usize,
    fp: usize,
    stack_info: StackInfo,
}

impl Unwind {
    fn is_fp_invalid(&self) -> bool {
        // Final frame; nothing to unwind
        if !self.stack_info.contains(self.fp) {
            return true;
        }

        // not align
        if self.fp & 0x7 != 0 {
            return true;
        }

        false
    }

    fn unwind_next(&mut self) -> bool {
        unsafe {
            self.fp = *(self.fp as *const usize);
            if self.is_fp_invalid() {
                return false;
            }
            self.pc = *((self.fp + 8) as *const usize);
        }

        true
    }
}


#[inline(always)]
pub(crate) fn current_unwind() -> (usize, usize) {
    unsafe {
        let mut pc: usize;
        asm!("ldr {ptr}, [x29, #8]", ptr = out(reg) pc);
        let mut fp: usize;
        asm!("mov {ptr}, x29", ptr = out(reg) fp);
        (pc, fp)
    }
}

impl UnwindIf for Unwind {
    #[inline(always)]
    fn new_from_cur_ctx(stack_info: StackInfo) -> Self {
        let (pc, fp) = current_unwind();
        Self::new(pc, fp, stack_info)
    }

    fn new(pc:usize, fp:usize, stack_info: StackInfo) -> Self {
        Unwind{pc,fp,stack_info}
    }

    fn unwind(&mut self) {
        if self.is_fp_invalid() {
            error!("unwind init fp: {:#016x} is invalid",self.fp);
            return;
        }

        loop {
            debug!("pc: {:#016X}  fp={:#016X}", self.pc, self.fp);
            error!("{:#016X} ", self.pc);
            if !self.unwind_next() {
                break;
            }
        }
    }
}
