use crate::memory::mbc::interface::MemoryBankController;
use crate::memory::mbc::mbc3::MBC3BankController;
use crate::memory::mbc::no_controller::NoMemoryBankController;
use macros::AddEnumName;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::{error, fmt};

/// Information from: https://gbdev.io/pandocs/The_Cartridge_Header.html#the-cartridge-header
const CARTRIDGE_HEADER_SIZE: usize = 0x14F + 1;
const ADDRESS_TITLE: usize = 0x134;
const SIZE_TITLE: usize = 0x10;
const ADDRESS_CGB_FLAG: usize = 0x143;
const ADDRESS_CARTRIDGE_TYPE: usize = 0x147;
const ADDRESS_ROM_SIZE: usize = 0x148;
const ADDRESS_RAM_SIZE: usize = 0x149;
const ADDRESS_HEADER_CHECKSUM: usize = 0x14D;

pub fn load_cartridge(path: &Path) -> Result<Cartridge, Box<dyn error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut cartridge_header = [0u8; CARTRIDGE_HEADER_SIZE];
    reader.read_exact(&mut cartridge_header)?;

    let title = get_title(&cartridge_header)?;
    let cgb_flag = get_cgb_flag(&cartridge_header);
    let info = get_cartridge_info(&cartridge_header)?;
    let rom_info = get_rom_size(&cartridge_header)?;
    let ram_info = get_ram_size(&cartridge_header)?;
    let header_checksum = get_header_checksum(&cartridge_header);
    let computed_header_checksum = compute_header_checksum(&cartridge_header);
    let memory_controller = get_memory_controller(&info, &mut reader, &rom_info, &ram_info)?;

    Ok(Cartridge {
        title,
        cgb_flag,
        info,
        rom_info,
        ram_info,
        valid_header_checksum: header_checksum == computed_header_checksum,
        memory_controller,
    })
}

pub struct Cartridge {
    pub title: String,
    pub cgb_flag: CGBFlag,
    pub info: CartridgeInfo,
    pub rom_info: ROMSizeInfo,
    pub ram_info: RAMSizeInfo,
    pub valid_header_checksum: bool,
    pub memory_controller: Box<dyn MemoryBankController>,
}

impl Display for Cartridge {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Cartridge(title: {:?}, cgb_flag: {:?}, info: {:?}, rom: {:?}, ram: {:?}, valid checksum: {:?})",
            self.title, self.cgb_flag, self.info, self.rom_info, self.ram_info, self.valid_header_checksum
        )
    }
}

fn get_title(rom: &[u8]) -> Result<String, String> {
    let complete_title = &rom[ADDRESS_TITLE..ADDRESS_TITLE + SIZE_TITLE];
    let end = complete_title
        .iter()
        .position(|b| b == &0)
        .unwrap_or(SIZE_TITLE);
    String::from_utf8(complete_title[..end].to_vec())
        .map_err(|e| format!("Title is badly formatted: {:?}", e))
}

#[derive(Clone, Debug, PartialEq, AddEnumName)]
pub enum MBCType {
    RomOnly,
    MBC1,
    MBC2,
    MBC3,
    MBC4,
    MBC5,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CartridgeInfo {
    pub bank_type: MBCType,
    pub sram: bool,
    pub battery: bool,
    pub rtc: bool,
}

impl CartridgeInfo {
    pub fn new(bank_type: MBCType, sram: bool, battery: bool, rtc: bool) -> Self {
        Self {
            bank_type,
            sram,
            battery,
            rtc,
        }
    }
}

/// Information from: https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type
fn get_cartridge_info(rom: &[u8]) -> Result<CartridgeInfo, String> {
    match rom[ADDRESS_CARTRIDGE_TYPE] {
        0x00 => Ok(CartridgeInfo::new(MBCType::RomOnly, false, false, false)), // ROM
        0x01 => Ok(CartridgeInfo::new(MBCType::MBC1, false, false, false)),    // MBC1
        0x02 => Ok(CartridgeInfo::new(MBCType::MBC1, true, false, false)),     // MBC1+RAM
        0x03 => Ok(CartridgeInfo::new(MBCType::MBC1, true, true, false)),      // MBC1+RAM+BATT
        0x05 => Ok(CartridgeInfo::new(MBCType::MBC2, false, false, false)),    // MBC2
        0x06 => Ok(CartridgeInfo::new(MBCType::MBC2, false, true, false)),     // MBC2+BATTERY
        0x08 => Ok(CartridgeInfo::new(MBCType::RomOnly, true, false, false)),  // ROM+RAM
        0x09 => Ok(CartridgeInfo::new(MBCType::RomOnly, true, true, false)),   // ROM+RAM+BATTERY
        0x0F => Ok(CartridgeInfo::new(MBCType::MBC3, false, true, true)),      // MBC3+TIMER+BATT
        0x10 => Ok(CartridgeInfo::new(MBCType::MBC3, true, true, true)), // MBC3+TIMER+RAM+BATT
        0x11 => Ok(CartridgeInfo::new(MBCType::MBC3, false, false, false)), // MBC3
        0x12 => Ok(CartridgeInfo::new(MBCType::MBC3, true, false, false)), // MBC3+RAM
        0x13 => Ok(CartridgeInfo::new(MBCType::MBC3, true, true, false)), // MBC3+RAM+BATT
        0x15 => Ok(CartridgeInfo::new(MBCType::MBC4, false, false, false)), // MBC3
        0x16 => Ok(CartridgeInfo::new(MBCType::MBC4, true, false, false)), // MBC3+RAM
        0x17 => Ok(CartridgeInfo::new(MBCType::MBC4, true, true, false)), // MBC3+RAM+BATT
        0x19 => Ok(CartridgeInfo::new(MBCType::MBC5, false, false, false)), // MBC5
        0x1A => Ok(CartridgeInfo::new(MBCType::MBC5, true, false, false)), // MBC5+RAM
        0x1B => Ok(CartridgeInfo::new(MBCType::MBC5, true, true, false)), // MBC5+RAM+BATT
        0x1C => Ok(CartridgeInfo::new(MBCType::MBC5, false, false, false)), // MBC5+RUMBLE
        0x1D => Ok(CartridgeInfo::new(MBCType::MBC5, true, false, false)), // MBC5+RUMBLE+RAM
        0x1E => Ok(CartridgeInfo::new(MBCType::MBC5, true, true, false)), // MBC5+RUMBLE+RAM+BATT
        value => Err(format!("Unknown cartridge type value {:?}", value)),
    }
}

/// Source: https://gbdev.io/pandocs/#_0148-rom-size
#[derive(Clone, Debug, PartialEq, AddEnumName)]
pub enum ROMSize {
    S32KB,
    S64KB,
    S128KB,
    S256KB,
    S512KB,
    S1MB,
    S2MB,
    S4MB,
    S8MB,
    S1MB100KB,
    S1MB200KB,
    S1MB500KB,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ROMSizeInfo {
    pub rom_size: ROMSize,
    pub num_banks: usize,
}

impl ROMSizeInfo {
    pub fn new(rom_size: ROMSize, num_banks: usize) -> Self {
        Self {
            rom_size,
            num_banks,
        }
    }
}

/// Information from: https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type
fn get_rom_size(rom: &[u8]) -> Result<ROMSizeInfo, String> {
    match rom[ADDRESS_ROM_SIZE] {
        0x00 => Ok(ROMSizeInfo::new(ROMSize::S32KB, 2)),
        0x01 => Ok(ROMSizeInfo::new(ROMSize::S64KB, 4)),
        0x02 => Ok(ROMSizeInfo::new(ROMSize::S128KB, 8)),
        0x03 => Ok(ROMSizeInfo::new(ROMSize::S256KB, 16)),
        0x04 => Ok(ROMSizeInfo::new(ROMSize::S512KB, 32)),
        0x05 => Ok(ROMSizeInfo::new(ROMSize::S1MB, 64)),
        0x06 => Ok(ROMSizeInfo::new(ROMSize::S2MB, 128)),
        0x07 => Ok(ROMSizeInfo::new(ROMSize::S4MB, 256)),
        0x08 => Ok(ROMSizeInfo::new(ROMSize::S8MB, 512)),
        0x52 => Ok(ROMSizeInfo::new(ROMSize::S1MB100KB, 72)),
        0x53 => Ok(ROMSizeInfo::new(ROMSize::S1MB200KB, 80)),
        0x54 => Ok(ROMSizeInfo::new(ROMSize::S1MB500KB, 96)),
        value => Err(format!("Unknown rom size value {:?}", value)),
    }
}

/// Source: https://gbdev.io/pandocs/#_0149-ram-size
#[derive(Clone, Debug, PartialEq, AddEnumName)]
pub enum RAMSize {
    NONE,
    S2KB,
    S8KB,
    S32KB,
    S128KB,
    S64KB,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RAMSizeInfo {
    pub ram_size: RAMSize,
    pub num_banks: usize,
}

impl RAMSizeInfo {
    pub fn new(ram_size: RAMSize, num_banks: usize) -> Self {
        Self {
            ram_size,
            num_banks,
        }
    }
}

/// Information from: https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type
fn get_ram_size(rom: &[u8]) -> Result<RAMSizeInfo, String> {
    match rom[ADDRESS_RAM_SIZE] {
        0x00 => Ok(RAMSizeInfo::new(RAMSize::NONE, 0)),
        0x01 => Ok(RAMSizeInfo::new(RAMSize::S2KB, 1)),
        0x02 => Ok(RAMSizeInfo::new(RAMSize::S8KB, 1)),
        0x03 => Ok(RAMSizeInfo::new(RAMSize::S32KB, 4)),
        0x04 => Ok(RAMSizeInfo::new(RAMSize::S128KB, 16)),
        0x05 => Ok(RAMSizeInfo::new(RAMSize::S64KB, 8)),
        value => Err(format!("Unknown ram size value {:?}", value)),
    }
}

fn get_header_checksum(rom: &[u8]) -> u8 {
    rom[ADDRESS_HEADER_CHECKSUM]
}

fn compute_header_checksum(rom: &[u8]) -> u8 {
    let mut result = 0u8;
    for value in &rom[ADDRESS_TITLE..ADDRESS_HEADER_CHECKSUM] {
        result = result.wrapping_sub(value.wrapping_add(1));
    }
    result
}

fn get_memory_controller(
    info: &CartridgeInfo,
    rom_reader: &mut BufReader<File>,
    rom_info: &ROMSizeInfo,
    ram_info: &RAMSizeInfo,
) -> Result<Box<dyn MemoryBankController>, String> {
    match &info.bank_type {
        MBCType::RomOnly => NoMemoryBankController::new(rom_reader, ram_info.num_banks),
        MBCType::MBC1 => {
            MBC3BankController::new(rom_reader, rom_info.num_banks, ram_info.num_banks)
        }
        MBCType::MBC3 => {
            MBC3BankController::new(rom_reader, rom_info.num_banks, ram_info.num_banks)
        }
        value => Err(format!("Unsupported bank type {:?}", value.get_name())),
    }
}

/// Source: https://gbdev.io/pandocs/The_Cartridge_Header.html#0143--cgb-flag
#[derive(Clone, Debug, PartialEq, AddEnumName)]
pub enum CGBFlag {
    NonCgbCompatible,
    CgbCompatible,
    CgbOnly,
}

fn get_cgb_flag(rom: &[u8]) -> CGBFlag {
    match rom[ADDRESS_CGB_FLAG] {
        0x80 => CGBFlag::CgbCompatible,
        0xC0 => CGBFlag::CgbOnly,
        _ => CGBFlag::NonCgbCompatible,
    }
}
