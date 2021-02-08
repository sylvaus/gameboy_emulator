GameBoy Emulator
================

Goal
----
Being able to run a GameBoy game (sound optional)

Status
------
![Build](https://github.com/sylvaus/gameboy_emulator/workflows/CMake/badge.svg) [![codecov](https://codecov.io/gh/sylvaus/gameboy_emulator/branch/main/graph/badge.svg?token=HKHSPI4P2V)](https://codecov.io/gh/sylvaus/gameboy_emulator)    
see [Project](https://github.com/sylvaus/gameboy_emulator/projects/1)


Dependencies
------------
* GCC >= 5
* CMake >= 3.10
* conan [installation](https://docs.conan.io/en/latest/installation.html)


Installation
------------
In the folder, run the following commands:
* `mdkir build && cd build`
* `conan install ..`
* `cmake ..` or `cmake .. -DTEST=ON` for tests
* `make`


Important Findings
------------------
### Cycle vs M-Cycles
Cycle refers to an actual clock cycle for a clock frequency of (4.194304 MHz)
and M-Cycle refers to 4 cycles. [source](https://gbdev.io/pandocs/#instruction-set)

M-Cycles are used pretty often since all the instruction cycles are divisible by 4.
The current implementation uses cycle as the [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html) uses it.


Errors in resources
---------------------
### [Gameboy Programming Manual](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
* Chapter 4 page 95: register code for D should be 010 instead of 101 (which is the code for L)


### [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* 0xE2 and 0xF2 should have a length of one instead of 2
* JP (HL) should be JP HL


Resources
---------
* [Gameboy Programming Manual](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
* [Game Boy technical reference](https://gbdev.io/pandocs/#video-display)
* [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Rust implementation](https://mattbruv.github.io/gameboy-crust/)
* [Pyboy implementation](https://github.com/Baekalfen/PyBoy)
