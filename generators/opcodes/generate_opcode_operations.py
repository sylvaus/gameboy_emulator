import os
from pprint import pprint
from typing import Dict, Callable, List

from gb_opcode import read_opcode_csv, InstructionType, GbOpcode

THIS_FOLDER = os.path.dirname(os.path.abspath(__file__))


GENERATORS: Dict[InstructionType, Callable[[GbOpcode], List[str]]] = {}


def register_generator(instruction: InstructionType):
    def decorator(f):
        if instruction in GENERATORS:
            raise ValueError(
                f"A generator has already been defined for instruction type: {instruction}"
            )
        GENERATORS[instruction] = f
        return f

    return decorator


@register_generator(InstructionType.NOP)
def nop_generator(opcode):
    pass


def main():
    opcodes = read_opcode_csv(os.path.join(THIS_FOLDER, "opcodes.csv"))
    pprint(opcodes)


if __name__ == '__main__':
    main()
