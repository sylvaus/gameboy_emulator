import os
from dataclasses import dataclass
from typing import Dict, Callable, Optional, Tuple, List
from collections import OrderedDict

from gbinstruction import read_instruction_csv, InstructionType, GbInstruction, Argument, ArgumentType, FlagAction
from utils.formatters import indent_code, make_function

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
_ADD_SUB_FLAG = "add_sub_flag"
ZERO_FLAG = "zero_flag"


ARGUMENT_STRUCT_NAME = "Arguments"
ARGUMENT_STRUCT = f"""union {ARGUMENT_STRUCT_NAME}
{{
    uint8_t data[2];
    uint16_t uint16;
    struct 
    {{
        uint8_t uint8;
        uint8_t unused;
    }};
}};"""

ARGUMENT_NAME = "arguments"
REGISTERS = "registers"
MEMORY_CONTROLLER = "controller"
OPCODE_FUNC_ARGUMENTS = f"(const {ARGUMENT_STRUCT_NAME}& {ARGUMENT_NAME}, " \
                        f"emulator::Registers& {REGISTERS}, emulator::MemoryController& {MEMORY_CONTROLLER})"

ARGUMENT_UINT8 = f"{ARGUMENT_NAME}.uint8"
ARGUMENT_UINT16 = f"{ARGUMENT_NAME}.uint16"
REGISTERS_FLAGS = f"{REGISTERS}.F"
REGISTERS_STACK_POINTER = f"{REGISTERS}.SP"
REGISTERS_PROGRAM_COUNTER = f"{REGISTERS}.PC"

REGISTERS_WITH_GETTER_SETTERS = {"AF", "BC", "DE", "HL"}

SRC_HEADER = f"""#include "emulator/{GENERATED_FOLDER_NAME}/{FILE_NAME}.h"\n\n"""

INCLUDE_HEADER = f"""#pragma once\n
#include <cstdint>
#include "emulator/registers.h"
#include "emulator/memory_controller.h"\n\n"""


@dataclass
class InstructionFunction:
    name: str
    declaration: str
    definition: str
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


def make_instruction_function(
        instruction: GbInstruction, code: str, remove_pc_update: bool = False, remove_duration_return: bool = False
) -> InstructionFunction:
    code_lines = code.splitlines(False)
    if not remove_pc_update:
        code_lines.append(f"{REGISTERS_PROGRAM_COUNTER} += {instruction.length};")
    if not remove_duration_return:
        code_lines.append(f"return {instruction.duration};")

    func_name = f"{instruction.type_.value.lower()}_{instruction.value:03x}"
    signature = f"uint16_t {func_name}{OPCODE_FUNC_ARGUMENTS}"
    declaration = f"{signature}; // {instruction.short_repr}"
    definition = make_function(f"{signature} // {instruction.short_repr}", code_lines)

    return InstructionFunction(func_name, declaration, definition, instruction)


def make_get_code(argument: Argument, address_offset: Optional[str] = None):
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
        code = ARGUMENT_UINT8
    elif argument.type_ == ArgumentType.VALUE:
        code = f"{argument.value}"
    elif argument.type_ == ArgumentType.INDICATION:
        code = ""
    else:
        raise RuntimeError(f"Unknown Argument Type for get  {argument.type_}")

    if argument.is_address:
        if address_offset:
            code = f"({code}) + {address_offset}"
        return f"{MEMORY_CONTROLLER}.get({code})"
    return code


def make_set_code(
        dst: Argument, value: Argument,
        dst_address_offset: Optional[str] = None, value_address_offset: Optional[str] = None,
        value_offset: Optional[str] = None,
) -> str:
    code_value = make_get_code(value, value_address_offset)
    if value_offset:
        code_value = f"({code_value}) + {value_offset}"

    if dst.is_address:
        code_address = make_get_code(dst)
        if dst_address_offset:
            code_address = f"({code_address}) + {dst_address_offset}"

        if (not value.is_address) and (value.nb_bytes == 2):
            return f"{MEMORY_CONTROLLER}.set16bits({code_address}, {code_value})"
        return f"{MEMORY_CONTROLLER}.set({code_address}, {code_value})"

    if dst.type_ == ArgumentType.REGISTER:
        if dst.name in REGISTERS_WITH_GETTER_SETTERS:
            return f"{REGISTERS}.set_{dst.name}({code_value})"
        else:
            return f"{REGISTERS}.{dst.name} = {code_value}"
    elif dst.type_ in {
            ArgumentType.IMMEDIATE_8_BITS, ArgumentType.IMMEDIATE_16_BITS, ArgumentType.UNSIGNED_8_BIT,
            ArgumentType.ADDRESS_16_BIT, ArgumentType.PC_INCREMENT_8_BIT, ArgumentType.VALUE, ArgumentType.INDICATION}:
        raise RuntimeError(f"Argument Type cannot be a destination for set {dst.type_}")

    raise RuntimeError(f"Unknown Destination Argument Type {dst.type_}")


def make_add_sub_flag_code(instruction: GbInstruction, is_add: bool) -> Tuple[str, str]:
    two_bytes_op = ((instruction.first_arg.nb_bytes > 1) and (not instruction.first_arg.is_address)) or \
                   ((instruction.second_arg.nb_bytes > 1) and (not instruction.second_arg.is_address))

    carry_max_value = "0xFFFF" if two_bytes_op else "0xFF"
    half_carry_max_value = "0xFFF" if two_bytes_op else "0xF"
    sign = "+" if is_add else "-"

    flag_names = []
    flag_values = []
    if instruction.zero_flag == FlagAction.CUSTOM:
        flag_names.append(ZERO_FLAG)
        flag_values.append(f"uint8_t {ZERO_FLAG} = ((result == 0) << {OFFSET_ZERO_FLAG});")
    if instruction.half_carry_flag == FlagAction.CUSTOM:
        flag_names.append(HALF_CARRY_FLAG)
        flag_values.append(make_half_carry_flag(half_carry_max_value, is_add))
    if instruction.carry_flag == FlagAction.CUSTOM:
        flag_names.append(CARRY_FLAG)
        flag_values.append(make_carry_flag(carry_max_value, is_add))

    flag_value_code = "\n".join(flag_values)
    return (
        f"result & {carry_max_value}",
        f"int32_t lhs = {make_get_code(instruction.first_arg)};\n"
        f"int32_t rhs = {make_get_code(instruction.second_arg)};\n"
        f"int32_t half_result = (lhs & {half_carry_max_value}) {sign} (rhs & {half_carry_max_value});\n"
        f"int32_t result = lhs {sign} rhs;\n"
        f"{flag_value_code}\n"
        f"{make_flag_code(instruction, flag_names)}"
    )


def make_half_carry_flag(half_carry_max_value: str, is_add: bool) -> str:
    flag = f"uint8_t {HALF_CARRY_FLAG}"
    if is_add:
        return f"{flag} = ((half_result > {half_carry_max_value}) << {OFFSET_HALF_CARRY_FLAG});"

    return f"{flag} = ((half_result < 0) << {OFFSET_HALF_CARRY_FLAG});"


def make_carry_flag(carry_max_value: str, is_add: bool) -> str:
    flag = f"uint8_t {CARRY_FLAG}"
    if is_add:
        return f"{flag} = ((result > {carry_max_value}) << {OFFSET_CARRY_FLAG});"

    return f"{flag} = ((result < 0) << {OFFSET_CARRY_FLAG});"


def make_flag_code(instruction: GbInstruction, flags: List[str]) -> str:
    initial_flag = ((instruction.zero_flag == FlagAction.SET) << OFFSET_ZERO_FLAG_VALUE) + \
                   ((instruction.add_sub_flag == FlagAction.SET) << OFFSET_ADD_SUB_FLAG_VALUE) + \
                   ((instruction.half_carry_flag == FlagAction.SET) << OFFSET_HALF_CARRY_FLAG_VALUE) + \
                   ((instruction.carry_flag == FlagAction.SET) << OFFSET_CARRY_FLAG_VALUE)

    current_flag_mask = ((instruction.zero_flag == FlagAction.NONE) << OFFSET_ZERO_FLAG_VALUE) + \
                        ((instruction.add_sub_flag == FlagAction.NONE) << OFFSET_ADD_SUB_FLAG_VALUE) + \
                        ((instruction.half_carry_flag == FlagAction.NONE) << OFFSET_HALF_CARRY_FLAG_VALUE) + \
                        ((instruction.carry_flag == FlagAction.NONE) << OFFSET_CARRY_FLAG_VALUE)

    flag_setting = f"{REGISTERS_FLAGS} &= {current_flag_mask};\n{REGISTERS_FLAGS} |= {initial_flag:08b}"
    if flags:
        flag_setting += " + " + " + ".join(flags)
    return flag_setting + ";"


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

    code = f"{make_set_code(instruction.first_arg, instruction.second_arg, address_offset, address_offset)};"

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


def put_code_in_namespace(code: str) -> str:
    return f"namespace {NAMESPACE}\n{{\n{indent_code(code)}\n}}"


def main():
    instructions = read_instruction_csv(os.path.join(THIS_FOLDER, "instructions.csv"))
    functions = [
        GENERATORS[instruction.type_](instruction)
        for instruction in instructions if instruction.type_ in GENERATORS
    ]

    with open(INCLUDE_FILE, "w") as f:
        f.write(INCLUDE_HEADER)
        code = f"{ARGUMENT_STRUCT}\n\n" + "\n\n".join(func.declaration for func in functions)
        f.write(put_code_in_namespace(code))

    with open(SRC_FILE, "w") as f:
        f.write(SRC_HEADER)
        f.write(put_code_in_namespace("\n\n".join(func.definition for func in functions)))


if __name__ == '__main__':
    main()
