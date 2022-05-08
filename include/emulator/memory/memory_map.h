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

    constexpr std::size_t ROM_BANK_SIZE = 0x4000;
    constexpr std::size_t START_ROM_0 = 0x0000;
    constexpr std::size_t END_ROM_0 = 0x3FFF;
    constexpr std::size_t START_ROM_1_N = 0x4000;
    constexpr std::size_t END_ROM_1_N = 0x7FFF;

    constexpr std::size_t VRAM_SIZE = 0x2000;
    constexpr std::size_t START_VRAM = 0x8000;
    constexpr std::size_t END_VRAM = 0x9FFF;

    constexpr std::size_t EXT_RAM_BANK_SIZE = 0x2000;
    constexpr std::size_t START_EXT_RAM = 0xA000;
    constexpr std::size_t END_EXT_RAM = 0xBFFF;

    constexpr std::size_t WORK_RAM_SIZE = 0x1000;
    constexpr std::size_t START_FIRST_WORK_RAM = 0xC000;
    constexpr std::size_t END_FIRST_WORK_RAM = 0xCFFF;
    constexpr std::size_t START_SECOND_WORK_RAM = 0xD000;
    constexpr std::size_t END_SECOND_WORK_RAM = 0xDFFF;

    constexpr std::size_t ECHO_RAM_SIZE = 0x1E00;
    constexpr std::size_t START_ECHO_RAM = 0xE000;
    constexpr std::size_t END_ECHO_RAM = 0xFDFF;

    constexpr std::size_t OAM_SIZE = 0x100;
    constexpr std::size_t START_OAM = 0xFE00;
    constexpr std::size_t END_OAM = 0xFE9F;

    constexpr std::size_t START_NOT_USABLE = 0xFEA0;
    constexpr std::size_t END_NOT_USABLE = 0xFEFF;

    constexpr std::size_t IO_REGISTER_SIZE = 0x80;
    constexpr std::size_t START_IO_REGISTER = 0xFF00;
    constexpr std::size_t END_IO_REGISTER = 0xFF7F;

    constexpr std::size_t HRAM_SIZE = 0x100;
    constexpr std::size_t START_HRAM = 0xFF80;
    constexpr std::size_t END_HRAM = 0xFFFE;

    constexpr std::size_t INTERRUPT_ENABLE_ADDRESS = 0xFFFF;

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

    // UB but works so far: https://youtu.be/IAdLwUXRUvg?t=227
    union IoPorts
    {
        uint8_t data[0x80];
        struct
        {
            uint8_t joypad_p10:1;                       // 0xFF00 bit 0
            uint8_t joypad_p11:1;                       // 0xFF00 bit 1
            uint8_t joypad_p12:1;                       // 0xFF00 bit 2
            uint8_t joypad_p13:1;                       // 0xFF00 bit 3
            uint8_t joypad_p14:1;                       // 0xFF00 bit 4
            uint8_t joypad_p15:1;                       // 0xFF00 bit 5
            uint8_t unused48:2;                         // 0xFF00 bit 6..7
            uint8_t serial_transfer_data;               // 0xFF01
            uint8_t serial_transfer_control_shift:1;    // 0xFF02 bit 0
            uint8_t serial_transfer_control_clock:1;    // 0xFF02 bit 1
            uint8_t unused37:5;                         // 0xFF02 bit 2..6
            uint8_t serial_transfer_control_flag:1;     // 0xFF02 bit 7
            uint8_t unused38;                           // 0xFF03
            uint8_t divider_register;                   // 0xFF04
            uint8_t timer_counter;                      // 0xFF05
            uint8_t timer_modulo;                       // 0xFF06
            uint8_t timer_control_select:2;             // 0xFF07 bit 0..1
            uint8_t timer_control_stop:1;               // 0xFF07 bit 2
            uint8_t unused39:5;                         // 0xFF07 bit 3..7
            uint8_t unused40;                           // 0xFF08
            uint8_t unused41;                           // 0xFF09
            uint8_t unused42;                           // 0xFF0A
            uint8_t unused43;                           // 0xFF0B
            uint8_t unused44;                           // 0xFF0C
            uint8_t unused45;                           // 0xFF0D
            uint8_t unused46;                           // 0xFF0E
            uint8_t interrupt_flag_v_blank:1;           // 0xFF0F bit 0
            uint8_t interrupt_flag_lcd_stat:1;          // 0xFF0F bit 1
            uint8_t interrupt_flag_timer:1;             // 0xFF0F bit 2
            uint8_t interrupt_flag_serial:1;            // 0xFF0F bit 3
            uint8_t interrupt_flag_joypad:1;            // 0xFF0F bit 4
            uint8_t unused47:3;                         // 0xFF0F bit 5..7
            uint8_t sweep_register_ch1_nb_shift:3;      // 0xFF10 bit 0..2
            uint8_t sweep_register_ch1_inc:1;           // 0xFF10 bit 3
            uint8_t sweep_register_ch1_time:3;          // 0xFF10 bit 4..6
            uint8_t unused49:1;                         // 0xFF10 bit 7
            uint8_t sound_length_ch1:6;                 // 0xFF11 bit 0..5
            uint8_t wave_pattern_ch1:2;                 // 0xFF11 bit 6..7
            uint8_t volume_envelope_ch1_nb:3;           // 0xFF12 bit 0..2
            uint8_t volume_envelope_ch1_direction:1;    // 0xFF12 bit 3
            uint8_t volume_envelope_ch1_initial:4;      // 0xFF12 bit 4..7
            uint8_t frequency_low_ch1;                  // 0xFF13
            uint8_t frequency_hi_ch1:3;                 // 0xFF14 bit 0..2
            uint8_t unused51:3;                         // 0xFF14 bit 3..5
            uint8_t frequency_ch1_counter:1;            // 0xFF14 bit 6
            uint8_t frequency_ch1_initial:1;            // 0xFF14 bit 7
            uint8_t unused52;                           // 0xFF15
            uint8_t sound_length_ch2:6;                 // 0xFF16 bit 0..5
            uint8_t wave_pattern_ch2:2;                 // 0xFF16 bit 6..7
            uint8_t volume_envelope_ch2_nb:3;           // 0xFF17 bit 0..2
            uint8_t volume_envelope_ch2_direction:1;    // 0xFF17 bit 3
            uint8_t volume_envelope_ch2_initial:4;      // 0xFF17 bit 4..7
            uint8_t frequency_low_ch2;                  // 0xFF18
            uint8_t frequency_hi_ch2:3;                 // 0xFF19 bit 0..2
            uint8_t unused53:3;                         // 0xFF19 bit 3..5
            uint8_t frequency_ch2_counter:1;            // 0xFF19 bit 6
            uint8_t frequency_ch2_initial:1;            // 0xFF19 bit 7
            uint8_t unused54:7;                         // 0xFF1A bit 1..6
            uint8_t sound_on_off_ch3:1;                 // 0xFF1A bit 7
            uint8_t sound_length_ch3:1;                 // 0xFF1B
            uint8_t unused55:5;                         // 0xFF1C bit 0..4
            uint8_t select_output_level_ch3:3;          // 0xFF1C bit 5..6
            uint8_t unused56:1;                         // 0xFF1C bit 7
            uint8_t frequency_low_ch3;                  // 0xFF1D
            uint8_t frequency_hi_ch3:3;                 // 0xFF1E bit 0..2
            uint8_t unused57:3;                         // 0xFF1E bit 3..5
            uint8_t frequency_ch3_counter:1;            // 0xFF1E bit 6
            uint8_t frequency_ch3_initial:1;            // 0xFF1E bit 7
            uint8_t unused58;                           // 0xFF1F
            uint8_t sound_length_ch4:6;                 // 0xFF20 bit 0..5
            uint8_t unused59:2;                         // 0xFF20 bit 6..7
            uint8_t volume_envelope_ch4_nb:3;           // 0xFF21 bit 0..2
            uint8_t volume_envelope_ch4_direction:1;    // 0xFF21 bit 3
            uint8_t volume_envelope_ch4_initial:4;      // 0xFF21 bit 4..7
            uint8_t polynomial_counter_ch4_ratio:3;     // 0xFF22 bit 0..2
            uint8_t polynomial_counter_ch4_counter:1;   // 0xFF22 bit 3
            uint8_t polynomial_counter_ch4_shift:4;     // 0xFF22 bit 4..7
            uint8_t unused60:6;                         // 0xFF22 bit 0..5
            uint8_t counter_ch4_counter:1;              // 0xFF23 bit 6
            uint8_t counter_ch4_initial:1;              // 0xFF23 bit 7
            uint8_t channel_control_so1_level:3;        // 0xFF24 bit 0..2
            uint8_t channel_control_so1_enable:1;       // 0xFF24 bit 3
            uint8_t channel_control_so2_level:3;        // 0xFF24 bit 4..6
            uint8_t channel_control_so2_enable:1;       // 0xFF24 bit 7
            uint8_t so1_sound_1:1;                      // 0xFF25 bit 0
            uint8_t so1_sound_2:1;                      // 0xFF25 bit 1
            uint8_t so1_sound_3:1;                      // 0xFF25 bit 2
            uint8_t so1_sound_4:1;                      // 0xFF25 bit 3
            uint8_t so2_sound_1:1;                      // 0xFF25 bit 4
            uint8_t so2_sound_2:1;                      // 0xFF25 bit 5
            uint8_t so2_sound_3:1;                      // 0xFF25 bit 6
            uint8_t so2_sound_4:1;                      // 0xFF25 bit 7
            uint8_t sound_1_on_flag:1;                  // 0xFF26 bit 0
            uint8_t sound_2_on_flag:1;                  // 0xFF26 bit 1
            uint8_t sound_3_on_flag:1;                  // 0xFF26 bit 2
            uint8_t sound_4_on_flag:1;                  // 0xFF26 bit 3
            uint8_t unused61:3;                         // 0xFF26 bit 4..6
            uint8_t all_sound_on_off:1;                 // 0xFF26 bit 7
            uint8_t unused62;                           // 0xFF27
            uint8_t unused63;                           // 0xFF28
            uint8_t unused64;                           // 0xFF29
            uint8_t unused65;                           // 0xFF2A
            uint8_t unused66;                           // 0xFF2B
            uint8_t unused67;                           // 0xFF2C
            uint8_t unused68;                           // 0xFF2D
            uint8_t unused69;                           // 0xFF2E
            uint8_t unused70;                           // 0xFF2F
            uint8_t wave_pattern_0:4;                   // 0xFF30 bit 0..4
            uint8_t wave_pattern_1:4;                   // 0xFF30 bit 5..7
            uint8_t wave_pattern_2:4;                   // 0xFF31 bit 0..4
            uint8_t wave_pattern_3:4;                   // 0xFF31 bit 5..7
            uint8_t wave_pattern_4:4;                   // 0xFF32 bit 0..4
            uint8_t wave_pattern_5:4;                   // 0xFF32 bit 5..7
            uint8_t wave_pattern_6:4;                   // 0xFF33 bit 0..4
            uint8_t wave_pattern_7:4;                   // 0xFF33 bit 5..7
            uint8_t wave_pattern_8:4;                   // 0xFF34 bit 0..4
            uint8_t wave_pattern_9:4;                   // 0xFF34 bit 5..7
            uint8_t wave_pattern_10:4;                  // 0xFF35 bit 0..4
            uint8_t wave_pattern_11:4;                  // 0xFF35 bit 5..7
            uint8_t wave_pattern_12:4;                  // 0xFF36 bit 0..4
            uint8_t wave_pattern_13:4;                  // 0xFF36 bit 5..7
            uint8_t wave_pattern_14:4;                  // 0xFF37 bit 0..4
            uint8_t wave_pattern_15:4;                  // 0xFF37 bit 5..7
            uint8_t wave_pattern_16:4;                  // 0xFF38 bit 0..4
            uint8_t wave_pattern_17:4;                  // 0xFF38 bit 5..7
            uint8_t wave_pattern_18:4;                  // 0xFF39 bit 0..4
            uint8_t wave_pattern_19:4;                  // 0xFF39 bit 5..7
            uint8_t wave_pattern_20:4;                  // 0xFF3A bit 0..4
            uint8_t wave_pattern_21:4;                  // 0xFF3A bit 5..7
            uint8_t wave_pattern_22:4;                  // 0xFF3B bit 0..4
            uint8_t wave_pattern_23:4;                  // 0xFF3B bit 5..7
            uint8_t wave_pattern_24:4;                  // 0xFF3C bit 0..4
            uint8_t wave_pattern_25:4;                  // 0xFF3C bit 5..7
            uint8_t wave_pattern_26:4;                  // 0xFF3D bit 0..4
            uint8_t wave_pattern_27:4;                  // 0xFF3D bit 5..7
            uint8_t wave_pattern_28:4;                  // 0xFF3E bit 0..4
            uint8_t wave_pattern_29:4;                  // 0xFF3E bit 5..7
            uint8_t wave_pattern_30:4;                  // 0xFF3F bit 0..4
            uint8_t wave_pattern_31:4;                  // 0xFF3F bit 5..7
            uint8_t bg_display:1;                       // 0xFF40
            uint8_t obj_display_enable:1;
            uint8_t obj_size:1;
            uint8_t bg_tile_map_display_select:1;
            uint8_t bg_window_tile_data_select:1;
            uint8_t window_display_enable:1;
            uint8_t window_tile_map_display_select:1;
            uint8_t lcd_display_enable:1;
            uint8_t mode_flag:2;                        // 0xFF41
            uint8_t coincidence_flag:1;
            uint8_t mode_0_h_blank_interrupt:1;
            uint8_t mode_1_v_blank_interrupt:1;
            uint8_t mode_2_oam_interrupt:1;
            uint8_t coincidence_interrupt:1;
            uint8_t unused:1;
            uint8_t scroll_y;                           // 0xFF42
            uint8_t scroll_x;                           // 0xFF43
            uint8_t lcdc_y;                             // 0xFF44
            uint8_t ly_compare;                         // 0xFF45
            uint8_t dma_transfer;                       // 0xFF46
            uint8_t bg_palette_data;                    // 0xFF47
            uint8_t object_palette_0;                   // 0xFF48
            uint8_t object_palette_1;                   // 0xFF49
            uint8_t window_y_position;                  // 0xFF4A
            uint8_t window_x_position;                  // 0xFF4B
            uint8_t unused1;                            // 0xFF4C
            uint8_t prepare_speed_switch:1;             // 0xFF4D bit 0
            uint8_t unused2:6;                          // 0xFF4D bit 1..6
            uint8_t current_speed:1;                    // 0xFF4D bit 7
            uint8_t unused3;                            // 0xFF4E
            uint8_t vbk;                                // 0xFF4F
            uint8_t unused4;                            // 0xFF50
            uint8_t hdma1;                              // 0xFF51
            uint8_t hdma2;                              // 0xFF52
            uint8_t hdma3;                              // 0xFF53
            uint8_t hdma4;                              // 0xFF54
            uint8_t hdma5_length:7;                     // 0xFF55 bit 0..6
            uint8_t hdma5_mode:1;                       // 0xFF55 bit 7
            uint8_t rp_write_data:1;                    // 0xFF56 bit 0
            uint8_t rp_read_data:1;                     // 0xFF56 bit 1
            uint8_t unused5:4;                          // 0xFF56 bit 2..5
            uint8_t data_read_enable:2;                 // 0xFF56 bit 6..7
            uint8_t unused6;                            // 0xFF57
            uint8_t unused7;                            // 0xFF58
            uint8_t unused8;                            // 0xFF59
            uint8_t unused9;                            // 0xFF5A
            uint8_t unused10;                           // 0xFF5B
            uint8_t unused11;                           // 0xFF5C
            uint8_t unused12;                           // 0xFF5D
            uint8_t unused13;                           // 0xFF5E
            uint8_t unused14;                           // 0xFF5F
            uint8_t unused15;                           // 0xFF60
            uint8_t unused16;                           // 0xFF61
            uint8_t unused17;                           // 0xFF62
            uint8_t unused18;                           // 0xFF63
            uint8_t unused19;                           // 0xFF64
            uint8_t unused20;                           // 0xFF65
            uint8_t unused21;                           // 0xFF66
            uint8_t unused22;                           // 0xFF67
            uint8_t bcps_bgpi_index:6;                  // 0xFF68 bit 0..5
            uint8_t unused23:1;                         // 0xFF68 bit 6
            uint8_t bcps_bgpi_auto_inc:1;               // 0xFF68 bit 7
            uint8_t bcpd_bgpd;                          // 0xFF69
            uint8_t ocps_obpi;                          // 0xFF6A
            uint8_t ocpd_obpd;                          // 0xFF6B
            uint8_t undocumented0;                      // 0xFF6C
            uint8_t unused24;                           // 0xFF6D
            uint8_t unused25;                           // 0xFF6E
            uint8_t unused26;                           // 0xFF6F
            uint8_t svbk;                               // 0xFF70
            uint8_t unused27;                           // 0xFF71
            uint8_t undocumented1;                      // 0xFF72
            uint8_t undocumented2;                      // 0xFF73
            uint8_t undocumented3;                      // 0xFF74
            uint8_t undocumented4;                      // 0xFF75
            uint8_t undocumented5;                      // 0xFF76
            uint8_t undocumented6;                      // 0xFF77
            uint8_t unused28;                           // 0xFF78
            uint8_t unused29;                           // 0xFF79
            uint8_t unused30;                           // 0xFF7A
            uint8_t unused31;                           // 0xFF7B
            uint8_t unused32;                           // 0xFF7C
            uint8_t unused33;                           // 0xFF7D
            uint8_t unused34;                           // 0xFF7E
            uint8_t unused35;                           // 0xFF7F
        };
    };
}