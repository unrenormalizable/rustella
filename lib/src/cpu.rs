use bitflags::bitflags;

bitflags! {
    struct PSR: u8 {
        const C = 1 << 0;
        const Z = 1 << 1;
        const I = 1 << 2;
        const D = 1 << 3;
        const B = 1 << 4;
        const V = 1 << 6;
        const N = 1 << 7;
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct MCS6502 {
    A: u8,
    Y: u8,
    X: u8,
    PC: u16,
    S: u8,
    P: PSR,
}
