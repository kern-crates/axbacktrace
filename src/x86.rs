use core::arch::asm;

pub(crate) fn current_unwind() -> (usize, usize) {
    unsafe {
        let mut pc: usize;
        asm!("mov 8(%rbp), {ptr}", ptr = out(reg) pc);
        let mut fp: usize;
        asm!("mov %rbp, {ptr}", ptr = out(reg) fp);
        (pc, fp)
    }
}
