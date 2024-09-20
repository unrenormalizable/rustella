use super::{core, tv};

pub const TIA_MAX_ADDRESS: usize = 0x003F;

pub mod read_regs {
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
    pub static IMPLEMENTED_REGISTERS: &[(bool, &str); super::TIA_MAX_ADDRESS + 1] = &[
        (true , "VSYNC"),   // = $00   0000 00x0   Vertical Sync Set-Clear
        (true , "VBLANK"),  // = $01   xx00 00x0   Vertical Blank Set-Clear
        (true , "WSYNC"),   // = $02   ---- ----   Wait for Horizontal Blank
        (false, "RSYNC"),   // = $03   ---- ----   Reset Horizontal Sync Counter
        (false, "NUSIZ0"),  // = $04   00xx 0xxx   Number-Size player/missle 0
        (false, "NUSIZ1"),  // = $05   00xx 0xxx   Number-Size player/missle 1
        (false, "COLUP0"),  // = $06   xxxx xxx0   Color-Luminance Player 0
        (false, "COLUP1"),  // = $07   xxxx xxx0   Color-Luminance Player 1
        (false, "COLUPF"),  // = $08   xxxx xxx0   Color-Luminance Playfield
        (true , "COLUBK"),  // = $09   xxxx xxx0   Color-Luminance Background
        (false, "CTRLPF"),  // = $0A   00xx 0xxx   Control Playfield, Ball, Collisions
        (false, "REFP0"),   // = $0B   0000 x000   Reflection Player 0
        (false, "REFP1"),   // = $0C   0000 x000   Reflection Player 1
        (false, "PF0"),     // = $0D   xxxx 0000   Playfield Register Byte 0
        (false, "PF1"),     // = $0E   xxxx xxxx   Playfield Register Byte 1
        (false, "PF2"),     // = $0F   xxxx xxxx   Playfield Register Byte 2
        (false, "RESP0"),   // = $10   ---- ----   Reset Player 0
        (false, "RESP1"),   // = $11   ---- ----   Reset Player 1
        (false, "RESM0"),   // = $12   ---- ----   Reset Missle 0
        (false, "RESM1"),   // = $13   ---- ----   Reset Missle 1
        (false, "RESBL"),   // = $14   ---- ----   Reset Ball
        (false, "AUDC0"),   // = $15   0000 xxxx   Audio Control 0
        (false, "AUDC1"),   // = $16   0000 xxxx   Audio Control 1
        (false, "AUDF0"),   // = $17   000x xxxx   Audio Frequency 0
        (false, "AUDF1"),   // = $18   000x xxxx   Audio Frequency 1
        (false, "AUDV0"),   // = $19   0000 xxxx   Audio Volume 0
        (false, "AUDV1"),   // = $1A   0000 xxxx   Audio Volume 1
        (false, "GRP0"),    // = $1B   xxxx xxxx   Graphics Register Player 0
        (false, "GRP1"),    // = $1C   xxxx xxxx   Graphics Register Player 1
        (false, "ENAM0"),   // = $1D   0000 00x0   Graphics Enable Missle 0
        (false, "ENAM1"),   // = $1E   0000 00x0   Graphics Enable Missle 1
        (false, "ENABL"),   // = $1F   0000 00x0   Graphics Enable Ball
        (false, "HMP0"),    // = $20   xxxx 0000   Horizontal Motion Player 0
        (false, "HMP1"),    // = $21   xxxx 0000   Horizontal Motion Player 1
        (false, "HMM0"),    // = $22   xxxx 0000   Horizontal Motion Missle 0
        (false, "HMM1"),    // = $23   xxxx 0000   Horizontal Motion Missle 1
        (false, "HMBL"),    // = $24   xxxx 0000   Horizontal Motion Ball
        (false, "VDELP0"),  // = $25   0000 000x   Vertical Delay Player 0
        (false, "VDELP1"),  // = $26   0000 000x   Vertical Delay Player 1
        (false, "VDELBL"),  // = $27   0000 000x   Vertical Delay Ball
        (false, "RESMP0"),  // = $28   0000 00x0   Reset Missle 0 to Player 0
        (false, "RESMP1"),  // = $29   0000 00x0   Reset Missle 1 to Player 1
        (false, "HMOVE"),   // = $2A   ---- ----   Apply Horizontal Motion
        (false, "HMCLR"),   // = $2B   ---- ----   Clear Horizontal Move Registers
        (false, "CXCLR"),   // = $2C   ---- ----   Clear Collision Latches
        (false, "????"),    // = $2D
        (false, "????"),    // = $2E
        (false, "????"),    // = $2F
        (false, "????"),    // = $30
        (false, "????"),    // = $31
        (false, "????"),    // = $32
        (false, "????"),    // = $33
        (false, "????"),    // = $34
        (false, "????"),    // = $35
        (false, "????"),    // = $36
        (false, "????"),    // = $37
        (false, "????"),    // = $38
        (false, "????"),    // = $39
        (false, "????"),    // = $3A
        (false, "????"),    // = $3B
        (false, "????"),    // = $3C
        (false, "????"),    // = $3D
        (false, "????"),    // = $3E
        (false, "????"),    // = $3F
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
        68,
        // From https://www.randomterrain.com/atari-2600-memories-tia-color-charts.html
        [
            0xFF000000,
            0xFF000000,
            0xFF1A1A1A,
            0xFF1A1A1A,
            0xFF393939,
            0xFF393939,
            0xFF5B5B5B,
            0xFF5B5B5B,
            0xFF7E7E7E,
            0xFF7E7E7E,
            0xFFA2A2A2,
            0xFFA2A2A2,
            0xFFC7C7C7,
            0xFFC7C7C7,
            0xFFEDEDED,
            0xFFEDEDED,
            0xFF190200,
            0xFF190200,
            0xFF3A1F00,
            0xFF3A1F00,
            0xFF5D4100,
            0xFF5D4100,
            0xFF826400,
            0xFF826400,
            0xFFA78800,
            0xFFA78800,
            0xFFCCAD00,
            0xFFCCAD00,
            0xFFF2D219,
            0xFFF2D219,
            0xFFFEFA40,
            0xFFFEFA40,
            0xFF370000,
            0xFF370000,
            0xFF5E0800,
            0xFF5E0800,
            0xFF832700,
            0xFF832700,
            0xFFA94900,
            0xFFA94900,
            0xFFCF6C00,
            0xFFCF6C00,
            0xFFF58F17,
            0xFFF58F17,
            0xFFFEB438,
            0xFFFEB438,
            0xFFFEDF6F,
            0xFFFEDF6F,
            0xFF470000,
            0xFF470000,
            0xFF730000,
            0xFF730000,
            0xFF981300,
            0xFF981300,
            0xFFBE3216,
            0xFFBE3216,
            0xFFE45335,
            0xFFE45335,
            0xFFFE7657,
            0xFFFE7657,
            0xFFFE9C81,
            0xFFFE9C81,
            0xFFFEC6BB,
            0xFFFEC6BB,
            0xFF440008,
            0xFF440008,
            0xFF6F001F,
            0xFF6F001F,
            0xFF960640,
            0xFF960640,
            0xFFBB2462,
            0xFFBB2462,
            0xFFE14585,
            0xFFE14585,
            0xFFFE67AA,
            0xFFFE67AA,
            0xFFFE8CD6,
            0xFFFE8CD6,
            0xFFFEB7F6,
            0xFFFEB7F6,
            0xFF2D004A,
            0xFF2D004A,
            0xFF570067,
            0xFF570067,
            0xFF7D058C,
            0xFF7D058C,
            0xFFA122B1,
            0xFFA122B1,
            0xFFC743D7,
            0xFFC743D7,
            0xFFED65FE,
            0xFFED65FE,
            0xFFFE8AF6,
            0xFFFE8AF6,
            0xFFFEB5F7,
            0xFFFEB5F7,
            0xFF0D0082,
            0xFF0D0082,
            0xFF3300A2,
            0xFF3300A2,
            0xFF550FC9,
            0xFF550FC9,
            0xFF782DF0,
            0xFF782DF0,
            0xFF9C4EFE,
            0xFF9C4EFE,
            0xFFC372FE,
            0xFFC372FE,
            0xFFEB98FE,
            0xFFEB98FE,
            0xFFFEC0F9,
            0xFFFEC0F9,
            0xFF000091,
            0xFF000091,
            0xFF0A05BD,
            0xFF0A05BD,
            0xFF2822E4,
            0xFF2822E4,
            0xFF4842FE,
            0xFF4842FE,
            0xFF6B64FE,
            0xFF6B64FE,
            0xFF908AFE,
            0xFF908AFE,
            0xFFB7B0FE,
            0xFFB7B0FE,
            0xFFDFD8FE,
            0xFFDFD8FE,
            0xFF000072,
            0xFF000072,
            0xFF001CAB,
            0xFF001CAB,
            0xFF033CD6,
            0xFF033CD6,
            0xFF205EFD,
            0xFF205EFD,
            0xFF4081FE,
            0xFF4081FE,
            0xFF64A6FE,
            0xFF64A6FE,
            0xFF89CEFE,
            0xFF89CEFE,
            0xFFB0F6FE,
            0xFFB0F6FE,
            0xFF00103A,
            0xFF00103A,
            0xFF00316E,
            0xFF00316E,
            0xFF0055A2,
            0xFF0055A2,
            0xFF0579C8,
            0xFF0579C8,
            0xFF239DEE,
            0xFF239DEE,
            0xFF44C2FE,
            0xFF44C2FE,
            0xFF68E9FE,
            0xFF68E9FE,
            0xFF8FFEFE,
            0xFF8FFEFE,
            0xFF001F02,
            0xFF001F02,
            0xFF004326,
            0xFF004326,
            0xFF006957,
            0xFF006957,
            0xFF008D7A,
            0xFF008D7A,
            0xFF1BB19E,
            0xFF1BB19E,
            0xFF3BD7C3,
            0xFF3BD7C3,
            0xFF5DFEE9,
            0xFF5DFEE9,
            0xFF86FEFE,
            0xFF86FEFE,
            0xFF002403,
            0xFF002403,
            0xFF004A05,
            0xFF004A05,
            0xFF00700C,
            0xFF00700C,
            0xFF09952B,
            0xFF09952B,
            0xFF28BA4C,
            0xFF28BA4C,
            0xFF49E06E,
            0xFF49E06E,
            0xFF6CFE92,
            0xFF6CFE92,
            0xFF97FEB5,
            0xFF97FEB5,
            0xFF002102,
            0xFF002102,
            0xFF004604,
            0xFF004604,
            0xFF086B00,
            0xFF086B00,
            0xFF289000,
            0xFF289000,
            0xFF49B509,
            0xFF49B509,
            0xFF6BDB28,
            0xFF6BDB28,
            0xFF8FFE49,
            0xFF8FFE49,
            0xFFBBFE69,
            0xFFBBFE69,
            0xFF001501,
            0xFF001501,
            0xFF103600,
            0xFF103600,
            0xFF305900,
            0xFF305900,
            0xFF537E00,
            0xFF537E00,
            0xFF76A300,
            0xFF76A300,
            0xFF9AC800,
            0xFF9AC800,
            0xFFBFEE1E,
            0xFFBFEE1E,
            0xFFE8FE3E,
            0xFFE8FE3E,
            0xFF1A0200,
            0xFF1A0200,
            0xFF3B1F00,
            0xFF3B1F00,
            0xFF5E4100,
            0xFF5E4100,
            0xFF836400,
            0xFF836400,
            0xFFA88800,
            0xFFA88800,
            0xFFCEAD00,
            0xFFCEAD00,
            0xFFF4D218,
            0xFFF4D218,
            0xFFFEFA40,
            0xFFFEFA40,
            0xFF380000,
            0xFF380000,
            0xFF5F0800,
            0xFF5F0800,
            0xFF842700,
            0xFF842700,
            0xFFAA4900,
            0xFFAA4900,
            0xFFD06B00,
            0xFFD06B00,
            0xFFF68F18,
            0xFFF68F18,
            0xFFFEB439,
            0xFFFEB439,
            0xFFFEDF70,
            0xFFFEDF70,
    ])
}
