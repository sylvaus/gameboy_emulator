use log::trace;
use crate::memory::argument::Argument;
use crate::memory::Memory;
use crate::memory::registers::Registers;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImmediateArgumentType {
    None,
    Unsigned8Bits,
    Signed8Bits,
    Unsigned16Bits,
}

pub struct Instruction {
    pub function: fn(&mut Registers, &mut dyn Memory, &mut Argument) -> u16,
    pub argument: ImmediateArgumentType,
}

impl Instruction {
    pub fn new(
        function: fn(&mut Registers, &mut dyn Memory, &mut Argument) -> u16,
        argument: ImmediateArgumentType,
    ) -> Self {
        Instruction { function, argument }
    }
}

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

/// 0x10 STOP 0
pub fn stop_010(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.stopped = true;
    registers.pc = registers.pc + 2;
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

/// 0xd9 RETI
pub fn reti_0d9(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let lower_pc: u16 = memory.get(registers.sp) as u16;
    let upper_pc: u16 = (memory.get(registers.sp + 1u16)) as u16;
    registers.pc = lower_pc + (upper_pc << 8u16);
    registers.sp = registers.sp + 2u16;
    return 16;
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

/// 0x120 SLA B
pub fn sla_120(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.b;
    let result: u8 = value << 1u8;
    let carry_flag: u8 = (value >> 7u8) & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.b = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x121 SLA C
pub fn sla_121(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.c;
    let result: u8 = value << 1u8;
    let carry_flag: u8 = (value >> 7u8) & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.c = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x122 SLA D
pub fn sla_122(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.d;
    let result: u8 = value << 1u8;
    let carry_flag: u8 = (value >> 7u8) & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.d = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x123 SLA E
pub fn sla_123(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.e;
    let result: u8 = value << 1u8;
    let carry_flag: u8 = (value >> 7u8) & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.e = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x124 SLA H
pub fn sla_124(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.h;
    let result: u8 = value << 1u8;
    let carry_flag: u8 = (value >> 7u8) & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.h = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x125 SLA L
pub fn sla_125(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.l;
    let result: u8 = value << 1u8;
    let carry_flag: u8 = (value >> 7u8) & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.l = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x126 SLA (HL)
pub fn sla_126(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = memory.get(registers.get_hl());
    let result: u8 = value << 1u8;
    let carry_flag: u8 = (value >> 7u8) & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    memory.set(registers.get_hl(), result);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x127 SLA A
pub fn sla_127(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.a;
    let result: u8 = value << 1u8;
    let carry_flag: u8 = (value >> 7u8) & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.a = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x128 SRA B
pub fn sra_128(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.b;
    let result: u8 = (value >> 1u8) + (value & 0x80u8);
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.b = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x129 SRA C
pub fn sra_129(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.c;
    let result: u8 = (value >> 1u8) + (value & 0x80u8);
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.c = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x12a SRA D
pub fn sra_12a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.d;
    let result: u8 = (value >> 1u8) + (value & 0x80u8);
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.d = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x12b SRA E
pub fn sra_12b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.e;
    let result: u8 = (value >> 1u8) + (value & 0x80u8);
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.e = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x12c SRA H
pub fn sra_12c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.h;
    let result: u8 = (value >> 1u8) + (value & 0x80u8);
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.h = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x12d SRA L
pub fn sra_12d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.l;
    let result: u8 = (value >> 1u8) + (value & 0x80u8);
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.l = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x12e SRA (HL)
pub fn sra_12e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = memory.get(registers.get_hl());
    let result: u8 = (value >> 1u8) + (value & 0x80u8);
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    memory.set(registers.get_hl(), result);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x12f SRA A
pub fn sra_12f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.a;
    let result: u8 = (value >> 1u8) + (value & 0x80u8);
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.a = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x130 SWAP B
pub fn swap_130(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.b;
    let result: u8 = (value >> 4u8) + ((value & 0b1111u8) << 4u8);
    registers.flags = ((result == 0u8) as u8) << 7u8;
    registers.b = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x131 SWAP C
pub fn swap_131(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.c;
    let result: u8 = (value >> 4u8) + ((value & 0b1111u8) << 4u8);
    registers.flags = ((result == 0u8) as u8) << 7u8;
    registers.c = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x132 SWAP D
pub fn swap_132(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.d;
    let result: u8 = (value >> 4u8) + ((value & 0b1111u8) << 4u8);
    registers.flags = ((result == 0u8) as u8) << 7u8;
    registers.d = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x133 SWAP E
pub fn swap_133(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.e;
    let result: u8 = (value >> 4u8) + ((value & 0b1111u8) << 4u8);
    registers.flags = ((result == 0u8) as u8) << 7u8;
    registers.e = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x134 SWAP H
pub fn swap_134(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.h;
    let result: u8 = (value >> 4u8) + ((value & 0b1111u8) << 4u8);
    registers.flags = ((result == 0u8) as u8) << 7u8;
    registers.h = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x135 SWAP L
pub fn swap_135(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.l;
    let result: u8 = (value >> 4u8) + ((value & 0b1111u8) << 4u8);
    registers.flags = ((result == 0u8) as u8) << 7u8;
    registers.l = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x136 SWAP (HL)
pub fn swap_136(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = memory.get(registers.get_hl());
    let result: u8 = (value >> 4u8) + ((value & 0b1111u8) << 4u8);
    registers.flags = ((result == 0u8) as u8) << 7u8;
    memory.set(registers.get_hl(), result);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x137 SWAP A
pub fn swap_137(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.a;
    let result: u8 = (value >> 4u8) + ((value & 0b1111u8) << 4u8);
    registers.flags = ((result == 0u8) as u8) << 7u8;
    registers.a = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x138 SRL B
pub fn srl_138(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.b;
    let result: u8 = value >> 1u8;
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.b = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x139 SRL C
pub fn srl_139(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.c;
    let result: u8 = value >> 1u8;
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.c = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x13a SRL D
pub fn srl_13a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.d;
    let result: u8 = value >> 1u8;
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.d = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x13b SRL E
pub fn srl_13b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.e;
    let result: u8 = value >> 1u8;
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.e = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x13c SRL H
pub fn srl_13c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.h;
    let result: u8 = value >> 1u8;
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.h = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x13d SRL L
pub fn srl_13d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.l;
    let result: u8 = value >> 1u8;
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.l = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x13e SRL (HL)
pub fn srl_13e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = memory.get(registers.get_hl());
    let result: u8 = value >> 1u8;
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    memory.set(registers.get_hl(), result);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x13f SRL A
pub fn srl_13f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let value: u8 = registers.a;
    let result: u8 = value >> 1u8;
    let carry_flag: u8 = value & 0b1u8;
    registers.flags = (carry_flag << 4u8) + (((result == 0u8) as u8) << 7u8);
    registers.a = result;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x140 BIT 0 B
pub fn bit_140(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.b >> 0u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x141 BIT 0 C
pub fn bit_141(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.c >> 0u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x142 BIT 0 D
pub fn bit_142(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.d >> 0u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x143 BIT 0 E
pub fn bit_143(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.e >> 0u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x144 BIT 0 H
pub fn bit_144(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.h >> 0u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x145 BIT 0 L
pub fn bit_145(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.l >> 0u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x146 BIT 0 (HL)
pub fn bit_146(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((memory.get(registers.get_hl()) >> 0u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x147 BIT 0 A
pub fn bit_147(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.a >> 0u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x148 BIT 1 B
pub fn bit_148(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.b >> 1u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x149 BIT 1 C
pub fn bit_149(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.c >> 1u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x14a BIT 1 D
pub fn bit_14a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.d >> 1u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x14b BIT 1 E
pub fn bit_14b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.e >> 1u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x14c BIT 1 H
pub fn bit_14c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.h >> 1u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x14d BIT 1 L
pub fn bit_14d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.l >> 1u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x14e BIT 1 (HL)
pub fn bit_14e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((memory.get(registers.get_hl()) >> 1u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x14f BIT 1 A
pub fn bit_14f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.a >> 1u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x150 BIT 2 B
pub fn bit_150(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.b >> 2u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x151 BIT 2 C
pub fn bit_151(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.c >> 2u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x152 BIT 2 D
pub fn bit_152(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.d >> 2u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x153 BIT 2 E
pub fn bit_153(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.e >> 2u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x154 BIT 2 H
pub fn bit_154(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.h >> 2u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x155 BIT 2 L
pub fn bit_155(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.l >> 2u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x156 BIT 2 (HL)
pub fn bit_156(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((memory.get(registers.get_hl()) >> 2u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x157 BIT 2 A
pub fn bit_157(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.a >> 2u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x158 BIT 3 B
pub fn bit_158(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.b >> 3u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x159 BIT 3 C
pub fn bit_159(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.c >> 3u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x15a BIT 3 D
pub fn bit_15a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.d >> 3u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x15b BIT 3 E
pub fn bit_15b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.e >> 3u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x15c BIT 3 H
pub fn bit_15c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.h >> 3u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x15d BIT 3 L
pub fn bit_15d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.l >> 3u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x15e BIT 3 (HL)
pub fn bit_15e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((memory.get(registers.get_hl()) >> 3u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x15f BIT 3 A
pub fn bit_15f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.a >> 3u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x160 BIT 4 B
pub fn bit_160(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.b >> 4u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x161 BIT 4 C
pub fn bit_161(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.c >> 4u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x162 BIT 4 D
pub fn bit_162(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.d >> 4u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x163 BIT 4 E
pub fn bit_163(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.e >> 4u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x164 BIT 4 H
pub fn bit_164(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.h >> 4u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x165 BIT 4 L
pub fn bit_165(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.l >> 4u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x166 BIT 4 (HL)
pub fn bit_166(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((memory.get(registers.get_hl()) >> 4u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x167 BIT 4 A
pub fn bit_167(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.a >> 4u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x168 BIT 5 B
pub fn bit_168(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.b >> 5u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x169 BIT 5 C
pub fn bit_169(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.c >> 5u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x16a BIT 5 D
pub fn bit_16a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.d >> 5u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x16b BIT 5 E
pub fn bit_16b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.e >> 5u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x16c BIT 5 H
pub fn bit_16c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.h >> 5u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x16d BIT 5 L
pub fn bit_16d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.l >> 5u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x16e BIT 5 (HL)
pub fn bit_16e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((memory.get(registers.get_hl()) >> 5u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x16f BIT 5 A
pub fn bit_16f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.a >> 5u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x170 BIT 6 B
pub fn bit_170(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.b >> 6u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x171 BIT 6 C
pub fn bit_171(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.c >> 6u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x172 BIT 6 D
pub fn bit_172(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.d >> 6u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x173 BIT 6 E
pub fn bit_173(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.e >> 6u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x174 BIT 6 H
pub fn bit_174(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.h >> 6u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x175 BIT 6 L
pub fn bit_175(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.l >> 6u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x176 BIT 6 (HL)
pub fn bit_176(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((memory.get(registers.get_hl()) >> 6u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x177 BIT 6 A
pub fn bit_177(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.a >> 6u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x178 BIT 7 B
pub fn bit_178(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.b >> 7u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x179 BIT 7 C
pub fn bit_179(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.c >> 7u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x17a BIT 7 D
pub fn bit_17a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.d >> 7u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x17b BIT 7 E
pub fn bit_17b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.e >> 7u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x17c BIT 7 H
pub fn bit_17c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.h >> 7u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x17d BIT 7 L
pub fn bit_17d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.l >> 7u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x17e BIT 7 (HL)
pub fn bit_17e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((memory.get(registers.get_hl()) >> 7u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 12;
}

/// 0x17f BIT 7 A
pub fn bit_17f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    let zero_flag: u8 = (((registers.a >> 7u8) & 0b1u8) == 0u8) as u8;
    registers.flags = (zero_flag << 7u8) + 0b100000u8 + (registers.flags & 0b10000u8);
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x180 RES 0 B
pub fn res_180(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b & 0b11111110u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x181 RES 0 C
pub fn res_181(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c & 0b11111110u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x182 RES 0 D
pub fn res_182(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d & 0b11111110u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x183 RES 0 E
pub fn res_183(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e & 0b11111110u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x184 RES 0 H
pub fn res_184(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h & 0b11111110u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x185 RES 0 L
pub fn res_185(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l & 0b11111110u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x186 RES 0 (HL)
pub fn res_186(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) & 0b11111110u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x187 RES 0 A
pub fn res_187(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & 0b11111110u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x188 RES 1 B
pub fn res_188(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b & 0b11111101u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x189 RES 1 C
pub fn res_189(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c & 0b11111101u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x18a RES 1 D
pub fn res_18a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d & 0b11111101u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x18b RES 1 E
pub fn res_18b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e & 0b11111101u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x18c RES 1 H
pub fn res_18c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h & 0b11111101u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x18d RES 1 L
pub fn res_18d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l & 0b11111101u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x18e RES 1 (HL)
pub fn res_18e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) & 0b11111101u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x18f RES 1 A
pub fn res_18f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & 0b11111101u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x190 RES 2 B
pub fn res_190(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b & 0b11111011u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x191 RES 2 C
pub fn res_191(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c & 0b11111011u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x192 RES 2 D
pub fn res_192(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d & 0b11111011u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x193 RES 2 E
pub fn res_193(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e & 0b11111011u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x194 RES 2 H
pub fn res_194(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h & 0b11111011u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x195 RES 2 L
pub fn res_195(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l & 0b11111011u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x196 RES 2 (HL)
pub fn res_196(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) & 0b11111011u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x197 RES 2 A
pub fn res_197(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & 0b11111011u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x198 RES 3 B
pub fn res_198(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b & 0b11110111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x199 RES 3 C
pub fn res_199(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c & 0b11110111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x19a RES 3 D
pub fn res_19a(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d & 0b11110111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x19b RES 3 E
pub fn res_19b(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e & 0b11110111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x19c RES 3 H
pub fn res_19c(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h & 0b11110111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x19d RES 3 L
pub fn res_19d(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l & 0b11110111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x19e RES 3 (HL)
pub fn res_19e(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) & 0b11110111u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x19f RES 3 A
pub fn res_19f(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & 0b11110111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a0 RES 4 B
pub fn res_1a0(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b & 0b11101111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a1 RES 4 C
pub fn res_1a1(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c & 0b11101111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a2 RES 4 D
pub fn res_1a2(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d & 0b11101111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a3 RES 4 E
pub fn res_1a3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e & 0b11101111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a4 RES 4 H
pub fn res_1a4(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h & 0b11101111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a5 RES 4 L
pub fn res_1a5(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l & 0b11101111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a6 RES 4 (HL)
pub fn res_1a6(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) & 0b11101111u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1a7 RES 4 A
pub fn res_1a7(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & 0b11101111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a8 RES 5 B
pub fn res_1a8(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b & 0b11011111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1a9 RES 5 C
pub fn res_1a9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c & 0b11011111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1aa RES 5 D
pub fn res_1aa(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d & 0b11011111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ab RES 5 E
pub fn res_1ab(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e & 0b11011111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ac RES 5 H
pub fn res_1ac(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h & 0b11011111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ad RES 5 L
pub fn res_1ad(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l & 0b11011111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ae RES 5 (HL)
pub fn res_1ae(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) & 0b11011111u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1af RES 5 A
pub fn res_1af(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & 0b11011111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b0 RES 6 B
pub fn res_1b0(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b & 0b10111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b1 RES 6 C
pub fn res_1b1(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c & 0b10111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b2 RES 6 D
pub fn res_1b2(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d & 0b10111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b3 RES 6 E
pub fn res_1b3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e & 0b10111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b4 RES 6 H
pub fn res_1b4(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h & 0b10111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b5 RES 6 L
pub fn res_1b5(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l & 0b10111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b6 RES 6 (HL)
pub fn res_1b6(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) & 0b10111111u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1b7 RES 6 A
pub fn res_1b7(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & 0b10111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b8 RES 7 B
pub fn res_1b8(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b & 0b1111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1b9 RES 7 C
pub fn res_1b9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c & 0b1111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ba RES 7 D
pub fn res_1ba(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d & 0b1111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1bb RES 7 E
pub fn res_1bb(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e & 0b1111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1bc RES 7 H
pub fn res_1bc(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h & 0b1111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1bd RES 7 L
pub fn res_1bd(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l & 0b1111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1be RES 7 (HL)
pub fn res_1be(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) & 0b1111111u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1bf RES 7 A
pub fn res_1bf(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a & 0b1111111u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c0 SET 0 B
pub fn set_1c0(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b | 0b1u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c1 SET 0 C
pub fn set_1c1(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c | 0b1u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c2 SET 0 D
pub fn set_1c2(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d | 0b1u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c3 SET 0 E
pub fn set_1c3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e | 0b1u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c4 SET 0 H
pub fn set_1c4(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h | 0b1u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c5 SET 0 L
pub fn set_1c5(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l | 0b1u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c6 SET 0 (HL)
pub fn set_1c6(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) | 0b1u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1c7 SET 0 A
pub fn set_1c7(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | 0b1u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c8 SET 1 B
pub fn set_1c8(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b | 0b10u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1c9 SET 1 C
pub fn set_1c9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c | 0b10u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ca SET 1 D
pub fn set_1ca(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d | 0b10u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1cb SET 1 E
pub fn set_1cb(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e | 0b10u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1cc SET 1 H
pub fn set_1cc(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h | 0b10u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1cd SET 1 L
pub fn set_1cd(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l | 0b10u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ce SET 1 (HL)
pub fn set_1ce(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) | 0b10u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1cf SET 1 A
pub fn set_1cf(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | 0b10u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d0 SET 2 B
pub fn set_1d0(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b | 0b100u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d1 SET 2 C
pub fn set_1d1(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c | 0b100u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d2 SET 2 D
pub fn set_1d2(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d | 0b100u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d3 SET 2 E
pub fn set_1d3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e | 0b100u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d4 SET 2 H
pub fn set_1d4(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h | 0b100u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d5 SET 2 L
pub fn set_1d5(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l | 0b100u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d6 SET 2 (HL)
pub fn set_1d6(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) | 0b100u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1d7 SET 2 A
pub fn set_1d7(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | 0b100u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d8 SET 3 B
pub fn set_1d8(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b | 0b1000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1d9 SET 3 C
pub fn set_1d9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c | 0b1000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1da SET 3 D
pub fn set_1da(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d | 0b1000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1db SET 3 E
pub fn set_1db(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e | 0b1000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1dc SET 3 H
pub fn set_1dc(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h | 0b1000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1dd SET 3 L
pub fn set_1dd(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l | 0b1000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1de SET 3 (HL)
pub fn set_1de(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) | 0b1000u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1df SET 3 A
pub fn set_1df(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | 0b1000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e0 SET 4 B
pub fn set_1e0(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b | 0b10000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e1 SET 4 C
pub fn set_1e1(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c | 0b10000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e2 SET 4 D
pub fn set_1e2(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d | 0b10000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e3 SET 4 E
pub fn set_1e3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e | 0b10000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e4 SET 4 H
pub fn set_1e4(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h | 0b10000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e5 SET 4 L
pub fn set_1e5(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l | 0b10000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e6 SET 4 (HL)
pub fn set_1e6(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) | 0b10000u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1e7 SET 4 A
pub fn set_1e7(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | 0b10000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e8 SET 5 B
pub fn set_1e8(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b | 0b100000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1e9 SET 5 C
pub fn set_1e9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c | 0b100000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ea SET 5 D
pub fn set_1ea(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d | 0b100000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1eb SET 5 E
pub fn set_1eb(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e | 0b100000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ec SET 5 H
pub fn set_1ec(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h | 0b100000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ed SET 5 L
pub fn set_1ed(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l | 0b100000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1ee SET 5 (HL)
pub fn set_1ee(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) | 0b100000u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1ef SET 5 A
pub fn set_1ef(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | 0b100000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f0 SET 6 B
pub fn set_1f0(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b | 0b1000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f1 SET 6 C
pub fn set_1f1(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c | 0b1000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f2 SET 6 D
pub fn set_1f2(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d | 0b1000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f3 SET 6 E
pub fn set_1f3(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e | 0b1000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f4 SET 6 H
pub fn set_1f4(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h | 0b1000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f5 SET 6 L
pub fn set_1f5(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l | 0b1000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f6 SET 6 (HL)
pub fn set_1f6(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) | 0b1000000u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1f7 SET 6 A
pub fn set_1f7(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | 0b1000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f8 SET 7 B
pub fn set_1f8(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.b = registers.b | 0b10000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1f9 SET 7 C
pub fn set_1f9(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.c = registers.c | 0b10000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1fa SET 7 D
pub fn set_1fa(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.d = registers.d | 0b10000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1fb SET 7 E
pub fn set_1fb(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.e = registers.e | 0b10000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1fc SET 7 H
pub fn set_1fc(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.h = registers.h | 0b10000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1fd SET 7 L
pub fn set_1fd(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.l = registers.l | 0b10000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

/// 0x1fe SET 7 (HL)
pub fn set_1fe(registers: &mut Registers, memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    memory.set(registers.get_hl(), memory.get(registers.get_hl()) | 0b10000000u8);
    registers.pc = registers.pc + 2;
    return 16;
}

/// 0x1ff SET 7 A
pub fn set_1ff(registers: &mut Registers, _memory: &mut dyn Memory, _argument: &mut Argument) -> u16 {
    registers.a = registers.a | 0b10000000u8;
    registers.pc = registers.pc + 2;
    return 8;
}

pub fn get_instruction(opcode: u16) -> Instruction {
    match opcode {
        0 => Instruction::new(nop_000, ImmediateArgumentType::None),
        1 => Instruction::new(ld_001, ImmediateArgumentType::Unsigned16Bits),
        2 => Instruction::new(ld_002, ImmediateArgumentType::None),
        3 => Instruction::new(inc_003, ImmediateArgumentType::None),
        4 => Instruction::new(inc_004, ImmediateArgumentType::None),
        5 => Instruction::new(dec_005, ImmediateArgumentType::None),
        6 => Instruction::new(ld_006, ImmediateArgumentType::Unsigned8Bits),
        7 => Instruction::new(rlca_007, ImmediateArgumentType::None),
        8 => Instruction::new(ld_008, ImmediateArgumentType::Unsigned16Bits),
        9 => Instruction::new(add_009, ImmediateArgumentType::None),
        10 => Instruction::new(ld_00a, ImmediateArgumentType::None),
        11 => Instruction::new(dec_00b, ImmediateArgumentType::None),
        12 => Instruction::new(inc_00c, ImmediateArgumentType::None),
        13 => Instruction::new(dec_00d, ImmediateArgumentType::None),
        14 => Instruction::new(ld_00e, ImmediateArgumentType::Unsigned8Bits),
        15 => Instruction::new(rrca_00f, ImmediateArgumentType::None),
        16 => Instruction::new(stop_010, ImmediateArgumentType::None),
        17 => Instruction::new(ld_011, ImmediateArgumentType::Unsigned16Bits),
        18 => Instruction::new(ld_012, ImmediateArgumentType::None),
        19 => Instruction::new(inc_013, ImmediateArgumentType::None),
        20 => Instruction::new(inc_014, ImmediateArgumentType::None),
        21 => Instruction::new(dec_015, ImmediateArgumentType::None),
        22 => Instruction::new(ld_016, ImmediateArgumentType::Unsigned8Bits),
        23 => Instruction::new(rla_017, ImmediateArgumentType::None),
        24 => Instruction::new(jr_018, ImmediateArgumentType::Signed8Bits),
        25 => Instruction::new(add_019, ImmediateArgumentType::None),
        26 => Instruction::new(ld_01a, ImmediateArgumentType::None),
        27 => Instruction::new(dec_01b, ImmediateArgumentType::None),
        28 => Instruction::new(inc_01c, ImmediateArgumentType::None),
        29 => Instruction::new(dec_01d, ImmediateArgumentType::None),
        30 => Instruction::new(ld_01e, ImmediateArgumentType::Unsigned8Bits),
        31 => Instruction::new(rra_01f, ImmediateArgumentType::None),
        32 => Instruction::new(jr_020, ImmediateArgumentType::Signed8Bits),
        33 => Instruction::new(ld_021, ImmediateArgumentType::Unsigned16Bits),
        34 => Instruction::new(ldi_022, ImmediateArgumentType::None),
        35 => Instruction::new(inc_023, ImmediateArgumentType::None),
        36 => Instruction::new(inc_024, ImmediateArgumentType::None),
        37 => Instruction::new(dec_025, ImmediateArgumentType::None),
        38 => Instruction::new(ld_026, ImmediateArgumentType::Unsigned8Bits),
        39 => Instruction::new(daa_027, ImmediateArgumentType::None),
        40 => Instruction::new(jr_028, ImmediateArgumentType::Signed8Bits),
        41 => Instruction::new(add_029, ImmediateArgumentType::None),
        42 => Instruction::new(ldi_02a, ImmediateArgumentType::None),
        43 => Instruction::new(dec_02b, ImmediateArgumentType::None),
        44 => Instruction::new(inc_02c, ImmediateArgumentType::None),
        45 => Instruction::new(dec_02d, ImmediateArgumentType::None),
        46 => Instruction::new(ld_02e, ImmediateArgumentType::Unsigned8Bits),
        47 => Instruction::new(cpl_02f, ImmediateArgumentType::None),
        48 => Instruction::new(jr_030, ImmediateArgumentType::Signed8Bits),
        49 => Instruction::new(ld_031, ImmediateArgumentType::Unsigned16Bits),
        50 => Instruction::new(ldd_032, ImmediateArgumentType::None),
        51 => Instruction::new(inc_033, ImmediateArgumentType::None),
        52 => Instruction::new(inc_034, ImmediateArgumentType::None),
        53 => Instruction::new(dec_035, ImmediateArgumentType::None),
        54 => Instruction::new(ld_036, ImmediateArgumentType::Unsigned8Bits),
        55 => Instruction::new(scf_037, ImmediateArgumentType::None),
        56 => Instruction::new(jr_038, ImmediateArgumentType::Signed8Bits),
        57 => Instruction::new(add_039, ImmediateArgumentType::None),
        58 => Instruction::new(ldd_03a, ImmediateArgumentType::None),
        59 => Instruction::new(dec_03b, ImmediateArgumentType::None),
        60 => Instruction::new(inc_03c, ImmediateArgumentType::None),
        61 => Instruction::new(dec_03d, ImmediateArgumentType::None),
        62 => Instruction::new(ld_03e, ImmediateArgumentType::Unsigned8Bits),
        63 => Instruction::new(ccf_03f, ImmediateArgumentType::None),
        64 => Instruction::new(ld_040, ImmediateArgumentType::None),
        65 => Instruction::new(ld_041, ImmediateArgumentType::None),
        66 => Instruction::new(ld_042, ImmediateArgumentType::None),
        67 => Instruction::new(ld_043, ImmediateArgumentType::None),
        68 => Instruction::new(ld_044, ImmediateArgumentType::None),
        69 => Instruction::new(ld_045, ImmediateArgumentType::None),
        70 => Instruction::new(ld_046, ImmediateArgumentType::None),
        71 => Instruction::new(ld_047, ImmediateArgumentType::None),
        72 => Instruction::new(ld_048, ImmediateArgumentType::None),
        73 => Instruction::new(ld_049, ImmediateArgumentType::None),
        74 => Instruction::new(ld_04a, ImmediateArgumentType::None),
        75 => Instruction::new(ld_04b, ImmediateArgumentType::None),
        76 => Instruction::new(ld_04c, ImmediateArgumentType::None),
        77 => Instruction::new(ld_04d, ImmediateArgumentType::None),
        78 => Instruction::new(ld_04e, ImmediateArgumentType::None),
        79 => Instruction::new(ld_04f, ImmediateArgumentType::None),
        80 => Instruction::new(ld_050, ImmediateArgumentType::None),
        81 => Instruction::new(ld_051, ImmediateArgumentType::None),
        82 => Instruction::new(ld_052, ImmediateArgumentType::None),
        83 => Instruction::new(ld_053, ImmediateArgumentType::None),
        84 => Instruction::new(ld_054, ImmediateArgumentType::None),
        85 => Instruction::new(ld_055, ImmediateArgumentType::None),
        86 => Instruction::new(ld_056, ImmediateArgumentType::None),
        87 => Instruction::new(ld_057, ImmediateArgumentType::None),
        88 => Instruction::new(ld_058, ImmediateArgumentType::None),
        89 => Instruction::new(ld_059, ImmediateArgumentType::None),
        90 => Instruction::new(ld_05a, ImmediateArgumentType::None),
        91 => Instruction::new(ld_05b, ImmediateArgumentType::None),
        92 => Instruction::new(ld_05c, ImmediateArgumentType::None),
        93 => Instruction::new(ld_05d, ImmediateArgumentType::None),
        94 => Instruction::new(ld_05e, ImmediateArgumentType::None),
        95 => Instruction::new(ld_05f, ImmediateArgumentType::None),
        96 => Instruction::new(ld_060, ImmediateArgumentType::None),
        97 => Instruction::new(ld_061, ImmediateArgumentType::None),
        98 => Instruction::new(ld_062, ImmediateArgumentType::None),
        99 => Instruction::new(ld_063, ImmediateArgumentType::None),
        100 => Instruction::new(ld_064, ImmediateArgumentType::None),
        101 => Instruction::new(ld_065, ImmediateArgumentType::None),
        102 => Instruction::new(ld_066, ImmediateArgumentType::None),
        103 => Instruction::new(ld_067, ImmediateArgumentType::None),
        104 => Instruction::new(ld_068, ImmediateArgumentType::None),
        105 => Instruction::new(ld_069, ImmediateArgumentType::None),
        106 => Instruction::new(ld_06a, ImmediateArgumentType::None),
        107 => Instruction::new(ld_06b, ImmediateArgumentType::None),
        108 => Instruction::new(ld_06c, ImmediateArgumentType::None),
        109 => Instruction::new(ld_06d, ImmediateArgumentType::None),
        110 => Instruction::new(ld_06e, ImmediateArgumentType::None),
        111 => Instruction::new(ld_06f, ImmediateArgumentType::None),
        112 => Instruction::new(ld_070, ImmediateArgumentType::None),
        113 => Instruction::new(ld_071, ImmediateArgumentType::None),
        114 => Instruction::new(ld_072, ImmediateArgumentType::None),
        115 => Instruction::new(ld_073, ImmediateArgumentType::None),
        116 => Instruction::new(ld_074, ImmediateArgumentType::None),
        117 => Instruction::new(ld_075, ImmediateArgumentType::None),
        118 => Instruction::new(halt_076, ImmediateArgumentType::None),
        119 => Instruction::new(ld_077, ImmediateArgumentType::None),
        120 => Instruction::new(ld_078, ImmediateArgumentType::None),
        121 => Instruction::new(ld_079, ImmediateArgumentType::None),
        122 => Instruction::new(ld_07a, ImmediateArgumentType::None),
        123 => Instruction::new(ld_07b, ImmediateArgumentType::None),
        124 => Instruction::new(ld_07c, ImmediateArgumentType::None),
        125 => Instruction::new(ld_07d, ImmediateArgumentType::None),
        126 => Instruction::new(ld_07e, ImmediateArgumentType::None),
        127 => Instruction::new(ld_07f, ImmediateArgumentType::None),
        128 => Instruction::new(add_080, ImmediateArgumentType::None),
        129 => Instruction::new(add_081, ImmediateArgumentType::None),
        130 => Instruction::new(add_082, ImmediateArgumentType::None),
        131 => Instruction::new(add_083, ImmediateArgumentType::None),
        132 => Instruction::new(add_084, ImmediateArgumentType::None),
        133 => Instruction::new(add_085, ImmediateArgumentType::None),
        134 => Instruction::new(add_086, ImmediateArgumentType::None),
        135 => Instruction::new(add_087, ImmediateArgumentType::None),
        136 => Instruction::new(adc_088, ImmediateArgumentType::None),
        137 => Instruction::new(adc_089, ImmediateArgumentType::None),
        138 => Instruction::new(adc_08a, ImmediateArgumentType::None),
        139 => Instruction::new(adc_08b, ImmediateArgumentType::None),
        140 => Instruction::new(adc_08c, ImmediateArgumentType::None),
        141 => Instruction::new(adc_08d, ImmediateArgumentType::None),
        142 => Instruction::new(adc_08e, ImmediateArgumentType::None),
        143 => Instruction::new(adc_08f, ImmediateArgumentType::None),
        144 => Instruction::new(sub_090, ImmediateArgumentType::None),
        145 => Instruction::new(sub_091, ImmediateArgumentType::None),
        146 => Instruction::new(sub_092, ImmediateArgumentType::None),
        147 => Instruction::new(sub_093, ImmediateArgumentType::None),
        148 => Instruction::new(sub_094, ImmediateArgumentType::None),
        149 => Instruction::new(sub_095, ImmediateArgumentType::None),
        150 => Instruction::new(sub_096, ImmediateArgumentType::None),
        151 => Instruction::new(sub_097, ImmediateArgumentType::None),
        152 => Instruction::new(sbc_098, ImmediateArgumentType::None),
        153 => Instruction::new(sbc_099, ImmediateArgumentType::None),
        154 => Instruction::new(sbc_09a, ImmediateArgumentType::None),
        155 => Instruction::new(sbc_09b, ImmediateArgumentType::None),
        156 => Instruction::new(sbc_09c, ImmediateArgumentType::None),
        157 => Instruction::new(sbc_09d, ImmediateArgumentType::None),
        158 => Instruction::new(sbc_09e, ImmediateArgumentType::None),
        159 => Instruction::new(sbc_09f, ImmediateArgumentType::None),
        160 => Instruction::new(and_0a0, ImmediateArgumentType::None),
        161 => Instruction::new(and_0a1, ImmediateArgumentType::None),
        162 => Instruction::new(and_0a2, ImmediateArgumentType::None),
        163 => Instruction::new(and_0a3, ImmediateArgumentType::None),
        164 => Instruction::new(and_0a4, ImmediateArgumentType::None),
        165 => Instruction::new(and_0a5, ImmediateArgumentType::None),
        166 => Instruction::new(and_0a6, ImmediateArgumentType::None),
        167 => Instruction::new(and_0a7, ImmediateArgumentType::None),
        168 => Instruction::new(xor_0a8, ImmediateArgumentType::None),
        169 => Instruction::new(xor_0a9, ImmediateArgumentType::None),
        170 => Instruction::new(xor_0aa, ImmediateArgumentType::None),
        171 => Instruction::new(xor_0ab, ImmediateArgumentType::None),
        172 => Instruction::new(xor_0ac, ImmediateArgumentType::None),
        173 => Instruction::new(xor_0ad, ImmediateArgumentType::None),
        174 => Instruction::new(xor_0ae, ImmediateArgumentType::None),
        175 => Instruction::new(xor_0af, ImmediateArgumentType::None),
        176 => Instruction::new(or_0b0, ImmediateArgumentType::None),
        177 => Instruction::new(or_0b1, ImmediateArgumentType::None),
        178 => Instruction::new(or_0b2, ImmediateArgumentType::None),
        179 => Instruction::new(or_0b3, ImmediateArgumentType::None),
        180 => Instruction::new(or_0b4, ImmediateArgumentType::None),
        181 => Instruction::new(or_0b5, ImmediateArgumentType::None),
        182 => Instruction::new(or_0b6, ImmediateArgumentType::None),
        183 => Instruction::new(or_0b7, ImmediateArgumentType::None),
        184 => Instruction::new(cp_0b8, ImmediateArgumentType::None),
        185 => Instruction::new(cp_0b9, ImmediateArgumentType::None),
        186 => Instruction::new(cp_0ba, ImmediateArgumentType::None),
        187 => Instruction::new(cp_0bb, ImmediateArgumentType::None),
        188 => Instruction::new(cp_0bc, ImmediateArgumentType::None),
        189 => Instruction::new(cp_0bd, ImmediateArgumentType::None),
        190 => Instruction::new(cp_0be, ImmediateArgumentType::None),
        191 => Instruction::new(cp_0bf, ImmediateArgumentType::None),
        192 => Instruction::new(ret_0c0, ImmediateArgumentType::None),
        193 => Instruction::new(pop_0c1, ImmediateArgumentType::None),
        194 => Instruction::new(jp_0c2, ImmediateArgumentType::Unsigned16Bits),
        195 => Instruction::new(jp_0c3, ImmediateArgumentType::Unsigned16Bits),
        196 => Instruction::new(call_0c4, ImmediateArgumentType::Unsigned16Bits),
        197 => Instruction::new(push_0c5, ImmediateArgumentType::None),
        198 => Instruction::new(add_0c6, ImmediateArgumentType::Unsigned8Bits),
        199 => Instruction::new(rst_0c7, ImmediateArgumentType::None),
        200 => Instruction::new(ret_0c8, ImmediateArgumentType::None),
        201 => Instruction::new(ret_0c9, ImmediateArgumentType::None),
        202 => Instruction::new(jp_0ca, ImmediateArgumentType::Unsigned16Bits),
        203 => Instruction::new(prefix_0cb, ImmediateArgumentType::None),
        204 => Instruction::new(call_0cc, ImmediateArgumentType::Unsigned16Bits),
        205 => Instruction::new(call_0cd, ImmediateArgumentType::Unsigned16Bits),
        206 => Instruction::new(adc_0ce, ImmediateArgumentType::Unsigned8Bits),
        207 => Instruction::new(rst_0cf, ImmediateArgumentType::None),
        208 => Instruction::new(ret_0d0, ImmediateArgumentType::None),
        209 => Instruction::new(pop_0d1, ImmediateArgumentType::None),
        210 => Instruction::new(jp_0d2, ImmediateArgumentType::Unsigned16Bits),
        211 => Instruction::new(unknown_0d3, ImmediateArgumentType::None),
        212 => Instruction::new(call_0d4, ImmediateArgumentType::Unsigned16Bits),
        213 => Instruction::new(push_0d5, ImmediateArgumentType::None),
        214 => Instruction::new(sub_0d6, ImmediateArgumentType::Unsigned8Bits),
        215 => Instruction::new(rst_0d7, ImmediateArgumentType::None),
        216 => Instruction::new(ret_0d8, ImmediateArgumentType::None),
        217 => Instruction::new(reti_0d9, ImmediateArgumentType::None),
        218 => Instruction::new(jp_0da, ImmediateArgumentType::Unsigned16Bits),
        219 => Instruction::new(unknown_0db, ImmediateArgumentType::None),
        220 => Instruction::new(call_0dc, ImmediateArgumentType::Unsigned16Bits),
        221 => Instruction::new(unknown_0dd, ImmediateArgumentType::None),
        222 => Instruction::new(sbc_0de, ImmediateArgumentType::Unsigned8Bits),
        223 => Instruction::new(rst_0df, ImmediateArgumentType::None),
        224 => Instruction::new(ldh_0e0, ImmediateArgumentType::Unsigned8Bits),
        225 => Instruction::new(pop_0e1, ImmediateArgumentType::None),
        226 => Instruction::new(ldspecial_0e2, ImmediateArgumentType::None),
        227 => Instruction::new(unknown_0e3, ImmediateArgumentType::None),
        228 => Instruction::new(unknown_0e4, ImmediateArgumentType::None),
        229 => Instruction::new(push_0e5, ImmediateArgumentType::None),
        230 => Instruction::new(and_0e6, ImmediateArgumentType::Unsigned8Bits),
        231 => Instruction::new(rst_0e7, ImmediateArgumentType::None),
        232 => Instruction::new(add_0e8, ImmediateArgumentType::Signed8Bits),
        233 => Instruction::new(jp_0e9, ImmediateArgumentType::None),
        234 => Instruction::new(ld_0ea, ImmediateArgumentType::Unsigned16Bits),
        235 => Instruction::new(unknown_0eb, ImmediateArgumentType::None),
        236 => Instruction::new(unknown_0ec, ImmediateArgumentType::None),
        237 => Instruction::new(unknown_0ed, ImmediateArgumentType::None),
        238 => Instruction::new(xor_0ee, ImmediateArgumentType::Unsigned8Bits),
        239 => Instruction::new(rst_0ef, ImmediateArgumentType::None),
        240 => Instruction::new(ldh_0f0, ImmediateArgumentType::Unsigned8Bits),
        241 => Instruction::new(pop_0f1, ImmediateArgumentType::None),
        242 => Instruction::new(ldspecial_0f2, ImmediateArgumentType::None),
        243 => Instruction::new(di_0f3, ImmediateArgumentType::None),
        244 => Instruction::new(unknown_0f4, ImmediateArgumentType::None),
        245 => Instruction::new(push_0f5, ImmediateArgumentType::None),
        246 => Instruction::new(or_0f6, ImmediateArgumentType::Unsigned8Bits),
        247 => Instruction::new(rst_0f7, ImmediateArgumentType::None),
        248 => Instruction::new(ldhl_0f8, ImmediateArgumentType::Signed8Bits),
        249 => Instruction::new(ld_0f9, ImmediateArgumentType::None),
        250 => Instruction::new(ld_0fa, ImmediateArgumentType::Unsigned16Bits),
        251 => Instruction::new(ei_0fb, ImmediateArgumentType::None),
        252 => Instruction::new(unknown_0fc, ImmediateArgumentType::None),
        253 => Instruction::new(unknown_0fd, ImmediateArgumentType::None),
        254 => Instruction::new(cp_0fe, ImmediateArgumentType::Unsigned8Bits),
        255 => Instruction::new(rst_0ff, ImmediateArgumentType::None),
        256 => Instruction::new(rlc_100, ImmediateArgumentType::None),
        257 => Instruction::new(rlc_101, ImmediateArgumentType::None),
        258 => Instruction::new(rlc_102, ImmediateArgumentType::None),
        259 => Instruction::new(rlc_103, ImmediateArgumentType::None),
        260 => Instruction::new(rlc_104, ImmediateArgumentType::None),
        261 => Instruction::new(rlc_105, ImmediateArgumentType::None),
        262 => Instruction::new(rlc_106, ImmediateArgumentType::None),
        263 => Instruction::new(rlc_107, ImmediateArgumentType::None),
        264 => Instruction::new(rrc_108, ImmediateArgumentType::None),
        265 => Instruction::new(rrc_109, ImmediateArgumentType::None),
        266 => Instruction::new(rrc_10a, ImmediateArgumentType::None),
        267 => Instruction::new(rrc_10b, ImmediateArgumentType::None),
        268 => Instruction::new(rrc_10c, ImmediateArgumentType::None),
        269 => Instruction::new(rrc_10d, ImmediateArgumentType::None),
        270 => Instruction::new(rrc_10e, ImmediateArgumentType::None),
        271 => Instruction::new(rrc_10f, ImmediateArgumentType::None),
        272 => Instruction::new(rl_110, ImmediateArgumentType::None),
        273 => Instruction::new(rl_111, ImmediateArgumentType::None),
        274 => Instruction::new(rl_112, ImmediateArgumentType::None),
        275 => Instruction::new(rl_113, ImmediateArgumentType::None),
        276 => Instruction::new(rl_114, ImmediateArgumentType::None),
        277 => Instruction::new(rl_115, ImmediateArgumentType::None),
        278 => Instruction::new(rl_116, ImmediateArgumentType::None),
        279 => Instruction::new(rl_117, ImmediateArgumentType::None),
        280 => Instruction::new(rr_118, ImmediateArgumentType::None),
        281 => Instruction::new(rr_119, ImmediateArgumentType::None),
        282 => Instruction::new(rr_11a, ImmediateArgumentType::None),
        283 => Instruction::new(rr_11b, ImmediateArgumentType::None),
        284 => Instruction::new(rr_11c, ImmediateArgumentType::None),
        285 => Instruction::new(rr_11d, ImmediateArgumentType::None),
        286 => Instruction::new(rr_11e, ImmediateArgumentType::None),
        287 => Instruction::new(rr_11f, ImmediateArgumentType::None),
        288 => Instruction::new(sla_120, ImmediateArgumentType::None),
        289 => Instruction::new(sla_121, ImmediateArgumentType::None),
        290 => Instruction::new(sla_122, ImmediateArgumentType::None),
        291 => Instruction::new(sla_123, ImmediateArgumentType::None),
        292 => Instruction::new(sla_124, ImmediateArgumentType::None),
        293 => Instruction::new(sla_125, ImmediateArgumentType::None),
        294 => Instruction::new(sla_126, ImmediateArgumentType::None),
        295 => Instruction::new(sla_127, ImmediateArgumentType::None),
        296 => Instruction::new(sra_128, ImmediateArgumentType::None),
        297 => Instruction::new(sra_129, ImmediateArgumentType::None),
        298 => Instruction::new(sra_12a, ImmediateArgumentType::None),
        299 => Instruction::new(sra_12b, ImmediateArgumentType::None),
        300 => Instruction::new(sra_12c, ImmediateArgumentType::None),
        301 => Instruction::new(sra_12d, ImmediateArgumentType::None),
        302 => Instruction::new(sra_12e, ImmediateArgumentType::None),
        303 => Instruction::new(sra_12f, ImmediateArgumentType::None),
        304 => Instruction::new(swap_130, ImmediateArgumentType::None),
        305 => Instruction::new(swap_131, ImmediateArgumentType::None),
        306 => Instruction::new(swap_132, ImmediateArgumentType::None),
        307 => Instruction::new(swap_133, ImmediateArgumentType::None),
        308 => Instruction::new(swap_134, ImmediateArgumentType::None),
        309 => Instruction::new(swap_135, ImmediateArgumentType::None),
        310 => Instruction::new(swap_136, ImmediateArgumentType::None),
        311 => Instruction::new(swap_137, ImmediateArgumentType::None),
        312 => Instruction::new(srl_138, ImmediateArgumentType::None),
        313 => Instruction::new(srl_139, ImmediateArgumentType::None),
        314 => Instruction::new(srl_13a, ImmediateArgumentType::None),
        315 => Instruction::new(srl_13b, ImmediateArgumentType::None),
        316 => Instruction::new(srl_13c, ImmediateArgumentType::None),
        317 => Instruction::new(srl_13d, ImmediateArgumentType::None),
        318 => Instruction::new(srl_13e, ImmediateArgumentType::None),
        319 => Instruction::new(srl_13f, ImmediateArgumentType::None),
        320 => Instruction::new(bit_140, ImmediateArgumentType::None),
        321 => Instruction::new(bit_141, ImmediateArgumentType::None),
        322 => Instruction::new(bit_142, ImmediateArgumentType::None),
        323 => Instruction::new(bit_143, ImmediateArgumentType::None),
        324 => Instruction::new(bit_144, ImmediateArgumentType::None),
        325 => Instruction::new(bit_145, ImmediateArgumentType::None),
        326 => Instruction::new(bit_146, ImmediateArgumentType::None),
        327 => Instruction::new(bit_147, ImmediateArgumentType::None),
        328 => Instruction::new(bit_148, ImmediateArgumentType::None),
        329 => Instruction::new(bit_149, ImmediateArgumentType::None),
        330 => Instruction::new(bit_14a, ImmediateArgumentType::None),
        331 => Instruction::new(bit_14b, ImmediateArgumentType::None),
        332 => Instruction::new(bit_14c, ImmediateArgumentType::None),
        333 => Instruction::new(bit_14d, ImmediateArgumentType::None),
        334 => Instruction::new(bit_14e, ImmediateArgumentType::None),
        335 => Instruction::new(bit_14f, ImmediateArgumentType::None),
        336 => Instruction::new(bit_150, ImmediateArgumentType::None),
        337 => Instruction::new(bit_151, ImmediateArgumentType::None),
        338 => Instruction::new(bit_152, ImmediateArgumentType::None),
        339 => Instruction::new(bit_153, ImmediateArgumentType::None),
        340 => Instruction::new(bit_154, ImmediateArgumentType::None),
        341 => Instruction::new(bit_155, ImmediateArgumentType::None),
        342 => Instruction::new(bit_156, ImmediateArgumentType::None),
        343 => Instruction::new(bit_157, ImmediateArgumentType::None),
        344 => Instruction::new(bit_158, ImmediateArgumentType::None),
        345 => Instruction::new(bit_159, ImmediateArgumentType::None),
        346 => Instruction::new(bit_15a, ImmediateArgumentType::None),
        347 => Instruction::new(bit_15b, ImmediateArgumentType::None),
        348 => Instruction::new(bit_15c, ImmediateArgumentType::None),
        349 => Instruction::new(bit_15d, ImmediateArgumentType::None),
        350 => Instruction::new(bit_15e, ImmediateArgumentType::None),
        351 => Instruction::new(bit_15f, ImmediateArgumentType::None),
        352 => Instruction::new(bit_160, ImmediateArgumentType::None),
        353 => Instruction::new(bit_161, ImmediateArgumentType::None),
        354 => Instruction::new(bit_162, ImmediateArgumentType::None),
        355 => Instruction::new(bit_163, ImmediateArgumentType::None),
        356 => Instruction::new(bit_164, ImmediateArgumentType::None),
        357 => Instruction::new(bit_165, ImmediateArgumentType::None),
        358 => Instruction::new(bit_166, ImmediateArgumentType::None),
        359 => Instruction::new(bit_167, ImmediateArgumentType::None),
        360 => Instruction::new(bit_168, ImmediateArgumentType::None),
        361 => Instruction::new(bit_169, ImmediateArgumentType::None),
        362 => Instruction::new(bit_16a, ImmediateArgumentType::None),
        363 => Instruction::new(bit_16b, ImmediateArgumentType::None),
        364 => Instruction::new(bit_16c, ImmediateArgumentType::None),
        365 => Instruction::new(bit_16d, ImmediateArgumentType::None),
        366 => Instruction::new(bit_16e, ImmediateArgumentType::None),
        367 => Instruction::new(bit_16f, ImmediateArgumentType::None),
        368 => Instruction::new(bit_170, ImmediateArgumentType::None),
        369 => Instruction::new(bit_171, ImmediateArgumentType::None),
        370 => Instruction::new(bit_172, ImmediateArgumentType::None),
        371 => Instruction::new(bit_173, ImmediateArgumentType::None),
        372 => Instruction::new(bit_174, ImmediateArgumentType::None),
        373 => Instruction::new(bit_175, ImmediateArgumentType::None),
        374 => Instruction::new(bit_176, ImmediateArgumentType::None),
        375 => Instruction::new(bit_177, ImmediateArgumentType::None),
        376 => Instruction::new(bit_178, ImmediateArgumentType::None),
        377 => Instruction::new(bit_179, ImmediateArgumentType::None),
        378 => Instruction::new(bit_17a, ImmediateArgumentType::None),
        379 => Instruction::new(bit_17b, ImmediateArgumentType::None),
        380 => Instruction::new(bit_17c, ImmediateArgumentType::None),
        381 => Instruction::new(bit_17d, ImmediateArgumentType::None),
        382 => Instruction::new(bit_17e, ImmediateArgumentType::None),
        383 => Instruction::new(bit_17f, ImmediateArgumentType::None),
        384 => Instruction::new(res_180, ImmediateArgumentType::None),
        385 => Instruction::new(res_181, ImmediateArgumentType::None),
        386 => Instruction::new(res_182, ImmediateArgumentType::None),
        387 => Instruction::new(res_183, ImmediateArgumentType::None),
        388 => Instruction::new(res_184, ImmediateArgumentType::None),
        389 => Instruction::new(res_185, ImmediateArgumentType::None),
        390 => Instruction::new(res_186, ImmediateArgumentType::None),
        391 => Instruction::new(res_187, ImmediateArgumentType::None),
        392 => Instruction::new(res_188, ImmediateArgumentType::None),
        393 => Instruction::new(res_189, ImmediateArgumentType::None),
        394 => Instruction::new(res_18a, ImmediateArgumentType::None),
        395 => Instruction::new(res_18b, ImmediateArgumentType::None),
        396 => Instruction::new(res_18c, ImmediateArgumentType::None),
        397 => Instruction::new(res_18d, ImmediateArgumentType::None),
        398 => Instruction::new(res_18e, ImmediateArgumentType::None),
        399 => Instruction::new(res_18f, ImmediateArgumentType::None),
        400 => Instruction::new(res_190, ImmediateArgumentType::None),
        401 => Instruction::new(res_191, ImmediateArgumentType::None),
        402 => Instruction::new(res_192, ImmediateArgumentType::None),
        403 => Instruction::new(res_193, ImmediateArgumentType::None),
        404 => Instruction::new(res_194, ImmediateArgumentType::None),
        405 => Instruction::new(res_195, ImmediateArgumentType::None),
        406 => Instruction::new(res_196, ImmediateArgumentType::None),
        407 => Instruction::new(res_197, ImmediateArgumentType::None),
        408 => Instruction::new(res_198, ImmediateArgumentType::None),
        409 => Instruction::new(res_199, ImmediateArgumentType::None),
        410 => Instruction::new(res_19a, ImmediateArgumentType::None),
        411 => Instruction::new(res_19b, ImmediateArgumentType::None),
        412 => Instruction::new(res_19c, ImmediateArgumentType::None),
        413 => Instruction::new(res_19d, ImmediateArgumentType::None),
        414 => Instruction::new(res_19e, ImmediateArgumentType::None),
        415 => Instruction::new(res_19f, ImmediateArgumentType::None),
        416 => Instruction::new(res_1a0, ImmediateArgumentType::None),
        417 => Instruction::new(res_1a1, ImmediateArgumentType::None),
        418 => Instruction::new(res_1a2, ImmediateArgumentType::None),
        419 => Instruction::new(res_1a3, ImmediateArgumentType::None),
        420 => Instruction::new(res_1a4, ImmediateArgumentType::None),
        421 => Instruction::new(res_1a5, ImmediateArgumentType::None),
        422 => Instruction::new(res_1a6, ImmediateArgumentType::None),
        423 => Instruction::new(res_1a7, ImmediateArgumentType::None),
        424 => Instruction::new(res_1a8, ImmediateArgumentType::None),
        425 => Instruction::new(res_1a9, ImmediateArgumentType::None),
        426 => Instruction::new(res_1aa, ImmediateArgumentType::None),
        427 => Instruction::new(res_1ab, ImmediateArgumentType::None),
        428 => Instruction::new(res_1ac, ImmediateArgumentType::None),
        429 => Instruction::new(res_1ad, ImmediateArgumentType::None),
        430 => Instruction::new(res_1ae, ImmediateArgumentType::None),
        431 => Instruction::new(res_1af, ImmediateArgumentType::None),
        432 => Instruction::new(res_1b0, ImmediateArgumentType::None),
        433 => Instruction::new(res_1b1, ImmediateArgumentType::None),
        434 => Instruction::new(res_1b2, ImmediateArgumentType::None),
        435 => Instruction::new(res_1b3, ImmediateArgumentType::None),
        436 => Instruction::new(res_1b4, ImmediateArgumentType::None),
        437 => Instruction::new(res_1b5, ImmediateArgumentType::None),
        438 => Instruction::new(res_1b6, ImmediateArgumentType::None),
        439 => Instruction::new(res_1b7, ImmediateArgumentType::None),
        440 => Instruction::new(res_1b8, ImmediateArgumentType::None),
        441 => Instruction::new(res_1b9, ImmediateArgumentType::None),
        442 => Instruction::new(res_1ba, ImmediateArgumentType::None),
        443 => Instruction::new(res_1bb, ImmediateArgumentType::None),
        444 => Instruction::new(res_1bc, ImmediateArgumentType::None),
        445 => Instruction::new(res_1bd, ImmediateArgumentType::None),
        446 => Instruction::new(res_1be, ImmediateArgumentType::None),
        447 => Instruction::new(res_1bf, ImmediateArgumentType::None),
        448 => Instruction::new(set_1c0, ImmediateArgumentType::None),
        449 => Instruction::new(set_1c1, ImmediateArgumentType::None),
        450 => Instruction::new(set_1c2, ImmediateArgumentType::None),
        451 => Instruction::new(set_1c3, ImmediateArgumentType::None),
        452 => Instruction::new(set_1c4, ImmediateArgumentType::None),
        453 => Instruction::new(set_1c5, ImmediateArgumentType::None),
        454 => Instruction::new(set_1c6, ImmediateArgumentType::None),
        455 => Instruction::new(set_1c7, ImmediateArgumentType::None),
        456 => Instruction::new(set_1c8, ImmediateArgumentType::None),
        457 => Instruction::new(set_1c9, ImmediateArgumentType::None),
        458 => Instruction::new(set_1ca, ImmediateArgumentType::None),
        459 => Instruction::new(set_1cb, ImmediateArgumentType::None),
        460 => Instruction::new(set_1cc, ImmediateArgumentType::None),
        461 => Instruction::new(set_1cd, ImmediateArgumentType::None),
        462 => Instruction::new(set_1ce, ImmediateArgumentType::None),
        463 => Instruction::new(set_1cf, ImmediateArgumentType::None),
        464 => Instruction::new(set_1d0, ImmediateArgumentType::None),
        465 => Instruction::new(set_1d1, ImmediateArgumentType::None),
        466 => Instruction::new(set_1d2, ImmediateArgumentType::None),
        467 => Instruction::new(set_1d3, ImmediateArgumentType::None),
        468 => Instruction::new(set_1d4, ImmediateArgumentType::None),
        469 => Instruction::new(set_1d5, ImmediateArgumentType::None),
        470 => Instruction::new(set_1d6, ImmediateArgumentType::None),
        471 => Instruction::new(set_1d7, ImmediateArgumentType::None),
        472 => Instruction::new(set_1d8, ImmediateArgumentType::None),
        473 => Instruction::new(set_1d9, ImmediateArgumentType::None),
        474 => Instruction::new(set_1da, ImmediateArgumentType::None),
        475 => Instruction::new(set_1db, ImmediateArgumentType::None),
        476 => Instruction::new(set_1dc, ImmediateArgumentType::None),
        477 => Instruction::new(set_1dd, ImmediateArgumentType::None),
        478 => Instruction::new(set_1de, ImmediateArgumentType::None),
        479 => Instruction::new(set_1df, ImmediateArgumentType::None),
        480 => Instruction::new(set_1e0, ImmediateArgumentType::None),
        481 => Instruction::new(set_1e1, ImmediateArgumentType::None),
        482 => Instruction::new(set_1e2, ImmediateArgumentType::None),
        483 => Instruction::new(set_1e3, ImmediateArgumentType::None),
        484 => Instruction::new(set_1e4, ImmediateArgumentType::None),
        485 => Instruction::new(set_1e5, ImmediateArgumentType::None),
        486 => Instruction::new(set_1e6, ImmediateArgumentType::None),
        487 => Instruction::new(set_1e7, ImmediateArgumentType::None),
        488 => Instruction::new(set_1e8, ImmediateArgumentType::None),
        489 => Instruction::new(set_1e9, ImmediateArgumentType::None),
        490 => Instruction::new(set_1ea, ImmediateArgumentType::None),
        491 => Instruction::new(set_1eb, ImmediateArgumentType::None),
        492 => Instruction::new(set_1ec, ImmediateArgumentType::None),
        493 => Instruction::new(set_1ed, ImmediateArgumentType::None),
        494 => Instruction::new(set_1ee, ImmediateArgumentType::None),
        495 => Instruction::new(set_1ef, ImmediateArgumentType::None),
        496 => Instruction::new(set_1f0, ImmediateArgumentType::None),
        497 => Instruction::new(set_1f1, ImmediateArgumentType::None),
        498 => Instruction::new(set_1f2, ImmediateArgumentType::None),
        499 => Instruction::new(set_1f3, ImmediateArgumentType::None),
        500 => Instruction::new(set_1f4, ImmediateArgumentType::None),
        501 => Instruction::new(set_1f5, ImmediateArgumentType::None),
        502 => Instruction::new(set_1f6, ImmediateArgumentType::None),
        503 => Instruction::new(set_1f7, ImmediateArgumentType::None),
        504 => Instruction::new(set_1f8, ImmediateArgumentType::None),
        505 => Instruction::new(set_1f9, ImmediateArgumentType::None),
        506 => Instruction::new(set_1fa, ImmediateArgumentType::None),
        507 => Instruction::new(set_1fb, ImmediateArgumentType::None),
        508 => Instruction::new(set_1fc, ImmediateArgumentType::None),
        509 => Instruction::new(set_1fd, ImmediateArgumentType::None),
        510 => Instruction::new(set_1fe, ImmediateArgumentType::None),
        511 => Instruction::new(set_1ff, ImmediateArgumentType::None),
        _ => panic!("Unknown opcode {:?}", opcode)
    }
}
