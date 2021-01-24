GameBoy Emulator
================

Goal
----
Being able to run a GameBoy game (sound optional)

Status
------
see [Project](https://github.com/sylvaus/gameboy_emulator/projects/1)

Important Findings
------------------
### Cycle vs M-Cycles
Cycle refers to an actual clock cycle for a clock frequency of (4.194304 MHz)
and M-Cycle refers to 4 cycles. [source](https://gbdev.io/pandocs/#instruction-set)

M-Cycles are used pretty often since all the instruction cycles are divisible by 4.
The current implementation uses cycle as the [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html) uses it.

Resources
---------
* [Gameboy Programming Manual](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
* [Game Boy technical reference](https://gbdev.io/pandocs/#video-display)
* [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Rust implementation](https://mattbruv.github.io/gameboy-crust/)
* [Pyboy implementation](https://github.com/Baekalfen/PyBoy)
