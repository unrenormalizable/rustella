# TIA Technical Notes

- Display [From [2004_28c3-4711-Ultimate_Atari_2600_Talk ](link_generated_on_download)]

  <img src="https://remnote-user-data.s3.amazonaws.com/wySbIIHb79aRNF_M6IZegsR3n7qXXKPAQ4tOctIyaq5makgJ_sRfoFMEHcBfjtVQ0VRFlZpB_bR79eD8muqh4LW1Gw1GpiV4K7I2YdVdyOfg8hor6LSXgELYOjNzVBJI.png" alt="TV frame" width="400"/>

- TV
    - NTSC / PAL / SECAM
    - drawn a line at a time
    - CPU put color/intensity data for line into TIA ⇒ TIA convert data to video signals ⇒ TV renders signals
    - TIA has data only for current line
        - unless there is a change in state, next scanline is identical
    -  __**Interlacing**__  makes picture higher resolution

- **Control / Data flow** 
    - Docs
      - [TIA Hardware Notes](https://www.atarihq.com/danb/files/TIA_HW_Notes.txt)
      - [Stella Programmer's Guide (Unofficial HTML version)
	  ](https://www.alienbill.com/2600/101/docs/stella.html#tiaprog)
      - [www.atariage.com/2600/programming/2600 101/02breach.html](https://www.atariage.com/2600/programming/2600_101/02breach.html)
      - [I don't understand screen synchronisation - Atari 2600 Programming - AtariAge Forums](https://forums.atariage.com/topic/324606-i-dont-understand-screen-synchronisation/?isPin=false?aliasId=xp3jELn6BjDenOtjL)
    - TV picture: 60fps / 50fps, each frame drawn line by line
    - Instructions
        - VSYNC = TIA indicates to TV to start a new frame
        - VSYNC on + 3 x WSYNC + VSYNC off = TV gets starts a new frame
        - VBLANK on = beam doesn't draw but scans
        - WSYNC = halt CPU till beam returns to start of next line (h.blank + display)
    - 1 frame structure (Atari's research data shows max TV compat) 
        - ![](https://remnote-user-data.s3.amazonaws.com/Hicks-Cw6w6fNK_BFr72CacRgYqLfVCrlZJkWhCPUxjMWEWLbgSpeXolPfWfc5UkIF68ZxdqQ8YHFbGdFzwBf10Ktn5vx3WBolY4KmrsUfcqSeRoKv9MygLfAlaZls5h.png)
        - VerticalSync = VSYNC on + 3 x WSYNC / game logic + VSYNC off 
            - ![](https://remnote-user-data.s3.amazonaws.com/bVIXlowFCkEvGEWp_qIwxOvAWNPhm_KGjeLNhrtffITsaWAgbF3ccmAoliNvCsGxPr7HY7CSaS4MJYR1XQjeG45-XXICJ2iuLTcY487u8u1J2zDwV96pGJPYD5IrSLhC.png)
        - VerticalBlank = 37/45 WSYNC / game logic
            - ![](https://remnote-user-data.s3.amazonaws.com/h0bBRzUMZsjJBieXtSVwUyr820aXyfCeIeDtcZE74bkmhb-d0m3ImEOCmBkadDFcw200T3R4n_q-Daj9OpZ7nbR9mgVlEkemz6qF6dtHaVAhQmKA6CP5MMLJBpU_LQa9.png)
        - Display: VBLANK off + 192/228 display 
        - Overscan: VBLANK on + 30/36 WSYNC 
    - 1 line = 68 cycle horizontal blank + 160 cycle display
    - horizontal timing handled by TIA
        - WSYNC stops CPU till start of next line
    - vertical timing by CPU
        - after completion of a frame => VSYNC + VBLANK + pic + VBLANK
- **TIA CPU run in parallel shared clock** 
    - 1 CPU cycle = 3 TIA cycle
        - 70/85 blank lines for processing game logic
    - 1 TIA cycle = 1 pixel
        - It is possible of the code to know which pixel TIA is drawing
> min load/store = 5 cycles = 15 pixels ⇒ quickest changes 11 times per second per scanline

- TIA Registers
    - TIA State
        - Video: color + brightness x position x size x type
            - background, playfield, 2 x sprites, 2 x missiles, 1 x ball
            - properties of objects [Presentations here](https://spiceware.org/)
        - Audio: vol + pitch + type
        - Collisions detected by TIA, read by CPU
        - Input ports ⇒ status of handheld controllers
    - Properties
        - Latched
        - Addressed as part of address space
    - Types
        - CPU Write Strobe - Just write, data ignored
        - CPU Write - 
        - CPU Read - Collision registers + input port registers

- Timing - [Let's Make a Game! - Step 1: Generate a Stable Display](https://www.randomterrain.com/atari-2600-lets-make-a-game-spiceware-01.html)
    - Horizontal - Auto handled by TIA
        - Same for all NTSC / PAL / SECAM
        - TIA has pulse counter
        - Line = turn beam off [68 cycles] ⇒ Electron beam to right edge [160 cycles] ⇒ HSYNC (beam moves to the left edge, next line)
        - CPU synced with TIA every line ⇒ WSYNC = CPU halt + Wait for HSYNC ⇒ CPU resumes start of h.blank
    - Vertical - Controlled by CPU
        - see up: [1 frame structure (Atari's research data shows max TV compat) ](link_generated_on_download)
> CPU is creating the frame ⇔ CPU is the graphics H/W ⇔ TV has no concept of frames

- References
    - Test code
        - Blank screen
        - Rainbow - [Let's Make a Game! - Step 1: Generate a Stable Display](https://www.randomterrain.com/atari-2600-lets-make-a-game-spiceware-01.html)
        - [Collect Tutorial Index](https://forums.atariage.com/blogs/entry/13884-collect-tutorial-index/)
        - Colin's pic with the woman
        - [high resolution grid](https://www.biglist.com/lists/stella/archives/199810/msg00073.html)
        - [www.atariage.com/2600/programming/2600 101/bin/clock003.asm](https://www.atariage.com/2600/programming/2600_101/bin/clock003.asm)
        - test cart from [www.qotile.net/minidig/docs/tia color.html](https://www.qotile.net/minidig/docs/tia_color.html)
        - [An Atari 2600 "Hello, World!" program(it indeed prints "HELLO WORLD" vertically, twice) · GitHub](https://gist.github.com/chesterbr/5864935)
        - [I don't understand screen synchronisation - Atari 2600 Programming - AtariAge Forums](https://forums.atariage.com/topic/324606-i-dont-understand-screen-synchronisation/)
        - [www.atariage.com/2600/programming/2600 101/02breach.html](https://www.atariage.com/2600/programming/2600_101/02breach.html)
    - Docs
        - Racing the beam: [- YouTube](https://www.youtube.com/watch?v=sJFnWZH5FXc)
        - [Atari 2600 VCS Programming | PPT](https://www.slideshare.net/slideshow/atari-2600programming/23550414)
        - [Atari 2600 Hardware Design: Making Something out of (Almost) Nothing | Big Mess o' Wires](https://www.bigmessowires.com/2023/01/11/atari-2600-hardware-design-making-something-out-of-almost-nothing/)
        - [Atari 2600 Programming for Newbies - Revised Edition](link_generated_on_download)
        - [2004_28c3-4711-Ultimate_Atari_2600_Talk ](link_generated_on_download)
        - [www.atariage.com/2600/programming/2600 101/02breach.html](https://www.atariage.com/2600/programming/2600_101/02breach.html)
        - [www.qotile.net/minidig/docs/tia color.html](https://www.qotile.net/minidig/docs/tia_color.html)
