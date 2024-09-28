#![allow(non_snake_case)]

use crate::bits;
use crate::cmn::LoHi;
use crate::cpu::{am, cmn::IRQ_VECTOR, core::*, opc_info};
use crate::riot::Memory;

fn illegal(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let opc = mem.get(cpu.pc(), 0);
    let opc_info = &opc_info::ALL[opc as usize];
    unimplemented!(
        "Illegal opcode {opc:02X} ({}). CPU state: {cpu:?}",
        opc_info.assembler
    )
}

/// The break instruction (BRK) behaves like a NMI, but will push the value of PC+2 onto the stack to be used as the return address.
/// It will also set the I flag. See http://6502.org/tutorials/interrupts.html#2.2.
/// 0x00 | impl | BRK
fn BRK_impl(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    stack::push_interrupt_call_stack(cpu, mem, cpu.pc() + 2);
    cpu.set_psr_bit(PSR::I);

    let pc_lo = mem.get(IRQ_VECTOR, 0);
    let pc_hi = mem.get(IRQ_VECTOR, 1);

    Some(LoHi(pc_lo, pc_hi))
}

/// 0x01 | (ind,X) | ORA (oper,X)
fn ORA_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::pre_indexed_indirect::load(mem, pc, cpu.x());

    ORA_core(cpu, v2);

    None
}

/// 0x05 | zpg | ORA oper
fn ORA_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::zero_page::load(mem, pc);

    ORA_core(cpu, v2);

    None
}

/// 0x06 | zpg | ASL oper
fn ASL_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::zero_page::load(mem, pc);
    let new_v = old_v << 1;
    am::zero_page::store(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x08 | impl | PHP
fn PHP_impl(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    stack::push_psr(cpu, mem);

    None
}

/// 0x09 | # | ORA #oper
fn ORA_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::immediate::load(mem, pc);

    ORA_core(cpu, v2);

    None
}

/// 0x0A | A | ASL A
fn ASL_A(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let old_v = cpu.a();
    let new_v = old_v << 1;
    cpu.set_a(new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x0D | abs | ORA oper
fn ORA_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::absolute::load(mem, pc);

    ORA_core(cpu, v2);

    None
}

/// 0x0E | abs | ASL oper
fn ASL_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::absolute::load(mem, pc);
    let new_v = old_v << 1;
    am::absolute::store(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x15 | zpg,X | ORA oper,X
fn ORA_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_zero_page::load(mem, pc, cpu.x());

    ORA_core(cpu, v2);

    None
}

/// 0x16 | zpg,X | ASL oper,X
fn ASL_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::indexed_zero_page::load(mem, pc, cpu.x());
    let new_v = old_v << 1;
    am::indexed_zero_page::store(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x18 | impl | CLC
fn CLC_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    cpu.clr_psr_bit(PSR::C);

    None
}

/// 0x19 | abs,Y | ORA oper,Y
fn ORA_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_absolute::load(mem, pc, cpu.y());

    ORA_core(cpu, v2);

    None
}

/// 0x1D | abs,X | ORA oper,X
fn ORA_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_absolute::load(mem, pc, cpu.x());

    ORA_core(cpu, v2);

    None
}

/// 0x1E | abs,X | ASL oper,X
fn ASL_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::indexed_absolute::load(mem, pc, cpu.x());
    let new_v = old_v << 1;
    am::indexed_absolute::store(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x20 | abs | JSR oper
fn JSR_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let ret_addr = pc + 2;
    stack::push(cpu, mem, ret_addr.1);
    stack::push(cpu, mem, ret_addr.0);

    Some(am::absolute::load_lohi(mem, pc))
}

/// 0x21 | (ind,X) | AND (oper,X)
fn AND_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::pre_indexed_indirect::load(mem, pc, cpu.x());

    AND_core(cpu, v2);

    None
}

/// 0x24 | zpg | BIT oper
fn BIT_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::zero_page::load(mem, pc);

    adder::bit_core(cpu, v2);

    None
}

/// 0x25 | zpg | AND oper
fn AND_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::zero_page::load(mem, pc);

    AND_core(cpu, v2);

    None
}

/// 0x26 | zpg | ROL oper
fn ROL_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::zero_page::load(mem, pc);
    let new_v = adder::rol_core(cpu, old_v);
    am::zero_page::store(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x28 | impl | PLP
fn PLP_impl(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    stack::pop_psr(cpu, mem);

    None
}

/// 0x29 | # | AND #oper
fn AND_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::immediate::load(mem, pc);

    AND_core(cpu, v2);

    None
}

/// 0x2A | A | ROL A
fn ROL_A(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let old_v = cpu.a();
    let new_v = adder::rol_core(cpu, old_v);
    cpu.set_a(new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x2C | abs | BIT oper
fn BIT_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::absolute::load(mem, pc);

    adder::bit_core(cpu, v2);

    None
}

/// 0x2D | abs | AND oper
fn AND_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::absolute::load(mem, pc);

    AND_core(cpu, v2);

    None
}

/// 0x2E | abs | ROL oper
fn ROL_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::absolute::load(mem, pc);
    let new_v = adder::rol_core(cpu, old_v);
    am::absolute::store(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x35 | zpg,X | AND oper,X
fn AND_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_zero_page::load(mem, pc, cpu.x());

    AND_core(cpu, v2);

    None
}

/// 0x36 | zpg,X | ROL oper,X
fn ROL_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::indexed_zero_page::load(mem, pc, cpu.x());
    let new_v = adder::rol_core(cpu, old_v);
    am::indexed_zero_page::store(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x38 | impl | SEC
fn SEC_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    cpu.set_psr_bit(PSR::C);

    None
}

/// 0x39 | abs,Y | AND oper,Y
fn AND_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_absolute::load(mem, pc, cpu.y());

    AND_core(cpu, v2);

    None
}

/// 0x3D | abs,X | AND oper,X
fn AND_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_absolute::load(mem, pc, cpu.x());

    AND_core(cpu, v2);

    None
}

/// 0x3E | abs,X | ROL oper,X
fn ROL_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::indexed_absolute::load(mem, pc, cpu.x());
    let new_v = adder::rol_core(cpu, old_v);
    am::indexed_absolute::store(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x40 | impl | RTI
fn RTI_impl(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let ret_addr = stack::pop_interrupt_call_stack(cpu, mem);

    Some(ret_addr)
}

/// 0x41 | (ind,X) | EOR (oper,X)
fn EOR_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::pre_indexed_indirect::load(mem, pc, cpu.x());

    EOR_core(cpu, v2);

    None
}

/// 0x45 | zpg | EOR oper
fn EOR_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::zero_page::load(mem, pc);

    EOR_core(cpu, v2);

    None
}

/// 0x46 | zpg | LSR oper
fn LSR_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::zero_page::load(mem, pc);
    let new_v = old_v >> 1;
    am::zero_page::store(mem, pc, new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x48 | impl | PHA
fn PHA_impl(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let a = cpu.a();
    stack::push(cpu, mem, a);

    None
}

/// 0x49 | # | EOR #oper
fn EOR_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::immediate::load(mem, pc);

    EOR_core(cpu, v2);

    None
}

/// 0x4A | A | LSR A
fn LSR_A(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let old_v = cpu.a();
    let new_v = old_v >> 1;
    cpu.set_a(new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x4C | abs | JMP oper
fn JMP_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let pc = am::absolute::load_lohi(mem, pc);

    Some(pc)
}

/// 0x4D | abs | EOR oper
fn EOR_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::absolute::load(mem, pc);

    EOR_core(cpu, v2);

    None
}

/// 0x4E | abs | LSR oper
fn LSR_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::absolute::load(mem, pc);
    let new_v = old_v >> 1;
    am::absolute::store(mem, pc, new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x55 | zpg,X | EOR oper,X
fn EOR_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_zero_page::load(mem, pc, cpu.x());

    EOR_core(cpu, v2);

    None
}

/// 0x56 | zpg,X | LSR oper,X
fn LSR_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::indexed_zero_page::load(mem, pc, cpu.x());
    let new_v = old_v >> 1;
    am::indexed_zero_page::store(mem, pc, cpu.x(), new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x58 | impl | CLI
fn CLI_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    cpu.clr_psr_bit(PSR::I);

    None
}

/// 0x59 | abs,Y | EOR oper,Y
fn EOR_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_absolute::load(mem, pc, cpu.y());

    EOR_core(cpu, v2);

    None
}

/// 0x5D | abs,X | EOR oper,X
fn EOR_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let v2 = am::indexed_absolute::load(mem, pc, cpu.x());

    EOR_core(cpu, v2);

    None
}

/// 0x5E | abs,X | LSR oper,X
fn LSR_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::indexed_absolute::load(mem, pc, cpu.x());
    let new_v = old_v >> 1;
    am::indexed_absolute::store(mem, pc, cpu.x(), new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x60 | impl | RTS
fn RTS_impl(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc_lo = stack::pop(cpu, mem);
    let pc_hi = stack::pop(cpu, mem);

    let pc = LoHi::from((pc_lo, pc_hi)) + 1;

    Some(pc)
}

/// 0x61 | (ind,X) | ADC (oper,X)
fn ADC_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::pre_indexed_indirect::load(mem, pc, cpu.x());

    adder::ADC_core(cpu, n2);

    None
}

/// 0x65 | zpg | ADC oper
fn ADC_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::zero_page::load(mem, pc);

    adder::ADC_core(cpu, n2);

    None
}

/// 0x66 | zpg | ROR oper
fn ROR_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::zero_page::load(mem, pc);
    let new_v = adder::ror_core(cpu, old_v);
    am::zero_page::store(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x68 | impl | PLA
fn PLA_impl(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let val = stack::pop(cpu, mem);
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0x69 | # | ADC #oper
fn ADC_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::immediate::load(mem, pc);

    adder::ADC_core(cpu, n2);

    None
}

/// 0x6A | A | ROR A
fn ROR_A(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let old_v = cpu.a();
    let new_v = adder::ror_core(cpu, old_v);
    cpu.set_a(new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x6C | ind | JMP (oper)
fn JMP_ind(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let addr = am::indirect::load(mem, pc);

    Some(addr)
}

/// 0x6D | abs | ADC oper
fn ADC_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::absolute::load(mem, pc);

    adder::ADC_core(cpu, n2);

    None
}

/// 0x6E | abs | ROR oper
fn ROR_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::absolute::load(mem, pc);
    let new_v = adder::ror_core(cpu, old_v);
    am::absolute::store(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x75 | zpg,X | ADC oper,X
fn ADC_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::indexed_zero_page::load(mem, pc, cpu.x());

    adder::ADC_core(cpu, n2);

    None
}

/// 0x76 | zpg,X | ROR oper,X
fn ROR_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::indexed_zero_page::load(mem, pc, cpu.x());
    let new_v = adder::ror_core(cpu, old_v);
    am::indexed_zero_page::store(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x78 | impl | SEI
fn SEI_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    cpu.set_psr_bit(PSR::I);

    None
}

/// 0x79 | abs,Y | ADC oper,Y
fn ADC_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::indexed_absolute::load(mem, pc, cpu.y());

    adder::ADC_core(cpu, n2);

    None
}

/// 0x7D | abs,X | ADC oper,X
fn ADC_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::indexed_absolute::load(mem, pc, cpu.x());

    adder::ADC_core(cpu, n2);

    None
}

/// 0x7E | abs,X | ROR oper,X
fn ROR_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let old_v = am::indexed_absolute::load(mem, pc, cpu.x());
    let new_v = adder::ror_core(cpu, old_v);
    am::indexed_absolute::store(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x81 | (ind,X) | STA (oper,X)
fn STA_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::pre_indexed_indirect::store(mem, pc, cpu.x(), cpu.a());

    None
}

/// 0x84 | zpg | STY oper
fn STY_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::zero_page::store(mem, pc, cpu.y());

    None
}

/// 0x85 | zpg | STA oper
fn STA_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::zero_page::store(mem, pc, cpu.a());

    None
}

/// 0x86 | zpg | STX oper
fn STX_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::zero_page::store(mem, pc, cpu.x());

    None
}

/// 0x88 | impl | DEY
fn DEY_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let val = cpu.y().wrapping_sub(1);
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0x8A | impl | TXA
fn TXA_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let x = cpu.x();
    cpu.set_a(x);

    pcr::sync_pcr_n(cpu, x);
    pcr::sync_pcr_z(cpu, x);

    None
}

/// 0x8C | abs | STY oper
fn STY_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::absolute::store(mem, pc, cpu.y());

    None
}

/// 0x8D | abs | STA oper
fn STA_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::absolute::store(mem, pc, cpu.a());

    None
}

/// 0x8E | abs | STX oper
fn STX_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::absolute::store(mem, pc, cpu.x());

    None
}

/// 0x94 | zpg,X | STY oper,X
fn STY_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::indexed_zero_page::store(mem, pc, cpu.x(), cpu.y());

    None
}

/// 0x95 | zpg,X | STA oper,X
fn STA_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::indexed_zero_page::store(mem, pc, cpu.x(), cpu.a());

    None
}

/// 0x96 | zpg,Y | STX oper,Y
fn STX_zpg_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::indexed_zero_page::store(mem, pc, cpu.y(), cpu.x());

    None
}

/// 0x98 | impl | TYA
fn TYA_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let y = cpu.y();
    cpu.set_a(y);

    pcr::sync_pcr_n(cpu, y);
    pcr::sync_pcr_z(cpu, y);

    None
}

/// 0x99 | abs,Y | STA oper,Y
fn STA_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::indexed_absolute::store(mem, pc, cpu.y(), cpu.a());

    None
}

/// 0x9A | impl | TXS
fn TXS_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let x = cpu.x();
    cpu.set_s(x);

    None
}

/// 0x9D | abs,X | STA oper,X
fn STA_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    am::indexed_absolute::store(mem, pc, cpu.x(), cpu.a());

    None
}

/// 0xA0 | # | LDY #oper
fn LDY_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::immediate::load(mem, pc);
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA1 | (ind,X) | LDA (oper,X)
fn LDA_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::pre_indexed_indirect::load(mem, pc, cpu.x());
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA2 | # | LDX #oper
fn LDX_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::immediate::load(mem, pc);
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA4 | zpg | LDY oper
fn LDY_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::zero_page::load(mem, pc);
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA5 | zpg | LDA oper
fn LDA_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::zero_page::load(mem, pc);
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA6 | zpg | LDX oper
fn LDX_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::zero_page::load(mem, pc);
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA8 | impl | TAY
fn TAY_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let a = cpu.a();
    cpu.set_y(a);

    pcr::sync_pcr_n(cpu, a);
    pcr::sync_pcr_z(cpu, a);

    None
}

/// 0xA9 | # | LDA #oper
fn LDA_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::immediate::load(mem, pc);
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xAA | impl | TAX
fn TAX_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let a = cpu.a();
    cpu.set_x(a);

    pcr::sync_pcr_n(cpu, a);
    pcr::sync_pcr_z(cpu, a);

    None
}

/// 0xAC | abs | LDY oper
fn LDY_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::absolute::load(mem, pc);

    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xAD | abs | LDA oper
fn LDA_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::absolute::load(mem, pc);

    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xAE | abs | LDX oper
fn LDX_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::absolute::load(mem, pc);

    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB4 | zpg,X | LDY oper,X
fn LDY_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_zero_page::load(mem, pc, cpu.x());
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB5 | zpg,X | LDA oper,X
fn LDA_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_zero_page::load(mem, pc, cpu.x());
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB6 | zpg,Y | LDX oper,Y
fn LDX_zpg_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_zero_page::load(mem, pc, cpu.y());
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB8 | impl | CLV
fn CLV_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    cpu.clr_psr_bit(PSR::V);

    None
}

/// 0xB9 | abs,Y | LDA oper,Y
fn LDA_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_absolute::load(mem, pc, cpu.y());

    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xBA | impl | TSX
fn TSX_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let s = cpu.s();
    cpu.set_x(s);

    pcr::sync_pcr_n(cpu, s);
    pcr::sync_pcr_z(cpu, s);

    None
}

/// 0xBC | abs,X | LDY oper,X
fn LDY_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_absolute::load(mem, pc, cpu.x());

    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xBD | abs,X | LDA oper,X
fn LDA_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_absolute::load(mem, pc, cpu.x());

    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xBE | abs,Y | LDX oper,Y
fn LDX_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_absolute::load(mem, pc, cpu.y());

    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xC0 | # | CPY #oper
fn CPY_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.y();
    let n2 = am::immediate::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xC1 | (ind,X) | CMP (oper,X)
fn CMP_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.a();
    let n2 = am::pre_indexed_indirect::load(mem, pc, cpu.x());

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xC4 | zpg | CPY oper
fn CPY_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.y();
    let n2 = am::zero_page::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xC5 | zpg | CMP oper
fn CMP_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.a();
    let n2 = am::zero_page::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xC6 | zpg | DEC oper
fn DEC_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::zero_page::load(mem, pc);
    let val = val.wrapping_sub(1);
    am::zero_page::store(mem, pc, val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xC8 | impl | INY
fn INY_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let val = cpu.y().wrapping_add(1);
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xC9 | # | CMP #oper
fn CMP_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.a();
    let n2 = am::immediate::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xCA | impl | DEX
fn DEX_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let val = cpu.x().wrapping_sub(1);
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xCC | abs | CPY oper
fn CPY_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.y();
    let n2 = am::absolute::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xCD | abs | CMP oper
fn CMP_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.a();
    let n2 = am::absolute::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xCE | abs | DEC oper
fn DEC_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::absolute::load(mem, pc);
    let val = val.wrapping_sub(1);
    am::absolute::store(mem, pc, val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xD5 | zpg,X | CMP oper,X
fn CMP_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.a();
    let n2 = am::indexed_zero_page::load(mem, pc, cpu.x());

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xD6 | zpg,X | DEC oper,X
fn DEC_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_zero_page::load(mem, pc, cpu.x());
    let val = val.wrapping_sub(1);
    am::indexed_zero_page::store(mem, pc, cpu.x(), val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xD8 | impl | CLD
fn CLD_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    cpu.clr_psr_bit(PSR::D);

    None
}

/// 0xD9 | abs,Y | CMP oper,Y
fn CMP_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.a();
    let n2 = am::indexed_absolute::load(mem, pc, cpu.y());

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xDD | abs,X | CMP oper,X
fn CMP_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.a();
    let n2 = am::indexed_absolute::load(mem, pc, cpu.x());

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xDE | abs,X | DEC oper,X
fn DEC_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_absolute::load(mem, pc, cpu.x());
    let val = val.wrapping_sub(1);
    am::indexed_absolute::store(mem, pc, cpu.x(), val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xE0 | # | CPX #oper
fn CPX_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.x();
    let n2 = am::immediate::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xE1 | (ind,X) | SBC (oper,X)
fn SBC_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::pre_indexed_indirect::load(mem, pc, cpu.x());

    adder::SBC_core(cpu, n2);

    None
}

/// 0xE4 | zpg | CPX oper
fn CPX_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.x();
    let n2 = am::zero_page::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xE5 | zpg | SBC oper
fn SBC_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::zero_page::load(mem, pc);

    adder::SBC_core(cpu, n2);

    None
}

/// 0xE6 | zpg | INC oper
fn INC_zpg(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::zero_page::load(mem, pc);
    let val = val.wrapping_add(1);
    am::zero_page::store(mem, pc, val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xE8 | impl | INX
fn INX_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    let val = cpu.x().wrapping_add(1);
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xE9 | # | SBC #oper
fn SBC_imme(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::immediate::load(mem, pc);

    adder::SBC_core(cpu, n2);

    None
}

/// 0xEA | impl | NOP
fn NOP_impl(_: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    None
}

/// 0xEC | abs | CPX oper
fn CPX_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n1 = cpu.x();
    let n2 = am::absolute::load(mem, pc);

    adder::CMP_core(cpu, n1, n2);

    None
}

/// 0xED | abs | SBC oper
fn SBC_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::absolute::load(mem, pc);

    adder::SBC_core(cpu, n2);

    None
}

/// 0xEE | abs | INC oper
fn INC_abs(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::absolute::load(mem, pc);
    let val = val.wrapping_add(1);
    am::absolute::store(mem, pc, val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xF5 | zpg,X | SBC oper,X
fn SBC_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::indexed_zero_page::load(mem, pc, cpu.x());

    adder::SBC_core(cpu, n2);

    None
}

/// 0xF6 | zpg,X | INC oper,X
fn INC_zpg_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_zero_page::load(mem, pc, cpu.x());
    let val = val.wrapping_add(1);
    am::indexed_zero_page::store(mem, pc, cpu.x(), val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xF8 | impl | SED
fn SED_impl(cpu: &mut MOS6502, _: &mut Memory) -> Option<LoHi> {
    cpu.set_psr_bit(PSR::D);

    None
}

/// 0xF9 | abs,Y | SBC oper,Y
fn SBC_abs_Y(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::indexed_absolute::load(mem, pc, cpu.y());

    adder::SBC_core(cpu, n2);

    None
}

/// 0xFD | abs,X | SBC oper,X
fn SBC_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let n2 = am::indexed_absolute::load(mem, pc, cpu.x());

    adder::SBC_core(cpu, n2);

    None
}

/// 0xFE | abs,X | INC oper,X
fn INC_abs_X(cpu: &mut MOS6502, mem: &mut Memory) -> Option<LoHi> {
    let pc = cpu.pc();
    let val = am::indexed_absolute::load(mem, pc, cpu.x());
    let val = val.wrapping_add(1);
    am::indexed_absolute::store(mem, pc, cpu.x(), val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/*
To regenerated this run
$map = @{}; gc -Raw "D:\src\u\s\lib\src\opcodes.json" | ConvertFrom-Json | sort -Property opc | % { $map[$_.opc] = '/* 0x{0:x2} */ &{1}_{2} // {3} | {4}' -f ($_.opc, $_.assembler.split(" ")[0], ((@($_) | % { $_.addressing.replace(",", "_").replace("#", "imme")} | % { if ($_.StartsWith("(") -and $_.EndsWith(")")) { "idx_{0}" -f $_ } elseif ($_.StartsWith("(")) { "{0}_idx" -f $_ } else { $_ } } | % { $_.Replace("(", "").Replace(")", "") }) + ",").PadRight(11, " "), $_.addressing, $_.assembler) }
$opc_fns = 0..0xff | % { $opc = "{0:X2}" -f $_; if ($map.Contains($opc)) { "    {0}" -f $map[$opc] } else { '    /* 0x{0} */ &illegal,        //' -f $opc } }
$opc_fns

To regenerate the function stubs run
<run the above>
$opc_fns2 =  $opc_fns | ? { !$_.Contains("&illegal,") } | % { ,@($_.SubString(7, 4), $_.SubString(16).Substring(0, 16).Trim().Replace(",", ""), $_.SubString(35)) }
$opc_fns2 | % { "/* /// {0} | {1} */
 fn {2}(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {{`n`n" -f ($_[0],$_[2],$_[1]) }
*/
/// NOTE: See opcodes.json
#[rustfmt::skip]
pub const ALL_OPCODE_ROUTINES: &[&OpCodeFn; 0x1_00] = &[
    /* 0x00 */ &BRK_impl,       // impl | BRK
    /* 0x01 */ &ORA_idx_ind_X,  // (ind,X) | ORA (oper,X)
    /* 0x02 */ &illegal,        //
    /* 0x03 */ &illegal,        //
    /* 0x04 */ &illegal,        //
    /* 0x05 */ &ORA_zpg,        // zpg | ORA oper
    /* 0x06 */ &ASL_zpg,        // zpg | ASL oper
    /* 0x07 */ &illegal,        //
    /* 0x08 */ &PHP_impl,       // impl | PHP
    /* 0x09 */ &ORA_imme,       // # | ORA #oper
    /* 0x0A */ &ASL_A,          // A | ASL A
    /* 0x0B */ &illegal,        //
    /* 0x0C */ &illegal,        //
    /* 0x0D */ &ORA_abs,        // abs | ORA oper
    /* 0x0E */ &ASL_abs,        // abs | ASL oper
    /* 0x0F */ &illegal,        //
    /* 0x10 */ &illegal,        // rel | BPL oper
    /* 0x11 */ &illegal,  // (ind),Y | ORA (oper),Y
    /* 0x12 */ &illegal,        //
    /* 0x13 */ &illegal,        //
    /* 0x14 */ &illegal,        //
    /* 0x15 */ &ORA_zpg_X,      // zpg,X | ORA oper,X
    /* 0x16 */ &ASL_zpg_X,      // zpg,X | ASL oper,X
    /* 0x17 */ &illegal,        //
    /* 0x18 */ &CLC_impl,       // impl | CLC
    /* 0x19 */ &ORA_abs_Y,      // abs,Y | ORA oper,Y
    /* 0x1A */ &illegal,        //
    /* 0x1B */ &illegal,        //
    /* 0x1C */ &illegal,        //
    /* 0x1D */ &ORA_abs_X,      // abs,X | ORA oper,X
    /* 0x1E */ &ASL_abs_X,      // abs,X | ASL oper,X
    /* 0x1F */ &illegal,        //
    /* 0x20 */ &JSR_abs,        // abs | JSR oper
    /* 0x21 */ &AND_idx_ind_X,  // (ind,X) | AND (oper,X)
    /* 0x22 */ &illegal,        //
    /* 0x23 */ &illegal,        //
    /* 0x24 */ &BIT_zpg,        // zpg | BIT oper
    /* 0x25 */ &AND_zpg,        // zpg | AND oper
    /* 0x26 */ &ROL_zpg,        // zpg | ROL oper
    /* 0x27 */ &illegal,        //
    /* 0x28 */ &PLP_impl,       // impl | PLP
    /* 0x29 */ &AND_imme,       // # | AND #oper
    /* 0x2A */ &ROL_A,          // A | ROL A
    /* 0x2B */ &illegal,        //
    /* 0x2C */ &BIT_abs,        // abs | BIT oper
    /* 0x2D */ &AND_abs,        // abs | AND oper
    /* 0x2E */ &ROL_abs,        // abs | ROL oper
    /* 0x2F */ &illegal,        //
    /* 0x30 */ &illegal,        // rel | BMI oper
    /* 0x31 */ &illegal,        // (ind),Y | AND (oper),Y
    /* 0x32 */ &illegal,        //
    /* 0x33 */ &illegal,        //
    /* 0x34 */ &illegal,        //
    /* 0x35 */ &AND_zpg_X,      // zpg,X | AND oper,X
    /* 0x36 */ &ROL_zpg_X,      // zpg,X | ROL oper,X
    /* 0x37 */ &illegal,        //
    /* 0x38 */ &SEC_impl,       // impl | SEC
    /* 0x39 */ &AND_abs_Y,      // abs,Y | AND oper,Y
    /* 0x3A */ &illegal,        //
    /* 0x3B */ &illegal,        //
    /* 0x3C */ &illegal,        //
    /* 0x3D */ &AND_abs_X,      // abs,X | AND oper,X
    /* 0x3E */ &ROL_abs_X,      // abs,X | ROL oper,X
    /* 0x3F */ &illegal,        //
    /* 0x40 */ &RTI_impl,       // impl | RTI
    /* 0x41 */ &EOR_idx_ind_X,  // (ind,X) | EOR (oper,X)
    /* 0x42 */ &illegal,        //
    /* 0x43 */ &illegal,        //
    /* 0x44 */ &illegal,        //
    /* 0x45 */ &EOR_zpg,        // zpg | EOR oper
    /* 0x46 */ &LSR_zpg,        // zpg | LSR oper
    /* 0x47 */ &illegal,        //
    /* 0x48 */ &PHA_impl,       // impl | PHA
    /* 0x49 */ &EOR_imme,       // # | EOR #oper
    /* 0x4A */ &LSR_A,          // A | LSR A
    /* 0x4B */ &illegal,        //
    /* 0x4C */ &JMP_abs,        // abs | JMP oper
    /* 0x4D */ &EOR_abs,        // abs | EOR oper
    /* 0x4E */ &LSR_abs,        // abs | LSR oper
    /* 0x4F */ &illegal,        //
    /* 0x50 */ &illegal,        // rel | BVC oper
    /* 0x51 */ &illegal,  // (ind),Y | EOR (oper),Y
    /* 0x52 */ &illegal,        //
    /* 0x53 */ &illegal,        //
    /* 0x54 */ &illegal,        //
    /* 0x55 */ &EOR_zpg_X,      // zpg,X | EOR oper,X
    /* 0x56 */ &LSR_zpg_X,      // zpg,X | LSR oper,X
    /* 0x57 */ &illegal,        //
    /* 0x58 */ &CLI_impl,       // impl | CLI
    /* 0x59 */ &EOR_abs_Y,      // abs,Y | EOR oper,Y
    /* 0x5A */ &illegal,        //
    /* 0x5B */ &illegal,        //
    /* 0x5C */ &illegal,        //
    /* 0x5D */ &EOR_abs_X,      // abs,X | EOR oper,X
    /* 0x5E */ &LSR_abs_X,      // abs,X | LSR oper,X
    /* 0x5F */ &illegal,        //
    /* 0x60 */ &RTS_impl,       // impl | RTS
    /* 0x61 */ &ADC_idx_ind_X,  // (ind,X) | ADC (oper,X)
    /* 0x62 */ &illegal,        //
    /* 0x63 */ &illegal,        //
    /* 0x64 */ &illegal,        //
    /* 0x65 */ &ADC_zpg,        // zpg | ADC oper
    /* 0x66 */ &ROR_zpg,        // zpg | ROR oper
    /* 0x67 */ &illegal,        //
    /* 0x68 */ &PLA_impl,       // impl | PLA
    /* 0x69 */ &ADC_imme,       // # | ADC #oper
    /* 0x6A */ &ROR_A,          // A | ROR A
    /* 0x6B */ &illegal,        //
    /* 0x6C */ &JMP_ind,        // ind | JMP (oper)
    /* 0x6D */ &ADC_abs,        // abs | ADC oper
    /* 0x6E */ &ROR_abs,        // abs | ROR oper
    /* 0x6F */ &illegal,        //
    /* 0x70 */ &illegal,        // rel | BVS oper
    /* 0x71 */ &illegal,        // (ind),Y | ADC (oper),Y
    /* 0x72 */ &illegal,        //
    /* 0x73 */ &illegal,        //
    /* 0x74 */ &illegal,        //
    /* 0x75 */ &ADC_zpg_X,      // zpg,X | ADC oper,X
    /* 0x76 */ &ROR_zpg_X,      // zpg,X | ROR oper,X
    /* 0x77 */ &illegal,        //
    /* 0x78 */ &SEI_impl,       // impl | SEI
    /* 0x79 */ &ADC_abs_Y,      // abs,Y | ADC oper,Y
    /* 0x7A */ &illegal,        //
    /* 0x7B */ &illegal,        //
    /* 0x7C */ &illegal,        //
    /* 0x7D */ &ADC_abs_X,      // abs,X | ADC oper,X
    /* 0x7E */ &ROR_abs_X,      // abs,X | ROR oper,X
    /* 0x7F */ &illegal,        //
    /* 0x80 */ &illegal,        //
    /* 0x81 */ &STA_idx_ind_X,  // (ind,X) | STA (oper,X)
    /* 0x82 */ &illegal,        //
    /* 0x83 */ &illegal,        //
    /* 0x84 */ &STY_zpg,        // zpg | STY oper
    /* 0x85 */ &STA_zpg,        // zpg | STA oper
    /* 0x86 */ &STX_zpg,        // zpg | STX oper
    /* 0x87 */ &illegal,        //
    /* 0x88 */ &DEY_impl,       // impl | DEY
    /* 0x89 */ &illegal,        //
    /* 0x8A */ &TXA_impl,       // impl | TXA
    /* 0x8B */ &illegal,        //
    /* 0x8C */ &STY_abs,        // abs | STY oper
    /* 0x8D */ &STA_abs,        // abs | STA oper
    /* 0x8E */ &STX_abs,        // abs | STX oper
    /* 0x8F */ &illegal,        //
    /* 0x90 */ &illegal,        // rel | BCC oper
    /* 0x91 */ &illegal,        // (ind),Y | STA (oper),Y
    /* 0x92 */ &illegal,        //
    /* 0x93 */ &illegal,        //
    /* 0x94 */ &STY_zpg_X,      // zpg,X | STY oper,X
    /* 0x95 */ &STA_zpg_X,      // zpg,X | STA oper,X
    /* 0x96 */ &STX_zpg_Y,      // zpg,Y | STX oper,Y
    /* 0x97 */ &illegal,        //
    /* 0x98 */ &TYA_impl,       // impl | TYA
    /* 0x99 */ &STA_abs_Y,      // abs,Y | STA oper,Y
    /* 0x9A */ &TXS_impl,       // impl | TXS
    /* 0x9B */ &illegal,        //
    /* 0x9C */ &illegal,        //
    /* 0x9D */ &STA_abs_X,      // abs,X | STA oper,X
    /* 0x9E */ &illegal,        //
    /* 0x9F */ &illegal,        //
    /* 0xA0 */ &LDY_imme,       // # | LDY #oper
    /* 0xA1 */ &LDA_idx_ind_X,  // (ind,X) | LDA (oper,X)
    /* 0xA2 */ &LDX_imme,       // # | LDX #oper
    /* 0xA3 */ &illegal,        //
    /* 0xA4 */ &LDY_zpg,        // zpg | LDY oper
    /* 0xA5 */ &LDA_zpg,        // zpg | LDA oper
    /* 0xA6 */ &LDX_zpg,        // zpg | LDX oper
    /* 0xA7 */ &illegal,        //
    /* 0xA8 */ &TAY_impl,       // impl | TAY
    /* 0xA9 */ &LDA_imme,       // # | LDA #oper
    /* 0xAA */ &TAX_impl,       // impl | TAX
    /* 0xAB */ &illegal,        //
    /* 0xAC */ &LDY_abs,        // abs | LDY oper
    /* 0xAD */ &LDA_abs,        // abs | LDA oper
    /* 0xAE */ &LDX_abs,        // abs | LDX oper
    /* 0xAF */ &illegal,        //
    /* 0xB0 */ &illegal,        // rel | BCS oper
    /* 0xB1 */ &illegal,        // (ind),Y | LDA (oper),Y
    /* 0xB2 */ &illegal,        //
    /* 0xB3 */ &illegal,        //
    /* 0xB4 */ &LDY_zpg_X,      // zpg,X | LDY oper,X
    /* 0xB5 */ &LDA_zpg_X,      // zpg,X | LDA oper,X
    /* 0xB6 */ &LDX_zpg_Y,      // zpg,Y | LDX oper,Y
    /* 0xB7 */ &illegal,        //
    /* 0xB8 */ &CLV_impl,       // impl | CLV
    /* 0xB9 */ &LDA_abs_Y,      // abs,Y | LDA oper,Y
    /* 0xBA */ &TSX_impl,       // impl | TSX
    /* 0xBB */ &illegal,        //
    /* 0xBC */ &LDY_abs_X,      // abs,X | LDY oper,X
    /* 0xBD */ &LDA_abs_X,      // abs,X | LDA oper,X
    /* 0xBE */ &LDX_abs_Y,      // abs,Y | LDX oper,Y
    /* 0xBF */ &illegal,        //
    /* 0xC0 */ &CPY_imme,       // # | CPY #oper
    /* 0xC1 */ &CMP_idx_ind_X,  // (ind,X) | CMP (oper,X)
    /* 0xC2 */ &illegal,        //
    /* 0xC3 */ &illegal,        //
    /* 0xC4 */ &CPY_zpg,        // zpg | CPY oper
    /* 0xC5 */ &CMP_zpg,        // zpg | CMP oper
    /* 0xC6 */ &DEC_zpg,        // zpg | DEC oper
    /* 0xC7 */ &illegal,        //
    /* 0xC8 */ &INY_impl,       // impl | INY
    /* 0xC9 */ &CMP_imme,       // # | CMP #oper
    /* 0xCA */ &DEX_impl,       // impl | DEX
    /* 0xCB */ &illegal,        //
    /* 0xCC */ &CPY_abs,        // abs | CPY oper
    /* 0xCD */ &CMP_abs,        // abs | CMP oper
    /* 0xCE */ &DEC_abs,        // abs | DEC oper
    /* 0xCF */ &illegal,        //
    /* 0xD0 */ &illegal,        // rel | BNE oper
    /* 0xD1 */ &illegal,        // (ind),Y | CMP (oper),Y
    /* 0xD2 */ &illegal,        //
    /* 0xD3 */ &illegal,        //
    /* 0xD4 */ &illegal,        //
    /* 0xD5 */ &CMP_zpg_X,      // zpg,X | CMP oper,X
    /* 0xD6 */ &DEC_zpg_X,      // zpg,X | DEC oper,X
    /* 0xD7 */ &illegal,        //
    /* 0xD8 */ &CLD_impl,       // impl | CLD
    /* 0xD9 */ &CMP_abs_Y,      // abs,Y | CMP oper,Y
    /* 0xDA */ &illegal,        //
    /* 0xDB */ &illegal,        //
    /* 0xDC */ &illegal,        //
    /* 0xDD */ &CMP_abs_X,      // abs,X | CMP oper,X
    /* 0xDE */ &DEC_abs_X,      // abs,X | DEC oper,X
    /* 0xDF */ &illegal,        //
    /* 0xE0 */ &CPX_imme,       // # | CPX #oper
    /* 0xE1 */ &SBC_idx_ind_X,  // (ind,X) | SBC (oper,X)
    /* 0xE2 */ &illegal,        //
    /* 0xE3 */ &illegal,        //
    /* 0xE4 */ &CPX_zpg,        // zpg | CPX oper
    /* 0xE5 */ &SBC_zpg,        // zpg | SBC oper
    /* 0xE6 */ &INC_zpg,        // zpg | INC oper
    /* 0xE7 */ &illegal,        //
    /* 0xE8 */ &INX_impl,       // impl | INX
    /* 0xE9 */ &SBC_imme,       // # | SBC #oper
    /* 0xEA */ &NOP_impl,       // impl | NOP
    /* 0xEB */ &illegal,        //
    /* 0xEC */ &CPX_abs,        // abs | CPX oper
    /* 0xED */ &SBC_abs,        // abs | SBC oper
    /* 0xEE */ &INC_abs,        // abs | INC oper
    /* 0xEF */ &illegal,        //
    /* 0xF0 */ &illegal,        // rel | BEQ oper
    /* 0xF1 */ &illegal,  // (ind),Y | SBC (oper),Y
    /* 0xF2 */ &illegal,        //
    /* 0xF3 */ &illegal,        //
    /* 0xF4 */ &illegal,        //
    /* 0xF5 */ &SBC_zpg_X,      // zpg,X | SBC oper,X
    /* 0xF6 */ &INC_zpg_X,      // zpg,X | INC oper,X
    /* 0xF7 */ &illegal,        //
    /* 0xF8 */ &SED_impl,       // impl | SED
    /* 0xF9 */ &SBC_abs_Y,      // abs,Y | SBC oper,Y
    /* 0xFA */ &illegal,        //
    /* 0xFB */ &illegal,        //
    /* 0xFC */ &illegal,        //
    /* 0xFD */ &SBC_abs_X,      // abs,X | SBC oper,X
    /* 0xFE */ &INC_abs_X,      // abs,X | INC oper,X
    /* 0xFF */ &illegal,        //
];

mod pcr {
    use super::*;

    #[inline]
    fn shift_ops_sync_pcr_c(cpu: &mut MOS6502, val: u8, bit_selector: u8) {
        if bits::tst_bits(val, bit_selector) {
            cpu.set_psr_bit(PSR::C)
        } else {
            cpu.clr_psr_bit(PSR::C)
        }
    }

    #[inline]
    pub fn shift_ops_sync_pcr_c_lsb(cpu: &mut MOS6502, val: u8) {
        shift_ops_sync_pcr_c(cpu, val, 0b0000_0001);
    }

    #[inline]
    pub fn shift_ops_sync_pcr_c_msb(cpu: &mut MOS6502, val: u8) {
        shift_ops_sync_pcr_c(cpu, val, 0b1000_0000);
    }

    #[inline]
    pub fn sync_pcr_z(cpu: &mut MOS6502, val: u8) {
        if val == 0 {
            cpu.set_psr_bit(PSR::Z)
        } else {
            cpu.clr_psr_bit(PSR::Z)
        }
    }

    #[inline]
    pub fn sync_pcr_n(cpu: &mut MOS6502, val: u8) {
        if bits::tst_bits(val, 0b1000_0000) {
            cpu.set_psr_bit(PSR::N)
        } else {
            cpu.clr_psr_bit(PSR::N)
        }
    }
}

mod stack {
    use super::*;

    pub const STACK_POINTER_HI: u8 = 0x01;

    #[inline]
    pub fn push(cpu: &mut MOS6502, mem: &mut Memory, val: u8) {
        mem.set(LoHi(cpu.s(), STACK_POINTER_HI), 0, val);

        let s = cpu.s().wrapping_sub(1);
        cpu.set_s(s);
    }

    #[inline]
    pub fn pop(cpu: &mut MOS6502, mem: &mut Memory) -> u8 {
        let s = cpu.s().wrapping_add(1);
        cpu.set_s(s);

        mem.get(LoHi(s, STACK_POINTER_HI), 0)
    }

    /// NOTE: Flags B & __ will be inserted when PSR is transferred to the stack by software (BRK or PHP).
    #[inline]
    pub fn push_psr(cpu: &mut MOS6502, mem: &mut Memory) {
        let psr = cpu.psr() | 0x30;
        stack::push(cpu, mem, psr | 0x30);
    }

    /// NOTE: Flags B & __ are ignored when retrieved by software (PLP or RTI).
    #[inline]
    pub fn pop_psr(cpu: &mut MOS6502, mem: &mut Memory) {
        let val = stack::pop(cpu, mem) & !0x30;
        cpu.set_psr(val);
    }

    #[inline]
    pub fn push_interrupt_call_stack(cpu: &mut MOS6502, mem: &mut Memory, ret_addr: LoHi) {
        stack::push(cpu, mem, ret_addr.1);
        stack::push(cpu, mem, ret_addr.0);
        stack::push_psr(cpu, mem);
    }

    #[inline]
    pub fn pop_interrupt_call_stack(cpu: &mut MOS6502, mem: &mut Memory) -> LoHi {
        stack::pop_psr(cpu, mem);
        let lo = stack::pop(cpu, mem);
        let hi = stack::pop(cpu, mem);

        LoHi(lo, hi)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;

        #[test]
        fn test_push_pop() {
            let mut cpu = MOS6502::default();
            let mut mem = Memory::new(true);

            const SP: u8 = 0xff;
            cpu.set_s(SP);
            let val = mem.get(LoHi(cpu.s(), STACK_POINTER_HI), 0);
            assert_eq!(val, 0x0d);

            push(&mut cpu, &mut mem, 0x55);
            assert_eq!(cpu.s(), SP - 1);
            assert_eq!(mem.get(LoHi(SP, STACK_POINTER_HI), 0), 0x55);
            let val = pop(&mut cpu, &mut mem);
            assert_eq!(val, 0x55);
            assert_eq!(cpu.s(), SP);
        }

        #[test_case(0b0000_0000)]
        #[test_case(0b1010_0010)]
        #[test_case(0b0101_1001)]
        #[test_case(0b1111_1111)]
        fn push_psr_always_keeps_bits_4_and_5_on(psr: u8) {
            let mut cpu = MOS6502::default();
            let mut mem = Memory::new(true);

            cpu.set_s(0xFF);
            cpu.set_psr(psr);

            push_psr(&mut cpu, &mut mem);
            let stack_psr = pop(&mut cpu, &mut mem);

            assert_eq!(
                stack_psr & 0b1100_1111,
                psr & 0b1100_1111,
                "all bits other 4 & 5 should be on stack."
            );
            assert!(
                bits::tst_bits(stack_psr, 0x30),
                "bits 4 & 5 should always be on stack."
            );
        }

        #[test_case(0b1111_1111)]
        #[test_case(0b1110_0000)]
        #[test_case(0b1101_0100)]
        #[test_case(0b0100_0011)]
        fn pop_psr_always_keep_bits_4_and_5_off(psr: u8) {
            let mut cpu = MOS6502::default();
            let mut mem = Memory::new(true);

            cpu.set_s(0xFF);
            push(&mut cpu, &mut mem, psr);

            pop_psr(&mut cpu, &mut mem);

            assert!(
                bits::tst_bits(cpu.psr() & 0b0011_0000, 0b0000_0000),
                "bits 4 & 5 should always be 0."
            );
            assert!(
                bits::tst_bits(cpu.psr(), psr & 0b1100_1111),
                "except bits 4 & 5 psr after pop should match."
            );
        }
    }
}

pub mod adder {
    use super::*;

    #[inline]
    pub fn ror_core(cpu: &MOS6502, val: u8) -> u8 {
        (val >> 1)
            | if cpu.tst_psr_bit(PSR::C) {
                0b10000000
            } else {
                0b00000000
            }
    }

    #[inline]
    pub fn rol_core(cpu: &MOS6502, val: u8) -> u8 {
        (val << 1)
            | if cpu.tst_psr_bit(PSR::C) {
                0b00000001
            } else {
                0b00000000
            }
    }

    #[inline]
    pub fn CMP_core(cpu: &mut MOS6502, n1: u8, n2: u8) {
        let res = adder::safe_sub_checked(n1, n2);
        pcr::sync_pcr_n(cpu, res.0);
        pcr::sync_pcr_z(cpu, res.0);
        if n1 < n2 {
            cpu.clr_psr_bit(PSR::C);
        } else {
            cpu.set_psr_bit(PSR::C);
        }
    }

    #[inline]
    pub fn safe_sub_checked(val1: u8, val2: u8) -> (u8, bool) {
        let res = val1 as i16 - val2 as i16;

        let v = res & 0b1_0000_0000 != 0;

        (res as u8, v)
    }

    #[inline]
    pub fn bit_core(cpu: &mut MOS6502, v2: u8) {
        let v1 = cpu.a();
        let res = v1 & v2;

        pcr::sync_pcr_n(cpu, v2);
        if bits::tst_bits(v2, 0b0100_0000) {
            cpu.set_psr_bit(PSR::V)
        } else {
            cpu.clr_psr_bit(PSR::V)
        }
        pcr::sync_pcr_z(cpu, res);
    }

    /// Refer:
    /// - https://www.masswerk.at/6502/6502_instruction_set.html#arithmetic
    /// - https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    /// - http://www.6502.org/tutorials/decimal_mode.html
    #[inline]
    pub fn ADC_core(cpu: &mut MOS6502, n2: u8) {
        if cpu.tst_psr_bit(PSR::D) {
            ADC_core_bcd(cpu, n2)
        } else {
            ADC_core_bin(cpu, n2)
        }
    }

    fn ADC_core_bin(cpu: &mut MOS6502, n2: u8) {
        let n1 = cpu.a();
        let res = n1 as u16 + n2 as u16 + if cpu.tst_psr_bit(PSR::C) { 0x01 } else { 0x00 };
        let res_u8 = res as u8;
        cpu.set_a(res_u8);

        pcr::sync_pcr_n(cpu, res_u8);
        let bit8u8 = 0b1000_0000;
        let c6 = ((n1 & !bit8u8) + (n2 & !bit8u8)) & bit8u8 == bit8u8;
        let bit8u16 = 0b0000_0001_0000_0000;
        let c7 = res & bit8u16 == bit8u16;
        if c6 != c7 {
            cpu.set_psr_bit(PSR::V)
        } else {
            cpu.clr_psr_bit(PSR::V)
        }
        pcr::sync_pcr_z(cpu, res_u8);
        if c7 {
            cpu.set_psr_bit(PSR::C)
        } else {
            cpu.clr_psr_bit(PSR::C)
        }
    }

    fn ADC_core_bcd(_cpu: &mut MOS6502, _n2: u8) {
        todo!("ADC in decimal mode is not yet implemented.")
    }

    /// Refer:
    /// - http://forum.6502.org/viewtopic.php?f=2&t=2944#p57780
    #[inline]
    pub fn SBC_core(cpu: &mut MOS6502, n2: u8) {
        if cpu.tst_psr_bit(PSR::D) {
            sbc_core_bcd(cpu, n2)
        } else {
            sbc_core_bin(cpu, n2)
        }
    }

    fn sbc_core_bin(cpu: &mut MOS6502, n2: u8) {
        ADC_core(cpu, !n2);
    }

    fn sbc_core_bcd(_cpu: &mut MOS6502, _n2: u8) {
        todo!("SBC in decimal mode is not yet implemented.")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;

        #[test_case(0x10, 0x10, (0x00, false))]
        #[test_case(0x00, 0x01, (0xFF, true))]
        #[test_case(0x10, 0x20, (0xF0, true))]
        fn test_safe_sub(v1: u8, v2: u8, exp: (u8, bool)) {
            let obt = safe_sub_checked(v1, v2);
            assert_eq!(exp, obt);
        }

        ///           C     n1    n2    res    N      V      Z      C
        #[test_case(false, 0x00, 0x00, 0x00, false, false, true, false)]
        #[test_case(true, 0x01, 0x01, 0x03, false, false, false, false)]
        #[test_case(false, 0x01, 0x02, 0x03, false, false, false, false)]
        #[test_case(false, 0x64, 0xE8, 0x4C, false, false, false, true)]
        #[test_case(false, 0x40, 0x80, 0xC0, true, false, false, false)]
        #[test_case(true, 0xD0, 0x8F, 0x60, false, true, false, true)]
        // Test cases from https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        #[test_case(false, 0x50, 0x10, 0x60, false, false, false, false)]
        #[test_case(false, 0x50, 0x50, 0xA0, true, true, false, false)]
        #[test_case(false, 0x50, 0x90, 0xE0, true, false, false, false)]
        #[test_case(false, 0x50, 0xD0, 0x20, false, false, false, true)]
        #[test_case(false, 0xD0, 0x10, 0xE0, true, false, false, false)]
        #[test_case(false, 0xD0, 0x50, 0x20, false, false, false, true)]
        #[test_case(false, 0xD0, 0x90, 0x60, false, true, false, true)]
        #[test_case(false, 0xD0, 0xD0, 0xA0, true, false, false, true)]
        #[allow(clippy::too_many_arguments)]
        fn test_binary_adc(
            carry: bool,
            v1: u8,
            v2: u8,
            exp: u8,
            exp_n: bool,
            exp_v: bool,
            exp_z: bool,
            exp_c: bool,
        ) {
            let mut cpu = MOS6502::default();
            cpu.clr_psr_bit(PSR::D);
            if carry {
                cpu.set_psr_bit(PSR::C)
            } else {
                cpu.clr_psr_bit(PSR::C)
            }
            cpu.set_a(v1);

            ADC_core(&mut cpu, v2);

            assert_eq!(cpu.a(), exp);
            assert_eq!(cpu.tst_psr_bit(PSR::N), exp_n, "N flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::V), exp_v, "V flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::Z), exp_z, "Z flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::C), exp_c, "C flag mismatch");
        }

        ///           C     n1    n2    res    N      V      Z      C
        #[test_case(true, 0x00, 0x00, 0x00, false, true, true, true)]
        // Test cases from https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        #[test_case(true, 0x50, 0xF0, 0x60, false, false, false, false)]
        #[test_case(true, 0x50, 0xB0, 0xA0, true, true, false, false)]
        #[test_case(true, 0x50, 0x70, 0xE0, true, false, false, false)]
        #[test_case(true, 0x50, 0x30, 0x20, false, false, false, true)]
        #[test_case(true, 0xD0, 0xF0, 0xE0, true, false, false, false)]
        #[test_case(true, 0xD0, 0xB0, 0x20, false, false, false, true)]
        #[test_case(true, 0xD0, 0x70, 0x60, false, true, false, true)]
        #[test_case(true, 0xD0, 0x30, 0xA0, true, false, false, true)]
        #[allow(clippy::too_many_arguments)]
        fn test_binary_sbc(
            carry: bool,
            v1: u8,
            v2: u8,
            exp: u8,
            exp_n: bool,
            exp_v: bool,
            exp_z: bool,
            exp_c: bool,
        ) {
            let mut cpu = MOS6502::default();
            cpu.clr_psr_bit(PSR::D);
            if carry {
                cpu.set_psr_bit(PSR::C)
            } else {
                cpu.clr_psr_bit(PSR::C)
            }
            cpu.set_a(v1);

            SBC_core(&mut cpu, v2);

            assert_eq!(cpu.a(), exp);
            assert_eq!(cpu.tst_psr_bit(PSR::N), exp_n, "N flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::V), exp_v, "V flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::Z), exp_z, "Z flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::C), exp_c, "C flag mismatch");
        }
    }
}

#[rustfmt::skip]
pub const NEW_CODE_PATH: &[bool; 0x1_00] = &[
    /* 0x00 */ false,   // impl | BRK
    /* 0x01 */ false,   // (ind,X) | ORA (oper,X)
    /* 0x02 */ false,   //
    /* 0x03 */ false,   //
    /* 0x04 */ false,   //
    /* 0x05 */ false,   // zpg | ORA oper
    /* 0x06 */ false,   // zpg | ASL oper
    /* 0x07 */ false,   //
    /* 0x08 */ false,   // impl | PHP
    /* 0x09 */ false,   // # | ORA #oper
    /* 0x0A */ false,   // A | ASL A
    /* 0x0B */ false,   //
    /* 0x0C */ false,   //
    /* 0x0D */ false,   // abs | ORA oper
    /* 0x0E */ false,   // abs | ASL oper
    /* 0x0F */ false,   //
    /* 0x10 */ true,    // rel | BPL oper
    /* 0x11 */ true,    // (ind),Y | ORA (oper),Y
    /* 0x12 */ false,   //
    /* 0x13 */ false,   //
    /* 0x14 */ false,   //
    /* 0x15 */ false,   // zpg,X | ORA oper,X
    /* 0x16 */ false,   // zpg,X | ASL oper,X
    /* 0x17 */ false,   //
    /* 0x18 */ false,   // impl | CLC
    /* 0x19 */ false,   // abs,Y | ORA oper,Y
    /* 0x1A */ false,   //
    /* 0x1B */ false,   //
    /* 0x1C */ false,   //
    /* 0x1D */ false,   // abs,X | ORA oper,X
    /* 0x1E */ false,   // abs,X | ASL oper,X
    /* 0x1F */ false,   //
    /* 0x20 */ false,   // abs | JSR oper
    /* 0x21 */ false,   // (ind,X) | AND (oper,X)
    /* 0x22 */ false,   //
    /* 0x23 */ false,   //
    /* 0x24 */ false,   // zpg | BIT oper
    /* 0x25 */ false,   // zpg | AND oper
    /* 0x26 */ false,   // zpg | ROL oper
    /* 0x27 */ false,   //
    /* 0x28 */ false,   // impl | PLP
    /* 0x29 */ false,   // # | AND #oper
    /* 0x2A */ false,   // A | ROL A
    /* 0x2B */ false,   //
    /* 0x2C */ false,   // abs | BIT oper
    /* 0x2D */ false,   // abs | AND oper
    /* 0x2E */ false,   // abs | ROL oper
    /* 0x2F */ false,   //
    /* 0x30 */ true,    // rel | BMI oper
    /* 0x31 */ true,    // (ind),Y | AND (oper),Y
    /* 0x32 */ false,   //
    /* 0x33 */ false,   //
    /* 0x34 */ false,   //
    /* 0x35 */ false,   // zpg,X | AND oper,X
    /* 0x36 */ false,   // zpg,X | ROL oper,X
    /* 0x37 */ false,   //
    /* 0x38 */ false,   // impl | SEC
    /* 0x39 */ false,   // abs,Y | AND oper,Y
    /* 0x3A */ false,   //
    /* 0x3B */ false,   //
    /* 0x3C */ false,   //
    /* 0x3D */ false,   // abs,X | AND oper,X
    /* 0x3E */ false,   // abs,X | ROL oper,X
    /* 0x3F */ false,   //
    /* 0x40 */ false,   // impl | RTI
    /* 0x41 */ false,   // (ind,X) | EOR (oper,X)
    /* 0x42 */ false,   //
    /* 0x43 */ false,   //
    /* 0x44 */ false,   //
    /* 0x45 */ false,   // zpg | EOR oper
    /* 0x46 */ false,   // zpg | LSR oper
    /* 0x47 */ false,   //
    /* 0x48 */ false,   // impl | PHA
    /* 0x49 */ false,   // # | EOR #oper
    /* 0x4A */ false,   // A | LSR A
    /* 0x4B */ false,   //
    /* 0x4C */ false,   // abs | JMP oper
    /* 0x4D */ false,   // abs | EOR oper
    /* 0x4E */ false,   // abs | LSR oper
    /* 0x4F */ false,   //
    /* 0x50 */ true,    // rel | BVC oper
    /* 0x51 */ true,    // (ind),Y | EOR (oper),Y
    /* 0x52 */ false,   //
    /* 0x53 */ false,   //
    /* 0x54 */ false,   //
    /* 0x55 */ false,   // zpg,X | EOR oper,X
    /* 0x56 */ false,   // zpg,X | LSR oper,X
    /* 0x57 */ false,   //
    /* 0x58 */ false,   // impl | CLI
    /* 0x59 */ false,   // abs,Y | EOR oper,Y
    /* 0x5A */ false,   //
    /* 0x5B */ false,   //
    /* 0x5C */ false,   //
    /* 0x5D */ false,   // abs,X | EOR oper,X
    /* 0x5E */ false,   // abs,X | LSR oper,X
    /* 0x5F */ false,   //
    /* 0x60 */ false,   // impl | RTS
    /* 0x61 */ false,   // (ind,X) | ADC (oper,X)
    /* 0x62 */ false,   //
    /* 0x63 */ false,   //
    /* 0x64 */ false,   //
    /* 0x65 */ false,   // zpg | ADC oper
    /* 0x66 */ false,   // zpg | ROR oper
    /* 0x67 */ false,   //
    /* 0x68 */ false,   // impl | PLA
    /* 0x69 */ false,   // # | ADC #oper
    /* 0x6A */ false,   // A | ROR A
    /* 0x6B */ false,   //
    /* 0x6C */ false,   // ind | JMP (oper)
    /* 0x6D */ false,   // abs | ADC oper
    /* 0x6E */ false,   // abs | ROR oper
    /* 0x6F */ false,   //
    /* 0x70 */ true,    // rel | BVS oper
    /* 0x71 */ true,    // (ind),Y | ADC (oper),Y
    /* 0x72 */ false,   //
    /* 0x73 */ false,   //
    /* 0x74 */ false,   //
    /* 0x75 */ false,   // zpg,X | ADC oper,X
    /* 0x76 */ false,   // zpg,X | ROR oper,X
    /* 0x77 */ false,   //
    /* 0x78 */ false,   // impl | SEI
    /* 0x79 */ false,   // abs,Y | ADC oper,Y
    /* 0x7A */ false,   //
    /* 0x7B */ false,   //
    /* 0x7C */ false,   //
    /* 0x7D */ false,   // abs,X | ADC oper,X
    /* 0x7E */ false,   // abs,X | ROR oper,X
    /* 0x7F */ false,   //
    /* 0x80 */ false,   //
    /* 0x81 */ false,   // (ind,X) | STA (oper,X)
    /* 0x82 */ false,   //
    /* 0x83 */ false,   //
    /* 0x84 */ false,   // zpg | STY oper
    /* 0x85 */ false,   // zpg | STA oper
    /* 0x86 */ false,   // zpg | STX oper
    /* 0x87 */ false,   //
    /* 0x88 */ false,   // impl | DEY
    /* 0x89 */ false,   //
    /* 0x8A */ false,   // impl | TXA
    /* 0x8B */ false,   //
    /* 0x8C */ false,   // abs | STY oper
    /* 0x8D */ false,   // abs | STA oper
    /* 0x8E */ false,   // abs | STX oper
    /* 0x8F */ false,   //
    /* 0x90 */ true,    // rel | BCC oper
    /* 0x91 */ true,    // (ind),Y | STA (oper),Y
    /* 0x92 */ false,   //
    /* 0x93 */ false,   //
    /* 0x94 */ false,   // zpg,X | STY oper,X
    /* 0x95 */ false,   // zpg,X | STA oper,X
    /* 0x96 */ false,   // zpg,Y | STX oper,Y
    /* 0x97 */ false,   //
    /* 0x98 */ false,   // impl | TYA
    /* 0x99 */ false,   // abs,Y | STA oper,Y
    /* 0x9A */ false,   // impl | TXS
    /* 0x9B */ false,   //
    /* 0x9C */ false,   //
    /* 0x9D */ false,   // abs,X | STA oper,X
    /* 0x9E */ false,   //
    /* 0x9F */ false,   //
    /* 0xA0 */ false,   // # | LDY #oper
    /* 0xA1 */ false,   // (ind,X) | LDA (oper,X)
    /* 0xA2 */ false,   // # | LDX #oper
    /* 0xA3 */ false,   //
    /* 0xA4 */ false,   // zpg | LDY oper
    /* 0xA5 */ false,   // zpg | LDA oper
    /* 0xA6 */ false,   // zpg | LDX oper
    /* 0xA7 */ false,   //
    /* 0xA8 */ false,   // impl | TAY
    /* 0xA9 */ false,   // # | LDA #oper
    /* 0xAA */ false,   // impl | TAX
    /* 0xAB */ false,   //
    /* 0xAC */ false,   // abs | LDY oper
    /* 0xAD */ false,   // abs | LDA oper
    /* 0xAE */ false,   // abs | LDX oper
    /* 0xAF */ false,   //
    /* 0xB0 */ true,    // rel | BCS oper
    /* 0xB1 */ true,   // (ind),Y | LDA (oper),Y
    /* 0xB2 */ false,   //
    /* 0xB3 */ false,   //
    /* 0xB4 */ false,   // zpg,X | LDY oper,X
    /* 0xB5 */ false,   // zpg,X | LDA oper,X
    /* 0xB6 */ false,   // zpg,Y | LDX oper,Y
    /* 0xB7 */ false,   //
    /* 0xB8 */ false,   // impl | CLV
    /* 0xB9 */ false,   // abs,Y | LDA oper,Y
    /* 0xBA */ false,   // impl | TSX
    /* 0xBB */ false,   //
    /* 0xBC */ false,   // abs,X | LDY oper,X
    /* 0xBD */ false,   // abs,X | LDA oper,X
    /* 0xBE */ false,   // abs,Y | LDX oper,Y
    /* 0xBF */ false,   //
    /* 0xC0 */ false,   // # | CPY #oper
    /* 0xC1 */ false,   // (ind,X) | CMP (oper,X)
    /* 0xC2 */ false,   //
    /* 0xC3 */ false,   //
    /* 0xC4 */ false,   // zpg | CPY oper
    /* 0xC5 */ false,   // zpg | CMP oper
    /* 0xC6 */ false,   // zpg | DEC oper
    /* 0xC7 */ false,   //
    /* 0xC8 */ false,   // impl | INY
    /* 0xC9 */ false,   // # | CMP #oper
    /* 0xCA */ false,   // impl | DEX
    /* 0xCB */ false,   //
    /* 0xCC */ false,   // abs | CPY oper
    /* 0xCD */ false,   // abs | CMP oper
    /* 0xCE */ false,   // abs | DEC oper
    /* 0xCF */ false,   //
    /* 0xD0 */ true,    // rel | BNE oper
    /* 0xD1 */ true,    // (ind),Y | CMP (oper),Y
    /* 0xD2 */ false,   //
    /* 0xD3 */ false,   //
    /* 0xD4 */ false,   //
    /* 0xD5 */ false,   // zpg,X | CMP oper,X
    /* 0xD6 */ false,   // zpg,X | DEC oper,X
    /* 0xD7 */ false,   //
    /* 0xD8 */ false,   // impl | CLD
    /* 0xD9 */ false,   // abs,Y | CMP oper,Y
    /* 0xDA */ false,   //
    /* 0xDB */ false,   //
    /* 0xDC */ false,   //
    /* 0xDD */ false,   // abs,X | CMP oper,X
    /* 0xDE */ false,   // abs,X | DEC oper,X
    /* 0xDF */ false,   //
    /* 0xE0 */ false,   // # | CPX #oper
    /* 0xE1 */ false,   // (ind,X) | SBC (oper,X)
    /* 0xE2 */ false,   //
    /* 0xE3 */ false,   //
    /* 0xE4 */ false,   // zpg | CPX oper
    /* 0xE5 */ false,   // zpg | SBC oper
    /* 0xE6 */ false,   // zpg | INC oper
    /* 0xE7 */ false,   //
    /* 0xE8 */ false,   // impl | INX
    /* 0xE9 */ false,   // # | SBC #oper
    /* 0xEA */ false,   // impl | NOP
    /* 0xEB */ false,   //
    /* 0xEC */ false,   // abs | CPX oper
    /* 0xED */ false,   // abs | SBC oper
    /* 0xEE */ false,   // abs | INC oper
    /* 0xEF */ false,   //
    /* 0xF0 */ true,    // rel | BEQ oper
    /* 0xF1 */ true,    // (ind),Y | SBC (oper),Y
    /* 0xF2 */ false,   //
    /* 0xF3 */ false,   //
    /* 0xF4 */ false,   //
    /* 0xF5 */ false,   // zpg,X | SBC oper,X
    /* 0xF6 */ false,   // zpg,X | INC oper,X
    /* 0xF7 */ false,   //
    /* 0xF8 */ false,   // impl | SED
    /* 0xF9 */ false,   // abs,Y | SBC oper,Y
    /* 0xFA */ false,   //
    /* 0xFB */ false,   //
    /* 0xFC */ false,   //
    /* 0xFD */ false,   // abs,X | SBC oper,X
    /* 0xFE */ false,   // abs,X | INC oper,X
    /* 0xFF */ false,   //
];

#[inline]
fn index_Y(cpu: &MOS6502) -> u8 {
    cpu.y()
}

#[inline]
fn LDA_core(cpu: &mut MOS6502, val: u8) {
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);
}

#[inline]
fn STA_core(cpu: &MOS6502) -> u8 {
    cpu.a()
}

#[inline]
fn relative_BPL_core(cpu: &MOS6502) -> bool {
    !cpu.tst_psr_bit(PSR::N)
}

#[inline]
fn relative_BMI_core(cpu: &MOS6502) -> bool {
    cpu.tst_psr_bit(PSR::N)
}

#[inline]
fn relative_BVC_core(cpu: &MOS6502) -> bool {
    !cpu.tst_psr_bit(PSR::V)
}

#[inline]
fn relative_BVS_core(cpu: &MOS6502) -> bool {
    cpu.tst_psr_bit(PSR::V)
}

#[inline]
fn relative_BCC_core(cpu: &MOS6502) -> bool {
    !cpu.tst_psr_bit(PSR::C)
}

#[inline]
fn relative_BCS_core(cpu: &MOS6502) -> bool {
    cpu.tst_psr_bit(PSR::C)
}

#[inline]
fn relative_BNE_core(cpu: &MOS6502) -> bool {
    !cpu.tst_psr_bit(PSR::Z)
}

#[inline]
fn relative_BEQ_core(cpu: &MOS6502) -> bool {
    cpu.tst_psr_bit(PSR::Z)
}

#[inline]
fn AND_core(cpu: &mut MOS6502, val: u8) {
    let res = cpu.a() & val;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);
}

#[inline]
fn CMP_A_core(cpu: &mut MOS6502, val: u8) {
    adder::CMP_core(cpu, cpu.a(), val);
}

#[inline]
fn EOR_core(cpu: &mut MOS6502, val: u8) {
    let res = cpu.a() ^ val;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);
}

#[inline]
fn ORA_core(cpu: &mut MOS6502, val: u8) {
    let res = cpu.a() | val;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);
}

/// Refer: https://www.nesdev.org/6502_cpu.txt
#[rustfmt::skip]
pub const ALL_OPCODE_STEPS: &[OpCodeSteps; 0x1_00] = &[
    /* 0x00 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | BRK
    /* 0x01 */ am::stub_opcode_steps!(am::opc_step_illegal),   // (ind,X) | ORA (oper,X)
    /* 0x02 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x03 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x04 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x05 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | ORA oper
    /* 0x06 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | ASL oper
    /* 0x07 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x08 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | PHP
    /* 0x09 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | ORA #oper
    /* 0x0A */ am::stub_opcode_steps!(am::opc_step_illegal),   // A | ASL A
    /* 0x0B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x0C */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x0D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | ORA oper
    /* 0x0E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | ASL oper
    /* 0x0F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x10 */ am::relative::opcode_steps!(relative_BPL_core, am::opc_step_illegal),   // rel | BPL oper
    /* 0x11 */ am::post_indexed_indirect::opcode_steps_read!(ORA_core, index_Y, am::opc_step_illegal),   // (ind),Y | ORA (oper),Y
    /* 0x12 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x13 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x14 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x15 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | ORA oper,X
    /* 0x16 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | ASL oper,X
    /* 0x17 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x18 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | CLC
    /* 0x19 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | ORA oper,Y
    /* 0x1A */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x1B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x1C */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x1D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | ORA oper,X
    /* 0x1E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | ASL oper,X
    /* 0x1F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x20 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | JSR oper
    /* 0x21 */ am::stub_opcode_steps!(am::opc_step_illegal),   // (ind,X) | AND (oper,X)
    /* 0x22 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x23 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x24 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | BIT oper
    /* 0x25 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | AND oper
    /* 0x26 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | ROL oper
    /* 0x27 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x28 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | PLP
    /* 0x29 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | AND #oper
    /* 0x2A */ am::stub_opcode_steps!(am::opc_step_illegal),   // A | ROL A
    /* 0x2B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x2C */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | BIT oper
    /* 0x2D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | AND oper
    /* 0x2E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | ROL oper
    /* 0x2F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x30 */ am::relative::opcode_steps!(relative_BMI_core, am::opc_step_illegal),   // rel | BMI oper
    /* 0x31 */ am::post_indexed_indirect::opcode_steps_read!(AND_core, index_Y, am::opc_step_illegal),   // (ind),Y | AND (oper),Y
    /* 0x32 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x33 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x34 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x35 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | AND oper,X
    /* 0x36 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | ROL oper,X
    /* 0x37 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x38 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | SEC
    /* 0x39 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | AND oper,Y
    /* 0x3A */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x3B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x3C */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x3D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | AND oper,X
    /* 0x3E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | ROL oper,X
    /* 0x3F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x40 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | RTI
    /* 0x41 */ am::stub_opcode_steps!(am::opc_step_illegal),   // (ind,X) | EOR (oper,X)
    /* 0x42 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x43 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x44 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x45 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | EOR oper
    /* 0x46 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | LSR oper
    /* 0x47 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x48 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | PHA
    /* 0x49 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | EOR #oper
    /* 0x4A */ am::stub_opcode_steps!(am::opc_step_illegal),   // A | LSR A
    /* 0x4B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x4C */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | JMP oper
    /* 0x4D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | EOR oper
    /* 0x4E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | LSR oper
    /* 0x4F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x50 */ am::relative::opcode_steps!(relative_BVC_core, am::opc_step_illegal),   // rel | BVC oper
    /* 0x51 */ am::post_indexed_indirect::opcode_steps_read!(EOR_core, index_Y, am::opc_step_illegal),   // (ind),Y | EOR (oper),Y
    /* 0x52 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x53 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x54 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x55 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | EOR oper,X
    /* 0x56 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | LSR oper,X
    /* 0x57 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x58 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | CLI
    /* 0x59 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | EOR oper,Y
    /* 0x5A */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x5B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x5C */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x5D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | EOR oper,X
    /* 0x5E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | LSR oper,X
    /* 0x5F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x60 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | RTS
    /* 0x61 */ am::stub_opcode_steps!(am::opc_step_illegal),   // (ind,X) | ADC (oper,X)
    /* 0x62 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x63 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x64 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x65 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | ADC oper
    /* 0x66 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | ROR oper
    /* 0x67 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x68 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | PLA
    /* 0x69 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | ADC #oper
    /* 0x6A */ am::stub_opcode_steps!(am::opc_step_illegal),   // A | ROR A
    /* 0x6B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x6C */ am::stub_opcode_steps!(am::opc_step_illegal),   // ind | JMP (oper)
    /* 0x6D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | ADC oper
    /* 0x6E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | ROR oper
    /* 0x6F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x70 */ am::relative::opcode_steps!(relative_BVS_core, am::opc_step_illegal),   // rel | BVS oper
    /* 0x71 */ am::post_indexed_indirect::opcode_steps_read!(adder::ADC_core, index_Y, am::opc_step_illegal),   // (ind),Y | ADC (oper),Y
    /* 0x72 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x73 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x74 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x75 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | ADC oper,X
    /* 0x76 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | ROR oper,X
    /* 0x77 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x78 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | SEI
    /* 0x79 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | ADC oper,Y
    /* 0x7A */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x7B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x7C */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x7D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | ADC oper,X
    /* 0x7E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | ROR oper,X
    /* 0x7F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x80 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x81 */ am::stub_opcode_steps!(am::opc_step_illegal),   // (ind,X) | STA (oper,X)
    /* 0x82 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x83 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x84 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | STY oper
    /* 0x85 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | STA oper
    /* 0x86 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | STX oper
    /* 0x87 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x88 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | DEY
    /* 0x89 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x8A */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | TXA
    /* 0x8B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x8C */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | STY oper
    /* 0x8D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | STA oper
    /* 0x8E */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | STX oper
    /* 0x8F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x90 */ am::relative::opcode_steps!(relative_BCC_core, am::opc_step_illegal),   // rel | BCC oper
    /* 0x91 */ am::post_indexed_indirect::opcode_steps_write!(STA_core, index_Y, am::opc_step_illegal),   // (ind),Y | STA (oper),Y
    /* 0x92 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x93 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x94 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | STY oper,X
    /* 0x95 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | STA oper,X
    /* 0x96 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,Y | STX oper,Y
    /* 0x97 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x98 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | TYA
    /* 0x99 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | STA oper,Y
    /* 0x9A */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | TXS
    /* 0x9B */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x9C */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x9D */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | STA oper,X
    /* 0x9E */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0x9F */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xA0 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | LDY #oper
    /* 0xA1 */ am::stub_opcode_steps!(am::opc_step_illegal),   // (ind,X) | LDA (oper,X)
    /* 0xA2 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | LDX #oper
    /* 0xA3 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xA4 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | LDY oper
    /* 0xA5 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | LDA oper
    /* 0xA6 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | LDX oper
    /* 0xA7 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xA8 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | TAY
    /* 0xA9 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | LDA #oper
    /* 0xAA */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | TAX
    /* 0xAB */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xAC */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | LDY oper
    /* 0xAD */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | LDA oper
    /* 0xAE */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | LDX oper
    /* 0xAF */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xB0 */ am::relative::opcode_steps!(relative_BCS_core, am::opc_step_illegal),   // rel | BCS oper
    /* 0xB1 */ am::post_indexed_indirect::opcode_steps_read!(LDA_core, index_Y, am::opc_step_illegal),   // (ind),Y | LDA (oper),Y
    /* 0xB2 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xB3 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xB4 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | LDY oper,X
    /* 0xB5 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | LDA oper,X
    /* 0xB6 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,Y | LDX oper,Y
    /* 0xB7 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xB8 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | CLV
    /* 0xB9 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | LDA oper,Y
    /* 0xBA */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | TSX
    /* 0xBB */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xBC */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | LDY oper,X
    /* 0xBD */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | LDA oper,X
    /* 0xBE */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | LDX oper,Y
    /* 0xBF */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xC0 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | CPY #oper
    /* 0xC1 */ am::stub_opcode_steps!(am::opc_step_illegal),   // (ind,X) | CMP (oper,X)
    /* 0xC2 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xC3 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xC4 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | CPY oper
    /* 0xC5 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | CMP oper
    /* 0xC6 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | DEC oper
    /* 0xC7 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xC8 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | INY
    /* 0xC9 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | CMP #oper
    /* 0xCA */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | DEX
    /* 0xCB */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xCC */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | CPY oper
    /* 0xCD */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | CMP oper
    /* 0xCE */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | DEC oper
    /* 0xCF */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xD0 */ am::relative::opcode_steps!(relative_BNE_core, am::opc_step_illegal),   // rel | BNE oper
    /* 0xD1 */ am::post_indexed_indirect::opcode_steps_read!(CMP_A_core, index_Y, am::opc_step_illegal),   // (ind),Y | CMP (oper),Y
    /* 0xD2 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xD3 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xD4 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xD5 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | CMP oper,X
    /* 0xD6 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | DEC oper,X
    /* 0xD7 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xD8 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | CLD
    /* 0xD9 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | CMP oper,Y
    /* 0xDA */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xDB */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xDC */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xDD */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | CMP oper,X
    /* 0xDE */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | DEC oper,X
    /* 0xDF */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xE0 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | CPX #oper
    /* 0xE1 */ am::stub_opcode_steps!(am::opc_step_illegal),   // (ind,X) | SBC (oper,X)
    /* 0xE2 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xE3 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xE4 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | CPX oper
    /* 0xE5 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | SBC oper
    /* 0xE6 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg | INC oper
    /* 0xE7 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xE8 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | INX
    /* 0xE9 */ am::stub_opcode_steps!(am::opc_step_illegal),   // # | SBC #oper
    /* 0xEA */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | NOP
    /* 0xEB */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xEC */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | CPX oper
    /* 0xED */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | SBC oper
    /* 0xEE */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs | INC oper
    /* 0xEF */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xF0 */ am::relative::opcode_steps!(relative_BEQ_core, am::opc_step_illegal),   // rel | BEQ oper
    /* 0xF1 */ am::post_indexed_indirect::opcode_steps_read!(adder::SBC_core, index_Y, am::opc_step_illegal),   // (ind),Y | SBC (oper),Y
    /* 0xF2 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xF3 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xF4 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xF5 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | SBC oper,X
    /* 0xF6 */ am::stub_opcode_steps!(am::opc_step_illegal),   // zpg,X | INC oper,X
    /* 0xF7 */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xF8 */ am::stub_opcode_steps!(am::opc_step_illegal),   // impl | SED
    /* 0xF9 */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,Y | SBC oper,Y
    /* 0xFA */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xFB */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xFC */ am::stub_opcode_steps!(am::opc_step_illegal),   //
    /* 0xFD */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | SBC oper,X
    /* 0xFE */ am::stub_opcode_steps!(am::opc_step_illegal),   // abs,X | INC oper,X
    /* 0xFF */ am::stub_opcode_steps!(am::opc_step_illegal),   //
];
