use core::arch::asm;
use crate::{StackInfo, UnwindIf};

#[repr(C)]
struct StackFrame {
    fp: usize,
    ra: usize,
}

pub struct Unwind {
    init_curr: bool,
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

    // Assume Riscv not open -no-omit-frame-pointer
    fn unwind_next(&mut self) -> bool {
        unsafe {
            let stack: *const StackFrame = 
                (self.fp as *const StackFrame).sub(1);
            self.fp = (*stack).fp;

            if self.is_fp_invalid() {
                return false;
            }

            // go to next ra
            let stack: *const StackFrame = 
                (self.fp as *const StackFrame).sub(1);
            self.pc = (*stack).ra;   
        }

        true
    }
}


#[inline(always)]
fn current_unwind() -> (usize, usize) {
    unsafe {
        let mut fp: usize;
        asm!("addi {ptr}, s0, 0", ptr = out(reg) fp);
        let stack: *const StackFrame = (fp as *const StackFrame).sub(1);
        ((*stack).ra, fp)
    }
}

impl UnwindIf for Unwind {
    fn new(pc:usize, fp: usize, stack_info: StackInfo, 
        text:KtextAddress) 
        -> Self {
        Unwind{init_curr: false, pc, fp, stack_info}
    }

    fn new_from_cur_ctx(stack_info: StackInfo) 
        -> Self {
        Unwind{init_curr: true, pc: 0, fp: 0, stack_info}
    }

    #[inline(always)]
    fn unwind(&mut self) {
        if self.init_curr {
            let (pc, fp) = current_unwind();
            self.pc = pc;
            self.fp = fp;
        }

        if self.is_fp_invalid() {
            error!("unwind init sp: {:#016x} is invalid",self.fp);
            return;
        }

        loop {
            debug!("pc: {:#016X}  sp={:#016X}", self.pc, self.fp);
            error!("{:#016X} ", self.pc);
            if !self.unwind_next() {
                break;
            }
        }
    }
}
