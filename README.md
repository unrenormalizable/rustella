# rustella - Atari 2600 Emulator written in Rust

[![CDP](https://github.com/unrenormalizable/rustella/actions/workflows/cdp.yml/badge.svg)](https://github.com/unrenormalizable/rustella/actions/workflows/cdp.yml) [![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg?label=license)](https://creativecommons.org/licenses/by-nc-sa/4.0/)

> Built with [**rust-analyzer.vs**](https://marketplace.visualstudio.com/items?itemName=kitamstudios.RustAnalyzer&ssr=false#overview) (free Visual Studio 2022 Extension)

> [!NOTE]
> Deployed at [rustella](https://rustella.vercel.app/).

## What?

![image](https://github.com/user-attachments/assets/812f6e79-a023-4fff-8241-93f8d1af6d33)

## Why?

Why not?

## Development Principles

- Clean room implementation
  - Not allowed: Referring to any existing emulator code or design. 
  - Allowed: Datasheets / tutorials / discussions on the web that is not emulator code / design.
- In no_std, safe Rust.
- v1: Just sufficient to play the very basic games.
- v1: Web front-end with hardware debugger.

## Specs

- CPU: 6507, 1.19 MHz
- Memory: RAM - 128 Bytes in VLSI, ROM - 4K max
- Graphics: A custom chip named Stella and a graphics clock that run at 1.19 MHz
- Storage Method: Carts
- Input: 2 Joystick ports

## v2+

- Clock cycle accurate

## References

- [2600 Tech specs](https://problemkaputt.de/2k6specs.htm)
- [Reference hardware - 1](https://www.masswerk.at/6502/)
- [Reference hardware - 2](https://stella-emu.github.io/docs/index.html#ROMInfo)
- CPU
  - [6502 Instruction set](https://www.masswerk.at/6502/6502_instruction_set.html)
  - [6502 Family CPU Reference](https://www.pagetable.com/c64ref/6502/?tab=2)
  - [Ultimate Commodore64 Reference Guide](https://github.com/mist64/c64ref)
  - [6502 primer](https://wilsonminesco.com/6502primer/)
  - [The 6502 Microprocessor Resource](http://www.6502.org/)
  - [6502 datasheet](https://www.princeton.edu/~mae412/HANDOUTS/Datasheets/6502.pdf)
  - [The 6502/65C02/65C816 Instruction Set Decoded](https://llx.com/Neil/a2/opcodes.html)
  - Tests
    - [6502 CPU tests](https://codegolf.stackexchange.com/questions/12844/emulate-a-mos-6502-cpu)
    - [6502_65C02_functional_tests](https://github.com/Klaus2m5/6502_65C02_functional_tests)
    - [Tom Harte style JSON](https://github.com/SingleStepTests/65x02)
- Memory
  - [2600 Memory layout](https://forums.atariage.com/topic/192418-mirrored-memory/#comment-2439795)
  - [6502 Memory map requirements](https://wilsonminesco.com/6502primer/MemMapReqs.html)
- TIA
  - [The TIA and the 6502](https://www.randomterrain.com/atari-2600-memories-tutorial-andrew-davie-03.html)
  - [Atari 2600 TIA Hardware Notes](https://www.atarihq.com/danb/files/TIA_HW_Notes.txt)
- References
  - ROMS: Open AI Gym: &lt;user&gt;\\.conda\envs\rlenvs\Lib\site-packages\AutoROM\roms
