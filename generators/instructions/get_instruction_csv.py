import os
import urllib.request
from html.parser import HTMLParser
from typing import List

from gbinstruction import GbInstruction, InstructionType, FlagAction, SPECIAL_MNEMONIC_MAP, REGISTERS_NAME_SIZES, \
    IMMEDIATE_8_BITS_NAME, IMMEDIATE_16_BITS_NAME, UNSIGNED_8_BIT_NAME, ADDRESS_16_BIT_NAME, PC_INCREMENT_8_BIT_NAME, \
    ArgumentType, Argument, INDICATION_NAME, INSTRUCTION_TYPE_MAP, \
    write_gb_instruction_to_csv

THIS_FOLDER = os.path.dirname(os.path.abspath(__file__))


def _make_argument_from_str(argument_str):
    is_address = "(" in argument_str
    name = argument_str.strip("() ")
    value = None
    nb_bytes = 1
    if name in REGISTERS_NAME_SIZES:
        type_ = ArgumentType.REGISTER
        nb_bytes = REGISTERS_NAME_SIZES[name]
    elif name == IMMEDIATE_8_BITS_NAME:
        type_ = ArgumentType.IMMEDIATE_8_BITS
    elif name == IMMEDIATE_16_BITS_NAME:
        type_ = ArgumentType.IMMEDIATE_16_BITS
        nb_bytes = 2
    elif name == UNSIGNED_8_BIT_NAME:
        type_ = ArgumentType.UNSIGNED_8_BIT
    elif name == ADDRESS_16_BIT_NAME:
        type_ = ArgumentType.ADDRESS_16_BIT
        nb_bytes = 2
    elif name == PC_INCREMENT_8_BIT_NAME:
        type_ = ArgumentType.PC_INCREMENT_8_BIT
    elif name == INDICATION_NAME:
        type_ = ArgumentType.INDICATION
    else:
        type_ = ArgumentType.VALUE
        value = int(name.strip("H"), 16 if "H" in name else 10)

    return Argument(type_, is_address, nb_bytes, name, value)


FLAG_MAP = {"1": FlagAction.SET, "0": FlagAction.RESET, "-": FlagAction.NONE}


def _parse_flag_action(action_str):
    return FLAG_MAP.get(action_str, FlagAction.CUSTOM)


class OpcodeHTMLParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self._in_cell = False
        self._table_nb = 0
        self._col = 0
        self._row = 0
        self._data = []
        self._opcodes: List[GbInstruction] = []

    @property
    def opcodes(self) -> List[GbInstruction]:
        return self._opcodes

    def handle_starttag(self, tag, attrs):
        if tag == "table":
            self._col = 0
            self._row = 0
        if tag == "td":
            self._in_cell = True
            self._data = []

    def handle_endtag(self, tag):
        if tag == "table":
            self._table_nb += 1
        if tag == "tr":
            self._row += 1
            self._col = 0
        if tag == "td":
            if self._is_opcode_cell():
                self._handle_opcode()

            self._col += 1
            self._in_cell = False

    def handle_data(self, data):
        if self._is_opcode_cell():
            self._data.append(data)

    def _is_opcode_cell(self):
        return self._col > 0 and self._row > 0 and self._in_cell and self._table_nb < 2

    def _handle_opcode(self):
        value = self._table_nb * 0x100 + (self._row - 1) * 0x10 + (self._col - 1)
        if len(self._data) != 3:
            self._opcodes.append(GbInstruction(
                value, InstructionType.UNKNOWN, None, None, 1, 1, 1
                , FlagAction.NONE, FlagAction.NONE, FlagAction.NONE, FlagAction.NONE
            ))
            return

        name, *argument_strs = self._data[0].strip().replace(",", " ").split()
        argument_strs = tuple(map(str.strip, argument_strs))
        if (name, argument_strs) in SPECIAL_MNEMONIC_MAP:
            name, argument_strs = SPECIAL_MNEMONIC_MAP[name, argument_strs]

        type_ = INSTRUCTION_TYPE_MAP[name]
        arguments = {i: _make_argument_from_str(argument_str) for i, argument_str in enumerate(argument_strs)}

        length, *durations = map(int, self._data[1].strip().replace("/", " ").split())
        if len(durations) == 2:
            duration, duration_no_action = durations
        else:
            duration, duration_no_action = durations[0], durations[0]

        z_flag, n_flag, h_flag, c_flag = map(
            lambda action_str: _parse_flag_action(action_str.strip()),
            self._data[2].split()
        )

        # Issue in the source
        if value in {0xE2, 0xF2}:
            length = 1

        self._opcodes.append(GbInstruction(
            value, type_, arguments.get(0), arguments.get(1),
            length, duration, duration_no_action,
            z_flag, n_flag, h_flag, c_flag
        ))

    def error(self, message):
        pass


def main():
    with urllib.request.urlopen("https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html") as f:
        text = f.read().decode()
        text = text.replace("JP (HL)", "JP HL")  # wrong value
        parser = OpcodeHTMLParser()
        parser.feed(text)

    write_gb_instruction_to_csv(os.path.join(THIS_FOLDER, "instructions.csv"), parser.opcodes)


if __name__ == '__main__':
    main()
