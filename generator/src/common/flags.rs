use crate::instruction::{FlagAction, Instruction};
use crate::interface::{Code, Expression, IntFormat, Language, Type, Variable};
use crate::common::base::Operation;

const OFFSET_CARRY_FLAG_VALUE: i64 = 4;
const OFFSET_HALF_CARRY_FLAG_VALUE: i64 = 5;
const OFFSET_ADD_SUB_FLAG_VALUE: i64 = 6;
const OFFSET_ZERO_FLAG_VALUE: i64 = 7;

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

pub fn compute_zero_flag(language: &Language, result: &Expression, carry_max_value: i64) -> Variable {
    let equals = language.equals_int(
        &language.bitwise_and_int(result, carry_max_value, IntFormat::Hex),
        0,
        IntFormat::Decimal,
    );

    language
        .statements
        .variable("zero_flag", &language.operations.cast(&equals, Type::Uint8))
}

pub fn compute_half_carry_flag(
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

pub fn compute_carry_flag(
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
pub fn create_set_flags(
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