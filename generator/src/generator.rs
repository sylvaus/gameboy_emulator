use crate::common::base::Operation;
use crate::common::flags::{
    create_carry_flag_value, create_set_flags, create_zero_flag_value, get_flag_from_name,
};
use crate::common::function::{
    create_function, create_function_custom, get_used_params, FunctionDetails, NO_USED_PARAMS,
    ONLY_USE_REGISTER, USE_ALL_PARAMETERS, USE_REGISTER_AND_ARGUMENT, USE_REGISTER_AND_MEMORY,
};
use crate::common::getset::{
    create_get_code, create_get_code_no_address, create_get_code_with_offset, create_set_code,
    create_set_code_with_offset, create_set_memory_code,
};
use crate::common::operation::{
    create_op_with_flag_code, create_op_with_flag_code_3_custom_values,
};
use crate::common::register::{
    decrement_register, decrement_register_int, get_sub_registers_from_name, increment_register,
    increment_register_int,
};
use crate::instruction;
use crate::instruction::{Argument, FlagAction, Instruction, InstructionType};
use crate::interface::{Code, Function, IntFormat, Language, Register, Type};

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

    create_function(&instruction, &language, get_used_params(instruction), code)
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

    create_function(&instruction, &language, get_used_params(instruction), code)
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

    create_function(&instruction, &language, get_used_params(instruction), code)
}

fn create_ldhl(instruction: &Instruction, language: &Language) -> Function {
    let operation = create_op_with_flag_code(language, instruction, Operation::Add);
    let code = language
        .registers
        .hl
        .set(&operation.result)
        .prepend(operation.code);

    create_function(instruction, language, USE_REGISTER_AND_ARGUMENT, code)
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

    create_function(instruction, language, get_used_params(instruction), code)
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
    custom_flags.push(create_carry_flag_value(language, &carry_value.name));
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
        custom_flags.push(create_zero_flag_value(language, &zero_flag.name));
    }

    let code = Code::create_empty()
        .append(value_u8.code)
        .append(carry_value.code)
        .append(value.code)
        .append(result.code)
        .append(custom_flag_code)
        .append(create_set_flags(instruction, language, &custom_flags))
        .append(create_set_code(language, &argument, &result.name));

    create_function(instruction, language, get_used_params(instruction), code)
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
        .append(language.return_duration(instruction.duration_no_action));
    let jump = program_counter
        .set(&language.operations.cast(&jump_pc.name, Type::Uint16))
        .prepend(jump_pc.code)
        .append(language.return_duration(instruction.duration));

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
        USE_REGISTER_AND_ARGUMENT,
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

    return create_function(instruction, language, get_used_params(instruction), code);
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

pub fn create_comparison(instruction: &Instruction, language: &Language) -> Function {
    // Comparison is implemented by subtracting the input to the register a.
    let operation = create_op_with_flag_code_3_custom_values(
        language,
        instruction,
        Operation::Sub,
        &language.registers.a.get(),
        &create_get_code(language, instruction.first_argument.as_ref().unwrap()),
        None,
    );

    return create_function(
        instruction,
        language,
        get_used_params(instruction),
        operation.code,
    );
}

pub fn create_return(instruction: &Instruction, language: &Language) -> Function {
    let stack = language.registers.stack_pointer.as_ref();
    let program_counter = language.registers.program_counter.as_ref();

    let lower_pc = language.variable_with_cast(
        "lower_pc",
        &language.get_from_address(&stack.get()),
        Type::Uint16,
    );
    let upper_pc = language.variable_with_cast(
        "upper_pc",
        &language.get_from_address(&language.add_int(stack.get(), 1, IntFormat::Decimal)),
        Type::Uint16,
    );

    let update_pc = program_counter.set(&language.operations.add(&[
        lower_pc.name,
        language.shift_left_int(&upper_pc.name, 8, IntFormat::Decimal),
    ]));
    let update_stack = increment_register_int(language, stack, 2, IntFormat::Decimal);

    let code = Code::create_empty()
        .append(lower_pc.code)
        .append(upper_pc.code)
        .append(update_pc)
        .append(update_stack)
        .append(language.return_duration(instruction.duration));

    let code = if let Some(argument) = instruction.first_argument.as_ref() {
        let no_return = language
            .increment_pc_with_int(instruction.length)
            .append(language.return_duration(instruction.duration_no_action));
        language.statements.if_else(
            &get_flag_from_name(language, &argument.name),
            &code,
            &no_return,
        )
    } else {
        code
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

pub fn create_pop(instruction: &Instruction, language: &Language) -> Function {
    // Comparison is implemented by subtracting the input to the register a.
    let stack = language.registers.stack_pointer.as_ref();
    let (lower, upper) =
        get_sub_registers_from_name(language, &instruction.first_argument.as_ref().unwrap().name);

    let code = Code::create_empty()
        .append(lower.set(&language.get_from_address(&stack.get())))
        .append(upper.set(&language.get_from_address(&language.add_int(
            stack.get(),
            1,
            IntFormat::Decimal,
        ))))
        .append(increment_register_int(
            language,
            stack,
            2,
            IntFormat::Decimal,
        ));

    return create_function(instruction, language, USE_REGISTER_AND_MEMORY, code);
}

pub fn create_jump(instruction: &Instruction, language: &Language) -> Function {
    let program_counter = language.registers.program_counter.as_ref();

    let pc_argument = instruction
        .second_argument
        .as_ref()
        .unwrap_or(instruction.first_argument.as_ref().unwrap());

    let code = program_counter
        .set(&create_get_code(language, &pc_argument))
        .append(language.return_duration(instruction.duration));

    let code = if instruction.second_argument.is_some() {
        let no_jump = language
            .increment_pc_with_int(instruction.length)
            .append(language.return_duration(instruction.duration_no_action));
        language.statements.if_else(
            &get_flag_from_name(language, &instruction.first_argument.as_ref().unwrap().name),
            &code,
            &no_jump,
        )
    } else {
        code
    };

    return create_function_custom(
        instruction,
        language,
        get_used_params(instruction),
        code,
        FunctionDetails {
            doc: None,
            pc_increment: None,
            return_value: None,
        },
    );
}

pub fn create_call(instruction: &Instruction, language: &Language) -> Function {
    let program_counter = language.registers.program_counter.as_ref();

    let lower_pc_value = language.bitwise_and_int(&program_counter.get(), 0xFF, IntFormat::Hex);
    let lower_pc_value = language.operations.cast(&lower_pc_value, Type::Uint8);
    let upper_pc_value = language.bitwise_and_int(
        &language.shift_right_int(&program_counter.get(), 8, IntFormat::Decimal),
        0xFF,
        IntFormat::Hex,
    );
    let upper_pc_value = language.operations.cast(&upper_pc_value, Type::Uint8);

    let stack_pointer = language.registers.stack_pointer.as_ref();

    let update_stack = decrement_register_int(language, stack_pointer, 2, IntFormat::Decimal);
    let code = language
        .increment_pc_with_int(instruction.length)
        .append(create_set_memory_code(
            language,
            &language.sub_int(stack_pointer.get(), 1, IntFormat::Decimal),
            &upper_pc_value,
        ))
        .append(create_set_memory_code(
            language,
            &language.sub_int(stack_pointer.get(), 2, IntFormat::Decimal),
            &lower_pc_value,
        ))
        .append(update_stack)
        .append(program_counter.set(&language.arguments.get_uint16()))
        .append(language.return_duration(instruction.duration));

    let code = if instruction.second_argument.is_some() {
        let no_call = language
            .increment_pc_with_int(instruction.length)
            .append(language.return_duration(instruction.duration_no_action));
        language.statements.if_else(
            &get_flag_from_name(language, &instruction.first_argument.as_ref().unwrap().name),
            &code,
            &no_call,
        )
    } else {
        code
    };

    return create_function_custom(
        instruction,
        language,
        USE_ALL_PARAMETERS,
        code,
        FunctionDetails {
            doc: None,
            pc_increment: None,
            return_value: None,
        },
    );
}

pub fn create_push(instruction: &Instruction, language: &Language) -> Function {
    // Comparison is implemented by subtracting the input to the register a.
    let stack = language.registers.stack_pointer.as_ref();
    let (lower, upper) =
        get_sub_registers_from_name(language, &instruction.first_argument.as_ref().unwrap().name);

    let code = Code::create_empty()
        .append(upper.set(&language.get_from_address(&language.sub_int(
            stack.get(),
            1,
            IntFormat::Decimal,
        ))))
        .append(lower.set(&language.get_from_address(&language.sub_int(
            stack.get(),
            2,
            IntFormat::Decimal,
        ))))
        .append(decrement_register_int(
            language,
            stack,
            2,
            IntFormat::Decimal,
        ));

    return create_function(instruction, language, USE_REGISTER_AND_MEMORY, code);
}

pub fn create_rst(instruction: &Instruction, language: &Language) -> Function {
    let program_counter = language.registers.program_counter.as_ref();

    let lower_pc_value = language.bitwise_and_int(&program_counter.get(), 0xFF, IntFormat::Hex);
    let lower_pc_value = language.operations.cast(&lower_pc_value, Type::Uint8);
    let upper_pc_value = language.bitwise_and_int(
        &language.shift_right_int(&program_counter.get(), 8, IntFormat::Decimal),
        0xFF,
        IntFormat::Hex,
    );
    let upper_pc_value = language.operations.cast(&upper_pc_value, Type::Uint8);

    let stack_pointer = language.registers.stack_pointer.as_ref();
    let update_stack = decrement_register_int(language, stack_pointer, 2, IntFormat::Decimal);

    let update_pc = language.hex_literal(
        instruction.first_argument.as_ref().unwrap().value.unwrap(),
        Type::Uint16,
    );
    let code = language
        .increment_pc_with_int(instruction.length)
        .append(create_set_memory_code(
            language,
            &language.sub_int(stack_pointer.get(), 1, IntFormat::Decimal),
            &upper_pc_value,
        ))
        .append(create_set_memory_code(
            language,
            &language.sub_int(stack_pointer.get(), 2, IntFormat::Decimal),
            &lower_pc_value,
        ))
        .append(update_stack)
        .append(program_counter.set(&update_pc))
        .append(language.return_duration(instruction.duration));

    return create_function_custom(
        instruction,
        language,
        USE_ALL_PARAMETERS,
        code,
        FunctionDetails {
            doc: None,
            pc_increment: None,
            return_value: None,
        },
    );
}

pub fn create_prefix(instruction: &Instruction, language: &Language) -> Function {
    return create_function_custom(
        instruction,
        language,
        NO_USED_PARAMS,
        language.statements.stop_with_message(
            "Opcode 0xCB should be handled separately, something bad must have happened",
        ),
        FunctionDetails {
            doc: None,
            pc_increment: None,
            return_value: None,
        },
    );
}

pub fn create_ime_operation(instruction: &Instruction, language: &Language) -> Function {
    let value = instruction.type_field == InstructionType::EI;
    return create_function(
        instruction,
        language,
        ONLY_USE_REGISTER,
        language
            .registers
            .ime_flag
            .set(&language.statements.bool_literal(value)),
    );
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
        InstructionType::CP => Some(create_comparison(instruction, language)),
        InstructionType::RET => Some(create_return(instruction, language)),
        InstructionType::POP => Some(create_pop(instruction, language)),
        InstructionType::JP => Some(create_jump(instruction, language)),
        InstructionType::CALL => Some(create_call(instruction, language)),
        InstructionType::PUSH => Some(create_push(instruction, language)),
        InstructionType::RST => Some(create_rst(instruction, language)),
        InstructionType::PREFIX => Some(create_prefix(instruction, language)),
        InstructionType::DI | InstructionType::EI => {
            Some(create_ime_operation(instruction, language))
        }
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
