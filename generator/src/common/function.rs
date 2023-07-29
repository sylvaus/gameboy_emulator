use crate::instruction::{Argument, Instruction};
use crate::interface::{Code, Expression, Function, Language, Parameter, Type};

#[derive(Debug, Clone)]
pub struct UsedFnParams {
    pub register: bool,
    pub memory: bool,
}

pub const NO_USED_PARAMS: UsedFnParams = UsedFnParams {
    register: false,
    memory: false,
};
pub const ONLY_USE_REGISTER: UsedFnParams = UsedFnParams {
    register: true,
    memory: false,
};
pub const ONLY_USE_MEMORY: UsedFnParams = UsedFnParams {
    register: false,
    memory: true,
};
pub const USE_REGISTER_AND_MEMORY: UsedFnParams = UsedFnParams {
    register: true,
    memory: true,
};

#[derive(Debug, Default)]
pub struct FunctionDetails {
    pub doc: Option<String>,
    pub pc_increment: Option<i64>,
    pub return_value: Option<Expression>,
}

pub fn create_function(
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

pub fn create_function_custom(
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

pub fn increment_pc(increment: i64, language: &Language) -> Code {
    let program_counter = &language.registers.program_counter;
    let update = language.operations.add(&[
        program_counter.get(),
        Expression::from(&increment, Type::Uint16),
    ]);
    program_counter.set(&update)
}

pub fn get_duration(instruction: &Instruction) -> Expression {
    Expression::from(&instruction.duration, Type::Uint16)
}

pub fn get_used_params(instruction: &Instruction) -> UsedFnParams {
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