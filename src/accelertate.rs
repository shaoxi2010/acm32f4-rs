use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::{dsb, isb};

struct Reg {
    address: usize,
}

impl Reg {
    fn new(address: usize) -> Self {
        Reg { address }
    }
    fn write(&self, v: u32) {
        unsafe { write_volatile(self.address as *mut u32, v) }
    }
    fn read(&self) -> u32 {
        unsafe { read_volatile(self.address as *const u32) }
    }
    fn modify(&self, mask: u32) {
        let tmp = self.read();
        self.write(tmp | mask)
    }
    fn modify_uset(&self, mask: u32) {
        let tmp = self.read();
        self.write(tmp & (!mask))
    }
}

pub fn enable_icache() {
    let l1_icache = Reg::new(0xe000_ef50);
    let l1_ctrl = Reg::new(0xe000_ed14);
    dsb();
    isb();
    l1_icache.write(0);
    dsb();
    isb();
    l1_ctrl.modify(0x20000);
    dsb();
    isb();
}

pub fn disable_icache() {
    let l1_icache = Reg::new(0xe000_ef50);
    let l1_ctrl = Reg::new(0xe000_ed14);
    dsb();
    isb();
    l1_ctrl.modify_uset(0x20000);
    dsb();
    isb();
    l1_icache.write(0);
    dsb();
    isb();
}
