import csv
from dataclasses import dataclass
from enum import Enum
from typing import Optional, List


NONE_STR = str(None)
TRUE_STR = str(True)
FALSE_STR = str(False)


class ArgumentType(Enum):
    REGISTER = "REGISTER"
    IMMEDIATE_8_BITS = "IMMEDIATE_8_BITS"
    IMMEDIATE_16_BITS = "IMMEDIATE_16_BITS"
    UNSIGNED_8_BIT = "UNSIGNED_8_BIT"
    ADDRESS_16_BIT = "ADDRESS_16_BIT"
    PC_INCREMENT_8_BIT = "PC_INCREMENT_8_BIT"
    VALUE = "VALUE"
    INDICATION = "INDICATION"


ARGUMENT_TYPE_MAP = {type_.value: type_ for type_ in ArgumentType}

REGISTERS_NAMES = {"AF", "A", "BC", "B", "C", "DE", "D", "E", "HL", "H", "L", "SP", "PC", "NZ", "Z", "NC", "C"}
IMMEDIATE_8_BITS_NAME = "d8"
IMMEDIATE_16_BITS_NAME = "d16"
UNSIGNED_8_BIT_NAME = "a8"
ADDRESS_16_BIT_NAME = "a16"
PC_INCREMENT_8_BIT_NAME = "r8"
INDICATION_NAME = "CB"
SPECIAL_MNEMONIC_MAP = {
    ("LD", ("A", "(HL+)")): ("LDI", ("A", "(HL)")),
    ("LD", ("(HL+)", "A")): ("LDI", ("(HL)", "A")),
    ("LD", ("A", "(HL-)")): ("LDD", ("A", "(HL)")),
    ("LD", ("(HL-)", "A")): ("LDD", ("(HL)", "A")),
    ("LD", ("HL", "SP+r8")): ("LDHL", ("SP", "r8")),
}


@dataclass
class Argument:
    type_: ArgumentType
    is_address: bool
    name: str
    value: Optional[int]


def argument_to_csv(argument: Optional[Argument]) -> List[str]:
    if argument is None:
        return [NONE_STR, NONE_STR, NONE_STR, NONE_STR]
    return [argument.type_.value, argument.is_address, argument.name, str(argument.value)]


def argument_from_csv(csv_strs: List[str]) -> Optional[Argument]:
    if csv_strs[0] == NONE_STR:
        return None

    return Argument(
        ARGUMENT_TYPE_MAP[csv_strs[0]],
        csv_strs[1].strip() == TRUE_STR,
        csv_strs[2],
        None if csv_strs[3] == NONE_STR else int(csv_strs[3])
    )


class InstructionType(Enum):
    UNKNOWN = "UNKNOWN"
    NOP = "NOP"
    LD = "LD"
    LDI = "LDI"
    LDD = "LDD"
    LDH = "LDH"
    LDHL = "LDHL"
    INC = "INC"
    DEC = "DEC"
    ADD = "ADD"
    RLCA = "RLCA"
    RRCA = "RRCA"
    RLA = "RLA"
    RRA = "RRA"
    JR = "JR"
    DAA = "DAA"
    CPL = "CPL"
    SCF = "SCF"
    CCF = "CCF"
    HALT = "HALT"
    ADC = "ADC"
    SUB = "SUB"
    SBC = "SBC"
    XOR = "XOR"
    OR = "OR"
    CP = "CP"
    RET = "RET"
    POP = "POP"
    JP = "JP"
    CALL = "CALL"
    PUSH = "PUSH"
    RST = "RST"
    PREFIX = "PREFIX"
    DI = "DI"
    EI = "EI"
    RLC = "RLC"
    RRC = "RRC"
    RL = "RL"
    RR = "RR"
    SLA = "SLA"
    SRA = "SRA"
    SWAP = "SWAP"
    SRL = "SRL"
    BIT = "BIT"
    RES = "RES"
    SET = "SET"
    STOP = "STOP"
    AND = "AND"
    RETI = "RETI"


INSTRUCTION_TYPE_MAP = {type_.value: type_ for type_ in InstructionType}


class FlagAction(Enum):
    CUSTOM = "CUSTOM"
    SET = "SET"
    RESET = "RESET"
    NONE = "NONE"


FLAG_ACTION_MAP = {type_.value: type_ for type_ in FlagAction}


@dataclass
class GbOpcode:
    value: int
    instruction: InstructionType
    first_arg: Optional[Argument]
    second_arg: Optional[Argument]
    length: int
    duration: int
    duration_no_action: int
    z_flag: FlagAction
    n_flag: FlagAction
    h_flag: FlagAction
    c_flag: FlagAction


GB_OPCODE_HEADER = [
    "Value", "Instruction",
    "First Argument Type", "First Argument Is Address", "First Argument Name", "First Argument Value",
    "Second Argument Type", "Second Argument Is Address", "Second Argument Name", "Second Argument Value",
    "Length (bytes)", "Duration (Cycle)", "Duration no action (Cycle)",
    "Z flag", "N flag", "H flag", "C flag",
]


def gb_opcode_to_csv(opcode: GbOpcode) -> List[str]:
    return [str(opcode.value), opcode.instruction.value] + \
        argument_to_csv(opcode.first_arg) + \
        argument_to_csv(opcode.second_arg) + [
            str(opcode.length), str(opcode.duration), str(opcode.duration_no_action),
            opcode.z_flag.value, opcode.n_flag.value, opcode.h_flag.value, opcode.c_flag.value,
    ]


def gb_opcode_from_csv(csv_strs: List[str]) -> GbOpcode:
    return GbOpcode(
        int(csv_strs[0]),
        INSTRUCTION_TYPE_MAP[csv_strs[1]],
        argument_from_csv(csv_strs[2:6]),
        argument_from_csv(csv_strs[6:10]),
        int(csv_strs[10]),
        int(csv_strs[11]),
        int(csv_strs[12]),
        FLAG_ACTION_MAP[csv_strs[13]],
        FLAG_ACTION_MAP[csv_strs[14]],
        FLAG_ACTION_MAP[csv_strs[15]],
        FLAG_ACTION_MAP[csv_strs[16]],
    )


def write_gb_opcode_to_csv(filepath, opcodes):
    with open(filepath, 'w', newline='') as csv_file:
        writer = csv.writer(csv_file, delimiter=",")
        writer.writerow(GB_OPCODE_HEADER)
        for opcode in opcodes:
            writer.writerow(gb_opcode_to_csv(opcode))


def read_opcode_csv(filepath):
    with open(filepath, 'r', newline='') as csv_file:
        next(csv_file)
        reader = csv.reader(csv_file, delimiter=",")

        return [gb_opcode_from_csv(csv_strs) for csv_strs in reader]
