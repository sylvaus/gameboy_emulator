import os
from dataclasses import dataclass
from typing import Dict, Callable, Optional, List, Union, Tuple
from collections import OrderedDict, namedtuple

from generators.instructions.gbinstruction import read_instruction_csv, InstructionType, \
    GbInstruction, Argument, ArgumentType, FlagAction, InstructionFlags
from generators.utils.formatters import indent_code, make_function, put_code_in_namespace, make_if_else_code

THIS_FOLDER = os.path.dirname(os.path.abspath(__file__))
PROJECT_FOLDER = os.path.abspath(os.path.join(THIS_FOLDER, "..", ".."))

FILE_NAME = "instructions"
GENERATED_FOLDER_NAME = "generated"
SRC_FILE = os.path.join(PROJECT_FOLDER, "src", GENERATED_FOLDER_NAME, f"{FILE_NAME}.cpp")
INCLUDE_FILE = os.path.join(PROJECT_FOLDER, "include", "emulator", GENERATED_FOLDER_NAME, f"{FILE_NAME}.h")
NAMESPACE = "emulator::generated"

OFFSET_CARRY_FLAG_VALUE = 4
OFFSET_HALF_CARRY_FLAG_VALUE = 5
OFFSET_ADD_SUB_FLAG_VALUE = 6
OFFSET_ZERO_FLAG_VALUE = 7
OFFSET_CARRY_FLAG = "OFFSET_CARRY_FLAG"
OFFSET_HALF_CARRY_FLAG = "OFFSET_HALF_CARRY_FLAG"
OFFSET_ADD_SUB_FLAG = "OFFSET_ADD_SUB_FLAG"
OFFSET_ZERO_FLAG = "OFFSET_ZERO_FLAG"
CARRY_FLAG = "carry_flag"
HALF_CARRY_FLAG = "half_carry_flag"
ADD_SUB_FLAG = "add_sub_flag"
ZERO_FLAG = "zero_flag"


ARGUMENT_STRUCT_NAME = "Arguments"
ARGUMENT_STRUCT = f"""union {ARGUMENT_STRUCT_NAME}
{{
    uint16_t uint16;
    uint8_t uint8;
    int8_t int8;
}};"""
ARGUMENT_ENUM_NAME = "ArgumentType"
ARGUMENT_ENUM_NONE = "none"
ARGUMENT_ENUM_INT8 = "int8"
ARGUMENT_ENUM_UINT8 = "uint8"
ARGUMENT_ENUM_UINT16 = "uint16"
ARGUMENT_ENUM = f"""enum class {ARGUMENT_ENUM_NAME}
{{
    {ARGUMENT_ENUM_NONE},
    {ARGUMENT_ENUM_INT8},
    {ARGUMENT_ENUM_UINT8},
    {ARGUMENT_ENUM_UINT16}
}};"""
ARGUMENT_TYPE_TO_IMMEDIATE_VALUE_TYPE = {
    ArgumentType.REGISTER: ARGUMENT_ENUM_NONE,
    ArgumentType.IMMEDIATE_8_BITS: ARGUMENT_ENUM_UINT8,
    ArgumentType.IMMEDIATE_16_BITS: ARGUMENT_ENUM_UINT16,
    ArgumentType.UNSIGNED_8_BIT: ARGUMENT_ENUM_UINT8,
    ArgumentType.ADDRESS_16_BIT: ARGUMENT_ENUM_UINT16,
    ArgumentType.PC_INCREMENT_8_BIT: ARGUMENT_ENUM_INT8,
    ArgumentType.VALUE: ARGUMENT_ENUM_NONE,
    ArgumentType.INDICATION: ARGUMENT_ENUM_NONE
}

ARGUMENT_NAME = "arguments"
REGISTERS = "registers"
MEMORY_CONTROLLER = "controller"
OPCODE_FUNC_PARAMETERS = f"(const {ARGUMENT_STRUCT_NAME}& {ARGUMENT_NAME}, " \
                        f"Registers& {REGISTERS}, MemoryController& {MEMORY_CONTROLLER})"

FUNC_PARAMETERS = [ARGUMENT_NAME, REGISTERS, MEMORY_CONTROLLER]

INSTRUCTION_FUNCTION_TYPE = "InstructionFunction"
INCLUDE_USING_DEFS = f"using emulator::memory::Registers;\n" \
                     f"using emulator::memory::MemoryController;\n" \
                     f"using {INSTRUCTION_FUNCTION_TYPE} = std::function<uint16_t {OPCODE_FUNC_PARAMETERS}>;"


SRC_INCLUDE_DEFS = f"using emulator::memory::{OFFSET_CARRY_FLAG};\n" \
                   f"using emulator::memory::{OFFSET_HALF_CARRY_FLAG};\n" \
                   f"using emulator::memory::{OFFSET_ADD_SUB_FLAG};\n" \
                   f"using emulator::memory::{OFFSET_ZERO_FLAG};\n"

ARGUMENT_UINT8 = f"{ARGUMENT_NAME}.uint8"
ARGUMENT_INT8 = f"{ARGUMENT_NAME}.int8"
ARGUMENT_UINT16 = f"{ARGUMENT_NAME}.uint16"
REGISTERS_A = f"{REGISTERS}.A"
REGISTERS_FLAGS = f"{REGISTERS}.F"
REGISTERS_B = f"{REGISTERS}.B"
REGISTERS_C = f"{REGISTERS}.C"
REGISTERS_D = f"{REGISTERS}.D"
REGISTERS_E = f"{REGISTERS}.E"
REGISTERS_H = f"{REGISTERS}.H"
REGISTERS_L = f"{REGISTERS}.L"
REGISTERS_FLAGS_GET_CARRY = f"{REGISTERS}.get_carry_flag()"
REGISTERS_FLAGS_GET_HALF_CARRY = f"{REGISTERS}.get_half_carry_flag()"
REGISTERS_FLAGS_GET_ADD_SUB = f"{REGISTERS}.get_add_sub_flag()"
REGISTERS_FLAGS_GET_ZERO = f"{REGISTERS}.get_zero_flag()"
REGISTERS_FLAGS_GET_NON_CARRY = f"{REGISTERS}.get_non_carry_flag()"
REGISTERS_FLAGS_GET_NON_HALF_CARRY = f"{REGISTERS}.get_non_half_carry_flag()"
REGISTERS_FLAGS_GET_NON_ADD_SUB = f"{REGISTERS}.get_non_add_sub_flag()"
REGISTERS_FLAGS_GET_NON_ZERO = f"{REGISTERS}.get_non_zero_flag()"
REGISTERS_STACK_POINTER = f"{REGISTERS}.SP"
REGISTERS_PROGRAM_COUNTER = f"{REGISTERS}.PC"
REGISTERS_HALTED = f"{REGISTERS}.halted"
REGISTERS_STOPPED = f"{REGISTERS}.stopped"
REGISTERS_IME_FLAG = f"{REGISTERS}.ime_flag"

REGISTERS_FLAG_TO_GETTER = {
    "C": REGISTERS_FLAGS_GET_CARRY,
    "Z": REGISTERS_FLAGS_GET_ZERO,
    "NC": REGISTERS_FLAGS_GET_NON_CARRY,
    "NZ": REGISTERS_FLAGS_GET_NON_ZERO
}

REGISTERS_16_TO_LOWER_UPPER_8_MAP = {
    "AF": (REGISTERS_FLAGS, REGISTERS_A),
    "BC": (REGISTERS_C, REGISTERS_B),
    "DE": (REGISTERS_E, REGISTERS_D),
    "HL": (REGISTERS_L, REGISTERS_H),
}

REGISTERS_WITH_GETTER_SETTERS = {"AF", "BC", "DE", "HL"}
ARGUMENT_REGISTERS_A = Argument(ArgumentType.REGISTER, is_address=False, nb_bytes=1, name="A")

SRC_HEADER = f"""#include "emulator/{GENERATED_FOLDER_NAME}/{FILE_NAME}.h"\n\n"""

INCLUDE_HEADER = f"""#pragma once\n
#include <cstdint>
#include <functional>
#include <array>
#include "emulator/memory/registers.h"
#include "emulator/memory/memory_controller.h"\n\n"""


@dataclass
class InstructionFunction:
    name: str
    declaration: str
    definition: str
    argument_type: str
    instruction: GbInstruction

    @property
    def instruction_opcode(self):
        return self.instruction.value

    def instruction_length(self):
        return self.instruction.length


GENERATORS: Dict[InstructionType, Callable[[GbInstruction], InstructionFunction]] = OrderedDict()


def register_generator(*instructions: InstructionType):
    def decorator(f):
        for instruction in instructions:
            if instruction in GENERATORS:
                raise ValueError(
                    f"A generator has already been defined for instruction type: {instruction}"
                )
            GENERATORS[instruction] = f
        return f

    return decorator


def get_argument_name(argument: Argument):
    name = argument.value if argument.value else argument.name
    if argument.is_address:
        name = f"addr_{name}"
    return name


def get_immediate_value_type(instruction: GbInstruction):
    """
    Return the enum corresponding to the immediate value used by the given instruction
    For example
    JR NZ, s8 would return the enum for s8
    JP a16 would return the enum for a16
    INC BC or NOP would return the enum for none immediate value
    """
    if instruction.first_arg is None:
        return ARGUMENT_ENUM_NONE

    first_argument_enum = ARGUMENT_TYPE_TO_IMMEDIATE_VALUE_TYPE[instruction.first_arg.type_]
    if first_argument_enum != ARGUMENT_ENUM_NONE:
        return first_argument_enum

    if instruction.second_arg is None:
        return ARGUMENT_ENUM_NONE

    return ARGUMENT_TYPE_TO_IMMEDIATE_VALUE_TYPE[instruction.second_arg.type_]


def make_instruction_function(
        instruction: GbInstruction, code: str, remove_pc_update: bool = False, remove_duration_return: bool = False
) -> InstructionFunction:
    code_lines = code.splitlines(False)
    if not remove_pc_update:
        code_lines.append(f"{REGISTERS_PROGRAM_COUNTER} += {instruction.length};")
    if not remove_duration_return:
        code_lines.append(f"return {instruction.duration};")

    func_name = f"{instruction.type_.value.lower()}_{instruction.value:03x}"
    signature = f"uint16_t {func_name}{OPCODE_FUNC_PARAMETERS}"
    declaration = f"{signature}; // {instruction.short_repr}"
    definition = make_function(f"{signature} // {instruction.short_repr}", code_lines, FUNC_PARAMETERS)

    return InstructionFunction(
        func_name, declaration, definition, get_immediate_value_type(instruction), instruction
    )


def make_get_code(argument: Argument, address_offset: Optional[str] = None, is_not_address: bool = False):
    if argument.type_ == ArgumentType.REGISTER:
        if argument.name in REGISTERS_WITH_GETTER_SETTERS:
            code = f"{REGISTERS}.get_{argument.name}()"
        else:
            code = f"{REGISTERS}.{argument.name}"
    elif argument.type_ == ArgumentType.IMMEDIATE_8_BITS:
        code = ARGUMENT_UINT8
    elif argument.type_ == ArgumentType.IMMEDIATE_16_BITS:
        code = ARGUMENT_UINT16
    elif argument.type_ == ArgumentType.UNSIGNED_8_BIT:
        code = ARGUMENT_UINT8
    elif argument.type_ == ArgumentType.ADDRESS_16_BIT:
        code = ARGUMENT_UINT16
    elif argument.type_ == ArgumentType.PC_INCREMENT_8_BIT:
        code = ARGUMENT_INT8
    elif argument.type_ == ArgumentType.VALUE:
        code = f"{argument.value}"
    elif argument.type_ == ArgumentType.INDICATION:
        code = ""
    else:
        raise RuntimeError(f"Unknown Argument Type for get  {argument.type_}")

    if argument.is_address and not is_not_address:
        if address_offset:
            code = f"({code}) + {address_offset}"
        return make_get_from_address(code)
    return code


def make_get_from_address(address: str) -> str:
    return f"{MEMORY_CONTROLLER}.get({address})"


def make_set_code(
        dst: Argument, value: Argument,
        dst_address_offset: Optional[str] = None, value_address_offset: Optional[str] = None,
        value_offset: Optional[str] = None,
) -> str:
    code_value = make_get_code(value, value_address_offset)
    if value_offset:
        code_value = f"({code_value}) + {value_offset}"
    return make_set_code_from_value(dst, code_value, value.value_nb_bytes, dst_address_offset)


def make_set_code_from_value(
        dst: Argument, code_value: str, nb_bytes: int, dst_address_offset: Optional[str] = None
) -> str:
    if dst.is_address:
        code_address = make_get_code(dst, is_not_address=True)
        if dst_address_offset:
            code_address = f"({code_address}) + {dst_address_offset}"
        return make_set_memory_address(code_address, code_value, nb_bytes)

    if dst.type_ == ArgumentType.REGISTER:
        if dst.name in REGISTERS_WITH_GETTER_SETTERS:
            return f"{REGISTERS}.set_{dst.name}({code_value});"
        else:
            return f"{REGISTERS}.{dst.name} = {code_value};"
    elif dst.type_ in {
            ArgumentType.IMMEDIATE_8_BITS, ArgumentType.IMMEDIATE_16_BITS, ArgumentType.UNSIGNED_8_BIT,
            ArgumentType.ADDRESS_16_BIT, ArgumentType.PC_INCREMENT_8_BIT, ArgumentType.VALUE, ArgumentType.INDICATION}:
        raise RuntimeError(f"Argument Type cannot be a destination for set {dst.type_}")

    raise RuntimeError(f"Unknown Destination Argument Type {dst.type_}")


def make_set_memory_address(address: str, value: str, nb_bytes: int = 1) -> str:
    if nb_bytes > 1:
        return f"{MEMORY_CONTROLLER}.set16bits({address}, {value});"
    return f"{MEMORY_CONTROLLER}.set({address}, {value});"


AddSubFlagCode = namedtuple("AddSubFlagCode", ["result", "code"])


def make_add_sub_flag_code(
        instruction: GbInstruction, is_add: bool,
        first_value: Optional[Union[str, int]] = None,
        second_value: Optional[Union[str, int]] = None,
        third_value: Optional[Union[str, int]] = None
) -> AddSubFlagCode:
    carry_max_value, half_carry_max_value = get_operation_carry_max_values(instruction)
    sign = "+" if is_add else "-"
    third_value_code = f"int32_t rrhs = {third_value};\n" if third_value else ""
    third_op = f" {sign} rrhs" if third_value else ""

    half_result_code = ""
    flag_values = []
    if instruction.zero_flag == FlagAction.CUSTOM:
        flag_values.append(f"uint8_t {ZERO_FLAG} = (result & {carry_max_value}) == 0;")
    if instruction.half_carry_flag == FlagAction.CUSTOM:
        flag_values.append(make_half_carry_flag(half_carry_max_value, is_add))
        half_result_code = make_half_carry_code(half_carry_max_value, sign, third_op)
    if instruction.carry_flag == FlagAction.CUSTOM:
        flag_values.append(make_carry_flag(carry_max_value, is_add))

    flag_value_code = "\n".join(flag_values)
    if not first_value:
        first_value = make_get_code(instruction.first_arg)
    if not second_value:
        second_value = make_get_code(instruction.second_arg)
    return AddSubFlagCode(
        f"result & {carry_max_value}",
        f"int32_t lhs = {first_value};\n"
        f"int32_t rhs = {second_value};\n"
        f"{third_value_code}"
        f"{half_result_code}\n"
        f"int32_t result = lhs {sign} rhs{third_op};\n"
        f"{flag_value_code}\n"
        f"{make_flag_code(instruction.flags)}"
    )


def get_operation_carry_max_values(instruction: GbInstruction) -> Tuple[str, str]:
    two_bytes_op = (instruction.first_arg.value_nb_bytes > 1) or \
                   (instruction.second_arg and (instruction.second_arg.value_nb_bytes > 1))

    carry_max_value = "0xFFFF" if two_bytes_op else "0xFF"
    half_carry_max_value = "0xFFF" if two_bytes_op else "0xF"
    return carry_max_value, half_carry_max_value


def make_half_carry_code(half_carry_max_value, sign, third_op):
    return f"int32_t half_result = (lhs & {half_carry_max_value}) {sign} (rhs & {half_carry_max_value}){third_op};"


def make_half_carry_flag(half_carry_max_value: str, is_add: bool) -> str:
    value = f"half_result > {half_carry_max_value}" if is_add else "half_result < 0"
    return f"uint8_t {HALF_CARRY_FLAG} = {value};"


def make_carry_flag(carry_max_value: str, is_add: bool) -> str:
    value = f"result > {carry_max_value}" if is_add else "result < 0"
    return f"uint8_t {CARRY_FLAG} = {value};"


def make_flag_code(flags: InstructionFlags) -> str:
    if all(flag == FlagAction.NONE for flag in flags):
        return "// No flag operation"

    initial_flag = ((flags.zero == FlagAction.SET) << OFFSET_ZERO_FLAG_VALUE) + \
                   ((flags.add_sub == FlagAction.SET) << OFFSET_ADD_SUB_FLAG_VALUE) + \
                   ((flags.half_carry == FlagAction.SET) << OFFSET_HALF_CARRY_FLAG_VALUE) + \
                   ((flags.carry == FlagAction.SET) << OFFSET_CARRY_FLAG_VALUE)

    current_flag_mask = ((flags.zero == FlagAction.NONE) << OFFSET_ZERO_FLAG_VALUE) + \
                        ((flags.add_sub == FlagAction.NONE) << OFFSET_ADD_SUB_FLAG_VALUE) + \
                        ((flags.half_carry == FlagAction.NONE) << OFFSET_HALF_CARRY_FLAG_VALUE) + \
                        ((flags.carry == FlagAction.NONE) << OFFSET_CARRY_FLAG_VALUE)

    flag_setting = f"{REGISTERS_FLAGS} &= 0b{current_flag_mask:08b};"
    flag_variables = get_custom_flag_values(flags)
    if initial_flag:
        flag_variables.insert(0, f"0b{initial_flag:08b}")

    if flag_variables:
        flag_setting += f"\n{REGISTERS_FLAGS} |= {' + '.join(flag_variables)};"
    return flag_setting


def get_custom_flag_values(flags: InstructionFlags) -> List[str]:
    flag_names = []
    if flags.zero == FlagAction.CUSTOM:
        flag_names.append(f"({ZERO_FLAG} << {OFFSET_ZERO_FLAG})")
    if flags.half_carry == FlagAction.CUSTOM:
        flag_names.append(f"({HALF_CARRY_FLAG} << {OFFSET_HALF_CARRY_FLAG})")
    if flags.carry == FlagAction.CUSTOM:
        flag_names.append(f"({CARRY_FLAG} << {OFFSET_CARRY_FLAG})")

    return flag_names


@register_generator(InstructionType.NOP)
def nop_generator(instruction: GbInstruction) -> InstructionFunction:
    return make_instruction_function(instruction, "// Nothing to be done")


@register_generator(InstructionType.UNKNOWN)
def unknown_generator(instruction: GbInstruction) -> InstructionFunction:
    return make_instruction_function(
        instruction,
        f"""throw std::runtime_error("Unknown opcode 0x{instruction.value:X}");""",
        remove_pc_update=True, remove_duration_return=True
    )


@register_generator(
    InstructionType.LD, InstructionType.LDI, InstructionType.LDD, InstructionType.LDH, InstructionType.LDSpecialC
)
def ld_generator(instruction: GbInstruction) -> InstructionFunction:
    address_offset = "0xFF00" if instruction.type_ in {InstructionType.LDH, InstructionType.LDSpecialC} else None
    code = f"{make_set_code(instruction.first_arg, instruction.second_arg, address_offset, address_offset)}"

    if instruction.type_ == InstructionType.LDI:
        code += f"\n{REGISTERS}.set_HL({REGISTERS}.get_HL() + 1);"
    if instruction.type_ == InstructionType.LDD:
        code += f"\n{REGISTERS}.set_HL({REGISTERS}.get_HL() - 1);"
    return make_instruction_function(instruction, code)


@register_generator(InstructionType.LDHL)
def ldhl_generator(instruction: GbInstruction) -> InstructionFunction:
    result_value, code = make_add_sub_flag_code(instruction, True)
    code = f"{code}\n{REGISTERS}.set_HL({result_value});"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.INC)
def inc_generator(instruction: GbInstruction) -> InstructionFunction:
    result_value, flag_code = make_add_sub_flag_code(instruction, True, second_value=1)
    set_code = make_set_code_from_value(instruction.first_arg, result_value, instruction.first_arg.value_nb_bytes)
    code = f"{flag_code}\n{set_code}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.DEC)
def dec_generator(instruction: GbInstruction) -> InstructionFunction:
    result_value, flag_code = make_add_sub_flag_code(instruction, False, second_value=1)
    set_code = make_set_code_from_value(instruction.first_arg, result_value, instruction.first_arg.value_nb_bytes)
    code = f"{flag_code}\n{set_code}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.ADD)
def add_generator(instruction: GbInstruction) -> InstructionFunction:
    result_value, flag_code = make_add_sub_flag_code(instruction, True)
    set_code = make_set_code_from_value(instruction.first_arg, result_value, instruction.first_arg.value_nb_bytes)
    code = f"{flag_code}\n{set_code}"

    return make_instruction_function(instruction, code)


ALL_ROTATIONS = (
    InstructionType.RLCA, InstructionType.RRCA, InstructionType.RLA, InstructionType.RRA,
    InstructionType.RLC, InstructionType.RL, InstructionType.RRC, InstructionType.RR
)
LEFT_ROTATIONS = (InstructionType.RLCA, InstructionType.RLA, InstructionType.RLC, InstructionType.RL)
CARRY_ROTATIONS = (InstructionType.RLCA, InstructionType.RRCA, InstructionType.RLC, InstructionType.RRC)


@register_generator(*ALL_ROTATIONS)
def rotate_generator(instruction: GbInstruction) -> InstructionFunction:
    argument = instruction.first_arg if instruction.first_arg else ARGUMENT_REGISTERS_A
    left = instruction.type_ in LEFT_ROTATIONS
    carry_value = "(value >> 7) & 0b1" if left else "value & 0b1"
    bit_carried = CARRY_FLAG if instruction.type_ in CARRY_ROTATIONS else REGISTERS_FLAGS_GET_CARRY
    result_value = f"(value << 1) + {bit_carried}" if left else f"(value >> 1) + ({bit_carried} << 7)"
    zero_flag_code = ""
    if instruction.zero_flag == FlagAction.CUSTOM:
        zero_flag_code = f"uint8_t {ZERO_FLAG} = (result & 0xFF) == 0;\n"
    code = f"uint8_t value = {make_get_code(argument)};\n" \
           f"uint8_t {CARRY_FLAG} = {carry_value};\n" \
           f"uint8_t result = {result_value};\n" \
           f"{zero_flag_code}" \
           f"{make_set_code_from_value(argument, 'result', nb_bytes=1)}\n" \
           f"{make_flag_code(instruction.flags)}"
    return make_instruction_function(instruction, code)


@register_generator(InstructionType.JR)
def jr_generator(instruction: GbInstruction) -> InstructionFunction:
    if not instruction.second_arg:
        code = f"{REGISTERS_PROGRAM_COUNTER} += {instruction.length} + {ARGUMENT_INT8};"
        return make_instruction_function(instruction, code, remove_pc_update=True)

    code = f"{REGISTERS_PROGRAM_COUNTER} += {instruction.length};\n" \
           f"if (!{REGISTERS_FLAG_TO_GETTER[instruction.first_arg.name]})\n" \
           f"    return {instruction.duration_no_action};\n" \
           f"{REGISTERS_PROGRAM_COUNTER} += {ARGUMENT_INT8};\n" \
           f"return {instruction.duration};"

    return make_instruction_function(instruction, code, remove_pc_update=True, remove_duration_return=True)


@register_generator(InstructionType.DAA)
def daa_generator(instruction: GbInstruction) -> InstructionFunction:
    code_add = f"{CARRY_FLAG} = (0x99 < {REGISTERS_A}) || {REGISTERS_FLAGS_GET_CARRY};\n" \
               f"{REGISTERS_A} += (0x6 * ((0xA < ({REGISTERS_A} & 0xF)) || {REGISTERS_FLAGS_GET_HALF_CARRY})) +" \
               f"(0x60 * {CARRY_FLAG});"

    code_sub = f"{CARRY_FLAG} = {REGISTERS_FLAGS_GET_CARRY};\n" \
               f"{REGISTERS_A} -= (0x6 * {REGISTERS_FLAGS_GET_HALF_CARRY}) + (0x60 * {CARRY_FLAG});"

    code = f"uint8_t {CARRY_FLAG};\n" \
           f"{make_if_else_code(REGISTERS_FLAGS_GET_ADD_SUB, code_sub, code_add)}\n" \
           f"uint8_t {ZERO_FLAG} = ({REGISTERS_A} == 0x00);\n" \
           f"{make_flag_code(instruction.flags)}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.CPL)
def cpl_generator(instruction: GbInstruction) -> InstructionFunction:
    code = f"{REGISTERS_A} = ~{REGISTERS_A};\n" \
           f"{make_flag_code(instruction.flags)}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.SCF)
def scf_generator(instruction: GbInstruction) -> InstructionFunction:
    return make_instruction_function(instruction, make_flag_code(instruction.flags))


@register_generator(InstructionType.CCF)
def ccf_generator(instruction: GbInstruction) -> InstructionFunction:
    code = f"uint8_t {CARRY_FLAG} = !{REGISTERS_FLAGS_GET_CARRY};\n" \
           f"{make_flag_code(instruction.flags)}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.HALT)
def halt_generator(instruction: GbInstruction) -> InstructionFunction:
    return make_instruction_function(instruction, f"{REGISTERS_HALTED} = true;")


@register_generator(InstructionType.ADC)
def adc_generator(instruction: GbInstruction) -> InstructionFunction:
    result_value, flag_code = make_add_sub_flag_code(
        instruction, True, third_value=f"{REGISTERS_FLAGS_GET_CARRY}"
    )
    set_code = make_set_code_from_value(instruction.first_arg, result_value, instruction.first_arg.value_nb_bytes)
    code = f"{flag_code}\n{set_code}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.SUB)
def sub_generator(instruction: GbInstruction) -> InstructionFunction:
    instruction.second_arg = instruction.first_arg
    instruction.first_arg = ARGUMENT_REGISTERS_A
    result_value, flag_code = make_add_sub_flag_code(instruction, False)
    set_code = make_set_code_from_value(instruction.first_arg, result_value, instruction.first_arg.value_nb_bytes)
    code = f"{flag_code}\n{set_code}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.SBC)
def sbc_generator(instruction: GbInstruction) -> InstructionFunction:
    result_value, flag_code = make_add_sub_flag_code(
        instruction, False, third_value=f"{REGISTERS_FLAGS_GET_CARRY}"
    )
    set_code = make_set_code_from_value(instruction.first_arg, result_value, instruction.first_arg.value_nb_bytes)
    code = f"{flag_code}\n{set_code}"

    return make_instruction_function(instruction, code)


BITWISE_OPERATOR_MAP = {InstructionType.XOR: "^", InstructionType.OR: "|", InstructionType.AND: "&"}


@register_generator(*BITWISE_OPERATOR_MAP.keys())
def bitwise_operations_generator(instruction: GbInstruction) -> InstructionFunction:
    operator = BITWISE_OPERATOR_MAP[instruction.type_]
    code = f"{REGISTERS_A} {operator}= {make_get_code(instruction.first_arg)};\n" \
           f"uint8_t {ZERO_FLAG} = {REGISTERS_A} == 0;\n" \
           f"{make_flag_code(instruction.flags)}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.CP)
def cp_generator(instruction: GbInstruction) -> InstructionFunction:
    instruction.second_arg = instruction.first_arg
    instruction.first_arg = ARGUMENT_REGISTERS_A
    _, flag_code = make_add_sub_flag_code(instruction, False)
    return make_instruction_function(instruction, flag_code)


@register_generator(InstructionType.RET)
def ret_generator(instruction: GbInstruction) -> InstructionFunction:
    code = f"{REGISTERS_PROGRAM_COUNTER} = {make_get_from_address(REGISTERS_STACK_POINTER + '++')};\n" \
           f"{REGISTERS_PROGRAM_COUNTER} += ({make_get_from_address(REGISTERS_STACK_POINTER + '++')}) << 8;"

    if instruction.first_arg is None:
        return make_instruction_function(instruction, code, remove_pc_update=True)

    code = f"{code}\nreturn {instruction.duration};"
    code = f"if ({REGISTERS_FLAG_TO_GETTER[instruction.first_arg.name]})\n" \
           f"{{\n{indent_code(code)}\n}}\n" \
           f"{REGISTERS_PROGRAM_COUNTER} += {instruction.length};\n" \
           f"return {instruction.duration_no_action};"

    return make_instruction_function(instruction, code, remove_pc_update=True, remove_duration_return=True)


@register_generator(InstructionType.POP)
def pop_generator(instruction: GbInstruction) -> InstructionFunction:
    lower, upper = REGISTERS_16_TO_LOWER_UPPER_8_MAP[instruction.first_arg.name]
    code = f"{lower} = {make_get_from_address(REGISTERS_STACK_POINTER + '++')};\n" \
           f"{upper} += {make_get_from_address(REGISTERS_STACK_POINTER + '++')};"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.JP)
def jp_generator(instruction: GbInstruction) -> InstructionFunction:
    if not instruction.second_arg:
        code = f"{REGISTERS_PROGRAM_COUNTER} = {ARGUMENT_UINT16};"
        return make_instruction_function(instruction, code, remove_pc_update=True)

    code = f"if ({REGISTERS_FLAG_TO_GETTER[instruction.first_arg.name]})\n" \
           f"{{\n" \
           f"    {REGISTERS_PROGRAM_COUNTER} = {ARGUMENT_UINT16};\n" \
           f"    return {instruction.duration};\n" \
           f"}}\n" \
           f"{REGISTERS_PROGRAM_COUNTER} += {instruction.length};\n" \
           f"return {instruction.duration_no_action};"

    return make_instruction_function(instruction, code, remove_pc_update=True, remove_duration_return=True)


@register_generator(InstructionType.CALL)
def call_generator(instruction: GbInstruction) -> InstructionFunction:
    lower_pc_value = f"{REGISTERS_PROGRAM_COUNTER} & 0xFF"
    upper_pc_value = f"({REGISTERS_PROGRAM_COUNTER} >> 8) & 0xFF"
    code = f"{make_set_memory_address('--' + REGISTERS_STACK_POINTER, upper_pc_value)}\n" \
           f"{make_set_memory_address('--' + REGISTERS_STACK_POINTER, lower_pc_value)}\n" \
           f"{REGISTERS_PROGRAM_COUNTER} = {ARGUMENT_UINT16};"

    if not instruction.second_arg:
        return make_instruction_function(instruction, code, remove_pc_update=True)

    code += f"\nreturn {instruction.duration};"
    code = f"if ({REGISTERS_FLAG_TO_GETTER[instruction.first_arg.name]})\n" \
           f"{{\n" \
           f"{indent_code(code)}\n" \
           f"}}\n" \
           f"{REGISTERS_PROGRAM_COUNTER} += {instruction.length};\n" \
           f"return {instruction.duration_no_action};"

    return make_instruction_function(instruction, code, remove_pc_update=True, remove_duration_return=True)


@register_generator(InstructionType.PUSH)
def push_generator(instruction: GbInstruction) -> InstructionFunction:
    lower, upper = REGISTERS_16_TO_LOWER_UPPER_8_MAP[instruction.first_arg.name]
    code = f"{make_set_memory_address('--' + REGISTERS_STACK_POINTER, upper)}\n" \
           f"{make_set_memory_address('--' + REGISTERS_STACK_POINTER, lower)}"

    return make_instruction_function(instruction, code)


@register_generator(InstructionType.RST)
def rst_generator(instruction: GbInstruction) -> InstructionFunction:
    upper = f"({REGISTERS_PROGRAM_COUNTER} >> 8) & 0xFF"
    lower = f"{REGISTERS_PROGRAM_COUNTER} & 0xFF"

    code = f"{REGISTERS_PROGRAM_COUNTER} += {instruction.length};\n" \
           f"{make_set_memory_address('--' + REGISTERS_STACK_POINTER, upper)}\n" \
           f"{make_set_memory_address('--' + REGISTERS_STACK_POINTER, lower)}\n" \
           f"{REGISTERS_PROGRAM_COUNTER} = 0x{int(instruction.first_arg.value):02x};"

    return make_instruction_function(instruction, code, remove_pc_update=True)


@register_generator(InstructionType.PREFIX)
def prefix_generator(instruction: GbInstruction) -> InstructionFunction:
    return make_instruction_function(
        instruction,
        f"""throw std::runtime_error("Opcode 0xCB should be handled separately, something bad must have happened");""",
        remove_pc_update=True, remove_duration_return=True
    )


@register_generator(InstructionType.DI, InstructionType.EI)
def di_generator(instruction: GbInstruction) -> InstructionFunction:
    value = "true" if instruction.type_ == InstructionType.EI else "false"
    return make_instruction_function(instruction, f"{REGISTERS_IME_FLAG} = {value};")


SHIFT_RESULT_MAP = {
    InstructionType.SLA: "value << 1",
    InstructionType.SRA: "(value >> 1) + (value & 0x80)",
    InstructionType.SRL: "value >> 1"
}


@register_generator(InstructionType.SLA, InstructionType.SRA, InstructionType.SRL)
def shift_generator(instruction: GbInstruction) -> InstructionFunction:
    argument = instruction.first_arg if instruction.first_arg else ARGUMENT_REGISTERS_A
    left = instruction.type_ == InstructionType.SLA
    carry_value = "(value >> 7) & 0b1" if left else "value & 0b1"
    code = f"uint8_t value = {make_get_code(argument)};\n" \
           f"uint8_t {CARRY_FLAG} = {carry_value};\n" \
           f"uint8_t result = {SHIFT_RESULT_MAP[instruction.type_]};\n" \
           f"uint8_t {ZERO_FLAG} = result == 0;\n" \
           f"{make_set_code_from_value(argument, 'result', nb_bytes=1)}\n" \
           f"{make_flag_code(instruction.flags)}"
    return make_instruction_function(instruction, code)


@register_generator(InstructionType.SWAP)
def swap_generator(instruction: GbInstruction) -> InstructionFunction:
    code = f"uint8_t value = {make_get_code(instruction.first_arg)};\n" \
           f"uint8_t result = (value >> 4) | (value  << 4);\n" \
           f"uint8_t {ZERO_FLAG} = result == 0;\n" \
           f"{make_set_code_from_value(instruction.first_arg, 'result', nb_bytes=1)}\n" \
           f"{make_flag_code(instruction.flags)}"
    return make_instruction_function(instruction, code)


@register_generator(InstructionType.BIT)
def bit_generator(instruction: GbInstruction) -> InstructionFunction:
    zero_flag_value = f"(({make_get_code(instruction.second_arg)} >> {instruction.first_arg.value}) & 0b1) == 0"
    return make_instruction_function(
        instruction, f"uint8_t {ZERO_FLAG} = {zero_flag_value};\n{make_flag_code(instruction.flags)}"
    )


@register_generator(InstructionType.RES)
def res_generator(instruction: GbInstruction) -> InstructionFunction:
    value = f"{make_get_code(instruction.second_arg)} & 0b{0xFF - (1 << instruction.first_arg.value):08b}"
    return make_instruction_function(instruction, make_set_code_from_value(instruction.second_arg, value, nb_bytes=1))


@register_generator(InstructionType.SET)
def set_generator(instruction: GbInstruction) -> InstructionFunction:
    value = f"{make_get_code(instruction.second_arg)} | 0b{1 << instruction.first_arg.value:08b}"
    return make_instruction_function(instruction, make_set_code_from_value(instruction.second_arg, value, nb_bytes=1))


@register_generator(InstructionType.STOP)
def stop_generator(instruction: GbInstruction) -> InstructionFunction:
    return make_instruction_function(instruction, f"{REGISTERS_STOPPED} = true;")


@register_generator(InstructionType.RETI)
def reti_generator(instruction: GbInstruction) -> InstructionFunction:
    code = f"{REGISTERS_IME_FLAG} = true;\n" \
           f"{REGISTERS_PROGRAM_COUNTER} = {make_get_from_address(REGISTERS_STACK_POINTER + '++')};\n" \
           f"{REGISTERS_PROGRAM_COUNTER} += ({make_get_from_address(REGISTERS_STACK_POINTER + '++')}) << 8;"

    return make_instruction_function(instruction, code, remove_pc_update=True)


def main():
    instructions = read_instruction_csv(os.path.join(THIS_FOLDER, "instructions.csv"))
    functions = [
        GENERATORS[instruction.type_](instruction)
        for instruction in instructions if instruction.type_ in GENERATORS
    ]

    with open(INCLUDE_FILE, "w") as f:
        f.write(INCLUDE_HEADER)
        code = f"{ARGUMENT_STRUCT}\n{ARGUMENT_ENUM}\n{INCLUDE_USING_DEFS}\n\n"
        code += "\n\n".join(func.declaration for func in functions)
        code += f"\n\nconst std::array<{INSTRUCTION_FUNCTION_TYPE}, {len(functions)}> INSTRUCTION_FUNCTIONS = {{\n"
        code += indent_code("&" + ",\n&".join(func.name for func in functions))
        code += "\n};"
        code += f"\n\nconst std::array<{ARGUMENT_ENUM_NAME}, {len(functions)}> INSTRUCTION_ARGUMENT_TYPES = {{\n"
        code += indent_code(",\n".join(f"{ARGUMENT_ENUM_NAME}::{func.argument_type}" for func in functions))
        code += "\n};"
        f.write(put_code_in_namespace(code, NAMESPACE))

    with open(SRC_FILE, "w") as f:
        f.write(SRC_HEADER)
        code = f"{SRC_INCLUDE_DEFS}\n" + "\n\n".join(func.definition for func in functions)
        f.write(put_code_in_namespace(code, NAMESPACE))


if __name__ == '__main__':
    main()
