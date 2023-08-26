use crate::common::register::get_register_from_name;
use crate::instruction;
use crate::instruction::{Argument, ArgumentType};
use crate::interface::{Code, Expression, Language, Register, Type};

pub fn create_set_code(language: &Language, argument: &Argument, value: &Expression) -> Code {
    create_set_code_with_offset(language, argument, value, None)
}

pub fn create_set_code_with_offset(
    language: &Language,
    argument: &Argument,
    value: &Expression,
    address_offset: Option<&Expression>,
) -> Code {
    if argument.is_address {
        let mut address = language.operations.cast(
            &create_get_code_no_address(language, argument),
            Type::Uint16,
        );
        if let Some(offset) = address_offset {
            address = language.operations.add(&[address, offset.clone()])
        }
        return create_set_memory_code(language, &address, value);
    }

    if argument.type_field != ArgumentType::Register {
        panic!(
            "Argument Type cannot be a destination for set {:?}",
            argument
        )
    }

    get_register_from_name(language, &argument.name).set(value)
}

pub fn create_set_memory_code(
    language: &Language,
    address: &Expression,
    value: &Expression,
) -> Code {
    match value.type_ {
        Type::Uint8 => language.memory.set_8_bits(address, value),
        Type::Uint16 => language.memory.set_16_bits(address, value),
        _ => panic!(
            "Impossible to set memory with: value: {:?}, address: {:?}",
            value, address
        ),
    }
}

pub fn create_get_code(language: &Language, argument: &Argument) -> Expression {
    create_get_code_with_offset(language, argument, None)
}

pub fn create_get_code_with_offset(
    language: &Language,
    argument: &Argument,
    address_offset: Option<&Expression>,
) -> Expression {
    let mut code = create_get_code_no_address(language, argument);

    if !argument.is_address {
        return code;
    }

    code = language.operations.cast(&code, Type::Uint16);
    if let Some(offset) = address_offset {
        code = language.operations.add(&[code, offset.clone()])
    }

    language.memory.get(&code)
}

pub fn create_get_code_no_address(language: &Language, argument: &Argument) -> Expression {
    match argument.type_field {
        ArgumentType::Register => get_register_from_name(language, &argument.name).get(),
        ArgumentType::Immediate8Bits => language.arguments.get_uint8(),
        ArgumentType::Immediate16Bits => language.arguments.get_uint16(),
        ArgumentType::Unsigned8Bit => language.arguments.get_uint8(),
        ArgumentType::Address16Bit => language.arguments.get_uint16(),
        ArgumentType::PCIncrement8Bit => language.arguments.get_int8(),

        _ => panic!(
            "Argument type does not support getting {}",
            argument.type_field.get_name()
        ),
    }
}

pub fn get_flag_from_name(language: &Language, name: &str) -> Expression {
    match name.to_lowercase().as_str() {
        instruction::FLAG_NAME_CARRY => language.registers.flags.get_carry_flag(),
        instruction::FLAG_NAME_ZERO => language.registers.flags.get_zero_flag(),
        instruction::FLAG_NAME_NON_CARRY => language.registers.flags.get_non_carry_flag(),
        instruction::FLAG_NAME_NON_ZERO => language.registers.flags.get_non_zero_flag(),

        _ => panic!("No flag for name {}", name),
    }
}
