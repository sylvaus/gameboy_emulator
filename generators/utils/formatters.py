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
    indent: str = INDENT * nb_indent
    return f"\n".join(
        indent + line if line.strip() else ""
        for line in code
    )


def make_if_else_code(condition: str, action: str, else_action: str):
    return f"if ({condition})\n" \
           f"{{\n{indent_code(action)}\n}}\n" \
           f"else\n" \
           f"{{\n{indent_code(else_action)}\n}}"


def make_function(signature: str, code: Union[str, List[str]], parameters: Iterable[str] = ()) -> str:
    indented_code = indent_code(code).replace("\n\n", "\n")  # Remove double empty line inside function
    # Remove unused parameters
    for parameter in parameters:
        if parameter not in indented_code:
            signature = signature.replace(parameter, "")

    return f"{signature}\n{{\n{indented_code}\n}}"


def put_code_in_namespace(code: str, namespace: str) -> str:
    return f"namespace {namespace}\n{{\n{indent_code(code)}\n}}"
