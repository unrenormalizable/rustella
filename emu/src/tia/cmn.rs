use super::{core, tv};

pub const TIA_MAX_ADDRESS: usize = 0x003F;

pub mod regs {
    /// $00   0000 00x0   Vertical Sync Set-Clear
    pub const VSYNC: usize = 0x00;
    /// $01   xx00 00x0   Vertical Blank Set-Clear
    pub const VBLANK: usize = 0x01;
    /// $02   ---- ----   Wait for Horizontal Blank
    pub const WSYNC: usize = 0x02;
    /// $03   ---- ----   Reset Horizontal Sync Counter
    pub const RSYNC: usize = 0x03;
    /// $04   00xx 0xxx   Number-Size player/missle 0
    pub const NUSIZ0: usize = 0x04;
    /// $05   00xx 0xxx   Number-Size player/missle 1
    pub const NUSIZ1: usize = 0x05;
    /// $06   xxxx xxx0   Color-Luminance Player 0
    pub const COLUP0: usize = 0x06;
    /// $07   xxxx xxx0   Color-Luminance Player 1
    pub const COLUP1: usize = 0x07;
    /// $08   xxxx xxx0   Color-Luminance Playfield
    pub const COLUPF: usize = 0x08;
    /// $09   xxxx xxx0   Color-Luminance Background
    pub const COLUBK: usize = 0x09;
    /// $0A   00xx 0xxx   Control Playfield, Ball, Collisions
    pub const CTRLPF: usize = 0x0A;
    /// $0B   0000 x000   Reflection Player 0
    pub const REFP0: usize = 0x0B;
    /// $0C   0000 x000   Reflection Player 1
    pub const REFP1: usize = 0x0C;
    /// $0D   xxxx 0000   Playfield Register Byte 0
    pub const PF0: usize = 0x0D;
    /// $0E   xxxx xxxx   Playfield Register Byte 1
    pub const PF1: usize = 0x0E;
    /// $0F   xxxx xxxx   Playfield Register Byte 2
    pub const PF2: usize = 0x0F;
    /// $10   ---- ----   Reset Player 0
    pub const RESP0: usize = 0x10;
    /// $11   ---- ----   Reset Player 1
    pub const RESP1: usize = 0x11;
    /// $12   ---- ----   Reset Missle 0
    pub const RESM0: usize = 0x12;
    /// $13   ---- ----   Reset Missle 1
    pub const RESM1: usize = 0x13;
    /// $14   ---- ----   Reset Ball
    pub const RESBL: usize = 0x14;
    /// $15   0000 xxxx   Audio Control 0
    pub const AUDC0: usize = 0x15;
    /// $16   0000 xxxx   Audio Control 1
    pub const AUDC1: usize = 0x16;
    /// $17   000x xxxx   Audio Frequency 0
    pub const AUDF0: usize = 0x17;
    /// $18   000x xxxx   Audio Frequency 1
    pub const AUDF1: usize = 0x18;
    /// $19   0000 xxxx   Audio Volume 0
    pub const AUDV0: usize = 0x19;
    /// $1A   0000 xxxx   Audio Volume 1
    pub const AUDV1: usize = 0x1A;
    /// $1B   xxxx xxxx   Graphics Register Player 0
    pub const GRP0: usize = 0x1B;
    /// $1C   xxxx xxxx   Graphics Register Player 1
    pub const GRP1: usize = 0x1C;
    /// $1D   0000 00x0   Graphics Enable Missle 0
    pub const ENAM0: usize = 0x1D;
    /// $1E   0000 00x0   Graphics Enable Missle 1
    pub const ENAM1: usize = 0x1E;
    /// $1F   0000 00x0   Graphics Enable Ball
    pub const ENABL: usize = 0x1F;
    /// $20   xxxx 0000   Horizontal Motion Player 0
    pub const HMP0: usize = 0x20;
    /// $21   xxxx 0000   Horizontal Motion Player 1
    pub const HMP1: usize = 0x21;
    /// $22   xxxx 0000   Horizontal Motion Missle 0
    pub const HMM0: usize = 0x22;
    /// $23   xxxx 0000   Horizontal Motion Missle 1
    pub const HMM1: usize = 0x23;
    /// $24   xxxx 0000   Horizontal Motion Ball
    pub const HMBL: usize = 0x24;
    /// $25   0000 000x   Vertical Delay Player 0
    pub const VDELP0: usize = 0x25;
    /// $26   0000 000x   Vertical Delay Player 1
    pub const VDELP1: usize = 0x26;
    /// $27   0000 000x   Vertical Delay Ball
    pub const VDELBL: usize = 0x27;
    /// $28   0000 00x0   Reset Missle 0 to Player 0
    pub const RESMP0: usize = 0x28;
    /// $29   0000 00x0   Reset Missle 1 to Player 1
    pub const RESMP1: usize = 0x29;
    /// $2A   ---- ----   Apply Horizontal Motion
    pub const HMOVE: usize = 0x2A;
    /// $2B   ---- ----   Clear Horizontal Move Registers
    pub const HMCLR: usize = 0x2B;
    /// $2C   ---- ----   Clear Collision Latches
    pub const CXCLR: usize = 0x2C;

    #[rustfmt::skip]
    pub static IMPLEMENTED_REGISTERS: &[(bool, u8, &str); super::TIA_MAX_ADDRESS + 1] = &[
        (true , 0b_0000_0010, "VSYNC"),   // = $00   0000 00x0   Vertical Sync Set-Clear
        (true , 0b_1100_0010, "VBLANK"),  // = $01   xx00 00x0   Vertical Blank Set-Clear
        (true , 0b_0000_0000, "WSYNC"),   // = $02   ---- ----   Wait for Horizontal Blank
        (false, 0b_0000_0000, "RSYNC"),   // = $03   ---- ----   Reset Horizontal Sync Counter
        (false, 0b_0011_0111, "NUSIZ0"),  // = $04   00xx 0xxx   Number-Size player/missle 0
        (false, 0b_0011_0111, "NUSIZ1"),  // = $05   00xx 0xxx   Number-Size player/missle 1
        (false, 0b_1111_1110, "COLUP0"),  // = $06   xxxx xxx0   Color-Luminance Player 0
        (false, 0b_1111_1110, "COLUP1"),  // = $07   xxxx xxx0   Color-Luminance Player 1
        (false, 0b_1111_1110, "COLUPF"),  // = $08   xxxx xxx0   Color-Luminance Playfield
        (true , 0b_1111_1110, "COLUBK"),  // = $09   xxxx xxx0   Color-Luminance Background
        (false, 0b_0011_0111, "CTRLPF"),  // = $0A   00xx 0xxx   Control Playfield, Ball, Collisions
        (false, 0b_0000_1000, "REFP0"),   // = $0B   0000 x000   Reflection Player 0
        (false, 0b_0000_1000, "REFP1"),   // = $0C   0000 x000   Reflection Player 1
        (false, 0b_1111_0000, "PF0"),     // = $0D   xxxx 0000   Playfield Register Byte 0
        (false, 0b_1111_1111, "PF1"),     // = $0E   xxxx xxxx   Playfield Register Byte 1
        (false, 0b_1111_1111, "PF2"),     // = $0F   xxxx xxxx   Playfield Register Byte 2
        (false, 0b_0000_0000, "RESP0"),   // = $10   ---- ----   Reset Player 0
        (false, 0b_0000_0000, "RESP1"),   // = $11   ---- ----   Reset Player 1
        (false, 0b_0000_0000, "RESM0"),   // = $12   ---- ----   Reset Missle 0
        (false, 0b_0000_0000, "RESM1"),   // = $13   ---- ----   Reset Missle 1
        (false, 0b_0000_0000, "RESBL"),   // = $14   ---- ----   Reset Ball
        (false, 0b_0000_1111, "AUDC0"),   // = $15   0000 xxxx   Audio Control 0
        (false, 0b_0000_1111, "AUDC1"),   // = $16   0000 xxxx   Audio Control 1
        (false, 0b_0001_1111, "AUDF0"),   // = $17   000x xxxx   Audio Frequency 0
        (false, 0b_0001_1111, "AUDF1"),   // = $18   000x xxxx   Audio Frequency 1
        (false, 0b_0000_1111, "AUDV0"),   // = $19   0000 xxxx   Audio Volume 0
        (false, 0b_0000_1111, "AUDV1"),   // = $1A   0000 xxxx   Audio Volume 1
        (false, 0b_1111_1111, "GRP0"),    // = $1B   xxxx xxxx   Graphics Register Player 0
        (false, 0b_1111_1111, "GRP1"),    // = $1C   xxxx xxxx   Graphics Register Player 1
        (false, 0b_0000_0010, "ENAM0"),   // = $1D   0000 00x0   Graphics Enable Missle 0
        (false, 0b_0000_0010, "ENAM1"),   // = $1E   0000 00x0   Graphics Enable Missle 1
        (false, 0b_0000_0010, "ENABL"),   // = $1F   0000 00x0   Graphics Enable Ball
        (false, 0b_1111_0000, "HMP0"),    // = $20   xxxx 0000   Horizontal Motion Player 0
        (false, 0b_1111_0000, "HMP1"),    // = $21   xxxx 0000   Horizontal Motion Player 1
        (false, 0b_1111_0000, "HMM0"),    // = $22   xxxx 0000   Horizontal Motion Missle 0
        (false, 0b_1111_0000, "HMM1"),    // = $23   xxxx 0000   Horizontal Motion Missle 1
        (false, 0b_1111_0000, "HMBL"),    // = $24   xxxx 0000   Horizontal Motion Ball
        (false, 0b_0000_0001, "VDELP0"),  // = $25   0000 000x   Vertical Delay Player 0
        (false, 0b_0000_0001, "VDELP1"),  // = $26   0000 000x   Vertical Delay Player 1
        (false, 0b_0000_0001, "VDELBL"),  // = $27   0000 000x   Vertical Delay Ball
        (false, 0b_0000_0010, "RESMP0"),  // = $28   0000 00x0   Reset Missle 0 to Player 0
        (false, 0b_0000_0010, "RESMP1"),  // = $29   0000 00x0   Reset Missle 1 to Player 1
        (false, 0b_0000_0000, "HMOVE"),   // = $2A   ---- ----   Apply Horizontal Motion
        (false, 0b_0000_0000, "HMCLR"),   // = $2B   ---- ----   Clear Horizontal Move Registers
        (false, 0b_0000_0000, "CXCLR"),   // = $2C   ---- ----   Clear Collision Latches
        (false, 0b_0000_0000, "????"),    // = $2D   ---- ----
        (false, 0b_0000_0000, "????"),    // = $2E   ---- ----
        (false, 0b_0000_0000, "????"),    // = $2F   ---- ----
        (false, 0b_0000_0000, "????"),    // = $30   ---- ----
        (false, 0b_0000_0000, "????"),    // = $31   ---- ----
        (false, 0b_0000_0000, "????"),    // = $32   ---- ----
        (false, 0b_0000_0000, "????"),    // = $33   ---- ----
        (false, 0b_0000_0000, "????"),    // = $34   ---- ----
        (false, 0b_0000_0000, "????"),    // = $35   ---- ----
        (false, 0b_0000_0000, "????"),    // = $36   ---- ----
        (false, 0b_0000_0000, "????"),    // = $37   ---- ----
        (false, 0b_0000_0000, "????"),    // = $38   ---- ----
        (false, 0b_0000_0000, "????"),    // = $39   ---- ----
        (false, 0b_0000_0000, "????"),    // = $3A   ---- ----
        (false, 0b_0000_0000, "????"),    // = $3B   ---- ----
        (false, 0b_0000_0000, "????"),    // = $3C   ---- ----
        (false, 0b_0000_0000, "????"),    // = $3D   ---- ----
        (false, 0b_0000_0000, "????"),    // = $3E   ---- ----
        (false, 0b_0000_0000, "????"),    // = $3F   ---- ----
    ];
}

pub const NTSC_SCANLINES: usize = 262;
pub const NTSC_PIXELS_PER_SCANLINE: usize = 228;

pub type NtscTV = tv::InMemoryTV<NTSC_SCANLINES, NTSC_PIXELS_PER_SCANLINE>;
pub type NtscTIA = core::InMemoryTIA<NTSC_SCANLINES, NTSC_PIXELS_PER_SCANLINE>;
#[rustfmt::skip]
pub fn ntsc_tv_config() -> tv::TVConfig<NTSC_SCANLINES, NTSC_PIXELS_PER_SCANLINE> {
    tv::TVConfig::<NTSC_SCANLINES, NTSC_PIXELS_PER_SCANLINE>::new(
        3,
        37,
        192,
        160,
        // From https://www.randomterrain.com/atari-2600-memories-tia-color-charts.html
        [
            0xFF000000, // $00
            0xFF1A1A1A, // $02
            0xFF393939, // $04
            0xFF5B5B5B, // $06
            0xFF7E7E7E, // $08
            0xFFA2A2A2, // $0A
            0xFFC7C7C7, // $0C
            0xFFEDEDED, // $0E
            0xFF190200, // $10
            0xFF3A1F00, // $12
            0xFF5D4100, // $14
            0xFF826400, // $16
            0xFFA78800, // $18
            0xFFCCAD00, // $1A
            0xFFF2D219, // $1C
            0xFFFEFA40, // $1E
            0xFF370000, // $20
            0xFF5E0800, // $22
            0xFF832700, // $24
            0xFFA94900, // $26
            0xFFCF6C00, // $28
            0xFFF58F17, // $2A
            0xFFFEB438, // $2C
            0xFFFEDF6F, // $2E
            0xFF470000, // $30
            0xFF730000, // $32
            0xFF981300, // $34
            0xFFBE3216, // $36
            0xFFE45335, // $38
            0xFFFE7657, // $3A
            0xFFFE9C81, // $3C
            0xFFFEC6BB, // $3E
            0xFF440008, // $40
            0xFF6F001F, // $42
            0xFF960640, // $44
            0xFFBB2462, // $46
            0xFFE14585, // $48
            0xFFFE67AA, // $4A
            0xFFFE8CD6, // $4C
            0xFFFEB7F6, // $4E
            0xFF2D004A, // $50
            0xFF570067, // $52
            0xFF7D058C, // $54
            0xFFA122B1, // $56
            0xFFC743D7, // $58
            0xFFED65FE, // $5A
            0xFFFE8AF6, // $5C
            0xFFFEB5F7, // $5E
            0xFF0D0082, // $60
            0xFF3300A2, // $62
            0xFF550FC9, // $64
            0xFF782DF0, // $66
            0xFF9C4EFE, // $68
            0xFFC372FE, // $6A
            0xFFEB98FE, // $6C
            0xFFFEC0F9, // $6E
            0xFF000091, // $70
            0xFF0A05BD, // $72
            0xFF2822E4, // $74
            0xFF4842FE, // $76
            0xFF6B64FE, // $78
            0xFF908AFE, // $7A
            0xFFB7B0FE, // $7C
            0xFFDFD8FE, // $7E
            0xFF000072, // $80
            0xFF001CAB, // $82
            0xFF033CD6, // $84
            0xFF205EFD, // $86
            0xFF4081FE, // $88
            0xFF64A6FE, // $8A
            0xFF89CEFE, // $8C
            0xFFB0F6FE, // $8E
            0xFF00103A, // $90
            0xFF00316E, // $92
            0xFF0055A2, // $94
            0xFF0579C8, // $96
            0xFF239DEE, // $98
            0xFF44C2FE, // $9A
            0xFF68E9FE, // $9C
            0xFF8FFEFE, // $9E
            0xFF001F02, // $A0
            0xFF004326, // $A2
            0xFF006957, // $A4
            0xFF008D7A, // $A6
            0xFF1BB19E, // $A8
            0xFF3BD7C3, // $AA
            0xFF5DFEE9, // $AC
            0xFF86FEFE, // $AE
            0xFF002403, // $B0
            0xFF004A05, // $B2
            0xFF00700C, // $B4
            0xFF09952B, // $B6
            0xFF28BA4C, // $B8
            0xFF49E06E, // $BA
            0xFF6CFE92, // $BC
            0xFF97FEB5, // $BE
            0xFF002102, // $C0
            0xFF004604, // $C2
            0xFF086B00, // $C4
            0xFF289000, // $C6
            0xFF49B509, // $C8
            0xFF6BDB28, // $CA
            0xFF8FFE49, // $CC
            0xFFBBFE69, // $CE
            0xFF001501, // $D0
            0xFF103600, // $D2
            0xFF305900, // $D4
            0xFF537E00, // $D6
            0xFF76A300, // $D8
            0xFF9AC800, // $DA
            0xFFBFEE1E, // $DC
            0xFFE8FE3E, // $DE
            0xFF1A0200, // $E0
            0xFF3B1F00, // $E2
            0xFF5E4100, // $E4
            0xFF836400, // $E6
            0xFFA88800, // $E8
            0xFFCEAD00, // $EA
            0xFFF4D218, // $EC
            0xFFFEFA40, // $EE
            0xFF380000, // $F0
            0xFF5F0800, // $F2
            0xFF842700, // $F4
            0xFFAA4900, // $F6
            0xFFD06B00, // $F8
            0xFFF68F18, // $FA
            0xFFFEB439, // $FC
            0xFFFEDF70, // $FE
    ])
}
