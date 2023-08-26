use crate::instruction;
use crate::interface::{Code, Expression, IntFormat, Language, Register};

pub fn increment_register(language: &Language, register: &dyn Register, value: Expression) -> Code {
    register.set(&language.operations.add(&[register.get(), value]))
}

pub fn increment_register_int(language: &Language, register: &dyn Register, value: i64, format: IntFormat) -> Code {
    register.set(&language.add_int(register.get(), value, format))
}

pub fn decrement_register(language: &Language, register: &dyn Register, value: Expression) -> Code {
    register.set(&language.operations.sub(&[register.get(), value]))
}

pub fn get_register_from_name<'a>(language: &'a Language, name: &str) -> &'a Box<dyn Register> {
    match name.to_lowercase().as_str() {
        instruction::REGISTER_NAME_A => &language.registers.a,
        instruction::REGISTER_NAME_B => &language.registers.b,
        instruction::REGISTER_NAME_C => &language.registers.c,
        instruction::REGISTER_NAME_D => &language.registers.d,
        instruction::REGISTER_NAME_E => &language.registers.e,
        instruction::REGISTER_NAME_H => &language.registers.h,
        instruction::REGISTER_NAME_L => &language.registers.l,
        instruction::REGISTER_NAME_AF => &language.registers.af,
        instruction::REGISTER_NAME_BC => &language.registers.bc,
        instruction::REGISTER_NAME_DE => &language.registers.de,
        instruction::REGISTER_NAME_HL => &language.registers.hl,
        instruction::REGISTER_NAME_SP => &language.registers.stack_pointer,
        instruction::REGISTER_NAME_PC => &language.registers.program_counter,

        _ => panic!("No register for name {}", name),
    }
}