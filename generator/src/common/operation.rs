use crate::common::base::{Operation, OperationWithFlag};
use crate::common::flags::{
    compute_carry_flag, compute_half_carry_flag, compute_zero_flag, create_carry_flag_value,
    create_half_carry_flag_value, create_set_flags, create_zero_flag_value, get_carry_max_value,
    get_half_carry_max_value,
};
use crate::common::getset::create_get_code;
use crate::instruction::{FlagAction, Instruction};
use crate::interface::{Expression, IntFormat, Language, Type};

pub fn create_op_with_flag_code(
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

pub fn create_op_with_flag_code_3_custom_values(
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
    let is_only_comparing_lower_8_bites = instruction.value == 0xE8 || instruction.value == 0xF8;
    let carry_max_value: i64 = get_carry_max_value(instruction);
    if instruction.zero_flag == FlagAction::CUSTOM {
        let variable = compute_zero_flag(language, &result.name, carry_max_value);
        custom_flag_values.push(create_zero_flag_value(language, &variable.name));
        result.code.iappend(variable.code);
    }
    if instruction.half_carry_flag == FlagAction::CUSTOM {
        let half_carry_max_value: i64 = if is_only_comparing_lower_8_bites {
            0xF
        } else {
            get_half_carry_max_value(instruction)
        };
        let variable =
            compute_half_carry_flag(language, &variable_names, half_carry_max_value, operation);
        custom_flag_values.push(create_half_carry_flag_value(language, &variable.name));
        result.code.iappend(variable.code);
    }
    if instruction.carry_flag == FlagAction::CUSTOM {
        let (value_to_compare, carry_max_value) = if is_only_comparing_lower_8_bites {
            let masked_values = variable_names
                .iter()
                .map(|variable| language.bitwise_and_int(variable, 0xFF, IntFormat::Hex))
                .collect::<Vec<Expression>>();
            (&language.operations.add(&masked_values), 0xFF)
        } else {
            (&result.name, carry_max_value)
        };
        let variable = compute_carry_flag(language, &value_to_compare, carry_max_value, operation);
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
