#![allow(non_snake_case)]

use super::{am, cmn::LoHi, cpu::*, mem::Memory};

fn illegal(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    panic!("Illegal opcode {} @ {:?}", opc, pc)
}

/// 0x00 | impl | BRK
fn BRK_impl(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x01 | (ind,X) | ORA (oper,X)
fn ORA_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_pre_indexed_indirect(mem, pc, cpu.x());

    let res = v1 | v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x05 | zpg | ORA oper
fn ORA_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_zero_page(mem, pc);

    let res = v1 | v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x06 | zpg | ASL oper
fn ASL_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_zero_page(mem, pc);
    let new_v = old_v << 1;
    am::store_zero_page(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x08 | impl | PHP
fn PHP_impl(cpu: &mut MOS6502, mem: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let p = cpu.p();
    stack::push(cpu, mem, p);

    None
}

/// 0x09 | # | ORA #oper
fn ORA_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_immediate(mem, pc);

    let res = v1 | v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x0A | A | ASL A
fn ASL_A(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let old_v = cpu.a();
    let new_v = old_v << 1;
    cpu.set_a(new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x0D | abs | ORA oper
fn ORA_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute(mem, pc);

    let res = v1 | v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x0E | abs | ASL oper
fn ASL_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_absolute(mem, pc);
    let new_v = old_v << 1;
    am::store_absolute(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x10 | rel | BPL oper
fn BPL_rel(cpu: &mut MOS6502, mem: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    if !cpu.tst_psr_bit(PSR::N) {
        return Some(am::utils::relative(mem, opc, pc));
    }

    None
}

/// 0x11 | (ind),Y | ORA (oper),Y
fn ORA_ind_Y_idx(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_post_indexed_indirect(mem, pc, cpu.y());

    let res = v1 | v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x15 | zpg,X | ORA oper,X
fn ORA_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_zero_page_indexed(mem, pc, cpu.x());

    let res = v1 | v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x16 | zpg,X | ASL oper,X
fn ASL_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_zero_page_indexed(mem, pc, cpu.x());
    let new_v = old_v << 1;
    am::store_zero_page_indexed(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x18 | impl | CLC
fn CLC_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    cpu.clr_psr_bit(PSR::C);

    None
}

/// 0x19 | abs,Y | ORA oper,Y
fn ORA_abs_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute_indexed(mem, pc, cpu.y());

    let res = v1 | v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x1D | abs,X | ORA oper,X
fn ORA_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute_indexed(mem, pc, cpu.x());

    let res = v1 | v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x1E | abs,X | ASL oper,X
fn ASL_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_absolute_indexed(mem, pc, cpu.x());
    let new_v = old_v << 1;
    am::store_absolute_indexed(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x20 | abs | JSR oper
fn JSR_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let pc = pc + 2;
    stack::push(cpu, mem, pc.1);
    stack::push(cpu, mem, pc.0);

    Some(am::load_immediate_2(mem, pc))
}

/// 0x21 | (ind,X) | AND (oper,X)
fn AND_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_pre_indexed_indirect(mem, pc, cpu.x());

    let res = v1 & v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x24 | zpg | BIT oper
fn BIT_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_zero_page(mem, pc);

    let res = v1 & v2;

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_v_BIT(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x25 | zpg | AND oper
fn AND_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_zero_page(mem, pc);

    let res = v1 & v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x26 | zpg | ROL oper
fn ROL_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_zero_page(mem, pc);
    let new_v = adder::rol_core(cpu, old_v);
    am::store_zero_page(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x28 | impl | PLP
fn PLP_impl(cpu: &mut MOS6502, mem: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let val = stack::pop(cpu, mem);
    cpu.set_p(val);

    None
}

/// 0x29 | # | AND #oper
fn AND_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_immediate(mem, pc);

    let res = v1 & v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x2A | A | ROL A
fn ROL_A(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let old_v = cpu.a();
    let new_v = adder::rol_core(cpu, old_v);
    cpu.set_a(new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x2C | abs | BIT oper
fn BIT_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute(mem, pc);

    let res = v1 & v2;

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_v_BIT(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x2D | abs | AND oper
fn AND_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute(mem, pc);

    let res = v1 & v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x2E | abs | ROL oper
fn ROL_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_absolute(mem, pc);
    let new_v = adder::rol_core(cpu, old_v);
    am::store_absolute(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x30 | rel | BMI oper
fn BMI_rel(cpu: &mut MOS6502, mem: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    if cpu.tst_psr_bit(PSR::N) {
        return Some(am::utils::relative(mem, opc, pc));
    }

    None
}

/// 0x31 | (ind),Y | AND (oper),Y
fn AND_ind_Y_idx(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_post_indexed_indirect(mem, pc, cpu.y());

    let res = v1 & v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x35 | zpg,X | AND oper,X
fn AND_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_zero_page_indexed(mem, pc, cpu.x());

    let res = v1 & v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x36 | zpg,X | ROL oper,X
fn ROL_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_zero_page_indexed(mem, pc, cpu.x());
    let new_v = adder::rol_core(cpu, old_v);
    am::store_zero_page_indexed(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x38 | impl | SEC
fn SEC_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    cpu.set_psr_bit(PSR::C);

    None
}

/// 0x39 | abs,Y | AND oper,Y
fn AND_abs_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute_indexed(mem, pc, cpu.y());

    let res = v1 & v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x3D | abs,X | AND oper,X
fn AND_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute_indexed(mem, pc, cpu.x());

    let res = v1 & v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x3E | abs,X | ROL oper,X
fn ROL_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_absolute_indexed(mem, pc, cpu.x());
    let new_v = adder::rol_core(cpu, old_v);
    am::store_absolute_indexed(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_msb(cpu, old_v);

    None
}

/// 0x40 | impl | RTI
fn RTI_impl(cpu: &mut MOS6502, mem: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let p = stack::pop(cpu, mem);
    let pc_lo = stack::pop(cpu, mem);
    let pc_hi = stack::pop(cpu, mem);

    cpu.set_p(p);

    Some((pc_lo, pc_hi).into())
}

/// 0x41 | (ind,X) | EOR (oper,X)
fn EOR_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_pre_indexed_indirect(mem, pc, cpu.x());

    let res = v1 ^ v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x45 | zpg | EOR oper
fn EOR_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_zero_page(mem, pc);

    let res = v1 ^ v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x46 | zpg | LSR oper
fn LSR_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_zero_page(mem, pc);
    let new_v = old_v >> 1;
    am::store_zero_page(mem, pc, new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x48 | impl | PHA
fn PHA_impl(cpu: &mut MOS6502, mem: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let a = cpu.a();
    stack::push(cpu, mem, a);

    None
}

/// 0x49 | # | EOR #oper
fn EOR_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_immediate(mem, pc);

    let res = v1 ^ v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x4A | A | LSR A
fn LSR_A(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let old_v = cpu.a();
    let new_v = old_v >> 1;
    cpu.set_a(new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x4C | abs | JMP oper
fn JMP_abs(_: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let pc = am::load_immediate_2(mem, pc);

    Some(pc)
}

/// 0x4D | abs | EOR oper
fn EOR_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute(mem, pc);

    let res = v1 ^ v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x4E | abs | LSR oper
fn LSR_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_absolute(mem, pc);
    let new_v = old_v >> 1;
    am::store_absolute(mem, pc, new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x50 | rel | BVC oper
fn BVC_rel(cpu: &mut MOS6502, mem: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    if !cpu.tst_psr_bit(PSR::V) {
        return Some(am::utils::relative(mem, opc, pc));
    }

    None
}

/// 0x51 | (ind),Y | EOR (oper),Y
fn EOR_ind_Y_idx(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_post_indexed_indirect(mem, pc, cpu.y());

    let res = v1 ^ v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x55 | zpg,X | EOR oper,X
fn EOR_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_zero_page_indexed(mem, pc, cpu.x());

    let res = v1 ^ v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x56 | zpg,X | LSR oper,X
fn LSR_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_zero_page_indexed(mem, pc, cpu.x());
    let new_v = old_v >> 1;
    am::store_zero_page_indexed(mem, pc, cpu.x(), new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x58 | impl | CLI
fn CLI_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    cpu.clr_psr_bit(PSR::I);

    None
}

/// 0x59 | abs,Y | EOR oper,Y
fn EOR_abs_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute_indexed(mem, pc, cpu.y());

    let res = v1 ^ v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x5D | abs,X | EOR oper,X
fn EOR_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let v1 = cpu.a();
    let v2 = am::load_absolute_indexed(mem, pc, cpu.x());

    let res = v1 ^ v2;
    cpu.set_a(res);

    pcr::sync_pcr_n(cpu, res);
    pcr::sync_pcr_z(cpu, res);

    None
}

/// 0x5E | abs,X | LSR oper,X
fn LSR_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_absolute_indexed(mem, pc, cpu.x());
    let new_v = old_v >> 1;
    am::store_absolute_indexed(mem, pc, cpu.x(), new_v);

    cpu.clr_psr_bit(PSR::N);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x60 | impl | RTS
fn RTS_impl(cpu: &mut MOS6502, mem: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let pc_lo = stack::pop(cpu, mem);
    let pc_hi = stack::pop(cpu, mem);

    let pc = LoHi::from((pc_lo, pc_hi)) + 1;

    Some(pc)
}

/// 0x61 | (ind,X) | ADC (oper,X)
fn ADC_idx_ind_X(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x65 | zpg | ADC oper
fn ADC_zpg(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x66 | zpg | ROR oper
fn ROR_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_zero_page(mem, pc);
    let new_v = adder::ror_core(cpu, old_v);
    am::store_zero_page(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x68 | impl | PLA
fn PLA_impl(cpu: &mut MOS6502, mem: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let val = stack::pop(cpu, mem);
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0x69 | # | ADC #oper
fn ADC_imme(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x6A | A | ROR A
fn ROR_A(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let old_v = cpu.a();
    let new_v = adder::ror_core(cpu, old_v);
    cpu.set_a(new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x6C | ind | JMP (oper)
fn JMP_ind(_: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    // NOTE: 6502: The indirect jump instruction does not increment the page address when the indirect pointer crosses a page boundary.
    // JMP ($xxFF) will fetch the address from $xxFF and $xx00.
    let addr = am::load_immediate_2(mem, pc);
    let lo = mem.get(addr, 0);
    let hi = mem.get(addr, 0).wrapping_add(1);

    Some((lo, hi).into())
}

/// 0x6D | abs | ADC oper
fn ADC_abs(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x6E | abs | ROR oper
fn ROR_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_absolute(mem, pc);
    let new_v = adder::ror_core(cpu, old_v);
    am::store_absolute(mem, pc, new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x70 | rel | BVS oper
fn BVS_rel(cpu: &mut MOS6502, mem: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    if cpu.tst_psr_bit(PSR::V) {
        return Some(am::utils::relative(mem, opc, pc));
    }

    None
}

/// 0x71 | (ind),Y | ADC (oper),Y
fn ADC_ind_Y_idx(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x75 | zpg,X | ADC oper,X
fn ADC_zpg_X(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x76 | zpg,X | ROR oper,X
fn ROR_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_zero_page_indexed(mem, pc, cpu.x());
    let new_v = adder::ror_core(cpu, old_v);
    am::store_zero_page_indexed(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x78 | impl | SEI
fn SEI_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    cpu.set_psr_bit(PSR::I);

    None
}

/// 0x79 | abs,Y | ADC oper,Y
fn ADC_abs_Y(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x7D | abs,X | ADC oper,X
fn ADC_abs_X(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0x7E | abs,X | ROR oper,X
fn ROR_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let old_v = am::load_absolute_indexed(mem, pc, cpu.x());
    let new_v = adder::ror_core(cpu, old_v);
    am::store_absolute_indexed(mem, pc, cpu.x(), new_v);

    pcr::sync_pcr_n(cpu, new_v);
    pcr::sync_pcr_z(cpu, new_v);
    pcr::shift_ops_sync_pcr_c_lsb(cpu, old_v);

    None
}

/// 0x81 | (ind,X) | STA (oper,X)
fn STA_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_pre_indexed_indirect(mem, pc, cpu.x(), cpu.a());

    None
}

/// 0x84 | zpg | STY oper
fn STY_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_zero_page(mem, pc, cpu.y());

    None
}

/// 0x85 | zpg | STA oper
fn STA_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_zero_page(mem, pc, cpu.a());

    None
}

/// 0x86 | zpg | STX oper
fn STX_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_zero_page(mem, pc, cpu.x());

    None
}

/// 0x88 | impl | DEY
fn DEY_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let val = cpu.y().wrapping_sub(1);
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0x8A | impl | TXA
fn TXA_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let x = cpu.x();
    cpu.set_a(x);

    pcr::sync_pcr_n(cpu, x);
    pcr::sync_pcr_z(cpu, x);

    None
}

/// 0x8C | abs | STY oper
fn STY_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_absolute(mem, pc, cpu.y());

    None
}

/// 0x8D | abs | STA oper
fn STA_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_absolute(mem, pc, cpu.a());

    None
}

/// 0x8E | abs | STX oper
fn STX_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_absolute(mem, pc, cpu.x());

    None
}

/// 0x90 | rel | BCC oper
fn BCC_rel(cpu: &mut MOS6502, mem: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    if !cpu.tst_psr_bit(PSR::C) {
        return Some(am::utils::relative(mem, opc, pc));
    }

    None
}

/// 0x91 | (ind),Y | STA (oper),Y
fn STA_ind_Y_idx(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_post_indexed_indirect(mem, pc, cpu.y(), cpu.a());

    None
}

/// 0x94 | zpg,X | STY oper,X
fn STY_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_zero_page_indexed(mem, pc, cpu.x(), cpu.y());

    None
}

/// 0x95 | zpg,X | STA oper,X
fn STA_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_zero_page_indexed(mem, pc, cpu.x(), cpu.a());

    None
}

/// 0x96 | zpg,Y | STX oper,Y
fn STX_zpg_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_zero_page_indexed(mem, pc, cpu.y(), cpu.x());

    None
}

/// 0x98 | impl | TYA
fn TYA_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let y = cpu.y();
    cpu.set_a(y);

    pcr::sync_pcr_n(cpu, y);
    pcr::sync_pcr_z(cpu, y);

    None
}

/// 0x99 | abs,Y | STA oper,Y
fn STA_abs_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_absolute_indexed(mem, pc, cpu.y(), cpu.a());

    None
}

/// 0x9A | impl | TXS
fn TXS_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let x = cpu.x();
    cpu.set_s(x);

    None
}

/// 0x9D | abs,X | STA oper,X
fn STA_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    am::store_absolute_indexed(mem, pc, cpu.x(), cpu.a());

    None
}

/// 0xA0 | # | LDY #oper
fn LDY_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_immediate(mem, pc);
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA1 | (ind,X) | LDA (oper,X)
fn LDA_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_pre_indexed_indirect(mem, pc, cpu.x());
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA2 | # | LDX #oper
fn LDX_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_immediate(mem, pc);
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA4 | zpg | LDY oper
fn LDY_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page(mem, pc);
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA5 | zpg | LDA oper
fn LDA_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page(mem, pc);
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA6 | zpg | LDX oper
fn LDX_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page(mem, pc);
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xA8 | impl | TAY
fn TAY_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let a = cpu.a();
    cpu.set_y(a);

    pcr::sync_pcr_n(cpu, a);
    pcr::sync_pcr_z(cpu, a);

    None
}

/// 0xA9 | # | LDA #oper
fn LDA_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_immediate(mem, pc);
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xAA | impl | TAX
fn TAX_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let a = cpu.a();
    cpu.set_x(a);

    pcr::sync_pcr_n(cpu, a);
    pcr::sync_pcr_z(cpu, a);

    None
}

/// 0xAC | abs | LDY oper
fn LDY_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute(mem, pc);

    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xAD | abs | LDA oper
fn LDA_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute(mem, pc);

    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xAE | abs | LDX oper
fn LDX_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute(mem, pc);

    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB0 | rel | BCS oper
fn BCS_rel(cpu: &mut MOS6502, mem: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    if cpu.tst_psr_bit(PSR::C) {
        return Some(am::utils::relative(mem, opc, pc));
    }

    None
}

/// 0xB1 | (ind),Y | LDA (oper),Y
fn LDA_ind_Y_idx(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_post_indexed_indirect(mem, pc, cpu.y());
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB4 | zpg,X | LDY oper,X
fn LDY_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page_indexed(mem, pc, cpu.x());
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB5 | zpg,X | LDA oper,X
fn LDA_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page_indexed(mem, pc, cpu.x());
    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB6 | zpg,Y | LDX oper,Y
fn LDX_zpg_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page_indexed(mem, pc, cpu.y());
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xB8 | impl | CLV
fn CLV_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    cpu.clr_psr_bit(PSR::V);

    None
}

/// 0xB9 | abs,Y | LDA oper,Y
fn LDA_abs_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute_indexed(mem, pc, cpu.y());

    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xBA | impl | TSX
fn TSX_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let s = cpu.s();
    cpu.set_x(s);

    pcr::sync_pcr_n(cpu, s);
    pcr::sync_pcr_z(cpu, s);

    None
}

/// 0xBC | abs,X | LDY oper,X
fn LDY_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute_indexed(mem, pc, cpu.x());

    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xBD | abs,X | LDA oper,X
fn LDA_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute_indexed(mem, pc, cpu.x());

    cpu.set_a(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xBE | abs,Y | LDX oper,Y
fn LDX_abs_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute_indexed(mem, pc, cpu.y());

    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xC0 | # | CPY #oper
fn CPY_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.y();
    let n2 = am::load_immediate(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xC1 | (ind,X) | CMP (oper,X)
fn CMP_idx_ind_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.a();
    let n2 = am::load_pre_indexed_indirect(mem, pc, cpu.x());

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xC4 | zpg | CPY oper
fn CPY_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.y();
    let n2 = am::load_zero_page(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xC5 | zpg | CMP oper
fn CMP_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.a();
    let n2 = am::load_zero_page(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xC6 | zpg | DEC oper
fn DEC_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page(mem, pc);
    let val = val.wrapping_sub(1);
    am::store_zero_page(mem, pc, val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xC8 | impl | INY
fn INY_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let val = cpu.y().wrapping_add(1);
    cpu.set_y(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xC9 | # | CMP #oper
fn CMP_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.a();
    let n2 = am::load_immediate(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xCA | impl | DEX
fn DEX_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let val = cpu.x().wrapping_sub(1);
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xCC | abs | CPY oper
fn CPY_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.y();
    let n2 = am::load_absolute(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xCD | abs | CMP oper
fn CMP_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.a();
    let n2 = am::load_absolute(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xCE | abs | DEC oper
fn DEC_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute(mem, pc);
    let val = val.wrapping_sub(1);
    am::store_absolute(mem, pc, val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xD0 | rel | BNE oper
fn BNE_rel(cpu: &mut MOS6502, mem: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    if !cpu.tst_psr_bit(PSR::Z) {
        return Some(am::utils::relative(mem, opc, pc));
    }

    None
}

/// 0xD1 | (ind),Y | CMP (oper),Y
fn CMP_ind_Y_idx(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.a();
    let n2 = am::load_post_indexed_indirect(mem, pc, cpu.y());

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xD5 | zpg,X | CMP oper,X
fn CMP_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.a();
    let n2 = am::load_zero_page_indexed(mem, pc, cpu.x());

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xD6 | zpg,X | DEC oper,X
fn DEC_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page_indexed(mem, pc, cpu.x());
    let val = val.wrapping_sub(1);
    am::store_zero_page_indexed(mem, pc, cpu.x(), val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xD8 | impl | CLD
fn CLD_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    cpu.clr_psr_bit(PSR::D);

    None
}

/// 0xD9 | abs,Y | CMP oper,Y
fn CMP_abs_Y(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.a();
    let n2 = am::load_absolute_indexed(mem, pc, cpu.y());

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xDD | abs,X | CMP oper,X
fn CMP_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.a();
    let n2 = am::load_absolute_indexed(mem, pc, cpu.x());

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xDE | abs,X | DEC oper,X
fn DEC_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute_indexed(mem, pc, cpu.x());
    let val = val.wrapping_sub(1);
    am::store_absolute_indexed(mem, pc, cpu.x(), val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xE0 | # | CPX #oper
fn CPX_imme(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.x();
    let n2 = am::load_immediate(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xE1 | (ind,X) | SBC (oper,X)
fn SBC_idx_ind_X(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0xE4 | zpg | CPX oper
fn CPX_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.x();
    let n2 = am::load_zero_page(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xE5 | zpg | SBC oper
fn SBC_zpg(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0xE6 | zpg | INC oper
fn INC_zpg(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page(mem, pc);
    let val = val.wrapping_add(1);
    am::store_zero_page(mem, pc, val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xE8 | impl | INX
fn INX_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    let val = cpu.x().wrapping_add(1);
    cpu.set_x(val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xE9 | # | SBC #oper
fn SBC_imme(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0xEA | impl | NOP
fn NOP_impl(_: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    None
}

/// 0xEC | abs | CPX oper
fn CPX_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let n1 = cpu.x();
    let n2 = am::load_absolute(mem, pc);

    adder::cmp_core(cpu, n1, n2);

    None
}

/// 0xED | abs | SBC oper
fn SBC_abs(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0xEE | abs | INC oper
fn INC_abs(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute(mem, pc);
    let val = val.wrapping_add(1);
    am::store_absolute(mem, pc, val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xF0 | rel | BEQ oper
fn BEQ_rel(cpu: &mut MOS6502, mem: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    if cpu.tst_psr_bit(PSR::Z) {
        return Some(am::utils::relative(mem, opc, pc));
    }

    None
}

/// 0xF1 | (ind),Y | SBC (oper),Y
fn SBC_ind_Y_idx(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0xF5 | zpg,X | SBC oper,X
fn SBC_zpg_X(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0xF6 | zpg,X | INC oper,X
fn INC_zpg_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_zero_page_indexed(mem, pc, cpu.x());
    let val = val.wrapping_add(1);
    am::store_zero_page_indexed(mem, pc, cpu.x(), val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/// 0xF8 | impl | SED
fn SED_impl(cpu: &mut MOS6502, _: &mut Memory, _: u8, _: LoHi) -> Option<LoHi> {
    cpu.set_psr_bit(PSR::D);

    None
}

/// 0xF9 | abs,Y | SBC oper,Y
fn SBC_abs_Y(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0xFD | abs,X | SBC oper,X
fn SBC_abs_X(_: &mut MOS6502, _: &mut Memory, opc: u8, pc: LoHi) -> Option<LoHi> {
    todo!("TBD: pcode {} @ {:?}", opc, pc)
}

/// 0xFE | abs,X | INC oper,X
fn INC_abs_X(cpu: &mut MOS6502, mem: &mut Memory, _: u8, pc: LoHi) -> Option<LoHi> {
    let val = am::load_absolute_indexed(mem, pc, cpu.x());
    let val = val.wrapping_add(1);
    am::store_absolute_indexed(mem, pc, cpu.x(), val);

    pcr::sync_pcr_n(cpu, val);
    pcr::sync_pcr_z(cpu, val);

    None
}

/*
To regenerated this run
$map = @{}; gc -Raw "D:\src\u\a2600\lib\src\opcodes.json" | ConvertFrom-Json | sort -Property opc | % { $map[$_.opc] = '/* 0x{0:x2} */ &{1}_{2} // {3} | {4}' -f ($_.opc, $_.assembler.split(" ")[0], ((@($_) | % { $_.addressing.replace(",", "_").replace("#", "imme")} | % { if ($_.StartsWith("(") -and $_.EndsWith(")")) { "idx_{0}" -f $_ } elseif ($_.StartsWith("(")) { "{0}_idx" -f $_ } else { $_ } } | % { $_.Replace("(", "").Replace(")", "") }) + ",").PadRight(11, " "), $_.addressing, $_.assembler) }
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
pub const ALL_OPCODE_ROUTINES: &[&OpCode; 0x1_00] = &[
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
    /* 0x10 */ &BPL_rel,        // rel | BPL oper
    /* 0x11 */ &ORA_ind_Y_idx,  // (ind),Y | ORA (oper),Y
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
    /* 0x30 */ &BMI_rel,        // rel | BMI oper
    /* 0x31 */ &AND_ind_Y_idx,  // (ind),Y | AND (oper),Y
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
    /* 0x50 */ &BVC_rel,        // rel | BVC oper
    /* 0x51 */ &EOR_ind_Y_idx,  // (ind),Y | EOR (oper),Y
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
    /* 0x70 */ &BVS_rel,        // rel | BVS oper
    /* 0x71 */ &ADC_ind_Y_idx,  // (ind),Y | ADC (oper),Y
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
    /* 0x90 */ &BCC_rel,        // rel | BCC oper
    /* 0x91 */ &STA_ind_Y_idx,  // (ind),Y | STA (oper),Y
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
    /* 0xB0 */ &BCS_rel,        // rel | BCS oper
    /* 0xB1 */ &LDA_ind_Y_idx,  // (ind),Y | LDA (oper),Y
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
    /* 0xD0 */ &BNE_rel,        // rel | BNE oper
    /* 0xD1 */ &CMP_ind_Y_idx,  // (ind),Y | CMP (oper),Y
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
    /* 0xF0 */ &BEQ_rel,        // rel | BEQ oper
    /* 0xF1 */ &SBC_ind_Y_idx,  // (ind),Y | SBC (oper),Y
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

    fn shift_ops_sync_pcr_c(cpu: &mut MOS6502, val: u8, bit_selector: u8) {
        if tst_bit(val, bit_selector) {
            cpu.set_psr_bit(PSR::C)
        } else {
            cpu.clr_psr_bit(PSR::C)
        }
    }

    pub fn shift_ops_sync_pcr_c_lsb(cpu: &mut MOS6502, val: u8) {
        shift_ops_sync_pcr_c(cpu, val, 0b0000_0001);
    }

    pub fn shift_ops_sync_pcr_c_msb(cpu: &mut MOS6502, val: u8) {
        shift_ops_sync_pcr_c(cpu, val, 0b1000_0000);
    }

    pub fn sync_pcr_z(cpu: &mut MOS6502, val: u8) {
        if val == 0 {
            cpu.set_psr_bit(PSR::Z)
        } else {
            cpu.clr_psr_bit(PSR::Z)
        }
    }

    pub fn sync_pcr_n(cpu: &mut MOS6502, val: u8) {
        if tst_bit(val, 0b1000_0000) {
            cpu.set_psr_bit(PSR::N)
        } else {
            cpu.clr_psr_bit(PSR::N)
        }
    }

    pub fn sync_pcr_v_BIT(cpu: &mut MOS6502, val: u8) {
        if tst_bit(val, 0b0100_0000) {
            cpu.set_psr_bit(PSR::V)
        } else {
            cpu.clr_psr_bit(PSR::V)
        }
    }
}

mod stack {
    use super::*;

    pub const STACK_POINTER_HI: u8 = 0x01;

    pub fn push(cpu: &mut MOS6502, mem: &mut Memory, val: u8) {
        mem.set(LoHi(cpu.s(), STACK_POINTER_HI), 0, val);

        let s = cpu.s().wrapping_sub(1);
        cpu.set_s(s);
    }

    pub fn pop(cpu: &mut MOS6502, mem: &mut Memory) -> u8 {
        let s = cpu.s().wrapping_add(1);
        cpu.set_s(s);

        mem.get(LoHi(s, STACK_POINTER_HI), 0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
    pub fn cmp_core(cpu: &mut MOS6502, n1: u8, n2: u8) {
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
    }
}
