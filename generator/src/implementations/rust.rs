use std::collections::HashMap;

use crate::interface::{ArgumentGetters, Code, Expression, Flags, FlagsRegister, Function, FunctionTableCall, IntFormat, Language, Memory, Operations, Parameter, Register, Registers, Statements, Type};
use crate::parser::Instruction;

const REGISTER_VAR_NAME: &str = "registers";
const MEMORY_VAR_NAME: &str = "memory";
const INDENT: &str = "    ";

struct ArgumentsImpl {}

impl ArgumentGetters for ArgumentsImpl {
    fn get_uint8(&self) -> Expression {
        Expression::new(
            format!(
                "{}.get({}.pc + 1)",
                MEMORY_VAR_NAME, REGISTER_VAR_NAME
            ),
            Type::Uint8,
        )
    }

    fn get_int8(&self) -> Expression {
        Expression::new(
            format!(
                "{}.get_signed({}.pc + 1)",
                MEMORY_VAR_NAME, REGISTER_VAR_NAME
            ),
            Type::Int8,
        )
    }

    fn get_uint16(&self) -> Expression {
        Expression::new(
            format!(
                "{}.get_16_bits({}.pc + 1)",
                MEMORY_VAR_NAME, REGISTER_VAR_NAME
            ),
            Type::Uint16,
        )
    }
}

struct AttributeRegister {
    name: String,
    type_: Type
}

impl Register for AttributeRegister {
    fn get(&self) -> Expression {
        Expression::new(format!("{}.{}", REGISTER_VAR_NAME, self.name), self.type_)
    }

    fn set(&self, value: &Expression) -> Code {
        assert_eq!(value.type_, self.type_, "Cannot set a {:?} register with type: {:?}", self.type_, value.type_);
        Code::from_str(&format!(
            "{}.{} = {};",
            REGISTER_VAR_NAME, self.name, value.text
        ))
    }
}

struct GetterSetterRegister {
    name: String,
}

impl GetterSetterRegister {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Register for GetterSetterRegister {
    fn get(&self) -> Expression {
        Expression::new(
            format!("{}.get_{}()", REGISTER_VAR_NAME, self.name),
            Type::Uint16,
        )
    }

    fn set(&self, value: &Expression) -> Code {
        assert_eq!(value.type_, Type::Uint16, "Cannot set a 16 bit register with type: {:?}", value.type_);
        Code::from_str(&format!(
            "{}.set_{}({});",
            REGISTER_VAR_NAME, self.name, value.text
        ))
    }
}

struct FlagsRegisterImpl {}

impl FlagsRegister for FlagsRegisterImpl {}

impl Register for FlagsRegisterImpl {
    fn get(&self) -> Expression {
        Expression::from_str("registers.flags", Type::Uint8)
    }

    fn set(&self, value: &Expression) -> Code {
        assert_eq!(value.type_, Type::Uint8, "Cannot set a 8 bit register with type: {:?}", value.type_);
        Code::from_str(&format!("registers.flags = {};", value.text))
    }
}

impl Flags for FlagsRegisterImpl {
    fn get_carry_flag(&self) -> Expression {
        Expression::from_str("registers.flags.get_carry_flag()", Type::Uint8)
    }

    fn get_half_carry_flag(&self) -> Expression {
        Expression::from_str("registers.flags.get_half_carry_flag()", Type::Uint8)
    }

    fn get_add_sub_flag(&self) -> Expression {
        Expression::from_str("registers.flags.get_add_sub_flag()", Type::Uint8)
    }

    fn get_zero_flag(&self) -> Expression {
        Expression::from_str("registers.flags.get_zero_flag()", Type::Uint8)
    }
}

struct MemoryImpl {}

impl Memory for MemoryImpl {
    fn name(&self) -> String {
        String::from(MEMORY_VAR_NAME)
    }

    fn get(&self, address: &Expression) -> Expression {
        assert_eq!(address.type_, Type::Uint16, "Address type should be uint16 and not: {:?}", address.type_);
        Expression::new(
            format!("{}.get({})", MEMORY_VAR_NAME, address.text),
            Type::Uint8,
        )
    }

    fn get_signed(&self, address: &Expression) -> Expression {
        assert_eq!(address.type_, Type::Uint16, "Address type should be uint16 and not: {:?}", address.type_);
        Expression::new(
            format!("{}.get_signed({})", MEMORY_VAR_NAME, address.text),
            Type::Int8,
        )
    }

    fn get_16_bits(&self, address: &Expression) -> Expression {
        assert_eq!(address.type_, Type::Uint16, "Address type should be uint16 and not: {:?}", address.type_);
        Expression::new(
            format!("{}.get_16_bits({})", MEMORY_VAR_NAME, address.text),
            Type::Uint16,
        )
    }

    fn set_8_bits(&self, address: &Expression, value: &Expression) -> Code {
        assert_eq!(address.type_, Type::Uint16, "Address type should be uint16 and not: {:?}", value.type_);
        assert_eq!(value.type_, Type::Uint8, "Value type should be uint8 and not: {:?}", value.type_);
        Code::from_str(&format!(
            "{}.set({}, {});",
            MEMORY_VAR_NAME, address.text, value.text
        ))
    }

    fn set_16_bits(&self, address: &Expression, value: &Expression) -> Code {
        assert_eq!(address.type_, Type::Uint16, "Address type should be uint16 and not: {:?}", value.type_);
        assert_eq!(value.type_, Type::Uint16, "Value type should be uint16 and not: {:?}", value.type_);
        Code::from_str(&format!(
            "{}.set_16_bits({}, {});",
            MEMORY_VAR_NAME, address.text, value.text
        ))
    }
}

struct StatementsImpl {}

impl Statements for StatementsImpl {
    fn header(&self) -> Option<Code> {
        Some(Code::from_str(
            "use log::trace\n\
            use crate::memory::Memory;\n\
            use crate::memory::registers::Registers;",
        ))
    }

    fn footer(&self) -> Option<Code> {
        None
    }

    fn comment(&self, text: &str) -> Code {
        Code::from_lines(
            text.split('\n')
                .map(|line| format!("// {}", line))
                .collect::<Vec<String>>(),
        )
    }

    fn log_trace(&self, text: &str) -> Code {
        Code::from_str(&format!("trace!(\"{}\")", text))
    }

    fn log_trace_registers(&self) -> Code {
        Code::from_str(&format!(
            "trace!(\"registers: {{:?}}\", {})",
            REGISTER_VAR_NAME
        ))
    }

    fn int_literal(&self, value: i64, type_: Type, format: IntFormat) -> Expression {
        let formatter = match format {
            IntFormat::Bin => |value, type_text| format!("0b{:b}{}", value, type_text),
            IntFormat::Hex => |value, type_text| format!("0x{:X}{}", value, type_text),
            IntFormat::Decimal => |value, type_text| format!("{}{}", value, type_text),
        };

        let text = match type_ {
            Type::Uint8 => formatter(value, "u8"),
            Type::Int8 => formatter(value, "i8"),
            Type::Uint16 => formatter(value, "u16"),
            Type::Int16 => formatter(value, "i16"),
            Type::Uint32 => formatter(value, "u32"),
            Type::Int32 => formatter(value, "i32"),
            Type::Uint64 => formatter(value, "u64"),
            Type::Int64 => formatter(value, "i64"),
            _ => panic!("Cannot create int literal of type {:?}", type_)
        };
        Expression::new(text, type_)
    }

    fn variable(&self, name: &str, code: &Expression) -> Code {
        match code.type_ {
            Type::Registers | Type::Memory | Type::Void => {
                panic!("Cannot create variable of type {:?}", code.type_)
            }
            _ => Code::from_str(&format!(
                "let {}: {} = {};",
                name,
                get_type_str(&code.type_),
                code.text
            )),
        }
    }

    fn single_if(&self, condition: &Expression, code: &Code) -> Code {
        Code::from_str(&format!("if ({}) {{", condition.text))
            .append(code.clone().indent(INDENT))
            .append_line("}}".to_string())
    }

    fn if_else(&self, condition: &Expression, true_code: &Code, false_code: &Code) -> Code {
        Code::from_str(&format!("if ({}) {{", condition.text))
            .append(true_code.clone().indent(INDENT))
            .append_line("} else {".to_string())
            .append(false_code.clone().indent(INDENT))
            .append_line("}".to_string())
    }

    fn stop_with_message(&self, message: &str) -> Code {
        Code::from_str(&format!("panic!(\"{}\");", message))
    }

    fn function(
        &self,
        name: &str,
        parameters: &[Parameter],
        code: &Code,
        doc: &str,
        return_value: &Expression,
    ) -> Function {
        let parameters = parameters
            .iter()
            .map(make_parameter)
            .collect::<Vec<String>>()
            .join(", ");

        let signature = format!(
            "pub fn {}({}) -> {}",
            name,
            parameters,
            get_type_str(&return_value.type_)
        );

        let definition = Code::from_lines(
            doc.split('\n')
                .map(|line| format!("/// {}", line))
                .collect::<Vec<String>>(),
        )
        .append_line(format!("{} {{", signature))
        .append(code.clone().indent(INDENT))
        .append_line(format!("{}return {};", INDENT, return_value.text))
        .append_line("}\n".to_string());

        Function::new(String::from(name), signature, definition)
    }

    fn function_table_call(&self, id_function_map: &HashMap<u16, &Function>) -> FunctionTableCall {
        todo!()
    }
}

fn make_parameter(parameter: &Parameter) -> String {
    let prefix: &str = if parameter.used { "" } else { "_" };
    format!(
        "{}{}: {}",
        prefix,
        parameter.name,
        get_type_str(&parameter.type_)
    )
}

fn get_type_str(type_: &Type) -> &'static str {
    match type_ {
        Type::Uint8 => "u8",
        Type::Int8 => "i8",
        Type::Uint16 => "u16",
        Type::Int16 => "i16",
        Type::Uint32 => "u32",
        Type::Int32 => "i32",
        Type::Uint64 => "u64",
        Type::Int64 => "i64",
        Type::Registers => "&mut Registers",
        Type::Memory => "&mut dyn Memory",
        _ => panic!(""),
    }
}

struct OperationsImpl {}

impl Operations for OperationsImpl {
    fn add(&self, lhs: &Expression, rhs: &Expression) -> Expression {
        assert_eq!(lhs.type_, rhs.type_, "Cannot add values of different types");
        Expression::new(format!("{} + {}", lhs.text, rhs.text), lhs.type_)
    }
    fn sub(&self, lhs: &Expression, rhs: &Expression) -> Expression {
        assert_eq!(lhs.type_, rhs.type_, "Cannot subtract values of different types");
        Expression::new(format!("{} - {}", lhs.text, rhs.text), lhs.type_)
    }
}

pub fn get_rust_language() -> Language {
    let registers = Registers {
        name: "registers".to_string(),
        a: Box::new(AttributeRegister {
            name: "a".to_string(), type_: Type::Uint8
        }),
        flags: Box::new(FlagsRegisterImpl {}),
        b: Box::new(AttributeRegister {
            name: "b".to_string(), type_: Type::Uint8
        }),
        c: Box::new(AttributeRegister {
            name: "c".to_string(), type_: Type::Uint8
        }),
        d: Box::new(AttributeRegister {
            name: "d".to_string(), type_: Type::Uint8
        }),
        e: Box::new(AttributeRegister {
            name: "e".to_string(), type_: Type::Uint8
        }),
        h: Box::new(AttributeRegister {
            name: "h".to_string(), type_: Type::Uint8
        }),
        l: Box::new(AttributeRegister {
            name: "l".to_string(), type_: Type::Uint8
        }),
        af: Box::new(GetterSetterRegister {
            name: "af".to_string(),
        }),
        bc: Box::new(GetterSetterRegister {
            name: "bc".to_string(),
        }),
        de: Box::new(GetterSetterRegister {
            name: "de".to_string(),
        }),
        hl: Box::new(GetterSetterRegister {
            name: "hl".to_string(),
        }),
        stack_pointer: Box::new(AttributeRegister {
            name: "sp".to_string(), type_: Type::Uint16
        }),
        program_counter: Box::new(AttributeRegister {
            name: "pc".to_string(), type_: Type::Uint16
        }),
        halted: Box::new(AttributeRegister {
            name: "halted".to_string(), type_: Type::Bool
        }),
        stopped: Box::new(AttributeRegister {
            name: "stopped".to_string(), type_: Type::Bool
        }),
        ime_flag: Box::new(AttributeRegister {
            name: "ime_flag".to_string(), type_: Type::Bool
        }),
    };

    Language {
        arguments: Box::new(ArgumentsImpl {}),
        registers,
        memory: Box::new(MemoryImpl {}),
        statements: Box::new(StatementsImpl {}),
        operations: Box::new(OperationsImpl {}),
    }
}
