use crate::interface::{
    Code, Expression, Function, IntFormat, Language, Parameter, Register, Type, Variable,
};
use crate::interface::Type::Uint16;
use crate::parser::{Argument, ArgumentType, FlagAction, Instruction, InstructionType};

const OFFSET_CARRY_FLAG_VALUE: i64 = 4;
const OFFSET_HALF_CARRY_FLAG_VALUE: i64 = 5;
const OFFSET_ADD_SUB_FLAG_VALUE: i64 = 6;
const OFFSET_ZERO_FLAG_VALUE: i64 = 7;

fn create_nop(instruction: &Instruction, language: &Language) -> Function {
    create_function(
        instruction,
        language,
        ONLY_USE_REGISTER,
        Code::create_empty(),
    )
}

fn create_unknown(instruction: &Instruction, language: &Language) -> Function {
    create_function_custom(
        instruction,
        language,
        NO_USED_PARAMS,
        language
            .statements
            .stop_with_message(&format!("Unknown opcode 0x{:X}", instruction.value)),
        FunctionDetails::default(),
    )
}

fn create_ld(instruction: &Instruction, language: &Language) -> Function {
    let get_code = create_get_code(language, &instruction.second_argument.as_ref().unwrap());

    let code = create_set_code(
        language,
        instruction.first_argument.as_ref().unwrap(),
        &get_code,
    );

    create_function(
        &instruction,
        &language,
        create_ld_used_params(instruction),
        code,
    )
}

fn create_ldh_special(instruction: &Instruction, language: &Language) -> Function {
    let offset = language
        .statements
        .int_literal(0xFF00, Type::Uint16, IntFormat::Hex);
    let get_code = &create_get_code_with_offset(
        language,
        &instruction.second_argument.as_ref().unwrap(),
        Some(&offset)
    );

    let code = create_set_code_with_offset(
        language,
        instruction.first_argument.as_ref().unwrap(),
        &get_code,
        Some(&offset)
    );

    create_function(
        &instruction,
        &language,
        create_ld_used_params(instruction),
        code,
    )
}

fn create_ldid(instruction: &Instruction, language: &Language) -> Function {
    let get_code = create_get_code(language, &instruction.second_argument.as_ref().unwrap());

    let mut code = create_set_code(
        language,
        instruction.first_argument.as_ref().unwrap(),
        &get_code,
    );

    let hl = &language.registers.hl;
    let hl_one = [
        hl.get(),
        language
            .statements
            .int_literal(1, Type::Uint16, IntFormat::Decimal),
    ];
    if instruction.type_field == InstructionType::LDI {
        code = code.append(hl.set(&language.operations.add(&hl_one)));
    } else if instruction.type_field == InstructionType::LDD {
        code = code.append(hl.set(&language.operations.sub(&hl_one)));
    }

    create_function(
        &instruction,
        &language,
        create_ld_used_params(instruction),
        code,
    )
}

fn create_ld_used_params(instruction: &Instruction) -> UsedFnParams {
    UsedFnParams {
        register: true,
        memory: instruction.first_argument.as_ref().unwrap().is_address
            || instruction.second_argument.as_ref().unwrap().is_address
            || instruction.second_argument.as_ref().unwrap().is_immediate(),
    }
}

fn create_ldhl(instruction: &Instruction, language: &Language) -> Function {
    let operation = create_op_with_flag_code(language, instruction, Operation::Add);
    let value = language.operations.cast(&operation.result, Type::Uint16);
    let code = language.registers.hl.set(&value).prepend(operation.code);

    create_function(instruction, language, USE_REGISTER_AND_MEMORY, code)
}

#[derive(Debug, Clone)]
struct UsedFnParams {
    pub register: bool,
    pub memory: bool,
}

const NO_USED_PARAMS: UsedFnParams = UsedFnParams {
    register: false,
    memory: false,
};
const ONLY_USE_REGISTER: UsedFnParams = UsedFnParams {
    register: true,
    memory: false,
};
const ONLY_USE_MEMORY: UsedFnParams = UsedFnParams {
    register: false,
    memory: true,
};
const USE_REGISTER_AND_MEMORY: UsedFnParams = UsedFnParams {
    register: true,
    memory: true,
};

#[derive(Debug, Default)]
struct FunctionDetails {
    pub doc: Option<String>,
    pub pc_increment: Option<i64>,
    pub return_value: Option<Expression>,
}

fn create_function(
    instruction: &Instruction,
    language: &Language,
    used_params: UsedFnParams,
    code: Code,
) -> Function {
    create_function_custom(
        instruction,
        language,
        used_params,
        code,
        FunctionDetails {
            doc: None,
            pc_increment: Some(instruction.length),
            return_value: Some(get_duration(&instruction)),
        },
    )
}

fn create_function_custom(
    instruction: &Instruction,
    language: &Language,
    used_params: UsedFnParams,
    mut code: Code,
    details: FunctionDetails,
) -> Function {
    let name = format!(
        "{}_{:03x}",
        instruction.type_field.get_name().to_lowercase(),
        instruction.value
    );

    let mut complete_doc = instruction.get_repr();
    if let Some(doc) = &details.doc {
        complete_doc = format!("{}\n{}", complete_doc, doc)
    }

    let parameters = [
        Parameter::new(
            Type::Registers,
            language.registers.name.clone(),
            used_params.register,
        ),
        Parameter::new(Type::Memory, language.memory.name(), used_params.memory),
    ];

    if let Some(increment) = details.pc_increment {
        code.iappend(increment_pc(increment, language))
    }

    language.statements.function(
        &name,
        &parameters,
        &code,
        &complete_doc,
        details.return_value.as_ref(),
    )
}

fn increment_pc(increment: i64, language: &Language) -> Code {
    let program_counter = &language.registers.program_counter;
    let update = language.operations.add(&[
        program_counter.get(),
        Expression::from(&increment, Type::Uint16),
    ]);
    program_counter.set(&update)
}

fn get_duration(instruction: &Instruction) -> Expression {
    Expression::from(&instruction.duration, Type::Uint16)
}

fn create_set_code(language: &Language, argument: &Argument, value: &Expression) -> Code {
    create_set_code_with_offset(language, argument, value, None)
}

fn create_set_code_with_offset(
    language: &Language,
    argument: &Argument,
    value: &Expression,
    address_offset: Option<&Expression>,
) -> Code {
    if argument.is_address {
        let mut address = language.operations.cast(
            &create_get_code_no_address(language, argument),
            Type::Uint16
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

fn create_set_memory_code(language: &Language, address: &Expression, value: &Expression) -> Code {
    match value.type_ {
        Type::Uint8 => language.memory.set_8_bits(address, value),
        Type::Uint16 => language.memory.set_16_bits(address, value),
        _ => panic!(
            "Impossible to set memory with: value: {:?}, address: {:?}",
            value, address
        ),
    }
}

fn create_get_code(language: &Language, argument: &Argument) -> Expression {
    create_get_code_with_offset(language, argument, None)
}

fn create_get_code_with_offset(language: &Language, argument: &Argument, address_offset: Option<&Expression>) -> Expression {
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

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Sub,
}

struct OperationWithFlag {
    pub result: Expression,
    pub code: Code,
}

fn create_op_with_flag_code(
    language: &Language,
    instruction: &Instruction,
    operation: Operation,
) -> OperationWithFlag {
    create_op_with_flag_code_custom_3_values(
        language,
        instruction,
        operation,
        &create_get_code(language, instruction.first_argument.as_ref().unwrap()),
        &create_get_code(language, instruction.second_argument.as_ref().unwrap()),
        None,
    )
}

fn create_op_with_flag_code_custom_3_values(
    language: &Language,
    instruction: &Instruction,
    operation: Operation,
    first: &Expression,
    second: &Expression,
    third: Option<&Expression>,
) -> OperationWithFlag {
    let mut values = vec![
        language.statements.variable("lhs", first),
        language.statements.variable("rhs", second),
    ];
    if let Some(value) = third {
        values.push(language.statements.variable("rhs", value));
    }
    let variable_names = values
        .iter()
        .map(|value| language.operations.cast(&value.name, Type::Int32))
        .collect::<Vec<Expression>>();
    let mut result = language.statements.variable(
        "result",
        &match operation {
            Operation::Add => language.operations.add(&variable_names),
            Operation::Sub => language.operations.sub(&variable_names),
        },
    );
    values
        .iter()
        .for_each(|value| result.code.iprepend(value.code.clone()));

    let mut custom_flag_values = Vec::new();
    let carry_max_value: i64 = get_carry_max_value(instruction);
    let half_carry_max_value: i64 = get_half_carry_max_value(instruction);
    if instruction.zero_flag == FlagAction::CUSTOM {
        let variable = create_zero_flag(language, &result.name, carry_max_value);
        custom_flag_values.push(language.shift_left_int(
            &variable.name,
            OFFSET_ZERO_FLAG_VALUE,
            IntFormat::Decimal,
        ));
        result.code.iappend(variable.code);
    }
    if instruction.half_carry_flag == FlagAction::CUSTOM {
        let variable =
            create_half_carry_flag(language, &variable_names, half_carry_max_value, operation);
        custom_flag_values.push(language.shift_left_int(
            &variable.name,
            OFFSET_HALF_CARRY_FLAG_VALUE,
            IntFormat::Decimal,
        ));
        result.code.iappend(variable.code);
    }
    if instruction.carry_flag == FlagAction::CUSTOM {
        let variable = create_carry_flag(language, &result.name, carry_max_value, operation);
        custom_flag_values.push(language.shift_left_int(
            &variable.name,
            OFFSET_CARRY_FLAG_VALUE,
            IntFormat::Decimal,
        ));
        result.code.iappend(variable.code);
    }

    result
        .code
        .iappend(create_set_flags(language, instruction, &custom_flag_values));
    OperationWithFlag {
        result: language.bitwise_and_int(&result.name, carry_max_value, IntFormat::Hex),
        code: result.code,
    }
}

pub fn get_carry_max_value(instruction: &Instruction) -> i64 {
    if instruction.is_two_bytes_op() {
        0xFFFF
    } else {
        0xFF
    }
}

pub fn get_half_carry_max_value(instruction: &Instruction) -> i64 {
    if instruction.is_two_bytes_op() {
        0xFFF
    } else {
        0xF
    }
}

fn create_zero_flag(language: &Language, result: &Expression, carry_max_value: i64) -> Variable {
    let equals = language.operations.equals(
        result,
        &language
            .statements
            .int_literal(carry_max_value, result.type_, IntFormat::Hex),
    );

    language
        .statements
        .variable("zero_flag", &language.operations.cast(&equals, Type::Uint8))
}

fn create_half_carry_flag(
    language: &Language,
    values: &[Expression],
    half_carry_max_value: i64,
    operation: Operation,
) -> Variable {
    let elements = values
        .iter()
        .map(|value| language.operations.cast(value, Type::Int32))
        .map(|value| language.bitwise_and_int(&value, half_carry_max_value, IntFormat::Hex))
        .collect::<Vec<Expression>>();

    let half_value_expr = match operation {
        Operation::Add => language.operations.add(&elements),
        Operation::Sub => language.operations.sub(&elements),
    };
    let half_value = language.statements.variable("half_value", &half_value_expr);

    let half_flag_expr = match operation {
        Operation::Add => {
            language.greater_than_int(&half_value.name, half_carry_max_value, IntFormat::Hex)
        }
        Operation::Sub => language.lesser_than_int(&half_value.name, 0, IntFormat::Hex),
    };
    let mut half_flag = language.statements.variable(
        "half_flag",
        &language.operations.cast(&half_flag_expr, Type::Uint8),
    );

    half_flag.code = half_flag.code.prepend(half_value.code);
    half_flag
}

fn create_carry_flag(
    language: &Language,
    result: &Expression,
    carry_max_value: i64,
    operation: Operation,
) -> Variable {
    let carry_flag_expr = match operation {
        Operation::Add => language.greater_than_int(result, carry_max_value, IntFormat::Hex),
        Operation::Sub => language.lesser_than_int(result, 0, IntFormat::Hex),
    };

    language.statements.variable(
        "carry_flag",
        &language.operations.cast(&carry_flag_expr, Type::Uint8),
    )
}

/// Create the code to set the flags values
fn create_set_flags(
    language: &Language,
    instruction: &Instruction,
    custom_flags: &[Expression],
) -> Code {
    let flags = instruction.get_flags();
    if flags.iter().all(|flag| flag.action == FlagAction::NONE) {
        return Code::create_empty();
    }

    let mut values = custom_flags.to_vec();
    // Value which needs to be set
    let initial_flag = flags
        .iter()
        .map(|flag| flag.action.set_as_u8() << flag.offset)
        .sum::<u8>() as i64;
    if initial_flag > 0 {
        values.push(
            language
                .statements
                .int_literal(initial_flag, Type::Uint8, IntFormat::Bin),
        );
    }

    // Keep the flag values that do not need to change.
    let current_flag_mask = flags
        .iter()
        .map(|flag| flag.action.none_as_u8() << flag.offset)
        .sum::<u8>() as i64;

    if current_flag_mask > 0 {
        values.push(language.bitwise_and_int(
            &language.registers.flags.get(),
            current_flag_mask,
            IntFormat::Bin,
        ));
    }

    if values.is_empty() {
        Code::create_empty()
    } else {
        language
            .registers
            .flags
            .set(&language.operations.add(&values))
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
        InstructionType::LDI | InstructionType::LDD => Some(create_ldid(instruction, language)),
        InstructionType::LDH | InstructionType::LDSpecial => Some(create_ldh_special(instruction, language)),
        InstructionType::LDHL => Some(create_ldhl(instruction, language)),
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
        _ => None,
    }
}
