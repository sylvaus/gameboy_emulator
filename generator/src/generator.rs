use crate::instruction;
use crate::instruction::{Argument, ArgumentType, FlagAction, Instruction, InstructionType};
use crate::interface::{
    Code, Expression, Function, IntFormat, Language, Parameter, Register, Type, Variable,
};
use syn::token::In;

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
    let offset = language.hex_literal(0xFF00, Type::Uint16);
    let get_code = &create_get_code_with_offset(
        language,
        &instruction.second_argument.as_ref().unwrap(),
        Some(&offset),
    );

    let code = create_set_code_with_offset(
        language,
        instruction.first_argument.as_ref().unwrap(),
        &get_code,
        Some(&offset),
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
    let hl_one = [hl.get(), language.decimal_literal(1, Type::Uint16)];
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
    let code = language
        .registers
        .hl
        .set(&operation.result)
        .prepend(operation.code);

    create_function(instruction, language, USE_REGISTER_AND_MEMORY, code)
}

fn create_inc_dec(instruction: &Instruction, language: &Language) -> Function {
    let one = language.decimal_literal(1, Type::Int32);

    let operation = create_op_with_flag_code_3_custom_values(
        language,
        instruction,
        if instruction.type_field == InstructionType::INC {
            Operation::Add
        } else {
            Operation::Sub
        },
        &create_get_code(language, instruction.first_argument.as_ref().unwrap()),
        &one,
        None,
    );
    let code = create_set_code(
        language,
        instruction.first_argument.as_ref().unwrap(),
        &operation.result,
    )
    .prepend(operation.code);

    let used_params = if instruction.first_argument.as_ref().unwrap().is_address {
        USE_REGISTER_AND_MEMORY
    } else {
        ONLY_USE_REGISTER
    };
    create_function(instruction, language, used_params, code)
}

fn create_add_sub(instruction: &Instruction, language: &Language) -> Function {
    let operation = create_op_with_flag_code(
        language,
        instruction,
        if instruction.type_field == InstructionType::ADD {
            Operation::Add
        } else {
            Operation::Sub
        },
    );
    let code = create_set_code(
        language,
        instruction.first_argument.as_ref().unwrap(),
        &operation.result,
    )
    .prepend(operation.code);

    let argument = instruction.second_argument.as_ref().unwrap();
    let used_params = if argument.is_address || argument.is_immediate() {
        USE_REGISTER_AND_MEMORY
    } else {
        ONLY_USE_REGISTER
    };
    create_function(instruction, language, used_params, code)
}

fn create_rotate(instruction: &Instruction, language: &Language) -> Function {
    let argument = instruction
        .first_argument
        .clone()
        .unwrap_or(Argument::new_register(instruction::REGISTER_NAME_A));

    let value_u8 = language
        .statements
        .variable("value_u8", &create_get_code(language, &argument));
    let value = language.statements.variable(
        "value",
        &language.operations.cast(&value_u8.name, Type::Uint16),
    );

    let left_rotation = is_left_rotation(instruction);
    let carry_value = if left_rotation {
        language.bitwise_and_int(
            &language.shift_right_int(&value_u8.name, 7, IntFormat::Decimal),
            1,
            IntFormat::Bin,
        )
    } else {
        language.bitwise_and_int(&value_u8.name, 1, IntFormat::Bin)
    };
    let carry_value = language.statements.variable("carried_value", &carry_value);

    let bit_carried = if is_carry_rotation(instruction) {
        carry_value.name.clone()
    } else {
        language.registers.flags.get_carry_flag()
    };
    let bit_carried = language.operations.cast(&bit_carried, Type::Uint16);

    let result = if left_rotation {
        language.operations.add(&[
            language.shift_left_int(&value.name, 1, IntFormat::Decimal),
            bit_carried,
        ])
    } else {
        language.operations.add(&[
            language.shift_right_int(&value.name, 1, IntFormat::Decimal),
            language.shift_left_int(&bit_carried, 7, IntFormat::Decimal),
        ])
    };
    let result = language.statements.variable(
        "result",
        &language.operations.cast(
            &language.bitwise_and_int(&result, 0xFF, IntFormat::Hex),
            Type::Uint8,
        ),
    );

    let mut custom_flags = Vec::new();
    custom_flags.push(language.shift_left_int(
        &carry_value.name,
        OFFSET_CARRY_FLAG_VALUE,
        IntFormat::Decimal,
    ));
    let mut custom_flag_code = Code::create_empty();
    if instruction.zero_flag == FlagAction::CUSTOM {
        let zero_flag = language.statements.variable(
            "zero_flag",
            &language.operations.cast(
                &language.equals_int(
                    &language.bitwise_and_int(&result.name, 0xFF, IntFormat::Hex),
                    0,
                    IntFormat::Decimal,
                ),
                Type::Uint8,
            ),
        );

        custom_flag_code.iappend(zero_flag.code);
        custom_flags.push(language.shift_left_int(
            &zero_flag.name,
            OFFSET_ZERO_FLAG_VALUE,
            IntFormat::Decimal,
        ));
    }

    let code = Code::create_empty()
        .append(value_u8.code)
        .append(carry_value.code)
        .append(value.code)
        .append(result.code)
        .append(custom_flag_code)
        .append(create_set_flags(instruction, language, &custom_flags))
        .append(create_set_code(language, &argument, &result.name));

    let used_params = if argument.is_address {
        USE_REGISTER_AND_MEMORY
    } else {
        ONLY_USE_REGISTER
    };
    create_function(instruction, language, used_params, code)
}

fn is_left_rotation(instruction: &Instruction) -> bool {
    match instruction.type_field {
        InstructionType::RLCA
        | InstructionType::RLA
        | InstructionType::RLC
        | InstructionType::RL => true,
        InstructionType::RRCA
        | InstructionType::RRA
        | InstructionType::RRC
        | InstructionType::RR => false,
        _ => panic!("Only handle rotation"),
    }
}

// Is the rotation reusing the carried value in the rotation
fn is_carry_rotation(instruction: &Instruction) -> bool {
    match instruction.type_field {
        InstructionType::RLCA
        | InstructionType::RRCA
        | InstructionType::RLC
        | InstructionType::RRC => true,
        InstructionType::RLA | InstructionType::RRA | InstructionType::RL | InstructionType::RR => {
            false
        }
        _ => panic!("Only handle rotation"),
    }
}

fn create_jr(instruction: &Instruction, language: &Language) -> Function {
    let program_counter = &language.registers.program_counter;
    let program_counter_value = language
        .operations
        .cast(&program_counter.get(), Type::Int32);

    let increment = instruction
        .second_argument
        .as_ref()
        .unwrap_or(&instruction.first_argument.as_ref().unwrap());
    let increment = language.operations.cast(
        &create_get_code_no_address(&language, &increment),
        Type::Int32,
    );
    let instruction_length = language.operations.cast(
        &language
            .statements
            .int_literal(instruction.length, Type::Uint16, IntFormat::Decimal),
        Type::Int32,
    );
    let no_jump_pc = language.statements.variable(
        &"no_jump_pc",
        &language
            .operations
            .add(&[program_counter_value.clone(), instruction_length.clone()]),
    );
    let jump_pc = language.statements.variable(
        &"no_jump_pc",
        &language
            .operations
            .add(&[program_counter_value, instruction_length, increment]),
    );

    let no_jump = program_counter
        .set(&language.operations.cast(&no_jump_pc.name, Type::Uint16))
        .prepend(no_jump_pc.code)
        .append(language.return_int(
            instruction.duration_no_action,
            Type::Uint16,
            IntFormat::Decimal,
        ));
    let jump = program_counter
        .set(&language.operations.cast(&jump_pc.name, Type::Uint16))
        .prepend(jump_pc.code)
        .append(language.return_int(instruction.duration, Type::Uint16, IntFormat::Decimal));

    let code = if instruction.second_argument.is_none() {
        jump
    } else {
        language.statements.if_else(
            &get_flag_from_name(language, &instruction.first_argument.as_ref().unwrap().name),
            &jump,
            &no_jump,
        )
    };
    return create_function_custom(
        instruction,
        language,
        USE_REGISTER_AND_MEMORY,
        code,
        FunctionDetails {
            doc: None,
            pc_increment: None,
            return_value: None,
        },
    );
}

fn create_daa(instruction: &Instruction, language: &Language) -> Function {
    return create_function(
        instruction,
        language,
        ONLY_USE_REGISTER,
        language.statements.if_else(
            &language.registers.flags.get_add_sub_flag(),
            &create_daa_add(instruction, language),
            &create_daa_sub(instruction, language),
        ),
    );
}

fn create_daa_add(instruction: &Instruction, language: &Language) -> Code {
    let carry_flag = language.operations.or(&[
        language.greater_than_int(&language.registers.a.get(), 0x99, IntFormat::Hex),
        language.registers.flags.get_carry_flag(),
    ]);
    let carry_flag = language.variable_with_type("carry_flag", &carry_flag, Type::Uint8);
    let carry_part = language.operations.multiply(&[
        language.hex_literal(0x60, Type::Uint8),
        carry_flag.name.clone(),
    ]);
    let lower_bits_a = language.greater_than_int(
        &language.bitwise_or_int(&language.registers.a.get(), 0xF, IntFormat::Hex),
        0xA,
        IntFormat::Hex,
    );
    let half_carry_part = language
        .operations
        .or(&[lower_bits_a, language.registers.flags.get_half_carry_flag()]);
    let half_carry_part = language.operations.multiply(&[
        language.hex_literal(0x6, Type::Uint8),
        language.operations.cast(&half_carry_part, Type::Uint8),
    ]);

    let zero_flag = language.variable_with_type(
        "zero_flag",
        &language.equals_int(&language.registers.a.get(), 0, IntFormat::Hex),
        Type::Uint8,
    );
    let flags = [
        create_carry_flag_value(language, &carry_flag.name),
        create_zero_flag_value(language, &zero_flag.name),
    ];

    increment_register(
        language,
        language.registers.a.as_ref(),
        language.operations.add(&[carry_part, half_carry_part]),
    )
    .prepend(carry_flag.code)
    .append(zero_flag.code)
    .append(create_set_flags(instruction, language, &flags))
}

fn create_daa_sub(instruction: &Instruction, language: &Language) -> Code {
    let carry_flag = language.variable_with_type(
        "carry_flag",
        &language.registers.flags.get_carry_flag(),
        Type::Uint8,
    );
    let carry_part = language.operations.multiply(&[
        language.hex_literal(0x60, Type::Uint8),
        carry_flag.name.clone(),
    ]);
    let half_carry_part = language.operations.multiply(&[
        language.hex_literal(0x6, Type::Uint8),
        language
            .operations
            .cast(&language.registers.flags.get_half_carry_flag(), Type::Uint8),
    ]);

    let zero_flag = language.variable_with_type(
        "zero_flag",
        &language.equals_int(&language.registers.a.get(), 0, IntFormat::Hex),
        Type::Uint8,
    );
    let flags = [
        create_carry_flag_value(language, &carry_flag.name),
        create_zero_flag_value(language, &zero_flag.name),
    ];

    decrement_register(
        language,
        language.registers.a.as_ref(),
        language.operations.add(&[carry_part, half_carry_part]),
    )
    .prepend(carry_flag.code)
    .append(zero_flag.code)
    .append(create_set_flags(instruction, language, &flags))
}

pub fn increment_register(language: &Language, register: &dyn Register, value: Expression) -> Code {
    register.set(&language.operations.add(&[register.get(), value]))
}

pub fn decrement_register(language: &Language, register: &dyn Register, value: Expression) -> Code {
    register.set(&language.operations.sub(&[register.get(), value]))
}

fn create_cpl(instruction: &Instruction, language: &Language) -> Function {
    let a = &language.registers.a;
    let code = a
        .set(&language.operations.bitwise_not(&a.get()))
        .append(create_set_flags(instruction, language, &[]));

    return create_function(instruction, language, ONLY_USE_REGISTER, code);
}

fn create_scf(instruction: &Instruction, language: &Language) -> Function {
    return create_function(
        instruction,
        language,
        ONLY_USE_REGISTER,
        create_set_flags(instruction, language, &[]),
    );
}

fn create_ccf(instruction: &Instruction, language: &Language) -> Function {
    let carry = &language.operations.cast(
        &language
            .operations
            .bitwise_not(&language.registers.flags.get_carry_flag()),
        Type::Uint8,
    );
    let carry = language
        .statements
        .variable("carry_flag", &create_carry_flag_value(language, carry));

    return create_function(
        instruction,
        language,
        ONLY_USE_REGISTER,
        create_set_flags(instruction, language, &[carry.name]).prepend(carry.code),
    );
}

fn create_halt(instruction: &Instruction, language: &Language) -> Function {
    return create_function(
        instruction,
        language,
        ONLY_USE_REGISTER,
        language
            .registers
            .halted
            .set(&language.statements.bool_literal(true)),
    );
}

fn create_add_sub_with_carry(instruction: &Instruction, language: &Language) -> Function {
    let operation = create_op_with_flag_code_3_custom_values(
        language,
        instruction,
        if instruction.type_field == InstructionType::ADC {
            Operation::Add
        } else {
            Operation::Sub
        },
        &create_get_code(language, instruction.first_argument.as_ref().unwrap()),
        &create_get_code(language, instruction.second_argument.as_ref().unwrap()),
        Some(&language.registers.flags.get_half_carry_flag()),
    );

    let code = create_set_code(
        language,
        instruction.first_argument.as_ref().unwrap(),
        &operation.result,
    )
    .prepend(operation.code);

    let argument = instruction.second_argument.as_ref().unwrap();
    let used_params = if argument.is_address || argument.is_immediate() {
        USE_REGISTER_AND_MEMORY
    } else {
        ONLY_USE_REGISTER
    };
    return create_function(instruction, language, used_params, code);
}

fn create_bitwise_operation(instruction: &Instruction, language: &Language) -> Function {
    let register = &language.registers.a;
    let input = create_get_code(language, instruction.first_argument.as_ref().unwrap());

    let operation = match instruction.type_field {
        InstructionType::XOR => language.operations.bitwise_xor(&[register.get(), input]),
        InstructionType::OR => language.operations.bitwise_or(&[register.get(), input]),
        InstructionType::AND => language.operations.bitwise_and(&[register.get(), input]),
        _ => panic!(
            "Instruction type {:?} is not supported for bitwise operation: ",
            instruction.type_field
        ),
    };
    let operation = register.set(&operation);

    let zero_flag = language.equals_int(&register.get(), 0, IntFormat::Decimal);
    let zero_flag =
        create_zero_flag_value(language, &language.operations.cast(&zero_flag, Type::Uint8));

    return create_function(
        instruction,
        language,
        get_used_params(instruction),
        operation.append(create_set_flags(instruction, language, &[zero_flag])),
    );
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
        code.iappend(increment_pc(increment, language));
    }
    if let Some(value) = &details.return_value {
        code.iappend(language.statements.return_value(value));
    }

    language
        .statements
        .function(&name, &parameters, &code, &complete_doc, Type::Uint16)
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

fn create_get_code_with_offset(
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

fn get_flag_from_name(language: &Language, name: &str) -> Expression {
    match name.to_lowercase().as_str() {
        instruction::FLAG_NAME_CARRY => language.registers.flags.get_carry_flag(),
        instruction::FLAG_NAME_ZERO => language.registers.flags.get_zero_flag(),
        instruction::FLAG_NAME_NON_CARRY => language.registers.flags.get_non_carry_flag(),
        instruction::FLAG_NAME_NON_ZERO => language.registers.flags.get_non_zero_flag(),

        _ => panic!("No flag for name {}", name),
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
    create_op_with_flag_code_3_custom_values(
        language,
        instruction,
        operation,
        &create_get_code(language, instruction.first_argument.as_ref().unwrap()),
        &create_get_code(language, instruction.second_argument.as_ref().unwrap()),
        None,
    )
}

fn create_op_with_flag_code_3_custom_values(
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
        values.push(language.statements.variable("rrhs", value));
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
        .rev()
        .for_each(|value| result.code.iprepend(value.code.clone()));

    let mut custom_flag_values = Vec::new();
    let carry_max_value: i64 = get_carry_max_value(instruction);
    let half_carry_max_value: i64 = get_half_carry_max_value(instruction);
    if instruction.zero_flag == FlagAction::CUSTOM {
        let variable = compute_zero_flag(language, &result.name, carry_max_value);
        custom_flag_values.push(create_zero_flag_value(language, &variable.name));
        result.code.iappend(variable.code);
    }
    if instruction.half_carry_flag == FlagAction::CUSTOM {
        let variable =
            compute_half_carry_flag(language, &variable_names, half_carry_max_value, operation);
        custom_flag_values.push(create_half_carry_flag_value(language, &variable.name));
        result.code.iappend(variable.code);
    }
    if instruction.carry_flag == FlagAction::CUSTOM {
        let variable = compute_carry_flag(language, &result.name, carry_max_value, operation);
        custom_flag_values.push(create_carry_flag_value(language, &variable.name));
        result.code.iappend(variable.code);
    }

    let result_value = language.operations.cast(
        &language.bitwise_and_int(&result.name, carry_max_value, IntFormat::Hex),
        if instruction.is_two_bytes_op() {
            Type::Uint16
        } else {
            Type::Uint8
        },
    );

    result
        .code
        .iappend(create_set_flags(instruction, language, &custom_flag_values));
    OperationWithFlag {
        result: result_value,
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

fn compute_zero_flag(language: &Language, result: &Expression, carry_max_value: i64) -> Variable {
    let equals = language.equals_int(
        &language.bitwise_and_int(result, carry_max_value, IntFormat::Hex),
        0,
        IntFormat::Decimal,
    );

    language
        .statements
        .variable("zero_flag", &language.operations.cast(&equals, Type::Uint8))
}

fn compute_half_carry_flag(
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

fn compute_carry_flag(
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

pub fn create_zero_flag_value(language: &Language, name: &Expression) -> Expression {
    language.shift_left_int(name, OFFSET_ZERO_FLAG_VALUE, IntFormat::Decimal)
}

pub fn create_half_carry_flag_value(language: &Language, name: &Expression) -> Expression {
    language.shift_left_int(name, OFFSET_HALF_CARRY_FLAG_VALUE, IntFormat::Decimal)
}

pub fn create_carry_flag_value(language: &Language, name: &Expression) -> Expression {
    language.shift_left_int(name, OFFSET_CARRY_FLAG_VALUE, IntFormat::Decimal)
}

/// Create the code to set the flags values
fn create_set_flags(
    instruction: &Instruction,
    language: &Language,
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
        values.push(language.binary_literal(initial_flag, Type::Uint8));
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

fn get_used_params(instruction: &Instruction) -> UsedFnParams {
    if does_param_use_memory(&instruction.first_argument) || does_param_use_memory(&instruction.second_argument) {
        USE_REGISTER_AND_MEMORY
    } else {
        ONLY_USE_REGISTER
    }
}

fn does_param_use_memory(argument: &Option<Argument>) -> bool {
    if let Some(argument) = argument {
        argument.is_address || argument.is_immediate()
    } else {
        false
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
        InstructionType::LDH | InstructionType::LDSpecial => {
            Some(create_ldh_special(instruction, language))
        }
        InstructionType::LDHL => Some(create_ldhl(instruction, language)),
        InstructionType::INC | InstructionType::DEC => Some(create_inc_dec(instruction, language)),
        InstructionType::ADD | InstructionType::SUB => Some(create_add_sub(instruction, language)),
        InstructionType::RLCA
        | InstructionType::RRCA
        | InstructionType::RLA
        | InstructionType::RRA
        | InstructionType::RLC
        | InstructionType::RRC
        | InstructionType::RL
        | InstructionType::RR => Some(create_rotate(instruction, language)),
        InstructionType::JR => Some(create_jr(instruction, language)),
        InstructionType::DAA => Some(create_daa(instruction, language)),
        InstructionType::CPL => Some(create_cpl(instruction, language)),
        InstructionType::SCF => Some(create_scf(instruction, language)),
        InstructionType::CCF => Some(create_ccf(instruction, language)),
        InstructionType::HALT => Some(create_halt(instruction, language)),
        InstructionType::ADC | InstructionType::SBC => {
            Some(create_add_sub_with_carry(instruction, language))
        }
        InstructionType::XOR | InstructionType::OR | InstructionType::AND => {
            Some(create_bitwise_operation(instruction, language))
        }
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
        // InstructionType::SLA => {}
        // InstructionType::SRA => {}
        // InstructionType::SWAP => {}
        // InstructionType::SRL => {}
        // InstructionType::BIT => {}
        // InstructionType::RES => {}
        // InstructionType::SET => {}
        // InstructionType::STOP => {}
        // InstructionType::RETI => {}
        _ => None,
    }
}
