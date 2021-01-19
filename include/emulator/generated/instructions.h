#include <cstdint>
#include "emulator/registers.h"
#include "emulator/memory_controller.h"

namespace emulator::generated
{
    union Arguments
    {
        uint8_t data[2];
        uint16_t uint16;
        struct 
        {
            uint8_t uint8;
            uint8_t unused;
        };
    };
    
    uint16_t nop_000(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x0 NOP
    
    uint16_t ld_001(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x1 LD BC, d16
    
    uint16_t ld_002(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x2 LD (BC), A
    
    uint16_t ld_006(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x6 LD B, d8
    
    uint16_t ld_008(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x8 LD (a16), SP
    
    uint16_t ld_00a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xA LD A, (BC)
    
    uint16_t ld_00e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xE LD C, d8
    
    uint16_t ld_011(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x11 LD DE, d16
    
    uint16_t ld_012(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x12 LD (DE), A
    
    uint16_t ld_016(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x16 LD D, d8
    
    uint16_t ld_01a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x1A LD A, (DE)
    
    uint16_t ld_01e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x1E LD E, d8
    
    uint16_t ld_021(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x21 LD HL, d16
    
    uint16_t ldi_022(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x22 LDI (HL), A
    
    uint16_t ld_026(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x26 LD H, d8
    
    uint16_t ldi_02a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x2A LDI A, (HL)
    
    uint16_t ld_02e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x2E LD L, d8
    
    uint16_t ld_031(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x31 LD SP, d16
    
    uint16_t ldd_032(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x32 LDD (HL), A
    
    uint16_t ld_036(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x36 LD (HL), d8
    
    uint16_t ldd_03a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x3A LDD A, (HL)
    
    uint16_t ld_03e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x3E LD A, d8
    
    uint16_t ld_040(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x40 LD B, B
    
    uint16_t ld_041(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x41 LD B, C
    
    uint16_t ld_042(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x42 LD B, D
    
    uint16_t ld_043(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x43 LD B, E
    
    uint16_t ld_044(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x44 LD B, H
    
    uint16_t ld_045(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x45 LD B, L
    
    uint16_t ld_046(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x46 LD B, (HL)
    
    uint16_t ld_047(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x47 LD B, A
    
    uint16_t ld_048(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x48 LD C, B
    
    uint16_t ld_049(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x49 LD C, C
    
    uint16_t ld_04a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x4A LD C, D
    
    uint16_t ld_04b(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x4B LD C, E
    
    uint16_t ld_04c(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x4C LD C, H
    
    uint16_t ld_04d(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x4D LD C, L
    
    uint16_t ld_04e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x4E LD C, (HL)
    
    uint16_t ld_04f(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x4F LD C, A
    
    uint16_t ld_050(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x50 LD D, B
    
    uint16_t ld_051(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x51 LD D, C
    
    uint16_t ld_052(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x52 LD D, D
    
    uint16_t ld_053(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x53 LD D, E
    
    uint16_t ld_054(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x54 LD D, H
    
    uint16_t ld_055(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x55 LD D, L
    
    uint16_t ld_056(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x56 LD D, (HL)
    
    uint16_t ld_057(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x57 LD D, A
    
    uint16_t ld_058(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x58 LD E, B
    
    uint16_t ld_059(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x59 LD E, C
    
    uint16_t ld_05a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x5A LD E, D
    
    uint16_t ld_05b(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x5B LD E, E
    
    uint16_t ld_05c(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x5C LD E, H
    
    uint16_t ld_05d(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x5D LD E, L
    
    uint16_t ld_05e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x5E LD E, (HL)
    
    uint16_t ld_05f(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x5F LD E, A
    
    uint16_t ld_060(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x60 LD H, B
    
    uint16_t ld_061(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x61 LD H, C
    
    uint16_t ld_062(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x62 LD H, D
    
    uint16_t ld_063(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x63 LD H, E
    
    uint16_t ld_064(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x64 LD H, H
    
    uint16_t ld_065(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x65 LD H, L
    
    uint16_t ld_066(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x66 LD H, (HL)
    
    uint16_t ld_067(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x67 LD H, A
    
    uint16_t ld_068(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x68 LD L, B
    
    uint16_t ld_069(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x69 LD L, C
    
    uint16_t ld_06a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x6A LD L, D
    
    uint16_t ld_06b(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x6B LD L, E
    
    uint16_t ld_06c(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x6C LD L, H
    
    uint16_t ld_06d(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x6D LD L, L
    
    uint16_t ld_06e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x6E LD L, (HL)
    
    uint16_t ld_06f(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x6F LD L, A
    
    uint16_t ld_070(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x70 LD (HL), B
    
    uint16_t ld_071(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x71 LD (HL), C
    
    uint16_t ld_072(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x72 LD (HL), D
    
    uint16_t ld_073(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x73 LD (HL), E
    
    uint16_t ld_074(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x74 LD (HL), H
    
    uint16_t ld_075(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x75 LD (HL), L
    
    uint16_t ld_077(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x77 LD (HL), A
    
    uint16_t ld_078(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x78 LD A, B
    
    uint16_t ld_079(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x79 LD A, C
    
    uint16_t ld_07a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x7A LD A, D
    
    uint16_t ld_07b(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x7B LD A, E
    
    uint16_t ld_07c(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x7C LD A, H
    
    uint16_t ld_07d(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x7D LD A, L
    
    uint16_t ld_07e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x7E LD A, (HL)
    
    uint16_t ld_07f(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0x7F LD A, A
    
    uint16_t unknown_0d3(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xD3 UNKNOWN
    
    uint16_t unknown_0db(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xDB UNKNOWN
    
    uint16_t unknown_0dd(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xDD UNKNOWN
    
    uint16_t ldh_0e0(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xE0 LDH (a8), A
    
    uint16_t ldspecialc_0e2(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xE2 LDSpecialC (C), A
    
    uint16_t unknown_0e3(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xE3 UNKNOWN
    
    uint16_t unknown_0e4(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xE4 UNKNOWN
    
    uint16_t ld_0ea(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xEA LD (a16), A
    
    uint16_t unknown_0eb(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xEB UNKNOWN
    
    uint16_t unknown_0ec(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xEC UNKNOWN
    
    uint16_t unknown_0ed(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xED UNKNOWN
    
    uint16_t ldh_0f0(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xF0 LDH A, (a8)
    
    uint16_t ldspecialc_0f2(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xF2 LDSpecialC A, (C)
    
    uint16_t unknown_0f4(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xF4 UNKNOWN
    
    uint16_t ldhl_0f8(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xF8 LDHL SP, r8
    
    uint16_t ld_0f9(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xF9 LD SP, HL
    
    uint16_t ld_0fa(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xFA LD A, (a16)
    
    uint16_t unknown_0fc(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xFC UNKNOWN
    
    uint16_t unknown_0fd(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller); // 0xFD UNKNOWN
}