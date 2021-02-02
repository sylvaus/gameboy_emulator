from functools import singledispatch
from typing import List, Union, Iterable

from generators.utils.constants import INDENT


@singledispatch
def indent_code(code, nb_indent: int = 1) -> str:
    raise NotImplementedError


@indent_code.register
def _(code: str, nb_indent: int = 1) -> str:
    return indent_code(code.splitlines(False), nb_indent)


@indent_code.register
def _(code: list, nb_indent: int = 1) -> str:
    indent = INDENT * nb_indent
    return indent + f"\n{indent}".join(code)


def make_function(signature: str, code: Union[str, List[str]], parameters: Iterable[str] = ()) -> str:
    indented_code = indent_code(code)

    # Remove unused parameters
    for parameter in parameters:
        if parameter not in indented_code:
            signature = signature.replace(parameter, "")

    return f"{signature}\n{{\n{indented_code}\n}}"
