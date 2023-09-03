use crate::instruction::{Argument, Instruction};
use crate::interface::{Code, Expression, Function, Language, Parameter, Type, NB_CYCLE_TYPE};

#[derive(Debug, Clone)]
pub struct UsedFnParams {
    pub register: bool,
    pub memory: bool,
    pub argument: bool,
}

pub const NO_USED_PARAMS: UsedFnParams = UsedFnParams {
    register: false,
    memory: false,
    argument: false,
};
pub const ONLY_USE_REGISTER: UsedFnParams = UsedFnParams {
    register: true,
    memory: false,
    argument: false,
};
pub const ONLY_USE_MEMORY: UsedFnParams = UsedFnParams {
    register: false,
    memory: true,
    argument: false,
};
pub const USE_REGISTER_AND_MEMORY: UsedFnParams = UsedFnParams {
    register: true,
    memory: true,
    argument: false,
};
pub const USE_REGISTER_AND_ARGUMENT: UsedFnParams = UsedFnParams {
    register: true,
    memory: false,
    argument: true,
};
pub const USE_ALL_PARAMETERS: UsedFnParams = UsedFnParams {
    register: true,
    memory: true,
    argument: true,
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
        Parameter::new(
            Type::Argument,
            language.arguments.name(),
            used_params.argument,
        ),
    ];

    if let Some(increment) = details.pc_increment {
        code.iappend(increment_pc(increment, language));
    }
    if let Some(value) = &details.return_value {
        code.iappend(language.statements.return_value(value));
    }

    language
        .statements
        .function(&name, &parameters, &code, &complete_doc, NB_CYCLE_TYPE)
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
    Expression::from(&instruction.duration, NB_CYCLE_TYPE)
}

pub fn get_used_params(instruction: &Instruction) -> UsedFnParams {
    UsedFnParams {
        register: true,
        memory: does_param_use_memory(&instruction.first_argument)
            || does_param_use_memory(&instruction.second_argument),
        argument: does_param_use_argument(&instruction.first_argument)
            || does_param_use_argument(&instruction.second_argument),
    }
}

fn does_param_use_memory(argument: &Option<Argument>) -> bool {
    argument.as_ref().map(|arg| arg.is_address).unwrap_or(false)
}

fn does_param_use_argument(argument: &Option<Argument>) -> bool {
    argument
        .as_ref()
        .map(|arg| arg.is_immediate())
        .unwrap_or(false)
}
