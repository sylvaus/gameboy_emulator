use crate::instruction::RegisterName;
use crate::interface::{Code, Expression, IntFormat, Language, Register};

pub fn increment_register(language: &Language, register: &dyn Register, value: Expression) -> Code {
    register.set(&language.operations.add(&[register.get(), value]))
}

pub fn increment_register_int(
    language: &Language,
    register: &dyn Register,
    value: i64,
    format: IntFormat,
) -> Code {
    register.set(&language.add_int(register.get(), value, format))
}

pub fn decrement_register(language: &Language, register: &dyn Register, value: Expression) -> Code {
    register.set(&language.operations.sub(&[register.get(), value]))
}

pub fn decrement_register_int(
    language: &Language,
    register: &dyn Register,
    value: i64,
    format: IntFormat,
) -> Code {
    register.set(&language.sub_int(register.get(), value, format))
}

pub fn get_register_from_name<'a>(language: &'a Language, name: &str) -> &'a Box<dyn Register> {
    match RegisterName::from_name(name).unwrap_or_else(|| panic!("No register for name {}", name)) {
        RegisterName::A => &language.registers.a,
        RegisterName::B => &language.registers.b,
        RegisterName::C => &language.registers.c,
        RegisterName::D => &language.registers.d,
        RegisterName::E => &language.registers.e,
        RegisterName::H => &language.registers.h,
        RegisterName::L => &language.registers.l,
        RegisterName::AF => &language.registers.af,
        RegisterName::BC => &language.registers.bc,
        RegisterName::DE => &language.registers.de,
        RegisterName::HL => &language.registers.hl,
        RegisterName::SP => &language.registers.stack_pointer,
        RegisterName::PC => &language.registers.program_counter,
    }
}

pub fn get_sub_registers_from_name<'a>(
    language: &'a Language,
    name: &str,
) -> (&'a Box<dyn Register>, &'a Box<dyn Register>) {
    match RegisterName::from_name(name).unwrap_or_else(|| panic!("No u16 register for name {}", name)) {
        RegisterName::AF => (&language.registers.f, &language.registers.a),
        RegisterName::BC => (&language.registers.c, &language.registers.b),
        RegisterName::DE => (&language.registers.e, &language.registers.d),
        RegisterName::HL => (&language.registers.l, &language.registers.h),

        _ => panic!("Register {} is not supported", name),
    }
}
