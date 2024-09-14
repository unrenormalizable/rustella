pub const CYCLES_PER_SCAN_LINE: usize = COL_DRAWABLE_AREA_END;
pub const CYCLES_PER_SCAN_LINE_HORIZONTAL_BLANK: usize = COL_HORIZONTAL_BLANK_LEN;
pub const CYCLES_PER_SCAN_LINE_DRAWABLE_AREA: usize = COL_HORIZONTAL_BLANK_LEN;

pub const COL_HORIZONTAL_BLANK_START: usize = 0;
pub const COL_HORIZONTAL_BLANK_LEN: usize = 68;
pub const COL_HORIZONTAL_BLANK_END: usize = COL_HORIZONTAL_BLANK_START + COL_HORIZONTAL_BLANK_LEN;

pub const COL_DRAWABLE_AREA_START: usize = COL_HORIZONTAL_BLANK_LEN;
pub const COL_DRAWABLE_AREA_LEN: usize = 160;
pub const COL_DRAWABLE_AREA_END: usize = COL_DRAWABLE_AREA_START + COL_DRAWABLE_AREA_LEN;

pub const ROW_VERTICAL_SYNC_START: usize = 0;
pub const ROW_VERTICAL_SYNC_LEN: usize = 3;
pub const ROW_VERTICAL_SYNC_END: usize = ROW_VERTICAL_SYNC_START + ROW_VERTICAL_SYNC_LEN;

pub mod ntsc {
    pub const ROW_VERTICAL_BLANK_START: usize = super::ROW_VERTICAL_SYNC_END;
    pub const ROW_VERTICAL_BLANK_LEN: usize = 37;
    pub const ROW_VERTICAL_BLANK_END: usize = ROW_VERTICAL_BLANK_START + ROW_VERTICAL_BLANK_LEN;

    pub const ROW_DRAWABLE_AREA_START: usize = ROW_VERTICAL_BLANK_END;
    pub const ROW_DRAWABLE_AREA_LEN: usize = 192;
    pub const ROW_DRAWABLE_AREA_END: usize = ROW_DRAWABLE_AREA_START + ROW_DRAWABLE_AREA_LEN;

    pub const ROW_OVERSCAN_START: usize = ROW_DRAWABLE_AREA_END;
    pub const ROW_OVERSCAN_LEN: usize = 30;
    pub const ROW_OVERSCAN_END: usize = ROW_OVERSCAN_START + ROW_OVERSCAN_LEN;

    pub const SCAN_LINES: usize = ROW_OVERSCAN_END;
    pub const CYCLES_PER_VERTICAL_SYNC: usize =
        super::ROW_VERTICAL_SYNC_LEN * super::CYCLES_PER_SCAN_LINE;
    pub const CYCLES_PER_VERTICAL_BLANK: usize =
        ROW_VERTICAL_BLANK_LEN * super::CYCLES_PER_SCAN_LINE;
    pub const CYCLES_PER_OVERSCAN: usize = ROW_OVERSCAN_LEN * super::CYCLES_PER_SCAN_LINE;
    pub const CYCLES_PER_DRAWABLE_AREA_AND_HBLANK: usize = CYCLES_PER_FRAME
        - CYCLES_PER_VERTICAL_SYNC
        - CYCLES_PER_VERTICAL_BLANK
        - CYCLES_PER_OVERSCAN;
    pub const CYCLES_PER_FRAME: usize = SCAN_LINES * super::CYCLES_PER_SCAN_LINE;
}

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
}

#[rustfmt::skip]
pub static IMPLEMENTED_REGISTERS: &[bool] = &[
    true , // VSYNC   = $00   0000 00x0   Vertical Sync Set-Clear
    true , // VBLANK  = $01   xx00 00x0   Vertical Blank Set-Clear
    true , // WSYNC   = $02   ---- ----   Wait for Horizontal Blank
    false, // RSYNC   = $03   ---- ----   Reset Horizontal Sync Counter
    false, // NUSIZ0  = $04   00xx 0xxx   Number-Size player/missle 0
    false, // NUSIZ1  = $05   00xx 0xxx   Number-Size player/missle 1
    false, // COLUP0  = $06   xxxx xxx0   Color-Luminance Player 0
    false, // COLUP1  = $07   xxxx xxx0   Color-Luminance Player 1
    false, // COLUPF  = $08   xxxx xxx0   Color-Luminance Playfield
    true , // COLUBK  = $09   xxxx xxx0   Color-Luminance Background
    false, // CTRLPF  = $0A   00xx 0xxx   Control Playfield, Ball, Collisions
    false, // REFP0   = $0B   0000 x000   Reflection Player 0
    false, // REFP1   = $0C   0000 x000   Reflection Player 1
    false, // PF0     = $0D   xxxx 0000   Playfield Register Byte 0
    false, // PF1     = $0E   xxxx xxxx   Playfield Register Byte 1
    false, // PF2     = $0F   xxxx xxxx   Playfield Register Byte 2
    false, // RESP0   = $10   ---- ----   Reset Player 0
    false, // RESP1   = $11   ---- ----   Reset Player 1
    false, // RESM0   = $12   ---- ----   Reset Missle 0
    false, // RESM1   = $13   ---- ----   Reset Missle 1
    false, // RESBL   = $14   ---- ----   Reset Ball
    false, // AUDC0   = $15   0000 xxxx   Audio Control 0
    false, // AUDC1   = $16   0000 xxxx   Audio Control 1
    false, // AUDF0   = $17   000x xxxx   Audio Frequency 0
    false, // AUDF1   = $18   000x xxxx   Audio Frequency 1
    false, // AUDV0   = $19   0000 xxxx   Audio Volume 0
    false, // AUDV1   = $1A   0000 xxxx   Audio Volume 1
    false, // GRP0    = $1B   xxxx xxxx   Graphics Register Player 0
    false, // GRP1    = $1C   xxxx xxxx   Graphics Register Player 1
    false, // ENAM0   = $1D   0000 00x0   Graphics Enable Missle 0
    false, // ENAM1   = $1E   0000 00x0   Graphics Enable Missle 1
    false, // ENABL   = $1F   0000 00x0   Graphics Enable Ball
    false, // HMP0    = $20   xxxx 0000   Horizontal Motion Player 0
    false, // HMP1    = $21   xxxx 0000   Horizontal Motion Player 1
    false, // HMM0    = $22   xxxx 0000   Horizontal Motion Missle 0
    false, // HMM1    = $23   xxxx 0000   Horizontal Motion Missle 1
    false, // HMBL    = $24   xxxx 0000   Horizontal Motion Ball
    false, // VDELP0  = $25   0000 000x   Vertical Delay Player 0
    false, // VDELP1  = $26   0000 000x   Vertical Delay Player 1
    false, // VDELBL  = $27   0000 000x   Vertical Delay Ball
    false, // RESMP0  = $28   0000 00x0   Reset Missle 0 to Player 0
    false, // RESMP1  = $29   0000 00x0   Reset Missle 1 to Player 1
    false, // HMOVE   = $2A   ---- ----   Apply Horizontal Motion
    false, // HMCLR   = $2B   ---- ----   Clear Horizontal Move Registers
    false, // CXCLR   = $2C   ---- ----   Clear Collision Latches
];
