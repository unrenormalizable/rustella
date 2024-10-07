use crate::{cmn, cpu, riot, tia};
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};

pub struct NtscAtari {
    cpu: Rc<RefCell<cpu::NMOS6502>>,
    mem: riot::Memory,
    tia: Rc<RefCell<dyn tia::TIA>>,
    pia: Rc<RefCell<dyn riot::PIA6532>>,
}

impl NtscAtari {
    pub fn new(
        tv: Rc<RefCell<dyn tia::TV<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>>>,
    ) -> Self {
        let rdy = Rc::new(Cell::new(cmn::LineState::High));
        let tia = Rc::new(RefCell::new(tia::NtscTIA::new(rdy.clone(), tv.clone())));
        let pia = Rc::new(RefCell::new(riot::InMemory6532::default()));
        let mem = riot::Memory::new_with_rom(
            &[],
            cmn::LoHi(0x00, 0x00),
            riot::mm_6507,
            Some(tia.clone()),
            Some(pia.clone()),
            true,
        );
        let cpu = Rc::new(RefCell::new(cpu::NMOS6502::new(rdy.clone(), &mem)));

        Self { cpu, mem, tia, pia }
    }

    pub fn load_rom(&mut self, addr: u16, data: &[u8]) {
        self.mem.load(data, addr.into());
        self.cpu.borrow_mut().reset_pc(&self.mem);
    }

    pub fn tick(&mut self, cycles: usize) {
        for i in 0..cycles {
            if i % 3 == 0 {
                self.pia.borrow_mut().tick(1);
                self.cpu.borrow_mut().tick(&mut self.mem);
            }
            self.tia.borrow_mut().tick(1);
        }
    }

    pub fn run_for(&mut self, instructions: u64) {
        loop {
            self.tick(1);
            if self.cpu_state().instructions() == instructions {
                break;
            }
        }
    }

    pub fn cpu_state(&self) -> cpu::NMOS6502 {
        self.cpu.borrow().clone()
    }
}
