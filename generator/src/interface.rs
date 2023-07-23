use std::collections::HashMap;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Uint8,
    Int8,
    Uint16,
    Int16,
    Uint32,
    Int32,
    Uint64,
    Int64,
    Registers,
    Memory,
    Void,
    Bool,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub text: String,
    pub type_: Type,
}

impl Expression {
    pub fn new(text: String, type_: Type) -> Self {
        Expression { text, type_ }
    }

    pub fn from_str(text: &str, type_: Type) -> Self {
        Expression {
            text: String::from(text),
            type_,
        }
    }

    pub fn from<T: Display>(into_string: &T, type_: Type) -> Self {
        Expression {
            text: format!("{}", into_string),
            type_,
        }
    }

    pub fn op_safe_text(&self) -> String {
        if self.text.contains("\n") || self.text.contains(" ") {
            format!("({})", self.text)
        } else {
            self.text.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Code {
    lines: Vec<String>,
}

impl ToString for Code {
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}

impl Code {
    pub fn create_empty() -> Self {
        Code { lines: vec![] }
    }

    pub fn indent(mut self, indent: &str) -> Self {
        for line in &mut self.lines {
            *line = indent.to_string() + line;
        }
        self
    }

    pub fn prepend_line(mut self, line: String) -> Self {
        self.lines.insert(0, line);
        self
    }

    pub fn prepend(mut self, code: Self) -> Self {
        self.iprepend(code);
        self
    }

    pub fn iprepend(&mut self, code: Self) {
        code.lines
            .into_iter()
            .rev()
            .for_each(|line| self.lines.insert(0, line));
    }

    pub fn append_line(mut self, line: String) -> Self {
        self.iappend_line(line);
        self
    }

    pub fn iappend_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn append(mut self, code: Self) -> Self {
        self.iappend(code);
        self
    }

    pub fn iappend(&mut self, mut code: Self) {
        self.lines.append(&mut code.lines);
    }

    pub fn from_str(lines: &str) -> Self {
        Code {
            lines: lines.split('\n').map(String::from).collect::<Vec<String>>(),
        }
    }

    pub fn from_lines(lines: Vec<String>) -> Self {
        Code { lines }
    }
}

pub trait ArgumentGetters {
    fn get_uint8(&self) -> Expression;
    fn get_int8(&self) -> Expression;
    fn get_uint16(&self) -> Expression;
}

pub trait Register {
    fn get(&self) -> Expression;
    fn set(&self, value: &Expression) -> Code;
}

pub trait Flags {
    fn get_carry_flag(&self) -> Expression;
    fn get_half_carry_flag(&self) -> Expression;
    fn get_add_sub_flag(&self) -> Expression;
    fn get_zero_flag(&self) -> Expression;
    fn get_non_carry_flag(&self) -> Expression;
    fn get_non_zero_flag(&self) -> Expression;
}

pub trait FlagsRegister: Register + Flags {}

pub struct Registers {
    pub name: String,
    pub a: Box<dyn Register>,
    pub flags: Box<dyn FlagsRegister>,
    pub b: Box<dyn Register>,
    pub c: Box<dyn Register>,
    pub d: Box<dyn Register>,
    pub e: Box<dyn Register>,
    pub h: Box<dyn Register>,
    pub l: Box<dyn Register>,
    pub af: Box<dyn Register>,
    pub bc: Box<dyn Register>,
    pub de: Box<dyn Register>,
    pub hl: Box<dyn Register>,
    pub stack_pointer: Box<dyn Register>,
    pub program_counter: Box<dyn Register>,
    pub halted: Box<dyn Register>,
    pub stopped: Box<dyn Register>,
    pub ime_flag: Box<dyn Register>,
}

pub trait Memory {
    fn name(&self) -> String;
    fn get(&self, address: &Expression) -> Expression;
    fn get_signed(&self, address: &Expression) -> Expression;
    fn get_16_bits(&self, address: &Expression) -> Expression;
    fn set_8_bits(&self, address: &Expression, value: &Expression) -> Code;
    fn set_16_bits(&self, address: &Expression, value: &Expression) -> Code;
}

pub struct Parameter {
    pub type_: Type,
    pub name: String,
    pub used: bool,
}

impl Parameter {
    pub fn new(type_: Type, name: String, used: bool) -> Self {
        Self { type_, name, used }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub signature: String,
    pub definition: Code,
}

impl Function {
    pub fn new(name: String, signature: String, definition: Code) -> Self {
        Self {
            name,
            signature,
            definition,
        }
    }
}

pub struct FunctionTableCall {
    pub global_definition: String,
    pub function: Box<Function>,
}

impl FunctionTableCall {
    pub fn new(global_definition: String, function: Box<Function>) -> Self {
        Self {
            global_definition,
            function,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum IntFormat {
    Bin,
    Hex,
    Decimal,
}

pub struct Variable {
    pub code: Code,
    pub name: Expression,
}

pub trait Statements {
    fn header(&self) -> Option<Code>;
    fn footer(&self) -> Option<Code>;
    fn comment(&self, text: &str) -> Code;
    fn log_trace(&self, text: &str) -> Code;
    fn log_trace_registers(&self) -> Code;
    fn int_literal(&self, value: i64, type_: Type, format: IntFormat) -> Expression;
    fn bool_literal(&self, value: bool) -> Expression;
    fn variable(&self, name: &str, code: &Expression) -> Variable;
    fn single_if(&self, condition: &Expression, code: &Code) -> Code;
    fn if_else(&self, condition: &Expression, true_code: &Code, false_code: &Code) -> Code;
    fn stop_with_message(&self, message: &str) -> Code;
    fn return_value(&self, value: &Expression) -> Code;
    fn function(
        &self,
        name: &str,
        parameters: &[Parameter],
        code: &Code,
        doc: &str,
        return_type: Type,
    ) -> Function;
    fn function_table_call(&self, id_function_map: &HashMap<u16, &Function>) -> FunctionTableCall;
}

pub trait Operations {
    fn equals(&self, lhs: &Expression, rhs: &Expression) -> Expression;
    /// Returns expression lhs > rhs
    fn greater_than(&self, lhs: &Expression, rhs: &Expression) -> Expression;
    fn greater_equal(&self, lhs: &Expression, rhs: &Expression) -> Expression;
    fn lesser_than(&self, lhs: &Expression, rhs: &Expression) -> Expression;
    fn lesser_equal(&self, lhs: &Expression, rhs: &Expression) -> Expression;
    fn add(&self, values: &[Expression]) -> Expression;
    fn sub(&self, values: &[Expression]) -> Expression;
    fn multiply(&self, values: &[Expression]) -> Expression;
    fn cast(&self, value: &Expression, type_: Type) -> Expression;
    fn shift_left(&self, value: &Expression, shift: &Expression) -> Expression;
    fn shift_right(&self, value: &Expression, shift: &Expression) -> Expression;
    fn bitwise_and(&self, values: &[Expression]) -> Expression;
    fn bitwise_or(&self, values: &[Expression]) -> Expression;
    fn bitwise_not(&self, value: &Expression) -> Expression;
    fn or(&self, values: &[Expression]) -> Expression;
}

pub struct Language {
    pub arguments: Box<dyn ArgumentGetters>,
    pub registers: Registers,
    pub memory: Box<dyn Memory>,
    pub statements: Box<dyn Statements>,
    pub operations: Box<dyn Operations>,
}

impl Language {
    pub fn equals_int(&self, lhs: &Expression, rhs: i64, int_format: IntFormat) -> Expression {
        self.operations.equals(
            lhs,
            &self.statements.int_literal(rhs, lhs.type_, int_format),
        )
    }
    pub fn greater_than_int(
        &self,
        lhs: &Expression,
        rhs: i64,
        int_format: IntFormat,
    ) -> Expression {
        self.operations.greater_than(
            lhs,
            &self.statements.int_literal(rhs, lhs.type_, int_format),
        )
    }
    pub fn int_greater_than(
        &self,
        lhs: i64,
        rhs: &Expression,
        int_format: IntFormat,
    ) -> Expression {
        self.operations.greater_than(
            &self.statements.int_literal(lhs, rhs.type_, int_format),
            rhs,
        )
    }
    pub fn greater_equal_int(
        &self,
        lhs: &Expression,
        rhs: i64,
        int_format: IntFormat,
    ) -> Expression {
        self.operations.greater_equal(
            lhs,
            &self.statements.int_literal(rhs, lhs.type_, int_format),
        )
    }
    pub fn int_greater_equal(
        &self,
        lhs: i64,
        rhs: &Expression,
        int_format: IntFormat,
    ) -> Expression {
        self.operations.greater_equal(
            &self.statements.int_literal(lhs, rhs.type_, int_format),
            rhs,
        )
    }
    pub fn lesser_than_int(&self, lhs: &Expression, rhs: i64, int_format: IntFormat) -> Expression {
        self.operations.lesser_than(
            lhs,
            &self.statements.int_literal(rhs, lhs.type_, int_format),
        )
    }
    pub fn int_lesser_than(&self, lhs: i64, rhs: &Expression, int_format: IntFormat) -> Expression {
        self.operations.lesser_than(
            &self.statements.int_literal(lhs, rhs.type_, int_format),
            rhs,
        )
    }
    pub fn lesser_equal_int(
        &self,
        lhs: &Expression,
        rhs: i64,
        int_format: IntFormat,
    ) -> Expression {
        self.operations.lesser_equal(
            lhs,
            &self.statements.int_literal(rhs, lhs.type_, int_format),
        )
    }
    pub fn int_lesser_equal(
        &self,
        lhs: i64,
        rhs: &Expression,
        int_format: IntFormat,
    ) -> Expression {
        self.operations.lesser_equal(
            &self.statements.int_literal(lhs, rhs.type_, int_format),
            rhs,
        )
    }
    pub fn shift_left_int(
        &self,
        value: &Expression,
        shift: i64,
        int_format: IntFormat,
    ) -> Expression {
        self.operations.shift_left(
            value,
            &self.statements.int_literal(shift, value.type_, int_format),
        )
    }
    pub fn shift_right_int(
        &self,
        value: &Expression,
        shift: i64,
        int_format: IntFormat,
    ) -> Expression {
        self.operations.shift_right(
            value,
            &self.statements.int_literal(shift, value.type_, int_format),
        )
    }
    pub fn bitwise_and_int(&self, lhs: &Expression, rhs: i64, int_format: IntFormat) -> Expression {
        let literal = self.statements.int_literal(rhs, lhs.type_, int_format);
        self.operations.bitwise_and(&[lhs.clone(), literal])
    }
    pub fn bitwise_or_int(&self, lhs: &Expression, rhs: i64, int_format: IntFormat) -> Expression {
        let literal = self.statements.int_literal(rhs, lhs.type_, int_format);
        self.operations.bitwise_or(&[lhs.clone(), literal])
    }

    pub fn return_int(&self, value: i64, type_: Type, format: IntFormat) -> Code {
        self.statements
            .return_value(&self.statements.int_literal(value, type_, format))
    }

    pub fn variable_with_type(&self, name: &str, value: &Expression, type_: Type) -> Variable {
        self.statements
            .variable(name, &self.operations.cast(value, type_))
    }

    pub fn binary_literal(&self, value: i64, type_: Type) -> Expression {
        self.statements.int_literal(value, type_, IntFormat::Bin)
    }

    pub fn decimal_literal(&self, value: i64, type_: Type) -> Expression {
        self.statements
            .int_literal(value, type_, IntFormat::Decimal)
    }

    pub fn hex_literal(&self, value: i64, type_: Type) -> Expression {
        self.statements.int_literal(value, type_, IntFormat::Hex)
    }
}
