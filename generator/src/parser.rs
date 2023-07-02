use macros_derive::AddEnumName;
use serde::Deserialize;
use serde::Serialize;

const OFFSET_CARRY_FLAG_VALUE: u8 = 4;
const OFFSET_HALF_CARRY_FLAG_VALUE: u8 = 5;
const OFFSET_ADD_SUB_FLAG_VALUE: u8 = 6;
const OFFSET_ZERO_FLAG_VALUE: u8 = 7;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub value: i64,
    #[serde(rename = "type")]
    pub type_field: InstructionType,
    pub length: i64,
    pub duration: i64,
    pub duration_no_action: i64,
    pub zero_flag: FlagAction,
    pub add_sub_flag: FlagAction,
    pub half_carry_flag: FlagAction,
    pub carry_flag: FlagAction,
    pub first_argument: Option<Argument>,
    pub second_argument: Option<Argument>,
}

impl Instruction {
    pub fn get_repr(&self) -> String {
        let mut representation = format!("0x{:x} {}", self.value, self.type_field.get_name());
        if let Some(argument) = &self.first_argument {
            representation = format!("{} {}", representation, argument.get_repr());
        }
        if let Some(argument) = &self.second_argument {
            representation = format!("{} {}", representation, argument.get_repr());
        }

        representation
    }

    pub fn get_flags(&self) -> Vec<Flag> {
        vec![
            Flag {
                action: self.zero_flag.clone(),
                offset: OFFSET_ZERO_FLAG_VALUE,
            },
            Flag {
                action: self.add_sub_flag.clone(),
                offset: OFFSET_ADD_SUB_FLAG_VALUE,
            },
            Flag {
                action: self.half_carry_flag.clone(),
                offset: OFFSET_HALF_CARRY_FLAG_VALUE,
            },
            Flag {
                action: self.carry_flag.clone(),
                offset: OFFSET_CARRY_FLAG_VALUE,
            },
        ]
    }

    pub fn is_two_bytes_op(&self) -> bool {
        self.first_argument
            .as_ref()
            .map_or(false, |arg| arg.get_value_nb_bytes() > 1)
            || self
                .second_argument
                .as_ref()
                .map_or(false, |arg| arg.get_value_nb_bytes() > 1)
    }
}

pub struct Flag {
    pub action: FlagAction,
    pub offset: u8,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AddEnumName)]
pub enum InstructionType {
    UNKNOWN,
    NOP,
    LD,
    LDI,
    LDD,
    LDH,
    LDHL,
    INC,
    DEC,
    ADD,
    RLCA,
    RRCA,
    RLA,
    RRA,
    JR,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    XOR,
    OR,
    CP,
    RET,
    POP,
    JP,
    CALL,
    PUSH,
    RST,
    PREFIX,
    DI,
    EI,
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
    STOP,
    AND,
    RETI,
    LDSpecial,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlagAction {
    CUSTOM,
    SET,
    RESET,
    NONE,
}

impl FlagAction {
    pub fn set_as_u8(&self) -> u8 {
        match self {
            FlagAction::SET => 1,
            _ => 0,
        }
    }
    pub fn none_as_u8(&self) -> u8 {
        match self {
            FlagAction::NONE => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Argument {
    #[serde(rename = "type")]
    pub type_field: ArgumentType,
    pub is_address: bool,
    pub nb_bytes: u8,
    pub name: String,
    pub value: Option<i64>,
}

impl Argument {
    pub fn get_value_nb_bytes(&self) -> u8 {
        if (!self.is_address) && (self.nb_bytes == 2) {
            return 2;
        }
        1
    }

    pub fn get_repr(&self) -> String {
        if self.is_address {
            format!("({})", &self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn is_immediate(&self) -> bool {
        match self.type_field {
            ArgumentType::Immediate8Bits
            | ArgumentType::Immediate16Bits
            | ArgumentType::Unsigned8Bit
            | ArgumentType::Address16Bit
            | ArgumentType::PCIncrement8Bit => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AddEnumName)]
pub enum ArgumentType {
    Register,
    Immediate8Bits,
    Immediate16Bits,
    Unsigned8Bit,
    Address16Bit,
    PCIncrement8Bit,
    Value,
    Indication,
}

pub fn parse_instructions(text: &str) -> Result<Vec<Instruction>, serde_json::Error> {
    serde_json::from_str(text)
}
