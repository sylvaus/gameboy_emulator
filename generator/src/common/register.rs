use crate::interface::{Code, Expression, Language, Register};

pub fn increment_register(language: &Language, register: &dyn Register, value: Expression) -> Code {
    register.set(&language.operations.add(&[register.get(), value]))
}

pub fn decrement_register(language: &Language, register: &dyn Register, value: Expression) -> Code {
    register.set(&language.operations.sub(&[register.get(), value]))
}