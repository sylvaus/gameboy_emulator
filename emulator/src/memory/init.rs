use crate::cartridge::CGBFlag;
use crate::memory::gbmemory::{DISABLE_BOOT_ROM_ADDRESS, GBMemory};
use crate::memory::registers::Registers;
use crate::memory::Memory;

struct RegisterInitValues {
    pub a: u8,
    pub flags: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

/// Information from: https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
const DMG_REGISTER_INIT_VALUES: RegisterInitValues = RegisterInitValues {
    a: 0x01,
    flags: 0x0,
    b: 0xFF,
    c: 0x13,
    d: 0x00,
    e: 0xC1,
    h: 0x84,
    l: 0x03,
    pc: 0x0100,
    sp: 0xFFFE,
};

/// Information from: https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
const CGB_REGISTER_INIT_VALUES: RegisterInitValues = RegisterInitValues {
    a: 0x11,
    flags: 0x80,
    b: 0x00,
    c: 0x00,
    d: 0xFF,
    e: 0x56,
    h: 0x00,
    l: 0x0D,
    pc: 0x0100,
    sp: 0xFFFE,
};

pub fn init_registers(cgb_flag: CGBFlag, registers: &mut Registers) {
    let init_values = match cgb_flag {
        CGBFlag::NonCgbCompatible => DMG_REGISTER_INIT_VALUES,
        CGBFlag::CgbCompatible | CGBFlag::CgbOnly => CGB_REGISTER_INIT_VALUES,
    };

    registers.a = init_values.a;
    registers.flags = init_values.flags;
    registers.b = init_values.b;
    registers.c = init_values.c;
    registers.d = init_values.d;
    registers.e = init_values.e;
    registers.h = init_values.h;
    registers.l = init_values.l;
    registers.pc = init_values.pc;
    registers.sp = init_values.sp;
}

/// Information from: https://gbdev.io/pandocs/Power_Up_Sequence.html#hardware-registers
const DMG_INIT_MEMORY_VALUES: &[(u16, u8)] = &[
    (0xFF00, 0xCF), // P1
    (0xFF01, 0x00), // SB
    (0xFF02, 0x7E), // SC
    (0xFF04, 0x18), // DIV
    (0xFF05, 0x00), // TIMA
    (0xFF06, 0x00), // TMA
    (0xFF07, 0xF8), // TAC
    (0xFF0F, 0xE1), // IF
    (0xFF10, 0x80), // NR10
    (0xFF11, 0xBF), // NR11
    (0xFF12, 0xF3), // NR12
    (0xFF13, 0xFF), // NR13
    (0xFF14, 0xBF), // NR14
    (0xFF16, 0x3F), // NR21
    (0xFF17, 0x00), // NR22
    (0xFF18, 0xFF), // NR23
    (0xFF19, 0xBF), // NR24
    (0xFF1A, 0x7F), // NR30
    (0xFF1B, 0xFF), // NR31
    (0xFF1C, 0x9F), // NR32
    (0xFF1D, 0xFF), // NR33
    (0xFF1E, 0xBF), // NR34
    (0xFF20, 0xFF), // NR41
    (0xFF21, 0x00), // NR42
    (0xFF22, 0x00), // NR43
    (0xFF23, 0xBF), // NR44
    (0xFF24, 0x77), // NR50
    (0xFF25, 0xF3), // NR51
    (0xFF26, 0xF1), // NR52
    (0xFF40, 0x91), // LCDC
    (0xFF41, 0x81), // STAT
    (0xFF42, 0x00), // SCY
    (0xFF43, 0x00), // SCX
    (0xFF44, 0x91), // LY
    (0xFF45, 0x00), // LYC
    (0xFF46, 0xFF), // DMA
    (0xFF47, 0xFC), // BGP
    (0xFF48, 0x00), // OBP0
    (0xFF49, 0x00), // OBP1
    (0xFF4A, 0x00), // WY
    (0xFF4B, 0x00), // WX
    (0xFF4D, 0xFF), // KEY1
    (0xFF4F, 0xFF), // VBK
    (0xFF51, 0xFF), // HDMA1
    (0xFF52, 0xFF), // HDMA2
    (0xFF53, 0xFF), // HDMA3
    (0xFF54, 0xFF), // HDMA4
    (0xFF55, 0xFF), // HDMA5
    (0xFF56, 0xFF), // RP
    (0xFF68, 0xFF), // BCPS
    (0xFF69, 0xFF), // BCPD
    (0xFF6A, 0xFF), // OCPS
    (0xFF6B, 0xFF), // OCPD
    (0xFF70, 0xFF), // SVBK
    (0xFFFF, 0x00), // IE
];

/// Information from: https://gbdev.io/pandocs/Power_Up_Sequence.html#hardware-registers
const CGB_INIT_MEMORY_VALUES: &[(u16, u8)] = &[
    (0xFF00, 0xCF), // P1
    (0xFF01, 0x00), // SB
    (0xFF02, 0x7F), // SC
    (0xFF04, 0x18), // DIV
    (0xFF05, 0x00), // TIMA
    (0xFF06, 0x00), // TMA
    (0xFF07, 0xF8), // TAC
    (0xFF0F, 0xE1), // IF
    (0xFF10, 0x80), // NR10
    (0xFF11, 0xBF), // NR11
    (0xFF12, 0xF3), // NR12
    (0xFF13, 0xFF), // NR13
    (0xFF14, 0xBF), // NR14
    (0xFF16, 0x3F), // NR21
    (0xFF17, 0x00), // NR22
    (0xFF18, 0xFF), // NR23
    (0xFF19, 0xBF), // NR24
    (0xFF1A, 0x7F), // NR30
    (0xFF1B, 0xFF), // NR31
    (0xFF1C, 0x9F), // NR32
    (0xFF1D, 0xFF), // NR33
    (0xFF1E, 0xBF), // NR34
    (0xFF20, 0xFF), // NR41
    (0xFF21, 0x00), // NR42
    (0xFF22, 0x00), // NR43
    (0xFF23, 0xBF), // NR44
    (0xFF24, 0x77), // NR50
    (0xFF25, 0xF3), // NR51
    (0xFF26, 0xF1), // NR52
    (0xFF40, 0x91), // LCDC
    (0xFF41, 0x85), // STAT
    (0xFF42, 0x00), // SCY
    (0xFF43, 0x00), // SCX
    (0xFF44, 0x00), // LY
    (0xFF45, 0x00), // LYC
    (0xFF46, 0x00), // DMA
    (0xFF47, 0xFC), // BGP
    (0xFF48, 0x00), // OBP0
    (0xFF49, 0x00), // OBP1
    (0xFF4A, 0x00), // WY
    (0xFF4B, 0x00), // WX
    (0xFF4D, 0xFF), // KEY1
    (0xFF4F, 0xFF), // VBK
    (0xFF51, 0xFF), // HDMA1
    (0xFF52, 0xFF), // HDMA2
    (0xFF53, 0xFF), // HDMA3
    (0xFF54, 0xFF), // HDMA4
    (0xFF55, 0xFF), // HDMA5
    (0xFF56, 0xFF), // RP
    (0xFF68, 0xFF), // BCPS
    (0xFF69, 0xFF), // BCPD
    (0xFF6A, 0xFF), // OCPS
    (0xFF6B, 0xFF), // OCPD
    (0xFF70, 0xFF), // SVBK
    (0xFFFF, 0x00), // IE
];

pub fn init_memory(cgb_flag: CGBFlag, memory: &mut GBMemory) {
    let init_values = match cgb_flag {
        CGBFlag::NonCgbCompatible => DMG_INIT_MEMORY_VALUES,
        CGBFlag::CgbCompatible | CGBFlag::CgbOnly => CGB_INIT_MEMORY_VALUES,
    };

    for (address, value) in init_values.iter().cloned() {
        memory.write(address, value);
    }
    /// Disable boot rom: https://gbdev.io/pandocs/Power_Up_Sequence.html?highlight=ff50#monochrome-models-dmg0-dmg-mgb
    memory.write(DISABLE_BOOT_ROM_ADDRESS, 1);
}
