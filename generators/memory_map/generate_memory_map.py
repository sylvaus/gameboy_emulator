import csv
from dataclasses import dataclass


@dataclass
class Element:
    name: str
    address: int
    bit_size: int
    container: str


def make_element_from_csv(row: dict):
    return Element(row["name"], int(row["address"]), int(row['bit_size']), row["container"])


def make_getter_setter(element: Element, base_container: str, base_container_size: int):
    if element.bit_size > base_container_size:
        if (element.bit_size % base_container_size) != 0 or (element.address % base_container_size) != 0:
            raise ValueError(
                f"Invalid Element: {element}"
                "Elements bigger than base container size are only supported if aligned on a multiple of "
                "the base container size and their size is a multiple of the base container size"
            )

        return make_getter_setter_large_element(element, base_container, base_container_size)

    return get_getter_setter_small_element(element, base_container, base_container_size)


def make_getter_setter_large_element(element: Element, base_container: str, base_container_size: int):
    multiple = element.bit_size // base_container_size

    # TODO


def get_getter_setter_small_element(element: Element, base_container: str, base_container_size: int):

    # TODO


def make_memory_map_struct(filepath: str, struct_name: str, base_container: str, base_container_size: int):
    with open(filepath, newline='') as csvfile:
        reader = csv.DictReader(csvfile)



