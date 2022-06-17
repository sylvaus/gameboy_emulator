#pragma once

#include <cstdint>
#include <array>
#include <unordered_map>
#include <string>

#include "emulator/utils/map.h"

namespace emulator::memory
{
    using utils::UnMutableDefaultMap;

    enum class CGBFlag
    {
        UNKNOWN,
        CGB_SUPPORTED,
        CGB_ONLY,
    };
    const UnMutableDefaultMap<uint8_t, CGBFlag> CGBFLAG_MAP
    { CGBFlag::UNKNOWN, {
         {0x80, CGBFlag::CGB_SUPPORTED},
         {0xC0, CGBFlag::CGB_ONLY},
    }};

    enum class SGBFlag
    {
        UNKNOWN,
        SGB_NOT_SUPPORTED,
        SGB_SUPPORTED,
    };
    const UnMutableDefaultMap<uint8_t, SGBFlag> SGBFLAG_MAP
    { SGBFlag::UNKNOWN, {
         {0x00, SGBFlag::SGB_NOT_SUPPORTED},
         {0x30, SGBFlag::SGB_SUPPORTED},
    }};

    enum class MemoryBankType
    {
        UNKNOWN,
        ROM_ONLY,
        MBC1,
        MBC2,
        MBC3,
        MBC4,
        MBC5,
    };
    const UnMutableDefaultMap<MemoryBankType, std::string> MEMORY_BANK_NAME_MAP
    { "Unknown Memory Bank", {
        {MemoryBankType::ROM_ONLY, "Rom Only"},
        {MemoryBankType::MBC1, "Memory Bank Controller 1"},
        {MemoryBankType::MBC2, "Memory Bank Controller 2"},
        {MemoryBankType::MBC3, "Memory Bank Controller 3"},
        {MemoryBankType::MBC4, "Memory Bank Controller 4"},
        {MemoryBankType::MBC5, "Memory Bank Controller 5"},
    }};

    struct CartridgeType
    {
        MemoryBankType type;
        bool sram;
        bool battery;
        bool rtc;
    };
    const UnMutableDefaultMap<uint8_t, CartridgeType> CARTRIDGE_TYPE_MAP
    { {MemoryBankType::UNKNOWN, false, false, false}, {
        {0x00, {MemoryBankType::ROM_ONLY, false, false, false}}, // ROM
        {0x01, {MemoryBankType::MBC1, false, false, false}},  // MBC1
        {0x02, {MemoryBankType::MBC1, true, false, false}},  // MBC1+RAM
        {0x03, {MemoryBankType::MBC1, true, true, false}},  // MBC1+RAM+BATT
        {0x05, {MemoryBankType::MBC2, false, false, false}},  // MBC2
        {0x06, {MemoryBankType::MBC2, false, true, false}},  // MBC2+BATTERY
        {0x08, {MemoryBankType::ROM_ONLY, true, false, false}}, // ROM+RAM
        {0x09, {MemoryBankType::ROM_ONLY, true, true, false}}, // ROM+RAM+BATTERY
        {0x0F, {MemoryBankType::MBC3, false, true, true}},   // MBC3+TIMER+BATT
        {0x10, {MemoryBankType::MBC3, true, true, true}},   // MBC3+TIMER+RAM+BATT
        {0x11, {MemoryBankType::MBC3, false, false, false}},  // MBC3
        {0x12, {MemoryBankType::MBC3, true, false, false}},  // MBC3+RAM
        {0x13, {MemoryBankType::MBC3, true, true, false}},  // MBC3+RAM+BATT
        {0x15, {MemoryBankType::MBC4, false, false, false}},  // MBC3
        {0x16, {MemoryBankType::MBC4, true, false, false}},  // MBC3+RAM
        {0x17, {MemoryBankType::MBC4, true, true, false}},  // MBC3+RAM+BATT
        {0x19, {MemoryBankType::MBC5, false, false, false}},  // MBC5
        {0x1A, {MemoryBankType::MBC5, true, false, false}},  // MBC5+RAM
        {0x1B, {MemoryBankType::MBC5, true, true, false}},  // MBC5+RAM+BATT
        {0x1C, {MemoryBankType::MBC5, false, false, false}},  // MBC5+RUMBLE
        {0x1D, {MemoryBankType::MBC5, true, false, false}},  // MBC5+RUMBLE+RAM
        {0x1E, {MemoryBankType::MBC5, true, true, false}},  // MBC5+RUMBLE+RAM+BATT
    }};

    /// Source: https://gbdev.io/pandocs/#_0148-rom-size
    enum class ROMSize
    {
        UNKNOWN,
        S32KB,
        S64KB,
        S128KB,
        S256KB,
        S512KB,
        S1MB,
        S2MB,
        S4MB,
        S1MB100KB,
        S1MB200KB,
        S1MB500KB,
    };
    const UnMutableDefaultMap<uint8_t, ROMSize> ROM_SIZE_MAP
    { ROMSize::UNKNOWN, {
         {0x00, ROMSize::S32KB},
         {0x01, ROMSize::S64KB},
         {0x02, ROMSize::S128KB},
         {0x03, ROMSize::S256KB},
         {0x04, ROMSize::S512KB},
         {0x05, ROMSize::S1MB},
         {0x06, ROMSize::S2MB},
         {0x07, ROMSize::S4MB},
         {0x52, ROMSize::S1MB100KB},
         {0x53, ROMSize::S1MB200KB},
         {0x54, ROMSize::S1MB500KB},
    }};
    const UnMutableDefaultMap<ROMSize, std::string> ROM_SIZE_NAME_MAP
    { "Unknown ROM size", {
        {ROMSize::S32KB, "32 KBytes"},
        {ROMSize::S64KB, "64 KBytes"},
        {ROMSize::S128KB, "128 KBytes"},
        {ROMSize::S256KB, "256 KBytes"},
        {ROMSize::S512KB, "512 KBytes"},
        {ROMSize::S1MB, "1 MBytes"},
        {ROMSize::S2MB, "2 MBytes"},
        {ROMSize::S4MB, "4 MBytes"},
        {ROMSize::S1MB100KB, "1.1 MBytes"},
        {ROMSize::S1MB200KB, "1.2 MBytes"},
        {ROMSize::S1MB500KB, "1.5 MBytes"},
    }};

    const uint16_t INVALID_ROM_BANK_NB = 0xFFFF;
    const UnMutableDefaultMap<ROMSize, uint16_t> NB_ROM_BANK_MAP
    { INVALID_ROM_BANK_NB, {
         {ROMSize::S32KB, 2},
         {ROMSize::S64KB, 4},
         {ROMSize::S128KB, 8},
         {ROMSize::S256KB, 16},
         {ROMSize::S512KB, 32},
         {ROMSize::S1MB, 64},
         {ROMSize::S2MB, 128},
         {ROMSize::S4MB, 256},
         {ROMSize::S1MB100KB, 72},
         {ROMSize::S1MB200KB, 80},
         {ROMSize::S1MB500KB, 96},
    }};

    // Source: https://gbdev.io/pandocs/#_0149-ram-size
    enum class RAMSize
    {
        UNKNOWN,
        NONE,
        S2KB,
        S8KB,
        S32KB,
        S128KB,
        S64KB,
    };
    const UnMutableDefaultMap<uint8_t, RAMSize> RAM_SIZE_MAP
    { RAMSize::UNKNOWN, {
        {0x00, RAMSize::NONE},
        {0x01, RAMSize::S2KB},
        {0x02, RAMSize::S8KB},
        {0x03, RAMSize::S32KB},
        {0x04, RAMSize::S128KB},
        {0x05, RAMSize::S64KB},
    }};
    const UnMutableDefaultMap<RAMSize, std::string> RAM_SIZE_NAME_MAP
    { "Unknown Ram Size", {
        {RAMSize::NONE, "No external ram"},
        {RAMSize::S2KB, "2KB External ram"},
        {RAMSize::S8KB, "8KB External ram"},
        {RAMSize::S32KB, "32KB External ram"},
        {RAMSize::S8KB, "128KB External ram"},
        {RAMSize::S64KB, "64KB External ram"},
    }};

    const uint8_t INVALID_RAM_BANK_NB = 0xFF;
    const UnMutableDefaultMap<RAMSize, uint8_t> NB_RAM_BANK_MAP
    { INVALID_RAM_BANK_NB, {
        {RAMSize::NONE, 0},
        {RAMSize::S2KB, 1},
        {RAMSize::S8KB, 1},
        {RAMSize::S32KB, 4},
        {RAMSize::S128KB, 16},
        {RAMSize::S64KB, 8},
    }};

    enum class DESTINATION_CODE
    {
        UNKNOWN,
        JAPANESE,
        NON_JAPANESE,
    };
    const UnMutableDefaultMap<uint8_t, DESTINATION_CODE> DESTINATION_CODE_MAP
    { DESTINATION_CODE::UNKNOWN, {
         {0x00, DESTINATION_CODE::JAPANESE},
         {0x01, DESTINATION_CODE::NON_JAPANESE},
    }};

    constexpr uint16_t ROM_BANK_SIZE = 0x4000;
    constexpr uint16_t START_ROM_0 = 0x0000;
    constexpr uint16_t END_ROM_0 = 0x3FFF;
    constexpr uint16_t START_ROM_1_N = 0x4000;
    constexpr uint16_t END_ROM_1_N = 0x7FFF;

    constexpr uint16_t VRAM_SIZE = 0x2000;
    constexpr uint16_t START_VRAM = 0x8000;
    constexpr uint16_t END_VRAM = 0x9FFF;

    constexpr uint16_t EXT_RAM_BANK_SIZE = 0x2000;
    constexpr uint16_t START_EXT_RAM = 0xA000;
    constexpr uint16_t END_EXT_RAM = 0xBFFF;

    constexpr uint16_t WORK_RAM_SIZE = 0x1000;
    constexpr uint16_t START_FIRST_WORK_RAM = 0xC000;
    constexpr uint16_t END_FIRST_WORK_RAM = 0xCFFF;
    constexpr uint16_t START_SECOND_WORK_RAM = 0xD000;
    constexpr uint16_t END_SECOND_WORK_RAM = 0xDFFF;

    constexpr uint16_t ECHO_RAM_SIZE = 0x1E00;
    constexpr uint16_t START_ECHO_RAM = 0xE000;
    constexpr uint16_t END_ECHO_RAM = 0xFDFF;

    constexpr uint16_t OAM_SIZE = 0x100;
    constexpr uint16_t START_OAM = 0xFE00;
    constexpr uint16_t END_OAM = 0xFE9F;

    constexpr uint16_t START_NOT_USABLE = 0xFEA0;
    constexpr uint16_t END_NOT_USABLE = 0xFEFF;

    constexpr uint16_t IO_REGISTER_SIZE = 0x80;
    constexpr uint16_t START_IO_REGISTER = 0xFF00;
    constexpr uint16_t END_IO_REGISTER = 0xFF7F;

    // https://gbdev.io/pandocs/Memory_Map.html#io-ranges
    constexpr uint16_t JOYPAD_INPUT_ADDRESS = 0xFF00;

    constexpr uint16_t START_SERIAL_TRANSFER = 0xFF01;
    constexpr uint16_t END_SERIAL_TRANSFER = 0xFF02;

    constexpr uint16_t START_TIME_DIVIDER = 0xFF04;
    constexpr uint16_t END_TIME_DIVIDER = 0xFF07;

    constexpr uint16_t START_SOUND = 0xFF10;
    constexpr uint16_t END_SOUND = 0xFF26;

    constexpr uint16_t START_WAVE_PATTERN = 0xFF30;
    constexpr uint16_t END_WAVE_PATTERN = 0xFF3F;

    constexpr uint16_t START_IO_LCD = 0xFF40;
    constexpr uint16_t LCD_CONTROL_ADDRESS = 0xFF40;
    constexpr uint16_t LCD_STATUS_ADDRESS = 0xFF41;
    constexpr uint16_t LCD_SCROLL_Y_ADDRESS = 0xFF42;
    constexpr uint16_t LCD_SCROLL_X_ADDRESS = 0xFF43;
    constexpr uint16_t LCD_COORDINATE_Y_ADDRESS = 0xFF44;
    constexpr uint16_t LCD_LY_COMPARE_ADDRESS = 0xFF45;
    constexpr uint16_t OAM_DMA_ADDRESS = 0xFF46;
    constexpr uint16_t BGP_PALETTE_DATA_ADDRESS = 0xFF47;
    constexpr uint16_t OBJ_PALETTE_DATA_0_ADDRESS = 0xFF48;
    constexpr uint16_t OBJ_PALETTE_DATA_1_ADDRESS = 0xFF49;
    constexpr uint16_t LCD_WINDOWS_Y_ADDRESS = 0xFF4A;
    constexpr uint16_t LCD_WINDOWS_X_ADDRESS = 0xFF4B;
    constexpr uint16_t END_IO_LCD = 0xFF4B;

    constexpr uint16_t SELECT_VRAM_BANK_ADDRESS = 0xFF4F;

    constexpr uint16_t DISABLE_BOOT_ROM_ADDRESS = 0xFF50;

    constexpr uint16_t START_VRAM_DMA = 0xFF51;
    constexpr uint16_t END_VRAM_DMA = 0xFF55;

    constexpr uint16_t START_BJ_OBJ_PALETTES = 0xFF68;
    constexpr uint16_t END_BJ_OBJ_PALETTES = 0xFF69;

    constexpr uint16_t SELECT_WRAM_BANK_ADDRESS = 0xFF70;

    // End of io ranges

    constexpr uint16_t HRAM_SIZE = 0x100;
    constexpr uint16_t START_HRAM = 0xFF80;
    constexpr uint16_t END_HRAM = 0xFFFE;

    constexpr uint16_t INTERRUPT_ENABLE_ADDRESS = 0xFFFF;

    using RomBank = std::array<uint8_t, ROM_BANK_SIZE>;
    using VideoRam = std::array<uint8_t, VRAM_SIZE>;
    using RamBank = std::array<uint8_t, EXT_RAM_BANK_SIZE>;
    using WorkRamBank = std::array<uint8_t, WORK_RAM_SIZE>;
    using EchoRamBank = std::array<uint8_t, 0x1E00>;
    using SpriteAttributeTable = std::array<uint8_t, 0xA0>;
    using HighRam = std::array<uint8_t, 0x7F>;

    constexpr uint16_t ADDRESS_ENTRY_POINT = 0x100;
    constexpr uint16_t ADDRESS_LOGO = 0x104;
    constexpr uint8_t  SIZE_LOGO = 0x30;
    constexpr uint16_t ADDRESS_TITLE = 0x134;
    constexpr uint8_t SIZE_TITLE = 0x10;
    constexpr uint16_t ADDRESS_SHORT_TITLE = 0x134;
    const uint8_t SIZE_SHORT_TITLE = 0xB;
    constexpr uint16_t ADDRESS_MANUFACTURER_CODE = 0x13F;
    constexpr uint8_t SIZE_MANUFACTURER_CODE = 0x4;
    constexpr uint16_t ADDRESS_CBG_FLAG = 0x143;
    constexpr uint16_t ADDRESS_NEW_LICENSEE_CODE = 0x144;
    constexpr uint8_t SIZE_NEW_LICENSEE_CODE = 0x2;
    constexpr uint16_t ADDRESS_SGB_FLAG = 0x146;
    constexpr uint16_t ADDRESS_CARTRIDGE_TYPE = 0x147;
    constexpr uint16_t ADDRESS_ROM_SIZE = 0x148;
    constexpr uint16_t ADDRESS_RAM_SIZE = 0x149;
    constexpr uint16_t ADDRESS_DESTINATION_CODE = 0x14A;
    constexpr uint16_t ADDRESS_OLD_LICENSE_CODE = 0x14B;
    constexpr uint16_t ADDRESS_MASK_ROM_VERSION_NUMBER = 0x14C;
    constexpr uint16_t ADDRESS_HEADER_CHECKSUM = 0x14D;
    constexpr uint16_t ADDRESS_GLOBAL_CHECKSUM = 0x14E;
    constexpr uint8_t SIZE_GLOBAL_CHECKSUM = 0x2;

    CartridgeType get_cartridge_type(const RomBank& rom);
    ROMSize get_rom_size(const RomBank& rom);
    uint16_t get_nb_rom_banks(const RomBank& rom);
    RAMSize get_ram_size(const RomBank& rom);
    uint8_t get_nb_ram_banks(const RomBank& rom);
    std::string get_new_licensee_code(RomBank rom);
    uint8_t get_sgb_flag(const RomBank& rom);
    std::string get_short_title(RomBank rom);
    std::string get_manufacturer_code(RomBank rom);
    uint8_t get_cbg_flag(const RomBank& rom);
    std::string get_title(RomBank rom);
    uint8_t get_destination_code(const RomBank& rom);
    uint8_t get_old_license_code(const RomBank& rom);
    uint8_t get_mask_rom_version_number(const RomBank& rom);
    uint8_t get_header_checksum(const RomBank& rom);
    uint8_t compute_header_checksum(const RomBank& rom);
    uint16_t get_global_checksum(const RomBank& rom);
}