# Atari 2600 Emulator

[![CDP](https://github.com/unrenormalizable/atari2600.rs/actions/workflows/cdp.yml/badge.svg)](https://github.com/unrenormalizable/atari2600.rs/actions/workflows/cdp.yml) [![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg?label=license)](https://creativecommons.org/licenses/by-nc-sa/4.0/)

> Built with [**rust-analyzer.vs**](https://marketplace.visualstudio.com/items?itemName=kitamstudios.RustAnalyzer&ssr=false#overview) (free)

## Development Principles

- No referring to any existing A2600 emulator code 
  - Allowed: Datasheets and tutorials on the web.
- v1: Just sufficient to play the very basic games.

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
- [6502 Instruction set](https://www.masswerk.at/6502/6502_instruction_set.html)
- [The 6502/65C02/65C816 Instruction Set Decoded](https://llx.com/Neil/a2/opcodes.html)
- [The 6502 Microprocessor Resource](http://www.6502.org/)
- [6502 datasheet](https://www.princeton.edu/~mae412/HANDOUTS/Datasheets/6502.pdf)
- [2600 Memory layout](https://forums.atariage.com/topic/192418-mirrored-memory/#comment-2439795)
- [6502 Memory map requirements](https://wilsonminesco.com/6502primer/MemMapReqs.html)
- [Stella reference emulator](https://stella-emu.github.io/docs/index.html#ROMInfo)
- ROMS: Open AI Gym: &lt;user&gt;\\.conda\envs\rlenvs\Lib\site-packages\AutoROM\roms
- [iridium VM](https://gitlab.com/subnetzero/iridium)
