use core::arch::asm;

pub(crate) fn current_unwind() -> (usize, usize) {
    unsafe {
        let mut pc: usize;
        asm!("lw {ptr}, 0(sp)", ptr = out(reg) pc);
        let mut fp: usize;
        asm!("mov {ptr}, fp", ptr = out(reg) fp);
        (pc, fp)
    }
}
