GameBoy Emulator
================
[![Build and Test Multi OS](https://github.com/sylvaus/gameboy_emulator/actions/workflows/build_and_test_multi_os.yml/badge.svg)](https://github.com/sylvaus/gameboy_emulator/actions/workflows/build_and_test_multi_os.yml)
[![Rust](https://github.com/sylvaus/gameboy_emulator/actions/workflows/coverage.yml/badge.svg)](https://github.com/sylvaus/gameboy_emulator/actions/workflows/coverage.yml)
[![codecov](https://codecov.io/gh/sylvaus/gameboy_emulator/graph/badge.svg?token=ytstiuuAmR)](https://codecov.io/gh/sylvaus/gameboy_emulator)

Goal
----
Being able to run a GameBoy game (sound optional)


Compatibility
-------------
Tested on: Fedora 38

Setup 
-----
TODO


Installation
------------
TODO

Important Findings
------------------
### Endianness
* Gameboy uses little endian

### Cycle vs M-Cycles
Cycle refers to an actual clock cycle for a clock frequency of (4.194304 MHz)
and M-Cycle refers to 4 cycles. [source](https://gbdev.io/pandocs/#instruction-set)

M-Cycles are used pretty often since all the instruction cycles are divisible by 4.
The current implementation uses cycle as the [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html) uses it.

### Halt implementation
Halt instruction will be implemented as follows:
* When HALT is called, the system clock will be stopped and only an interruption will allow the 
  clock to restart
* If the [IME](https://gbdev.io/pandocs/#ime-interrupt-master-enable-flag-write-only) is not set, the interrupt code will 
  not be executed and the process will restart after the HALT
  
Sources:
* [Gameboy Programming Manual](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf) Page 124
* [GitHub Documentation](https://github.com/AntonioND/giibiiadvance/blob/master/docs/TCAGBD.pdf)


Errors in resources
---------------------
### [Gameboy Programming Manual](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
* Chapter 4 page 95: register code for D should be 010 instead of 101 (which is the code for L)
* Chapter 4 page 109: RLCA example, resulting register A should be 0x0B (and not 0x0A)

### [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* 0xE2 and 0xF2 should have a length of one instead of 2
* JP (HL) should be JP HL
* All SRA commands should compute the carry flag (instead of resetting it)
* BIT (HL) should take only 12 cycles


Resources
---------
* [Gameboy Programming Manual](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
* [Game Boy technical reference](https://gbdev.io/pandocs)
* [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Gameboy Instruction set with explanation](https://meganesulli.com/generate-gb-opcodes/)
* [Rust implementation](https://mattbruv.github.io/gameboy-crust/)
* [Pyboy implementation](https://github.com/Baekalfen/PyBoy)
