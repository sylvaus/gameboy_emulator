pub enum Argument {
    None,
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
}

impl Argument {
    pub fn new_empty() -> Self {
        Self::None
    }

    pub fn new_u8(value: u8) -> Self {
        Self::Uint8(value)
    }
    pub fn new_i8(value: i8) -> Self {
        Self::Int8(value)
    }
    pub fn new_u16(value: u16) -> Self {
        Self::Uint16(value)
    }

    pub fn get(&self) -> u8 {
        if let Self::Uint8(value) = &self {
            *value
        } else {
            panic!("Argument is not u8 but u8 getter was used")
        }
    }
    pub fn get_signed(&self) -> i8 {
        if let Self::Int8(value) = &self {
            *value
        } else {
            panic!("Argument is not u8 but u8 getter was used")
        }
    }
    pub fn get_16_bits(&self) -> u16 {
        if let Self::Uint16(value) = &self {
            *value
        } else {
            panic!("Argument is not u8 but u8 getter was used")
        }
    }
}
