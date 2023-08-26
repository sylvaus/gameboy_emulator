use clap::arg;
use crate::interface::{Code, Expression, Function, IntFormat, Language, Parameter, Register, Type};
use crate::parser::{Argument, ArgumentType, Instruction, InstructionType};

fn create_nop(instruction: &Instruction, language: &Language) -> Function {
    create_function(
        instruction,
        language,
        false,
        &increment_pc(instruction.length, language),
        None,
        &get_duration(instruction),
    )
}

fn create_unknown(instruction: &Instruction, language: &Language) -> Function {
    create_function(
        instruction,
        language,
        false,
        &language
            .statements
            .stop_with_message(&format!("Unknown opcode 0x{:X}", instruction.value)),
        None,
        &get_duration(instruction),
    )
}

fn create_ld(instruction: &Instruction, language: &Language) -> Function {
    let offset = language.statements.int_literal(0xFF00, Type::Uint16, IntFormat::Decimal);
    let get_code = match instruction.type_field {
        InstructionType::LDH | InstructionType::LDSpecial => create_get_code_with_offset(language, &instruction.second_argument.as_ref().unwrap(), Some(&offset)),
        _ => create_get_code(language, &instruction.second_argument.as_ref().unwrap())
    };

    let mut code = create_set_code(language, instruction.first_argument.as_ref().unwrap(), &get_code);

    let hl = &language.registers.hl;
    let one = language.statements.int_literal(1, Type::Uint16, IntFormat::Decimal);
    if instruction.type_field == InstructionType::LDI {
        code = code.append(hl.set(&language.operations.add(&hl.get(), &one)));
    }
    if instruction.type_field == InstructionType::LDD {
        code = code.append(hl.set(&language.operations.sub(&hl.get(), &one)));
    }
    code = code.append(increment_pc(instruction.length, language));

    create_function(
        &instruction,
        &language,
        true,
        &code,
        None,
        &get_duration(&instruction),
    )
}

fn create_function(
    instruction: &Instruction,
    language: &Language,
    use_memory: bool,
    code: &Code,
    doc: Option<&str>,
    return_value: &Expression,
) -> Function {
    let name = format!(
        "{}_{:03x}",
        instruction.type_field.get_name().to_lowercase(),
        instruction.value
    );

    let mut complete_doc = instruction.get_repr();
    if let Some(doc) = doc {
        complete_doc = format!("{}\n{}", complete_doc, doc)
    }

    let parameters = vec![
        // Registers is always used at least to update pc.
        Parameter::new(Type::Registers, language.registers.name.clone(), true),
        Parameter::new(Type::Memory, language.memory.name(), use_memory),
    ];

    language.statements.function(
        &name,
        &parameters,
        code,
        &complete_doc,
        return_value,
    )
}

fn increment_pc(increment: i64, language: &Language) -> Code {
    let program_counter = &language.registers.program_counter;
    let update = language.operations.add(
        &program_counter.get(),
        &Expression::from(&increment, Type::Uint16),
    );
    program_counter.set(&update)
}

fn get_duration(instruction: &Instruction) -> Expression {
    Expression::from(&instruction.duration, Type::Uint16)
}

fn create_set_code(language: &Language, argument: &Argument, value: &Expression) -> Code {
    create_set_code_with_offset(language, argument, value, None::<>)
}

fn create_set_code_with_offset(language: &Language, argument: &Argument, value: &Expression, address_offset: Option<&Expression>) -> Code {
    if argument.is_address {
        let mut address = create_get_code_no_address(language, argument);
        if let Some(offset) = address_offset {
            address = language.operations.add(&address, offset)
        }
        return create_set_memory_code(language, &address, value);
    }

    if argument.type_field != ArgumentType::Register {
        panic!("Argument Type cannot be a destination for set {:?}", argument)
    }

    get_register_from_name(language, &argument.name).set(value)
}

fn create_set_memory_code(language: &Language, address: &Expression, value: &Expression) -> Code {
    match value.type_ {
        Type::Uint8 => language.memory.set_8_bits(address, value),
        Type::Uint16 => language.memory.set_16_bits(address, value),
        _ => panic!("Impossible to set memory with: value: {:?}, address: {:?}", value, address)
    }
}

fn create_get_code(language: &Language, argument: &Argument) -> Expression {
    create_get_code_with_offset(language, argument, None::<>)
}

fn create_get_code_with_offset(
    language: &Language,
    argument: &Argument,
    offset: Option<&Expression>,
) -> Expression {
    let code = create_get_code_no_address(language, argument);
    if !argument.is_address {
        return code;
    }
    if let Some(address) = offset {
        return language.memory.get(&language.operations.add(address, &code));
    }
    language.memory.get(&code)
}

fn create_get_code_no_address(language: &Language, argument: &Argument) -> Expression {
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

fn get_register_from_name<'a>(language: &'a Language, name: &str) -> &'a Box<dyn Register> {
    match name.to_lowercase().as_str() {
        "a" => &language.registers.a,
        "b" => &language.registers.b,
        "c" => &language.registers.c,
        "d" => &language.registers.d,
        "e" => &language.registers.e,
        "h" => &language.registers.h,
        "l" => &language.registers.l,
        "af" => &language.registers.af,
        "bc" => &language.registers.bc,
        "de" => &language.registers.de,
        "hl" => &language.registers.hl,
        "sp" => &language.registers.stack_pointer,
        "pc" => &language.registers.program_counter,

        _ => panic!("No register for name {}", name),
    }
}

pub fn create_instruction_function(
    instruction: &Instruction,
    language: &Language,
) -> Option<Function> {
    match instruction.type_field {
        InstructionType::UNKNOWN => Some(create_unknown(instruction, language)),
        InstructionType::NOP => Some(create_nop(instruction, language)),
        InstructionType::LD => Some(create_ld(instruction, language)),
        // InstructionType::LD => {}
        // InstructionType::LDI => {}
        // InstructionType::LDD => {}
        // InstructionType::LDH => {}
        // InstructionType::LDHL => {}
        // InstructionType::INC => {}
        // InstructionType::DEC => {}
        // InstructionType::ADD => {}
        // InstructionType::RLCA => {}
        // InstructionType::RRCA => {}
        // InstructionType::RLA => {}
        // InstructionType::RRA => {}
        // InstructionType::JR => {}
        // InstructionType::DAA => {}
        // InstructionType::CPL => {}
        // InstructionType::SCF => {}
        // InstructionType::CCF => {}
        // InstructionType::HALT => {}
        // InstructionType::ADC => {}
        // InstructionType::SUB => {}
        // InstructionType::SBC => {}
        // InstructionType::XOR => {}
        // InstructionType::OR => {}
        // InstructionType::CP => {}
        // InstructionType::RET => {}
        // InstructionType::POP => {}
        // InstructionType::JP => {}
        // InstructionType::CALL => {}
        // InstructionType::PUSH => {}
        // InstructionType::RST => {}
        // InstructionType::PREFIX => {}
        // InstructionType::DI => {}
        // InstructionType::EI => {}
        // InstructionType::RLC => {}
        // InstructionType::RRC => {}
        // InstructionType::RL => {}
        // InstructionType::RR => {}
        // InstructionType::SLA => {}
        // InstructionType::SRA => {}
        // InstructionType::SWAP => {}
        // InstructionType::SRL => {}
        // InstructionType::BIT => {}
        // InstructionType::RES => {}
        // InstructionType::SET => {}
        // InstructionType::STOP => {}
        // InstructionType::AND => {}
        // InstructionType::RETI => {}
        // InstructionType::LDSpecial => {}
        _ => None,
    }
}
