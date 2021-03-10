import csv
from collections import namedtuple
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

REGISTERS_NAME_SIZES = {
    "AF": 2, "A": 1, "BC": 2, "B": 1, "C": 1,
    "DE": 2, "D": 1, "E": 1, "HL": 2, "H": 1, "L": 1,
    "SP": 2, "PC": 2, "NZ": 1, "Z": 1, "NC": 1
}
IMMEDIATE_8_BITS_NAME = "d8"
IMMEDIATE_16_BITS_NAME = "d16"
UNSIGNED_8_BIT_NAME = "a8"
ADDRESS_16_BIT_NAME = "a16"
PC_INCREMENT_8_BIT_NAME = "r8"
INDICATION_NAME = "CB"
SPECIAL_MNEMONIC_MAP = {
    ("LD", ("A", "(C)")): ("LDSpecialC", ("A", "(C)")),
    ("LD", ("(C)", "A")): ("LDSpecialC", ("(C)", "A")),
    ("LD", ("A", "(HL+)")): ("LDI", ("A", "(HL)")),
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
    nb_bytes: int
    name: str
    value: Optional[int] = None

    @property
    def value_nb_bytes(self):
        return 2 if (not self.is_address) and (self.nb_bytes == 2) else 1

    @property
    def short_repr(self):
        return f"({self.name})" if self.is_address else self.name

    def is_register(self):
        return self.type_ != ArgumentType.REGISTER


def argument_to_csv(argument: Optional[Argument]) -> List[str]:
    if argument is None:
        return [NONE_STR, NONE_STR, NONE_STR, NONE_STR, NONE_STR]
    return [argument.type_.value, argument.is_address, argument.nb_bytes, argument.name, str(argument.value)]


def argument_from_csv(csv_strs: List[str]) -> Optional[Argument]:
    if csv_strs[0] == NONE_STR:
        return None

    return Argument(
        ARGUMENT_TYPE_MAP[csv_strs[0]],
        csv_strs[1].strip() == TRUE_STR,
        int(csv_strs[2]),
        csv_strs[3],
        None if csv_strs[4] == NONE_STR else int(csv_strs[4])
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
    LDSpecialC = "LDSpecialC"


INSTRUCTION_TYPE_MAP = {type_.value: type_ for type_ in InstructionType}


class FlagAction(Enum):
    CUSTOM = "CUSTOM"
    SET = "SET"
    RESET = "RESET"
    NONE = "NONE"


InstructionFlags = namedtuple("InstructionFlags", ["zero", "add_sub", "half_carry", "carry"])
FLAG_ACTION_MAP = {type_.value: type_ for type_ in FlagAction}


@dataclass
class GbInstruction:
    value: int
    type_: InstructionType
    first_arg: Optional[Argument]
    second_arg: Optional[Argument]
    length: int
    duration: int
    duration_no_action: int
    zero_flag: FlagAction
    add_sub_flag: FlagAction
    half_carry_flag: FlagAction
    carry_flag: FlagAction

    @property
    def flags(self) -> InstructionFlags:
        return InstructionFlags(
            self.zero_flag, self.add_sub_flag, self.half_carry_flag, self.carry_flag
        )

    @property
    def short_repr(self):
        representation = f"0x{self.value:X} {self.type_.value}"
        if self.first_arg:
            representation += f" {self.first_arg.short_repr}"
        if self.second_arg:
            representation += f", {self.second_arg.short_repr}"

        return representation


GB_OPCODE_HEADER = [
    "Value", "Instruction",
    "First Argument Type", "First Argument Is Address", "First Argument Nb Bytes", "First Argument Name", "First Argument Value",
    "Second Argument Type", "Second Argument Is Address", "Second Argument Nb Bytes", "Second Argument Name", "Second Argument Value",
    "Length (bytes)", "Duration (Cycle)", "Duration no action (Cycle)",
    "Z flag", "N flag", "H flag", "C flag",
]


def gb_instruction_to_csv(instruction: GbInstruction) -> List[str]:
    return [str(instruction.value), instruction.type_.value] + \
           argument_to_csv(instruction.first_arg) + \
           argument_to_csv(instruction.second_arg) + [
            str(instruction.length), str(instruction.duration), str(instruction.duration_no_action),
            instruction.zero_flag.value, instruction.add_sub_flag.value, instruction.half_carry_flag.value, instruction.carry_flag.value,
    ]


def gb_instruction_from_csv(csv_strs: List[str]) -> GbInstruction:
    return GbInstruction(
        int(csv_strs[0]),
        INSTRUCTION_TYPE_MAP[csv_strs[1]],
        argument_from_csv(csv_strs[2:7]),
        argument_from_csv(csv_strs[7:12]),
        int(csv_strs[12]),
        int(csv_strs[13]),
        int(csv_strs[14]),
        FLAG_ACTION_MAP[csv_strs[15]],
        FLAG_ACTION_MAP[csv_strs[16]],
        FLAG_ACTION_MAP[csv_strs[17]],
        FLAG_ACTION_MAP[csv_strs[18]],
    )


def write_gb_instruction_to_csv(filepath, opcodes):
    with open(filepath, 'w', newline='') as csv_file:
        writer = csv.writer(csv_file, delimiter=",")
        writer.writerow(GB_OPCODE_HEADER)
        for opcode in opcodes:
            writer.writerow(gb_instruction_to_csv(opcode))


def read_instruction_csv(filepath) -> List[GbInstruction]:
    with open(filepath, 'r', newline='') as csv_file:
        next(csv_file)
        reader = csv.reader(csv_file, delimiter=",")

        return [gb_instruction_from_csv(csv_strs) for csv_strs in reader]
