use log::trace;
use crate::memory::Memory;
use crate::memory::registers::Registers;

/// 0x0 NOP
pub fn nop_000(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x1 LD BC d16
pub fn ld_001(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.set_bc(memory.get_16_bits(registers.pc + 1));
    registers.pc = registers.pc + 3;
    return 12;
}

/// 0x2 LD (BC) A
pub fn ld_002(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_bc(), registers.a);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x6 LD B d8
pub fn ld_006(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.b = memory.get(registers.pc + 1);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x8 LD (a16) SP
pub fn ld_008(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set_16_bits(memory.get_16_bits(registers.pc + 1), registers.sp);
    registers.pc = registers.pc + 3;
    return 20;
}

/// 0xa LD A (BC)
pub fn ld_00a(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.a = memory.get(registers.get_bc());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xe LD C d8
pub fn ld_00e(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.c = memory.get(registers.pc + 1);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x11 LD DE d16
pub fn ld_011(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.set_de(memory.get_16_bits(registers.pc + 1));
    registers.pc = registers.pc + 3;
    return 12;
}

/// 0x12 LD (DE) A
pub fn ld_012(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_de(), registers.a);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x16 LD D d8
pub fn ld_016(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.d = memory.get(registers.pc + 1);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a LD A (DE)
pub fn ld_01a(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.a = memory.get(registers.get_de());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x1e LD E d8
pub fn ld_01e(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.e = memory.get(registers.pc + 1);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x21 LD HL d16
pub fn ld_021(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.set_hl(memory.get_16_bits(registers.pc + 1));
    registers.pc = registers.pc + 3;
    return 12;
}

/// 0x26 LD H d8
pub fn ld_026(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.h = memory.get(registers.pc + 1);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x2e LD L d8
pub fn ld_02e(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.l = memory.get(registers.pc + 1);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x31 LD SP d16
pub fn ld_031(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.sp = memory.get_16_bits(registers.pc + 1);
    registers.pc = registers.pc + 3;
    return 12;
}

/// 0x36 LD (HL) d8
pub fn ld_036(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.pc + 1));
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x3e LD A d8
pub fn ld_03e(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.a = memory.get(registers.pc + 1);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x40 LD B B
pub fn ld_040(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.b = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x41 LD B C
pub fn ld_041(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.b = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x42 LD B D
pub fn ld_042(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.b = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x43 LD B E
pub fn ld_043(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.b = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x44 LD B H
pub fn ld_044(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.b = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x45 LD B L
pub fn ld_045(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.b = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x46 LD B (HL)
pub fn ld_046(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.b = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x47 LD B A
pub fn ld_047(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.b = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x48 LD C B
pub fn ld_048(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.c = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x49 LD C C
pub fn ld_049(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.c = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4a LD C D
pub fn ld_04a(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.c = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4b LD C E
pub fn ld_04b(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.c = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4c LD C H
pub fn ld_04c(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.c = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4d LD C L
pub fn ld_04d(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.c = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4e LD C (HL)
pub fn ld_04e(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.c = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x4f LD C A
pub fn ld_04f(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.c = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x50 LD D B
pub fn ld_050(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.d = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x51 LD D C
pub fn ld_051(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.d = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x52 LD D D
pub fn ld_052(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.d = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x53 LD D E
pub fn ld_053(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.d = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x54 LD D H
pub fn ld_054(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.d = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x55 LD D L
pub fn ld_055(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.d = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x56 LD D (HL)
pub fn ld_056(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.d = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x57 LD D A
pub fn ld_057(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.d = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x58 LD E B
pub fn ld_058(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.e = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x59 LD E C
pub fn ld_059(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.e = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5a LD E D
pub fn ld_05a(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.e = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5b LD E E
pub fn ld_05b(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.e = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5c LD E H
pub fn ld_05c(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.e = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5d LD E L
pub fn ld_05d(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.e = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5e LD E (HL)
pub fn ld_05e(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.e = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x5f LD E A
pub fn ld_05f(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.e = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x60 LD H B
pub fn ld_060(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.h = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x61 LD H C
pub fn ld_061(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.h = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x62 LD H D
pub fn ld_062(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.h = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x63 LD H E
pub fn ld_063(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.h = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x64 LD H H
pub fn ld_064(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.h = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x65 LD H L
pub fn ld_065(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.h = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x66 LD H (HL)
pub fn ld_066(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.h = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x67 LD H A
pub fn ld_067(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.h = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x68 LD L B
pub fn ld_068(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.l = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x69 LD L C
pub fn ld_069(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.l = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6a LD L D
pub fn ld_06a(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.l = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6b LD L E
pub fn ld_06b(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.l = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6c LD L H
pub fn ld_06c(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.l = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6d LD L L
pub fn ld_06d(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.l = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6e LD L (HL)
pub fn ld_06e(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.l = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x6f LD L A
pub fn ld_06f(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.l = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x70 LD (HL) B
pub fn ld_070(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_hl(), registers.b);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x71 LD (HL) C
pub fn ld_071(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_hl(), registers.c);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x72 LD (HL) D
pub fn ld_072(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_hl(), registers.d);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x73 LD (HL) E
pub fn ld_073(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_hl(), registers.e);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x74 LD (HL) H
pub fn ld_074(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_hl(), registers.h);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x75 LD (HL) L
pub fn ld_075(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_hl(), registers.l);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x77 LD (HL) A
pub fn ld_077(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(registers.get_hl(), registers.a);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x78 LD A B
pub fn ld_078(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.a = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x79 LD A C
pub fn ld_079(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.a = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7a LD A D
pub fn ld_07a(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.a = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7b LD A E
pub fn ld_07b(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.a = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7c LD A H
pub fn ld_07c(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.a = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7d LD A L
pub fn ld_07d(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.a = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7e LD A (HL)
pub fn ld_07e(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.a = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x7f LD A A
pub fn ld_07f(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.a = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xd3 UNKNOWN
pub fn unknown_0d3(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xD3");
    return 1;
}

/// 0xdb UNKNOWN
pub fn unknown_0db(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xDB");
    return 1;
}

/// 0xdd UNKNOWN
pub fn unknown_0dd(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xDD");
    return 1;
}

/// 0xe3 UNKNOWN
pub fn unknown_0e3(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xE3");
    return 1;
}

/// 0xe4 UNKNOWN
pub fn unknown_0e4(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xE4");
    return 1;
}

/// 0xea LD (a16) A
pub fn ld_0ea(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    memory.set(memory.get_16_bits(registers.pc + 1), registers.a);
    registers.pc = registers.pc + 3;
    return 16;
}

/// 0xeb UNKNOWN
pub fn unknown_0eb(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xEB");
    return 1;
}

/// 0xec UNKNOWN
pub fn unknown_0ec(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xEC");
    return 1;
}

/// 0xed UNKNOWN
pub fn unknown_0ed(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xED");
    return 1;
}

/// 0xf4 UNKNOWN
pub fn unknown_0f4(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xF4");
    return 1;
}

/// 0xf9 LD SP HL
pub fn ld_0f9(registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    registers.sp = registers.get_hl();
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xfa LD A (a16)
pub fn ld_0fa(registers: &mut Registers, memory: &mut dyn Memory) -> u16 {
    registers.a = memory.get(memory.get_16_bits(registers.pc + 1));
    registers.pc = registers.pc + 3;
    return 16;
}

/// 0xfc UNKNOWN
pub fn unknown_0fc(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xFC");
    return 1;
}

/// 0xfd UNKNOWN
pub fn unknown_0fd(_registers: &mut Registers, _memory: &mut dyn Memory) -> u16 {
    panic!("Unknown opcode 0xFD");
    return 1;
}

