GameBoy Emulator
================

Goal
----
Being able to run a GameBoy game (sound optional)

TODO
----
* Ensure little-endianness(x64) gives the right result for CPURegisters
* Create python generator to create all the opcode functions (resource: [opcodes](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html))
* Implement lcd logic

Resources
---------
* [Game Boy technical reference](https://gbdev.io/pandocs/#video-display)
* [Gameboy CPU (LR35902) instruction set](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Rust implementation](https://mattbruv.github.io/gameboy-crust/)
* [Pyboy implementation](https://github.com/Baekalfen/PyBoy)