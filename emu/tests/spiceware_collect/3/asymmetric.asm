	processor 6502

	include "vcs.h"
	include "macro.h"
	
BLUE         = $9A
	
;------------------------------------------------------------------------------
	SEG
	ORG $F000
	
Reset
; Clear RAM and all TIA registers
	ldx #0 
	lda #0 
Clear           
	sta 0,x 
	inx 
	bne Clear
;------------------------------------------------
; Once-only initialization. . .
	lda #BLUE
	sta COLUBK             ; set the background color
	
	lda #$45
	sta COLUPF
	
	;lda #%00000001
	;sta CTRLPF
	
;------------------------------------------------

StartOfFrame
; Start of new frame
; Start of vertical blank processing
	lda #0
	sta VBLANK
	lda #2
	sta VSYNC
	sta WSYNC
	sta WSYNC
	sta WSYNC               ; 3 scanlines of VSYNC signal
	lda #0
	sta VSYNC
;------------------------------------------------
; 37 scanlines of vertical blank. . .
	ldx #0
VerticalBlank   
	sta WSYNC
	inx
	cpx #37
	bne VerticalBlank
;------------------------------------------------
; Do 192 scanlines of color-changing (our picture)
    ldx #0   ; this counts our scanline number
;--------------------------------------------------------------------------

ALine
    lda galaga_STRIP_0,x
	sta PF0
	lda galaga_STRIP_1,x
	sta PF1
	lda galaga_STRIP_2,x
	sta PF2
	lda galaga_STRIP_3,x
	sta PF0
	lda galaga_STRIP_4,x
	sta PF1
	lda galaga_STRIP_5,x
	sta PF2
	
    sta WSYNC
    inx
	cpx #192
    bne ALine
;--------------------------------------------------------------------------
; CLEAR THE PLAYFIELD REGISTERS
	lda #0
	sta PF0
	sta PF1
	sta PF2
;------------------------------------------------
    lda #%01000010
    sta VBLANK          ; end of screen - enter blanking      
;------------------------------------------------
; 30 scanlines of overscan. . .
	ldx #0
Overscan        
	sta WSYNC
	inx
	cpx #30
	bne Overscan
	jmp StartOfFrame
;------------------------------------------------------------------------------
	include "galaga.asm"
	
	ORG $FFFA
	
InterruptVectors
	.word Reset          ; NMI
	.word Reset          ; RESET
	.word Reset          ; IRQ

END