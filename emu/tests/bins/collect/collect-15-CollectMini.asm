; CollectMini
; Darrell Spice, Jr
; December 7, 2015
;
; A simple 2600 game of collecting a randomly positioned box
;
; compile using DASM:
; dasm CollectMini.asm -f3 -v0 -sCollectMini.sym -lCollectMini.lst -oCollectMini.bin

;===============================================================================
; Initialize DASM
;===============================================================================
    ; DASM supports a number of processors, this line tells DASM the code
    ; is for the 6502 CPU.  The Atari has a 6507, which is 6502 with an 8K
    ; addressing space.  It also doesn't have any interrupt lines.
    PROCESSOR 6502
    
    ; vcs.h contains the standard definitions for TIA and RIOT registers
    include vcs.h       
    
    ; macro.h contains commonly used routines which aid in coding
    include macro.h
    

    
;===============================================================================
; Define Constants
;===============================================================================

ARENA_HEIGHT    = 200   ; height of gameplay area
PLAYER_COLOR    = $44   ; red
BOX_COLOR       = $84   ; blue



;===============================================================================
; Define RAM Usage
;===============================================================================

    ; define a segment for variables
    ; .U means uninitialized, does not end up in ROM
    SEG.U VARS
    
    ; RAM starts at $80
    ORG $80             

    ; holds background color
BackgroundColor:    ds 1    ; stored in $80

    ; holds X locations in $81-82
ObjectX:            ds 2    ; player0, player1

    ; holds Y locations in $83-84
ObjectY:            ds 2    ; player0, player1

    ; DoDraw storage in $85-86
Player0Draw:        ds 1    ; used for drawing player0
Player1Draw:        ds 1    ; used for drawing player1

    ; DoDraw Graphic Pointers in $87-8a
Player0Ptr:         ds 2    ; used for drawing player0
Player1Ptr:         ds 2    ; used for drawing player1

    ; current random number
Rand8:              ds 1    ; stored in $8b



;===============================================================================
; Define Start of Cartridge
;===============================================================================

    ; define a segment for code
    SEG CODE    
    
    ; 2K ROM starts at $F800, 4K ROM starts at $F000
    ORG $F800
    
    

;===============================================================================
; Subroutines
;===============================================================================

;-------------------------------------------------------------------------------
; PosObject
;----------
; subroutine for setting the X position of any TIA object
; when called, set the following registers:
;   A - holds the X position of the object
;   X - holds which object to position
;       0 = player0
;       1 = player1
;       2 = missile0
;       3 = missile1
;       4 = ball
; the routine will set the coarse X position of the object, as well as the
; fine-tune register.  The fine-tune register will be used to adjust the objects
; final position when an HMOVE is strobed.  The HMOVE must be strobed
; immediately after the WSYNC is strobed.
;
; Note: The X position differs based on the object, for player0 and player1
;       0 is the leftmost pixel while for missile0, missile1 and ball 1 is
;       the leftmost pixel:
;           players     - X range is 0-159
;           missiles    - X range is 1-160
;           ball        - X range is 1-160
;
; Note: Setting players to double or quad size will affect the position of
;       the players.
;-------------------------------------------------------------------------------

PosObject:
        sec
        sta WSYNC
DivideLoop
        sbc #15        ; 2  2 - each time thru this loop takes 5 cycles, which is 
        bcs DivideLoop ; 2  4 - the same amount of time it takes to draw 15 pixels
        eor #7         ; 2  6 - The EOR & ASL statements convert the remainder
        asl            ; 2  8 - of position/15 to the value needed to fine tune
        asl            ; 2 10 - the X position
        asl            ; 2 12
        asl            ; 2 14
        sta.wx HMP0,X  ; 5 19 - store fine tuning of X
        sta RESP0,X    ; 4 23 - set coarse X position of object
        rts            ; 6 29 - ReTurn from Subroutine


;-------------------------------------------------------------------------------
; Random
; --------------
; There is no such thing as Random in computers.  To simulate a random number
; it is common to use a Linear Feedback Shift Register, or LFSR for short.
; We're going to use one that's known as a Galois LFSR:
;       http://en.wikipedia.org/wiki/Linear_feedback_shift_register#Galois_LFSRs
;
; An LFSR will generate a seemingly random sequence of values, but the values
; will repeat after a while.  An 8 bit LFSR will repeat after 255 values.  A
; 16 bit LFSR will repeat after 65535 values.
;
; For CollectMini we're using an 8 bit LFSR that was written by Fred Quimby, aka
; batari.  His original code can be used for both 8 and 16 bit LFSR, so be
; sure to check it out:
;       http://atariage.com/forums/topic/159268-random-numbers/?p=1958751
;
; Fred is also the create of the Harmony Cartridge, which is very handy for
; testing your program on a real Atari, as well as batari BASIC.
;   http://harmony.atariage.com/Site/Harmony.html
;   http://bataribasic.com
;
; In order to use this function you need to allocation a RAM variable called
; Rand8.  Rand8 must also be set to an initial non-zero value, this value is
; known as the seed.
;
; The 255 numbers returned, in order, by subsequent calls to Random:
;       42 21 a4 52 29 a0 50 28 14 0a 05 b6 5b 99 f8 7c 
;       3e 1f bb e9 c0 60 30 18 0c 06 03 b5 ee 77 8f f3 
;       cd d2 69 80 40 20 10 08 04 02 01 b4 5a 2d a2 51 
;       9c 4e 27 a7 e7 c7 d7 df db d9 d8 6c 36 1b b9 e8 
;       74 3a 1d ba 5d 9a 4d 92 49 90 48 24 12 09 b0 58 
;       2c 16 0b b1 ec 76 3b a9 e0 70 38 1c 0e 07 b7 ef 
;       c3 d5 de 6f 83 f5 ce 67 87 f7 cf d3 dd da 6d 82 
;       41 94 4a 25 a6 53 9d fa 7d 8a 45 96 4b 91 fc 7e 
;       3f ab e1 c4 62 31 ac 56 2b a1 e4 72 39 a8 54 2a 
;       15 be 5f 9b f9 c8 64 32 19 b8 5c 2e 17 bf eb c1 
;       d4 6a 35 ae 57 9f fb c9 d0 68 34 1a 0d b2 59 98 
;       4c 26 13 bd ea 75 8e 47 97 ff cb d1 dc 6e 37 af 
;       e3 c5 d6 6b 81 f4 7a 3d aa 55 9e 4f 93 fd ca 65 
;       86 43 95 fe 7f 8b f1 cc 66 33 ad e2 71 8c 46 23 
;       a5 e6 73 8d f2 79 88 44 22 11 bc 5e 2f a3 e5 c6 
;       63 85 f6 7b 89 f0 78 3c 1e 0f b3 ed c2 61 84 ... numbers repeat at this point
;
; This list is valid if you started with a seed value of $84.  If you start with
; a different seed value the list will start with the value after it, such as:
;   seed_value  first_random
;       3e          1f
;       41          94
;       4c          26
;       a5          e6
;-------------------------------------------------------------------------------         
Random:
        lda Rand8
        lsr
        bcc noeor
        eor #$B4 
noeor: 
        sta Rand8
        rts   
        
        

;===============================================================================
; Initialize Atari
;===============================================================================    
    
InitSystem:
        ; CLEAN_START is a macro found in macro.h
        ; it sets all RAM, TIA registers
        ; and CPU registers to 0
        CLEAN_START            
                
        ; set initial player position
        lda #40         
        sta ObjectX
        lda #(ARENA_HEIGHT - HUMAN_HEIGHT)/2
        sta ObjectY
        
        ; set initial box position
        lda #120        
        sta ObjectX+1
        lda #(ARENA_HEIGHT - BOX_HEIGHT)/2
        sta ObjectY+1
        
        lda #12         ; light grey
        sta BackgroundColor
        
        lda #PLAYER_COLOR
        sta COLUP0
        
        lda #BOX_COLOR
        sta COLUP1
        
        sta Rand8       ; also use BOX_COLOR ($84) as seed for the LFSR

        
    ; from here we "fall into" the main loop    
              
    
            
;===============================================================================
; Main Program Loop
;===============================================================================
Main:
        jsr VerticalSync    ; Jump to SubRoutine VerticalSync
        jsr VerticalBlank   ; Jump to SubRoutine VerticalBlank
        jsr Kernel          ; Jump to SubRoutine Kernel
        jsr OverScan        ; Jump to SubRoutine OverScan
        jmp Main            ; JuMP to Main
        
    
    
;========================================
; Sync Signal
; --------------
; Could have used the macro VERTICAL_SYNC here, but by writing our own we can
; use what would have been wasted CPU time to set the timer and blank out the
; player graphics.
;========================================    

VerticalSync:
        lda #2      ; LoaD Accumulator with 2
        sta WSYNC   ; STore Accumulator to WSYNC, any value halts CPU until start of next scanline
        sta VSYNC   ; Accumulator D1=1, turns on Vertical Sync signal
        sta VBLANK  ; Accumulator D1=1, turns on Vertical Blank signal (image output off)
        lda #47
        sta TIM64T  ; set timer for end of Vertical Blank
        sta WSYNC   ; 1st scanline of VSYNC
        sta WSYNC   ; 2nd scanline of VSYNC
        lda #0      ; LoaD Accumulator with 0
        sta GRP0
        sta GRP1
        sta WSYNC   ; 3rd scanline of VSYNC
        sta VSYNC   ; Accumulator D1=0, turns off Vertical Sync signal
        rts
    
    
        
;========================================
; Vertical Blank
; --------------
; game logic runs here.
;
; By calling Random and not using the value, we will advance the LFSR to the
; next value. What this does is impose an outside element, namely the human
; player's reaction time, on the values that are actually used.
;
; If "jsr Random" is removed or commented out, the boxes will end up moving
; to the exact sequence of locations for every game. 
;========================================    
 
VerticalBlank:    
        jsr Random 
        jsr ProcessJoystick
        jsr PositionObjects
VBwait:
        sta WSYNC
        bit TIMINT
        bpl VBwait    ; wait for the timer to denote end of Vertical Blank
        rts
    
        
        
;===============================================================================
; ProcessJoystick
; --------------
; Read joystick and move player
;
; joystick directions are held in the SWCHA register of the RIOT chip.
; Directions are read via the following bit pattern:
;   76543210
;   RLDUrldu    - RIGHT LEFT DOWN UP right left down up
;
; UPPERCASE denotes the left joystick directions
; lowercase denotes the right joystick directions
;
; Note: The values are the opposite of what you might expect. If the direction
;       is held, the bit value will be 0.
;
; Note: Fire buttons are read via INPT4 (left) and INPT5 (right).  They are 
;       not used in Collect.
;===============================================================================
ProcessJoystick:
        lda SWCHA       ; fetch state of both joysticks
        
        asl             ; shift A bits left, R is now in carry bit
        bcs CheckLeft   ; branch if joystick is not held right
        ldy ObjectX     ; get position of player
        iny             ; and move it right
        cpy #160        ; test for edge of screen
        bne SaveX       ; save value as is if we're not at edge
        ldy #0          ; else wrap to left edge of screen
SaveX:  sty ObjectX     ; save player's new X position
        ldy #0          ; turn off reflect of player, which
        sty REFP0       ; makes player image face right
        
CheckLeft:
        asl             ; shift A bits left, L is now in the carry bit
        bcs CheckDown   ; branch if joystick not held left
        ldy ObjectX     ; get the object's X position
        dey             ; and move it left
        cpy #255        ; test for edge of screen
        bne SaveX2      ; save X if we're not at the edge
        ldy #159        ; else wrap to right edge
SaveX2: sty ObjectX     ; save player's new X position
        ldy #8          ; turn on reflect of player, which
        sty REFP0       ; makes player image face left 

CheckDown:
        asl                     ; shift A bits left, D is now in the carry bit
        bcs CheckUp             ; branch if joystick not held down
        ldy ObjectY             ; get the object's Y position
        dey                     ; move it down
        cpy #255                ; test for bottom of screen
        bne SaveY               ; save Y if we're not at the bottom
        ldy #ARENA_HEIGHT       ; else wrap to top
SaveY:  sty ObjectY             ; save Y

CheckUp:
        asl                     ; shift A bits left, U is now in the carry bit
        bcs DoneWithJoystick    ; branch if joystick not held up
        ldy ObjectY             ; get the object's Y position
        iny                     ; move it up
        cpy #ARENA_HEIGHT       ; test for top of screen
        bne SaveY2              ; save Y if we're not at the top
        ldy #0                  ; else wrap to bottom
SaveY2: sty ObjectY             ; save Y

DoneWithJoystick:
        rts
        
        
        
;===============================================================================
; PositionObjects
; --------------
; Updates TIA for X position of both player objects
; Updates Kernel variables for Y position of both player objects
;===============================================================================
PositionObjects:
        ldx #1              ; position players 0 and 1
POloop:
        lda ObjectX,x       ; get the object's X position
        jsr PosObject       ; set coarse X position and fine-tune amount 
        dex                 ; DEcrement X
        bpl POloop          ; Branch PLus so we position all objects
        sta WSYNC           ; wait for end of scanline
        sta HMOVE           ; Tell TIA to use fine-tune values to set final X positions
                
    ; Player0Draw = ARENA_HEIGHT + HUMAN_HEIGHT - Y_position
        lda #(ARENA_HEIGHT + HUMAN_HEIGHT)
        sec
        sbc ObjectY
        sta Player0Draw
        
    ; Set Player0Ptr to proper value for drawing player0
        lda #<(HumanGfx + HUMAN_HEIGHT - 1)
        sec
        sbc ObjectY
        sta Player0Ptr
        lda #>(HumanGfx + HUMAN_HEIGHT - 1)
        sbc #0
        sta Player0Ptr+1
        
    ; Player1Draw = ARENA_HEIGHT + BOX_HEIGHT - Y_position
        lda #(ARENA_HEIGHT + BOX_HEIGHT)
        sec
        sbc ObjectY+1
        sta Player1Draw        
        
    ; Set Player1Ptr to proper value for drawing player1
        lda #<(BoxGfx + BOX_HEIGHT - 1)
        sec
        sbc ObjectY+1
        sta Player1Ptr
        lda #>(BoxGfx + BOX_HEIGHT - 1)
        sbc #0
        sta Player1Ptr+1
        
        rts
        
        
        
;========================================
; Kernel
; --------------
; generate the display
;========================================        
Kernel:    
        ;     
        lda #0 
        ldy #ARENA_HEIGHT   ;        init loop counter
        sta WSYNC
        sta VBLANK          ; 3  3 - turn on video output
        lda BackgroundColor ; 3  6
        sta COLUBK          ; 3  9
 
ArenaLoop:                  ; - 11 - time of longest path here
        lda #HUMAN_HEIGHT-1 ; 2 13 - height of the human graphics, 
        dcp Player0Draw     ; 5 18 - Decrement Player0Draw and compare with height
        bcs DoDrawGrp0      ; 2 20 - (3 21) if Carry is Set then player0 is on current scanline
        lda #0              ; 2 22 - otherwise use 0 to turn off player0
        .byte $2C           ; 4 26 - $2C = BIT with absolute addressing, trick that
                            ;        causes the lda (Player0Ptr),y to be skipped
DoDrawGrp0:                 ;   21 - from bcs DoDrawGRP0
        lda (Player0Ptr),y  ; 5 26 - load the shape for player0
        tax                 ; 2 28 - save in X for update during Horizontal Blanking

        lda #BOX_HEIGHT-1   ; 2 30 - height of the box graphics, subtract 1 due to starting with 0
        dcp Player1Draw     ; 5 35 - Decrement Player1Draw and compare with height
        bcs DoDrawGrp1      ; 2 37 - (3 38) if Carry is Set, then player1 is on current scanline
        lda #0              ; 2 39 - otherwise use 0 to turn off player1
        .byte $2C           ; 4 43 - $2C = BIT with absolute addressing, trick that
                            ;        causes the lda (Player1Ptr),y to be skipped
DoDrawGrp1:                 ;   38 - from bcs DoDrawGrp1
        lda (Player1Ptr),y  ; 5 43 - load the shape for player1
        
        sta WSYNC           ; 3 46/0 - halts CPU until start of next scanline
        stx GRP0            ; 3  3 - draw the human
        sta GRP1            ; 3  6 - draw the box
        dey                 ; 2  8 - update loop counter
        bne ArenaLoop       ; 2 10 - 3 11 if taken
        
        rts                 ; 6 16
    
        
        
;========================================
; Overscan
; --------------
; additional game logic runs here.
;========================================  
OverScan:
        sta WSYNC   ; Wait for SYNC (start of next scanline)
        lda #2      ; LoaD Accumulator with 2
        sta VBLANK  ; STore Accumulator to VBLANK, D1=1 turns image output off
        lda #22
        sta TIM64T  ; set timer for end of Overscan
    
        jsr ProcessCollisions
    
OSwait:
        sta WSYNC
        bit TIMINT
        bpl OSwait  ; wait for the timer to denote end of Overscan
        rts 
       
        
;========================================
; Process Collisions
; --------------
; If player touchs box then change the background color and reposition the box
;========================================

ProcessCollisions:
        bit CXPPMM      ; check to see if player collided with player
                        ; (also used to check if missile collided with missile)
        bpl ExitPC
        
        ; collision detected so change background to the next color
        clc
        lda BackgroundColor
        adc #$10
        sta BackgroundColor
        sta COLUBK

NewX:        
        jsr Random      ; get a random value between 0-255
        cmp #152        ; compare it with 152
        bcs NewX        ; get a new random number if >= 152
        sta ObjectX+1   ; save box's new X location
        
NewY:        
        jsr Random      ; get a random value between 0-255
        cmp #ARENA_HEIGHT-BOX_HEIGHT
        bcs NewY        ; get a new random number if Y position is offscreen
        adc #BOX_HEIGHT ; adjust value so box is fully onscreen
        sta ObjectY+1   ; save box's new Y location
        
ExitPC: sta CXCLR       ; clear collision detection latches
        rts
        
        

;========================================
; Graphics
; --------------
; Yes, the images are stored upside-down.  This is because the Kernel Loop is
; written to count down instead of up.  The reason we count down is because
; the 6507 does an automatic compare with 0 for us, which saves 2 cycles of
; CPU time during time critical processing.
;
; Example code that loops 10 times while counting down:
;       ldy #10
;   Loop:
;       ... do something here
;       dey
;       bne Loop
;
; Example code that loops 10 times while counting up:
;       ldy #0
;   Loop:
;       ... do something here
;       iny
;       cpy #10     <-- this extra instruction is needed when counting up
;       bne Loop
;
;========================================
    align 256
HumanGfx:
        .byte %00011100
        .byte %00011100
        .byte %00011000
        .byte %00011000
        .byte %00011000
        .byte %00011000
        .byte %00011000
        .byte %00011000
        .byte %01011010
        .byte %01011010
        .byte %01011010
        .byte %01011010
        .byte %00111100
        .byte %00111100
        .byte %00000000
        .byte %00000000
        .byte %00011000
        .byte %00011000
        .byte %00011000
        .byte %00011000
HUMAN_HEIGHT = * - HumanGfx    

BoxGfx:
        .byte %11111111
        .byte %11111111
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %10000001
        .byte %11111111
        .byte %11111111
BOX_HEIGHT = * - BoxGfx
        
    
    
;========================================
; Define End of Cartridge
;========================================

    ORG $FFFA        ; set address to 6507 Interrupt Vectors 
    .WORD InitSystem ; NMI
    .WORD InitSystem ; RESET
    .WORD InitSystem ; IRQ
