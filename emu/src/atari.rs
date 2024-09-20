use crate::{cmn, cpu, riot, tia};
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};

pub struct NtscAtari {
    cpu: Rc<RefCell<cpu::MOS6502>>,
    mem: riot::Memory,
    tia: Rc<RefCell<dyn tia::TIA>>,
}

impl NtscAtari {
    pub fn new(
        tv: Rc<RefCell<dyn tia::TV<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>>>,
    ) -> Self {
        let rdy = Rc::new(Cell::new(cmn::LineState::Low));
        let tia = Rc::new(RefCell::new(tia::NtscTIA::new(rdy.clone(), tv.clone())));
        let mem = riot::Memory::new_with_rom(
            &[],
            cmn::LoHi(0x00, 0x00),
            riot::mm_6507,
            Some(tia.clone()),
            true,
        );
        let cpu = Rc::new(RefCell::new(cpu::MOS6502::new(rdy.clone(), &mem)));

        Self { cpu, mem, tia }
    }

    pub fn load_rom(&mut self, addr: u16, data: &[u8]) {
        self.mem.load(data, addr.into());
        self.cpu.borrow_mut().reset_pc(&self.mem);
    }

    pub fn tick(&mut self, loops: usize) {
        for _ in 0..loops {
            let cycles = self.cpu.borrow_mut().tick(&mut self.mem);
            let cycles = if cycles == 0 { 1 } else { cycles };
            for _ in 0..(cycles * 3) {
                self.tia.borrow_mut().tick();
            }
        }
    }

    pub fn cpu_state(&self) -> cpu::MOS6502 {
        self.cpu.borrow().clone()
    }
}
