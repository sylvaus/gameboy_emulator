use log::trace;
use crate::memory::argument::Argument;
use crate::memory::Memory;
use crate::memory::registers::Registers;

/// 0x0 NOP
pub fn nop_000(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x1 LD BC d16
pub fn ld_001(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.set_bc(argument.get_16_bits());
    registers.pc = registers.pc + 3;
    return 12;
}

/// 0x2 LD (BC) A
pub fn ld_002(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_bc(), registers.a);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x3 INC BC
pub fn inc_003(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_bc();
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    registers.set_bc((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x4 INC B
pub fn inc_004(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.b;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + (rhs & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (registers.flags & 0b10000u8);
    registers.b = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5 DEC B
pub fn dec_005(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.b;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - (rhs & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + 0b1000000u8 + (registers.flags & 0b10000u8);
    registers.b = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6 LD B d8
pub fn ld_006(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.b = argument.get();
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x7 RLCA
pub fn rlca_007(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.a;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    registers.flags = carried_value << 4u8;
    registers.a = result;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x8 LD (a16) SP
pub fn ld_008(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    memory.set_16_bits(argument.get_16_bits(), registers.sp);
    registers.pc = registers.pc + 3;
    return 20;
}

/// 0x9 ADD HL BC
pub fn add_009(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_hl();
    let rhs: u16 = registers.get_bc();
    let result: i32 = (lhs as i32) + (rhs as i32);
    let half_value: i32 = ((lhs as i32) & 0xFFFi32) + ((rhs as i32) & 0xFFFi32);
    let half_flag: u8 = (half_value > 0xFFFi32) as u8;
    let carry_flag: u8 = (result > 0xFFFFi32) as u8;
    registers.flags = (half_flag << 5u8) + (carry_flag << 4u8) + (registers.flags & 0b10000000u8);
    registers.set_hl((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xa LD A (BC)
pub fn ld_00a(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = memory.get(registers.get_bc());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xb DEC BC
pub fn dec_00b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_bc();
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    registers.set_bc((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xc INC C
pub fn inc_00c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.c;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + (rhs & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (registers.flags & 0b10000u8);
    registers.c = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xd DEC C
pub fn dec_00d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.c;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - (rhs & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + 0b1000000u8 + (registers.flags & 0b10000u8);
    registers.c = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xe LD C d8
pub fn ld_00e(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.c = argument.get();
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xf RRCA
pub fn rrca_00f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.a;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    registers.flags = carried_value << 4u8;
    registers.a = result;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x11 LD DE d16
pub fn ld_011(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.set_de(argument.get_16_bits());
    registers.pc = registers.pc + 3;
    return 12;
}

/// 0x12 LD (DE) A
pub fn ld_012(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_de(), registers.a);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x13 INC DE
pub fn inc_013(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_de();
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    registers.set_de((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x14 INC D
pub fn inc_014(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.d;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + (rhs & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (registers.flags & 0b10000u8);
    registers.d = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x15 DEC D
pub fn dec_015(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.d;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - (rhs & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + 0b1000000u8 + (registers.flags & 0b10000u8);
    registers.d = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x16 LD D d8
pub fn ld_016(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.d = argument.get();
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x17 RLA
pub fn rla_017(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.a;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    registers.flags = carried_value << 4u8;
    registers.a = result;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x18 JR r8
pub fn jr_018(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32) + (argument.get_signed() as i32);
    registers.pc = no_jump_pc as u16;
    return 12u16;
}

/// 0x19 ADD HL DE
pub fn add_019(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_hl();
    let rhs: u16 = registers.get_de();
    let result: i32 = (lhs as i32) + (rhs as i32);
    let half_value: i32 = ((lhs as i32) & 0xFFFi32) + ((rhs as i32) & 0xFFFi32);
    let half_flag: u8 = (half_value > 0xFFFi32) as u8;
    let carry_flag: u8 = (result > 0xFFFFi32) as u8;
    registers.flags = (half_flag << 5u8) + (carry_flag << 4u8) + (registers.flags & 0b10000000u8);
    registers.set_hl((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x1a LD A (DE)
pub fn ld_01a(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = memory.get(registers.get_de());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x1b DEC DE
pub fn dec_01b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_de();
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    registers.set_de((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x1c INC E
pub fn inc_01c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.e;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + (rhs & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (registers.flags & 0b10000u8);
    registers.e = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x1d DEC E
pub fn dec_01d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.e;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - (rhs & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + 0b1000000u8 + (registers.flags & 0b10000u8);
    registers.e = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x1e LD E d8
pub fn ld_01e(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.e = argument.get();
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f RRA
pub fn rra_01f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.a;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    registers.flags = carried_value << 4u8;
    registers.a = result;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x20 JR NZ r8
pub fn jr_020(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_non_zero_flag()) {
        let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32) + (argument.get_signed() as i32);
        registers.pc = no_jump_pc as u16;
        return 12u16;
    } else {
        let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32);
        registers.pc = no_jump_pc as u16;
        return 8u16;
    }
}

/// 0x21 LD HL d16
pub fn ld_021(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.set_hl(argument.get_16_bits());
    registers.pc = registers.pc + 3;
    return 12;
}

/// 0x22 LDI (HL) A
pub fn ldi_022(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.a);
    registers.set_hl(registers.get_hl() + 1u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x23 INC HL
pub fn inc_023(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_hl();
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    registers.set_hl((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x24 INC H
pub fn inc_024(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.h;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + (rhs & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (registers.flags & 0b10000u8);
    registers.h = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x25 DEC H
pub fn dec_025(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.h;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - (rhs & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + 0b1000000u8 + (registers.flags & 0b10000u8);
    registers.h = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x26 LD H d8
pub fn ld_026(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.h = argument.get();
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x27 DAA
pub fn daa_027(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    if (registers.get_add_sub_flag()) {
        let carry_flag: u8 = ((registers.a > 0x99u8) || registers.get_carry_flag()) as u8;
        registers.a = registers.a + ((0x60u8 * carry_flag) + (0x6u8 * ((((registers.a | 0xFu8) > 0xAu8) || registers.get_half_carry_flag()) as u8)));
        let zero_flag: u8 = (registers.a == 0x0u8) as u8;
        registers.flags = (carry_flag << 4u8) + (zero_flag << 7u8) + (registers.flags & 0b1000000u8);
    } else {
        let carry_flag: u8 = registers.get_carry_flag() as u8;
        registers.a = registers.a - ((0x60u8 * carry_flag) + (0x6u8 * (registers.get_half_carry_flag() as u8)));
        let zero_flag: u8 = (registers.a == 0x0u8) as u8;
        registers.flags = (carry_flag << 4u8) + (zero_flag << 7u8) + (registers.flags & 0b1000000u8);
    }
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x28 JR Z r8
pub fn jr_028(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_zero_flag()) {
        let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32) + (argument.get_signed() as i32);
        registers.pc = no_jump_pc as u16;
        return 12u16;
    } else {
        let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32);
        registers.pc = no_jump_pc as u16;
        return 8u16;
    }
}

/// 0x29 ADD HL HL
pub fn add_029(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_hl();
    let rhs: u16 = registers.get_hl();
    let result: i32 = (lhs as i32) + (rhs as i32);
    let half_value: i32 = ((lhs as i32) & 0xFFFi32) + ((rhs as i32) & 0xFFFi32);
    let half_flag: u8 = (half_value > 0xFFFi32) as u8;
    let carry_flag: u8 = (result > 0xFFFFi32) as u8;
    registers.flags = (half_flag << 5u8) + (carry_flag << 4u8) + (registers.flags & 0b10000000u8);
    registers.set_hl((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x2a LDI A (HL)
pub fn ldi_02a(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = memory.get(registers.get_hl());
    registers.set_hl(registers.get_hl() + 1u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x2b DEC HL
pub fn dec_02b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_hl();
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    registers.set_hl((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x2c INC L
pub fn inc_02c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.l;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + (rhs & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (registers.flags & 0b10000u8);
    registers.l = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x2d DEC L
pub fn dec_02d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.l;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - (rhs & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + 0b1000000u8 + (registers.flags & 0b10000u8);
    registers.l = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x2e LD L d8
pub fn ld_02e(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.l = argument.get();
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x2f CPL
pub fn cpl_02f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = !registers.a;
    registers.flags = 0b1100000u8 + (registers.flags & 0b10010000u8);
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x30 JR NC r8
pub fn jr_030(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_non_carry_flag()) {
        let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32) + (argument.get_signed() as i32);
        registers.pc = no_jump_pc as u16;
        return 12u16;
    } else {
        let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32);
        registers.pc = no_jump_pc as u16;
        return 8u16;
    }
}

/// 0x31 LD SP d16
pub fn ld_031(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.sp = argument.get_16_bits();
    registers.pc = registers.pc + 3;
    return 12;
}

/// 0x32 LDD (HL) A
pub fn ldd_032(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.a);
    registers.set_hl(registers.get_hl() - 1u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x33 INC SP
pub fn inc_033(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.sp;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    registers.sp = (result & 0xFFFFi32) as u16;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x34 INC (HL)
pub fn inc_034(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = memory.get(registers.get_hl());
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + (rhs & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (registers.flags & 0b10000u8);
    memory.set(registers.get_hl(), (result & 0xFFi32) as u8);
    registers.pc = registers.pc + 1;
    return 12;
}

/// 0x35 DEC (HL)
pub fn dec_035(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = memory.get(registers.get_hl());
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - (rhs & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + 0b1000000u8 + (registers.flags & 0b10000u8);
    memory.set(registers.get_hl(), (result & 0xFFi32) as u8);
    registers.pc = registers.pc + 1;
    return 12;
}

/// 0x36 LD (HL) d8
pub fn ld_036(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), argument.get());
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x37 SCF
pub fn scf_037(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.flags = 0b10000u8 + (registers.flags & 0b10000000u8);
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x38 JR C r8
pub fn jr_038(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_carry_flag()) {
        let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32) + (argument.get_signed() as i32);
        registers.pc = no_jump_pc as u16;
        return 12u16;
    } else {
        let no_jump_pc: i32 = (registers.pc as i32) + (2u16 as i32);
        registers.pc = no_jump_pc as u16;
        return 8u16;
    }
}

/// 0x39 ADD HL SP
pub fn add_039(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.get_hl();
    let rhs: u16 = registers.sp;
    let result: i32 = (lhs as i32) + (rhs as i32);
    let half_value: i32 = ((lhs as i32) & 0xFFFi32) + ((rhs as i32) & 0xFFFi32);
    let half_flag: u8 = (half_value > 0xFFFi32) as u8;
    let carry_flag: u8 = (result > 0xFFFFi32) as u8;
    registers.flags = (half_flag << 5u8) + (carry_flag << 4u8) + (registers.flags & 0b10000000u8);
    registers.set_hl((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x3a LDD A (HL)
pub fn ldd_03a(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = memory.get(registers.get_hl());
    registers.set_hl(registers.get_hl() - 1u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x3b DEC SP
pub fn dec_03b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.sp;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    registers.sp = (result & 0xFFFFi32) as u16;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x3c INC A
pub fn inc_03c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) + rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + (rhs & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (registers.flags & 0b10000u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x3d DEC A
pub fn dec_03d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: i32 = 1i32;
    let result: i32 = (lhs as i32) - rhs;
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - (rhs & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + 0b1000000u8 + (registers.flags & 0b10000u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x3e LD A d8
pub fn ld_03e(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.a = argument.get();
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x3f CCF
pub fn ccf_03f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let carry_flag: u8 = (!registers.get_carry_flag() as u8) << 4u8;
    registers.flags = carry_flag + (registers.flags & 0b10000000u8);
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x40 LD B B
pub fn ld_040(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x41 LD B C
pub fn ld_041(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x42 LD B D
pub fn ld_042(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x43 LD B E
pub fn ld_043(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x44 LD B H
pub fn ld_044(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x45 LD B L
pub fn ld_045(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x46 LD B (HL)
pub fn ld_046(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x47 LD B A
pub fn ld_047(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x48 LD C B
pub fn ld_048(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x49 LD C C
pub fn ld_049(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4a LD C D
pub fn ld_04a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4b LD C E
pub fn ld_04b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4c LD C H
pub fn ld_04c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4d LD C L
pub fn ld_04d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x4e LD C (HL)
pub fn ld_04e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x4f LD C A
pub fn ld_04f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x50 LD D B
pub fn ld_050(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x51 LD D C
pub fn ld_051(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x52 LD D D
pub fn ld_052(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x53 LD D E
pub fn ld_053(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x54 LD D H
pub fn ld_054(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x55 LD D L
pub fn ld_055(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x56 LD D (HL)
pub fn ld_056(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x57 LD D A
pub fn ld_057(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x58 LD E B
pub fn ld_058(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x59 LD E C
pub fn ld_059(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5a LD E D
pub fn ld_05a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5b LD E E
pub fn ld_05b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5c LD E H
pub fn ld_05c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5d LD E L
pub fn ld_05d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x5e LD E (HL)
pub fn ld_05e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x5f LD E A
pub fn ld_05f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x60 LD H B
pub fn ld_060(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x61 LD H C
pub fn ld_061(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x62 LD H D
pub fn ld_062(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x63 LD H E
pub fn ld_063(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x64 LD H H
pub fn ld_064(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x65 LD H L
pub fn ld_065(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x66 LD H (HL)
pub fn ld_066(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x67 LD H A
pub fn ld_067(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x68 LD L B
pub fn ld_068(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x69 LD L C
pub fn ld_069(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6a LD L D
pub fn ld_06a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6b LD L E
pub fn ld_06b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6c LD L H
pub fn ld_06c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6d LD L L
pub fn ld_06d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x6e LD L (HL)
pub fn ld_06e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x6f LD L A
pub fn ld_06f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x70 LD (HL) B
pub fn ld_070(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.b);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x71 LD (HL) C
pub fn ld_071(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.c);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x72 LD (HL) D
pub fn ld_072(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.d);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x73 LD (HL) E
pub fn ld_073(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.e);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x74 LD (HL) H
pub fn ld_074(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.h);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x75 LD (HL) L
pub fn ld_075(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.l);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x76 HALT
pub fn halt_076(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.halted = true;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x77 LD (HL) A
pub fn ld_077(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), registers.a);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x78 LD A B
pub fn ld_078(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.b;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x79 LD A C
pub fn ld_079(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.c;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7a LD A D
pub fn ld_07a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.d;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7b LD A E
pub fn ld_07b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.e;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7c LD A H
pub fn ld_07c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.h;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7d LD A L
pub fn ld_07d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.l;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x7e LD A (HL)
pub fn ld_07e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = memory.get(registers.get_hl());
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x7f LD A A
pub fn ld_07f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x80 ADD A B
pub fn add_080(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.b;
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x81 ADD A C
pub fn add_081(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.c;
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x82 ADD A D
pub fn add_082(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.d;
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x83 ADD A E
pub fn add_083(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.e;
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x84 ADD A H
pub fn add_084(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.h;
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x85 ADD A L
pub fn add_085(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.l;
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x86 ADD A (HL)
pub fn add_086(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = memory.get(registers.get_hl());
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x87 ADD A A
pub fn add_087(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.a;
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x88 ADC A B
pub fn adc_088(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.b;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x89 ADC A C
pub fn adc_089(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.c;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x8a ADC A D
pub fn adc_08a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.d;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x8b ADC A E
pub fn adc_08b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.e;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x8c ADC A H
pub fn adc_08c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.h;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x8d ADC A L
pub fn adc_08d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.l;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x8e ADC A (HL)
pub fn adc_08e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = memory.get(registers.get_hl());
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x8f ADC A A
pub fn adc_08f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.a;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x90 SUB A B
pub fn sub_090(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.b;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x91 SUB A C
pub fn sub_091(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.c;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x92 SUB A D
pub fn sub_092(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.d;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x93 SUB A E
pub fn sub_093(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.e;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x94 SUB A H
pub fn sub_094(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.h;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x95 SUB A L
pub fn sub_095(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.l;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x96 SUB A (HL)
pub fn sub_096(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = memory.get(registers.get_hl());
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x97 SUB A A
pub fn sub_097(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.a;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x98 SBC A B
pub fn sbc_098(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.b;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x99 SBC A C
pub fn sbc_099(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.c;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x9a SBC A D
pub fn sbc_09a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.d;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x9b SBC A E
pub fn sbc_09b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.e;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x9c SBC A H
pub fn sbc_09c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.h;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x9d SBC A L
pub fn sbc_09d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.l;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0x9e SBC A (HL)
pub fn sbc_09e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = memory.get(registers.get_hl());
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0x9f SBC A A
pub fn sbc_09f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.a;
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa0 AND B
pub fn and_0a0(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & registers.b;
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa1 AND C
pub fn and_0a1(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & registers.c;
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa2 AND D
pub fn and_0a2(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & registers.d;
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa3 AND E
pub fn and_0a3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & registers.e;
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa4 AND H
pub fn and_0a4(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & registers.h;
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa5 AND L
pub fn and_0a5(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & registers.l;
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa6 AND (HL)
pub fn and_0a6(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & memory.get(registers.get_hl());
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xa7 AND A
pub fn and_0a7(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & registers.a;
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa8 XOR B
pub fn xor_0a8(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ registers.b;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xa9 XOR C
pub fn xor_0a9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ registers.c;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xaa XOR D
pub fn xor_0aa(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ registers.d;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xab XOR E
pub fn xor_0ab(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ registers.e;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xac XOR H
pub fn xor_0ac(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ registers.h;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xad XOR L
pub fn xor_0ad(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ registers.l;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xae XOR (HL)
pub fn xor_0ae(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ memory.get(registers.get_hl());
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xaf XOR A
pub fn xor_0af(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ registers.a;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb0 OR B
pub fn or_0b0(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | registers.b;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb1 OR C
pub fn or_0b1(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | registers.c;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb2 OR D
pub fn or_0b2(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | registers.d;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb3 OR E
pub fn or_0b3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | registers.e;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb4 OR H
pub fn or_0b4(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | registers.h;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb5 OR L
pub fn or_0b5(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | registers.l;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb6 OR (HL)
pub fn or_0b6(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | memory.get(registers.get_hl());
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xb7 OR A
pub fn or_0b7(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | registers.a;
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb8 CP B
pub fn cp_0b8(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.b;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xb9 CP C
pub fn cp_0b9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.c;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xba CP D
pub fn cp_0ba(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.d;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xbb CP E
pub fn cp_0bb(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.e;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xbc CP H
pub fn cp_0bc(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.h;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xbd CP L
pub fn cp_0bd(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.l;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xbe CP (HL)
pub fn cp_0be(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = memory.get(registers.get_hl());
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xbf CP A
pub fn cp_0bf(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = registers.a;
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xc0 RET NZ
pub fn ret_0c0(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    if (registers.get_non_zero_flag()) {
        let lower_pc: u16 = memory.get(registers.sp) as u16;
        let upper_pc: u16 = (memory.get(registers.sp + 1u16)) as u16;
        registers.pc = lower_pc + (upper_pc << 8u16);
        registers.sp = registers.sp + 2u16;
        return 20u16;
    } else {
        registers.pc = registers.pc + 1u16;
        return 8u16;
    }
}

/// 0xc1 POP BC
pub fn pop_0c1(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = memory.get(registers.sp);
    registers.b = memory.get(registers.sp + 1u16);
    registers.sp = registers.sp + 2u16;
    registers.pc = registers.pc + 1;
    return 12;
}

/// 0xc2 JP NZ a16
pub fn jp_0c2(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_non_zero_flag()) {
        registers.pc = argument.get_16_bits();
        return 16u16;
    } else {
        registers.pc = registers.pc + 3u16;
        return 12u16;
    }
}

/// 0xc3 JP a16
pub fn jp_0c3(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = argument.get_16_bits();
    return 16u16;
}

/// 0xc4 CALL NZ a16
pub fn call_0c4(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_non_zero_flag()) {
        registers.pc = registers.pc + 3u16;
        memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
        memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
        registers.sp = registers.sp - 2u16;
        registers.pc = argument.get_16_bits();
        return 24u16;
    } else {
        registers.pc = registers.pc + 3u16;
        return 12u16;
    }
}

/// 0xc5 PUSH BC
pub fn push_0c5(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = memory.get(registers.sp - 1u16);
    registers.c = memory.get(registers.sp - 2u16);
    registers.sp = registers.sp - 2u16;
    registers.pc = registers.pc + 1;
    return 16;
}

/// 0xc6 ADD A d8
pub fn add_0c6(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = argument.get();
    let result: i32 = (lhs as i32) + (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xc7 RST 00H
pub fn rst_0c7(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = 0x0u16;
    return 16u16;
}

/// 0xc8 RET Z
pub fn ret_0c8(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    if (registers.get_zero_flag()) {
        let lower_pc: u16 = memory.get(registers.sp) as u16;
        let upper_pc: u16 = (memory.get(registers.sp + 1u16)) as u16;
        registers.pc = lower_pc + (upper_pc << 8u16);
        registers.sp = registers.sp + 2u16;
        return 20u16;
    } else {
        registers.pc = registers.pc + 1u16;
        return 8u16;
    }
}

/// 0xc9 RET
pub fn ret_0c9(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lower_pc: u16 = memory.get(registers.sp) as u16;
    let upper_pc: u16 = (memory.get(registers.sp + 1u16)) as u16;
    registers.pc = lower_pc + (upper_pc << 8u16);
    registers.sp = registers.sp + 2u16;
    return 16u16;
}

/// 0xca JP Z a16
pub fn jp_0ca(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_zero_flag()) {
        registers.pc = argument.get_16_bits();
        return 16u16;
    } else {
        registers.pc = registers.pc + 3u16;
        return 12u16;
    }
}

/// 0xcb PREFIX CB
pub fn prefix_0cb(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Opcode 0xCB should be handled separately, something bad must have happened");
}

/// 0xcc CALL Z a16
pub fn call_0cc(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_zero_flag()) {
        registers.pc = registers.pc + 3u16;
        memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
        memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
        registers.sp = registers.sp - 2u16;
        registers.pc = argument.get_16_bits();
        return 24u16;
    } else {
        registers.pc = registers.pc + 3u16;
        return 12u16;
    }
}

/// 0xcd CALL a16
pub fn call_0cd(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 3u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = argument.get_16_bits();
    return 24u16;
}

/// 0xce ADC A d8
pub fn adc_0ce(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = argument.get();
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) + (rhs as i32) + (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) + ((rhs as i32) & 0xFi32) + ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value > 0xFi32) as u8;
    let carry_flag: u8 = (result > 0xFFi32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8);
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xcf RST 08H
pub fn rst_0cf(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = 0x8u16;
    return 16u16;
}

/// 0xd0 RET NC
pub fn ret_0d0(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    if (registers.get_non_carry_flag()) {
        let lower_pc: u16 = memory.get(registers.sp) as u16;
        let upper_pc: u16 = (memory.get(registers.sp + 1u16)) as u16;
        registers.pc = lower_pc + (upper_pc << 8u16);
        registers.sp = registers.sp + 2u16;
        return 20u16;
    } else {
        registers.pc = registers.pc + 1u16;
        return 8u16;
    }
}

/// 0xd1 POP DE
pub fn pop_0d1(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = memory.get(registers.sp);
    registers.d = memory.get(registers.sp + 1u16);
    registers.sp = registers.sp + 2u16;
    registers.pc = registers.pc + 1;
    return 12;
}

/// 0xd2 JP NC a16
pub fn jp_0d2(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_non_carry_flag()) {
        registers.pc = argument.get_16_bits();
        return 16u16;
    } else {
        registers.pc = registers.pc + 3u16;
        return 12u16;
    }
}

/// 0xd3 UNKNOWN
pub fn unknown_0d3(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xD3");
}

/// 0xd4 CALL NC a16
pub fn call_0d4(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_non_carry_flag()) {
        registers.pc = registers.pc + 3u16;
        memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
        memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
        registers.sp = registers.sp - 2u16;
        registers.pc = argument.get_16_bits();
        return 24u16;
    } else {
        registers.pc = registers.pc + 3u16;
        return 12u16;
    }
}

/// 0xd5 PUSH DE
pub fn push_0d5(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = memory.get(registers.sp - 1u16);
    registers.e = memory.get(registers.sp - 2u16);
    registers.sp = registers.sp - 2u16;
    registers.pc = registers.pc + 1;
    return 16;
}

/// 0xd6 SUB A d8
pub fn sub_0d6(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = argument.get();
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xd7 RST 10H
pub fn rst_0d7(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = 0x10u16;
    return 16u16;
}

/// 0xd8 RET C
pub fn ret_0d8(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    if (registers.get_carry_flag()) {
        let lower_pc: u16 = memory.get(registers.sp) as u16;
        let upper_pc: u16 = (memory.get(registers.sp + 1u16)) as u16;
        registers.pc = lower_pc + (upper_pc << 8u16);
        registers.sp = registers.sp + 2u16;
        return 20u16;
    } else {
        registers.pc = registers.pc + 1u16;
        return 8u16;
    }
}

/// 0xda JP C a16
pub fn jp_0da(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_carry_flag()) {
        registers.pc = argument.get_16_bits();
        return 16u16;
    } else {
        registers.pc = registers.pc + 3u16;
        return 12u16;
    }
}

/// 0xdb UNKNOWN
pub fn unknown_0db(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xDB");
}

/// 0xdc CALL C a16
pub fn call_0dc(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    if (registers.get_carry_flag()) {
        registers.pc = registers.pc + 3u16;
        memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
        memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
        registers.sp = registers.sp - 2u16;
        registers.pc = argument.get_16_bits();
        return 24u16;
    } else {
        registers.pc = registers.pc + 3u16;
        return 12u16;
    }
}

/// 0xdd UNKNOWN
pub fn unknown_0dd(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xDD");
}

/// 0xde SBC A d8
pub fn sbc_0de(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = argument.get();
    let rrhs: bool = registers.get_half_carry_flag();
    let result: i32 = (lhs as i32) - (rhs as i32) - (rrhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32) - ((rrhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.a = (result & 0xFFi32) as u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xdf RST 18H
pub fn rst_0df(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = 0x18u16;
    return 16u16;
}

/// 0xe0 LDH (a8) A
pub fn ldh_0e0(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    memory.set((argument.get() as u16) + 0xFF00u16, registers.a);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0xe1 POP HL
pub fn pop_0e1(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = memory.get(registers.sp);
    registers.h = memory.get(registers.sp + 1u16);
    registers.sp = registers.sp + 2u16;
    registers.pc = registers.pc + 1;
    return 12;
}

/// 0xe2 LDSpecial (C) A
pub fn ldspecial_0e2(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set((registers.c as u16) + 0xFF00u16, registers.a);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xe3 UNKNOWN
pub fn unknown_0e3(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xE3");
}

/// 0xe4 UNKNOWN
pub fn unknown_0e4(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xE4");
}

/// 0xe5 PUSH HL
pub fn push_0e5(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = memory.get(registers.sp - 1u16);
    registers.l = memory.get(registers.sp - 2u16);
    registers.sp = registers.sp - 2u16;
    registers.pc = registers.pc + 1;
    return 16;
}

/// 0xe6 AND d8
pub fn and_0e6(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.a = registers.a & argument.get();
    registers.flags = (((registers.a == 0u8) as u8) << 7u8) + 0b100000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xe7 RST 20H
pub fn rst_0e7(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = 0x20u16;
    return 16u16;
}

/// 0xe8 ADD SP r8
pub fn add_0e8(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.sp;
    let rhs: i8 = argument.get_signed();
    let result: i32 = (lhs as i32) + (rhs as i32);
    let half_value: i32 = ((lhs as i32) & 0xFFFi32) + ((rhs as i32) & 0xFFFi32);
    let half_flag: u8 = (half_value > 0xFFFi32) as u8;
    let carry_flag: u8 = (result > 0xFFFFi32) as u8;
    registers.flags = (half_flag << 5u8) + (carry_flag << 4u8);
    registers.sp = (result & 0xFFFFi32) as u16;
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0xe9 JP HL
pub fn jp_0e9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.pc = registers.get_hl();
    return 4u16;
}

/// 0xea LD (a16) A
pub fn ld_0ea(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    memory.set(argument.get_16_bits(), registers.a);
    registers.pc = registers.pc + 3;
    return 16;
}

/// 0xeb UNKNOWN
pub fn unknown_0eb(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xEB");
}

/// 0xec UNKNOWN
pub fn unknown_0ec(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xEC");
}

/// 0xed UNKNOWN
pub fn unknown_0ed(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xED");
}

/// 0xee XOR d8
pub fn xor_0ee(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.a = registers.a ^ argument.get();
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xef RST 28H
pub fn rst_0ef(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = 0x28u16;
    return 16u16;
}

/// 0xf0 LDH A (a8)
pub fn ldh_0f0(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.a = memory.get((argument.get() as u16) + 0xFF00u16);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0xf1 POP AF
pub fn pop_0f1(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.flags = memory.get(registers.sp);
    registers.a = memory.get(registers.sp + 1u16);
    registers.sp = registers.sp + 2u16;
    registers.pc = registers.pc + 1;
    return 12;
}

/// 0xf2 LDSpecial A (C)
pub fn ldspecial_0f2(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = memory.get((registers.c as u16) + 0xFF00u16);
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xf3 DI
pub fn di_0f3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.ime_flag = false;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xf4 UNKNOWN
pub fn unknown_0f4(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xF4");
}

/// 0xf5 PUSH AF
pub fn push_0f5(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = memory.get(registers.sp - 1u16);
    registers.flags = memory.get(registers.sp - 2u16);
    registers.sp = registers.sp - 2u16;
    registers.pc = registers.pc + 1;
    return 16;
}

/// 0xf6 OR d8
pub fn or_0f6(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.a = registers.a | argument.get();
    registers.flags = ((registers.a == 0u8) as u8) << 7u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xf7 RST 30H
pub fn rst_0f7(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = 0x30u16;
    return 16u16;
}

/// 0xf8 LDHL SP r8
pub fn ldhl_0f8(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    let lhs: u16 = registers.sp;
    let rhs: i8 = argument.get_signed();
    let result: i32 = (lhs as i32) + (rhs as i32);
    let half_value: i32 = ((lhs as i32) & 0xFFFi32) + ((rhs as i32) & 0xFFFi32);
    let half_flag: u8 = (half_value > 0xFFFi32) as u8;
    let carry_flag: u8 = (result > 0xFFFFi32) as u8;
    registers.flags = (half_flag << 5u8) + (carry_flag << 4u8);
    registers.set_hl((result & 0xFFFFi32) as u16);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0xf9 LD SP HL
pub fn ld_0f9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.sp = registers.get_hl();
    registers.pc = registers.pc + 1;
    return 8;
}

/// 0xfa LD A (a16)
pub fn ld_0fa(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.a = memory.get(argument.get_16_bits());
    registers.pc = registers.pc + 3;
    return 16;
}

/// 0xfb EI
pub fn ei_0fb(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.ime_flag = true;
    registers.pc = registers.pc + 1;
    return 4;
}

/// 0xfc UNKNOWN
pub fn unknown_0fc(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xFC");
}

/// 0xfd UNKNOWN
pub fn unknown_0fd(_registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    panic!("Unknown opcode 0xFD");
}

/// 0xfe CP d8
pub fn cp_0fe(registers: &mut Registers, _memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    let lhs: u8 = registers.a;
    let rhs: u8 = argument.get();
    let result: i32 = (lhs as i32) - (rhs as i32);
    let zero_flag: u8 = ((result & 0xFFi32) == 0i32) as u8;
    let half_value: i32 = ((lhs as i32) & 0xFi32) - ((rhs as i32) & 0xFi32);
    let half_flag: u8 = (half_value < 0x0i32) as u8;
    let carry_flag: u8 = (result < 0x0i32) as u8;
    registers.flags = (zero_flag << 7u8) + (half_flag << 5u8) + (carry_flag << 4u8) + 0b1000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0xff RST 38H
pub fn rst_0ff(registers: &mut Registers, memory: &mut dyn Memory, argument: &mut Argument) -> u16 {
    registers.pc = registers.pc + 1u16;
    memory.set(registers.sp - 1u16, ((registers.pc >> 8u16) & 0xFFu16) as u8);
    memory.set(registers.sp - 2u16, (registers.pc & 0xFFu16) as u8);
    registers.sp = registers.sp - 2u16;
    registers.pc = 0x38u16;
    return 16u16;
}

/// 0x100 RLC B
pub fn rlc_100(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.b;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.b = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x101 RLC C
pub fn rlc_101(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.c;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.c = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x102 RLC D
pub fn rlc_102(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.d;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.d = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x103 RLC E
pub fn rlc_103(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.e;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.e = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x104 RLC H
pub fn rlc_104(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.h;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.h = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x105 RLC L
pub fn rlc_105(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.l;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.l = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x106 RLC (HL)
pub fn rlc_106(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = memory.get(registers.get_hl());
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    memory.set(registers.get_hl(), result);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x107 RLC A
pub fn rlc_107(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.a;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (carried_value as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.a = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x108 RRC B
pub fn rrc_108(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.b;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.b = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x109 RRC C
pub fn rrc_109(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.c;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.c = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x10a RRC D
pub fn rrc_10a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.d;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.d = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x10b RRC E
pub fn rrc_10b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.e;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.e = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x10c RRC H
pub fn rrc_10c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.h;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.h = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x10d RRC L
pub fn rrc_10d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.l;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.l = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x10e RRC (HL)
pub fn rrc_10e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = memory.get(registers.get_hl());
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    memory.set(registers.get_hl(), result);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x10f RRC A
pub fn rrc_10f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.a;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((carried_value as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.a = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x110 RL B
pub fn rl_110(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.b;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.b = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x111 RL C
pub fn rl_111(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.c;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.c = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x112 RL D
pub fn rl_112(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.d;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.d = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x113 RL E
pub fn rl_113(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.e;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.e = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x114 RL H
pub fn rl_114(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.h;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.h = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x115 RL L
pub fn rl_115(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.l;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.l = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x116 RL (HL)
pub fn rl_116(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = memory.get(registers.get_hl());
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    memory.set(registers.get_hl(), result);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x117 RL A
pub fn rl_117(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.a;
    let carried_value: u8 = (value_u8 >> 7u8) & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value << 1u16) + (registers.get_carry_flag() as u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.a = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x118 RR B
pub fn rr_118(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.b;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.b = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x119 RR C
pub fn rr_119(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.c;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.c = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x11a RR D
pub fn rr_11a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.d;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.d = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x11b RR E
pub fn rr_11b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.e;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.e = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x11c RR H
pub fn rr_11c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.h;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.h = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x11d RR L
pub fn rr_11d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.l;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.l = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x11e RR (HL)
pub fn rr_11e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = memory.get(registers.get_hl());
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    memory.set(registers.get_hl(), result);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x11f RR A
pub fn rr_11f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value_u8: u8 = registers.a;
    let carried_value: u8 = value_u8 & 0b1u8;
    let value: u16 = value_u8 as u16;
    let result: u8 = (((value >> 1u16) + ((registers.get_carry_flag() as u16) << 7u16)) & 0xFFu16) as u8;
    let zero_flag: u8 = ((result & 0xFFu8) == 0u8) as u8;
    registers.flags = (carried_value << 4u8) + (zero_flag << 7u8);
    registers.a = result;
    registers.pc = registers.pc + 2;
    return 8;
}

