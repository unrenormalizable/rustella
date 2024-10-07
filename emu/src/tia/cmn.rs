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
    pub static IMPLEMENTED_REGISTERS: &[(bool, u8, &str, u8); super::TIA_MAX_ADDRESS + 1] = &[
        // W     Valid Mask    Name     Supported Mask
        (true , 0b_0000_0010, "VSYNC",  0b_0000_0010),  // = $00   0000 00x0   Vertical Sync Set-Clear
        (true , 0b_1100_0010, "VBLANK", 0b_0000_0010),  // = $01   xx00 00x0   Vertical Blank Set-Clear
        (true , 0b_0000_0000, "WSYNC",  0b_0000_0000),  // = $02   ---- ----   Wait for Horizontal Blank
        (false, 0b_0000_0000, "RSYNC",  0b_0000_0000),  // = $03   ---- ----   Reset Horizontal Sync Counter
        (false, 0b_0011_0111, "NUSIZ0", 0b_0000_0000),  // = $04   00xx 0xxx   Number-Size player/missle 0
        (false, 0b_0011_0111, "NUSIZ1", 0b_0000_0000),  // = $05   00xx 0xxx   Number-Size player/missle 1
        (true,  0b_1111_1110, "COLUP0", 0b_1111_1111),  // = $06   xxxx xxx0   Color-Luminance Player 0
        (true,  0b_1111_1110, "COLUP1", 0b_1111_1111),  // = $07   xxxx xxx0   Color-Luminance Player 1
        (true,  0b_1111_1110, "COLUPF", 0b_1111_1111),  // = $08   xxxx xxx0   Color-Luminance Playfield
        (true,  0b_1111_1110, "COLUBK", 0b_1111_1111),  // = $09   xxxx xxx0   Color-Luminance Background
        (true,  0b_0011_0111, "CTRLPF", 0b_0000_0011),  // = $0A   00xx 0xxx   Control Playfield, Ball, Collisions
        (false, 0b_0000_1000, "REFP0",  0b_0000_0000),  // = $0B   0000 x000   Reflection Player 0
        (false, 0b_0000_1000, "REFP1",  0b_0000_0000),  // = $0C   0000 x000   Reflection Player 1
        (true,  0b_1111_0000, "PF0",    0b_1111_0000),  // = $0D   xxxx 0000   Playfield Register Byte 0
        (true,  0b_1111_1111, "PF1",    0b_1111_1111),  // = $0E   xxxx xxxx   Playfield Register Byte 1
        (true,  0b_1111_1111, "PF2",    0b_1111_1111),  // = $0F   xxxx xxxx   Playfield Register Byte 2
        (false, 0b_0000_0000, "RESP0",  0b_0000_0000),  // = $10   ---- ----   Reset Player 0
        (false, 0b_0000_0000, "RESP1",  0b_0000_0000),  // = $11   ---- ----   Reset Player 1
        (false, 0b_0000_0000, "RESM0",  0b_0000_0000),  // = $12   ---- ----   Reset Missle 0
        (false, 0b_0000_0000, "RESM1",  0b_0000_0000),  // = $13   ---- ----   Reset Missle 1
        (false, 0b_0000_0000, "RESBL",  0b_0000_0000),  // = $14   ---- ----   Reset Ball
        (false, 0b_0000_1111, "AUDC0",  0b_0000_0000),  // = $15   0000 xxxx   Audio Control 0
        (false, 0b_0000_1111, "AUDC1",  0b_0000_0000),  // = $16   0000 xxxx   Audio Control 1
        (false, 0b_0001_1111, "AUDF0",  0b_0000_0000),  // = $17   000x xxxx   Audio Frequency 0
        (false, 0b_0001_1111, "AUDF1",  0b_0000_0000),  // = $18   000x xxxx   Audio Frequency 1
        (false, 0b_0000_1111, "AUDV0",  0b_0000_0000),  // = $19   0000 xxxx   Audio Volume 0
        (false, 0b_0000_1111, "AUDV1",  0b_0000_0000),  // = $1A   0000 xxxx   Audio Volume 1
        (true,  0b_1111_1111, "GRP0",   0b_1111_1111),  // = $1B   xxxx xxxx   Graphics Register Player 0
        (false, 0b_1111_1111, "GRP1",   0b_0000_0000),  // = $1C   xxxx xxxx   Graphics Register Player 1
        (false, 0b_0000_0010, "ENAM0",  0b_0000_0000),  // = $1D   0000 00x0   Graphics Enable Missle 0
        (false, 0b_0000_0010, "ENAM1",  0b_0000_0000),  // = $1E   0000 00x0   Graphics Enable Missle 1
        (false, 0b_0000_0010, "ENABL",  0b_0000_0000),  // = $1F   0000 00x0   Graphics Enable Ball
        (true,  0b_1111_0000, "HMP0",   0b_1111_0000),  // = $20   xxxx 0000   Horizontal Motion Player 0
        (false, 0b_1111_0000, "HMP1",   0b_0000_0000),  // = $21   xxxx 0000   Horizontal Motion Player 1
        (false, 0b_1111_0000, "HMM0",   0b_0000_0000),  // = $22   xxxx 0000   Horizontal Motion Missle 0
        (false, 0b_1111_0000, "HMM1",   0b_0000_0000),  // = $23   xxxx 0000   Horizontal Motion Missle 1
        (false, 0b_1111_0000, "HMBL",   0b_0000_0000),  // = $24   xxxx 0000   Horizontal Motion Ball
        (false, 0b_0000_0001, "VDELP0", 0b_0000_0000),  // = $25   0000 000x   Vertical Delay Player 0
        (false, 0b_0000_0001, "VDELP1", 0b_0000_0000),  // = $26   0000 000x   Vertical Delay Player 1
        (false, 0b_0000_0001, "VDELBL", 0b_0000_0000),  // = $27   0000 000x   Vertical Delay Ball
        (false, 0b_0000_0010, "RESMP0", 0b_0000_0000),  // = $28   0000 00x0   Reset Missle 0 to Player 0
        (false, 0b_0000_0010, "RESMP1", 0b_0000_0000),  // = $29   0000 00x0   Reset Missle 1 to Player 1
        (false, 0b_0000_0000, "HMOVE",  0b_0000_0000),  // = $2A   ---- ----   Apply Horizontal Motion
        (false, 0b_0000_0000, "HMCLR",  0b_0000_0000),  // = $2B   ---- ----   Clear Horizontal Move Registers
        (false, 0b_0000_0000, "CXCLR",  0b_0000_0000),  // = $2C   ---- ----   Clear Collision Latches
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $2D   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $2E   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $2F   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $30   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $31   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $32   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $33   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $34   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $35   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $36   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $37   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $38   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $39   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $3A   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $3B   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $3C   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $3D   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $3E   ---- ----
        (false, 0b_0000_0000, "????",   0b_0000_0000),  // = $3F   ---- ----
    ];

    // Refer for read registers https://www.atarimax.com/freenet/freenet_material/12.AtariLibrary/2.MiscellaneousTextFiles/showarticle.php?129
}

pub const NTSC_SCANLINES: usize = 262;
pub const NTSC_PIXELS_PER_SCANLINE: usize = 228;

pub type NtscTV = tv::InMemoryTV<NTSC_SCANLINES, NTSC_PIXELS_PER_SCANLINE>;
pub type NtscTIA = core::InMemoryTIA<NTSC_SCANLINES, NTSC_PIXELS_PER_SCANLINE>;
#[rustfmt::skip]
pub fn ntsc_tv_config() -> tv::TVConfig<NTSC_SCANLINES, NTSC_PIXELS_PER_SCANLINE> {
    tv::TVConfig::<NTSC_SCANLINES, NTSC_PIXELS_PER_SCANLINE>::new(
        160,
        // From https://www.randomterrain.com/atari-2600-memories-tia-color-charts.html
        [
            0x000000FF, // $00
            0x1A1A1AFF, // $02
            0x393939FF, // $04
            0x5B5B5BFF, // $06
            0x7E7E7EFF, // $08
            0xA2A2A2FF, // $0A
            0xC7C7C7FF, // $0C
            0xEDEDEDFF, // $0E
            0x190200FF, // $10
            0x3A1F00FF, // $12
            0x5D4100FF, // $14
            0x826400FF, // $16
            0xA78800FF, // $18
            0xCCAD00FF, // $1A
            0xF2D219FF, // $1C
            0xFEFA40FF, // $1E
            0x370000FF, // $20
            0x5E0800FF, // $22
            0x832700FF, // $24
            0xA94900FF, // $26
            0xCF6C00FF, // $28
            0xF58F17FF, // $2A
            0xFEB438FF, // $2C
            0xFEDF6FFF, // $2E
            0x470000FF, // $30
            0x730000FF, // $32
            0x981300FF, // $34
            0xBE3216FF, // $36
            0xE45335FF, // $38
            0xFE7657FF, // $3A
            0xFE9C81FF, // $3C
            0xFEC6BBFF, // $3E
            0x440008FF, // $40
            0x6F001FFF, // $42
            0x960640FF, // $44
            0xBB2462FF, // $46
            0xE14585FF, // $48
            0xFE67AAFF, // $4A
            0xFE8CD6FF, // $4C
            0xFEB7F6FF, // $4E
            0x2D004AFF, // $50
            0x570067FF, // $52
            0x7D058CFF, // $54
            0xA122B1FF, // $56
            0xC743D7FF, // $58
            0xED65FEFF, // $5A
            0xFE8AF6FF, // $5C
            0xFEB5F7FF, // $5E
            0x0D0082FF, // $60
            0x3300A2FF, // $62
            0x550FC9FF, // $64
            0x782DF0FF, // $66
            0x9C4EFEFF, // $68
            0xC372FEFF, // $6A
            0xEB98FEFF, // $6C
            0xFEC0F9FF, // $6E
            0x000091FF, // $70
            0x0A05BDFF, // $72
            0x2822E4FF, // $74
            0x4842FEFF, // $76
            0x6B64FEFF, // $78
            0x908AFEFF, // $7A
            0xB7B0FEFF, // $7C
            0xDFD8FEFF, // $7E
            0x000072FF, // $80
            0x001CABFF, // $82
            0x033CD6FF, // $84
            0x205EFDFF, // $86
            0x4081FEFF, // $88
            0x64A6FEFF, // $8A
            0x89CEFEFF, // $8C
            0xB0F6FEFF, // $8E
            0x00103AFF, // $90
            0x00316EFF, // $92
            0x0055A2FF, // $94
            0x0579C8FF, // $96
            0x239DEEFF, // $98
            0x44C2FEFF, // $9A
            0x68E9FEFF, // $9C
            0x8FFEFEFF, // $9E
            0x001F02FF, // $A0
            0x004326FF, // $A2
            0x006957FF, // $A4
            0x008D7AFF, // $A6
            0x1BB19EFF, // $A8
            0x3BD7C3FF, // $AA
            0x5DFEE9FF, // $AC
            0x86FEFEFF, // $AE
            0x002403FF, // $B0
            0x004A05FF, // $B2
            0x00700CFF, // $B4
            0x09952BFF, // $B6
            0x28BA4CFF, // $B8
            0x49E06EFF, // $BA
            0x6CFE92FF, // $BC
            0x97FEB5FF, // $BE
            0x002102FF, // $C0
            0x004604FF, // $C2
            0x086B00FF, // $C4
            0x289000FF, // $C6
            0x49B509FF, // $C8
            0x6BDB28FF, // $CA
            0x8FFE49FF, // $CC
            0xBBFE69FF, // $CE
            0x001501FF, // $D0
            0x103600FF, // $D2
            0x305900FF, // $D4
            0x537E00FF, // $D6
            0x76A300FF, // $D8
            0x9AC800FF, // $DA
            0xBFEE1EFF, // $DC
            0xE8FE3EFF, // $DE
            0x1A0200FF, // $E0
            0x3B1F00FF, // $E2
            0x5E4100FF, // $E4
            0x836400FF, // $E6
            0xA88800FF, // $E8
            0xCEAD00FF, // $EA
            0xF4D218FF, // $EC
            0xFEFA40FF, // $EE
            0x380000FF, // $F0
            0x5F0800FF, // $F2
            0x842700FF, // $F4
            0xAA4900FF, // $F6
            0xD06B00FF, // $F8
            0xF68F18FF, // $FA
            0xFEB439FF, // $FC
            0xFEDF70FF, // $FE
    ])
}
