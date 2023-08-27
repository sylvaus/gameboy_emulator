use macros::{BitAccessor};

#[derive(BitAccessor)]
struct Value{
    #[bit_offset_size(value_lower_bits, 0, 4)]
    #[bit_offset_size(value_higher_bits, 4, 4)]
    pub value: u8,
    #[bit_offset_size(other_value_lower_bits, 0, 4)]
    #[bit_offset_size(other_value_higher_bits, 4, 4)]
    pub other_value: u8,
}

#[test]
fn bit_accessor_read() {
    let result = Value {value: 0b10101110, other_value: 0b11000011 };

    assert_eq!(result.read_value_lower_bits(), 0b1110);
    assert_eq!(result.read_value_higher_bits(), 0b1010);
    assert_eq!(result.read_other_value_lower_bits(), 0b0011);
    assert_eq!(result.read_other_value_higher_bits(), 0b1100);
}

#[test]
fn bit_accessor_write() {
    let mut result = Value {value: 0b10101110, other_value: 0b11000011 };

    result.write_value_lower_bits(0b0001);
    assert_eq!(result.value, 0b10100001);
    result.write_value_higher_bits(0b0101);
    assert_eq!(result.value, 0b01010001);
    assert_eq!(result.other_value, 0b11000011);
    result.write_other_value_lower_bits(0b0001);
    assert_eq!(result.other_value, 0b11000001);
    result.write_other_value_higher_bits(0b1111);
    assert_eq!(result.other_value, 0b11110001);
}