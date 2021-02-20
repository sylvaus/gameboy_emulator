#pragma once

#include <cstdint>
#include <functional>
#include "emulator/memory/registers.h"
#include "emulator/memory/memory_controller.h"

namespace emulator::generated
{
    union Arguments
    {
        uint16_t uint16;
        uint8_t uint8;
        int8_t int8;
    };
    enum class ArgumentType
    {
        none,
        int8,
        uint8,
        uint16
    };
    using emulator::memory::Registers;
    using emulator::memory::MemoryController;
    using InstructionFunction = std::function<uint16_t (const Arguments& arguments, Registers& registers, MemoryController& controller)>;

    uint16_t nop_000(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x0 NOP

    uint16_t ld_001(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1 LD BC, d16

    uint16_t ld_002(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x2 LD (BC), A

    uint16_t inc_003(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x3 INC BC

    uint16_t inc_004(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x4 INC B

    uint16_t dec_005(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x5 DEC B

    uint16_t ld_006(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x6 LD B, d8

    uint16_t rlca_007(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x7 RLCA

    uint16_t ld_008(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x8 LD (a16), SP

    uint16_t add_009(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x9 ADD HL, BC

    uint16_t ld_00a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA LD A, (BC)

    uint16_t dec_00b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB DEC BC

    uint16_t inc_00c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC INC C

    uint16_t dec_00d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD DEC C

    uint16_t ld_00e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE LD C, d8

    uint16_t rrca_00f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF RRCA

    uint16_t ld_011(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x11 LD DE, d16

    uint16_t ld_012(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x12 LD (DE), A

    uint16_t inc_013(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13 INC DE

    uint16_t inc_014(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x14 INC D

    uint16_t dec_015(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x15 DEC D

    uint16_t ld_016(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x16 LD D, d8

    uint16_t rla_017(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x17 RLA

    uint16_t jr_018(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x18 JR r8

    uint16_t add_019(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x19 ADD HL, DE

    uint16_t ld_01a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A LD A, (DE)

    uint16_t dec_01b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B DEC DE

    uint16_t inc_01c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C INC E

    uint16_t dec_01d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D DEC E

    uint16_t ld_01e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E LD E, d8

    uint16_t rra_01f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F RRA

    uint16_t jr_020(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x20 JR NZ, r8

    uint16_t ld_021(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x21 LD HL, d16

    uint16_t ldi_022(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x22 LDI (HL), A

    uint16_t inc_023(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x23 INC HL

    uint16_t inc_024(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x24 INC H

    uint16_t dec_025(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x25 DEC H

    uint16_t ld_026(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x26 LD H, d8

    uint16_t daa_027(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x27 DAA

    uint16_t jr_028(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x28 JR Z, r8

    uint16_t add_029(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x29 ADD HL, HL

    uint16_t ldi_02a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x2A LDI A, (HL)

    uint16_t dec_02b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x2B DEC HL

    uint16_t inc_02c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x2C INC L

    uint16_t dec_02d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x2D DEC L

    uint16_t ld_02e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x2E LD L, d8

    uint16_t cpl_02f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x2F CPL

    uint16_t jr_030(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x30 JR NC, r8

    uint16_t ld_031(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x31 LD SP, d16

    uint16_t ldd_032(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x32 LDD (HL), A

    uint16_t inc_033(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x33 INC SP

    uint16_t inc_034(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x34 INC (HL)

    uint16_t dec_035(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x35 DEC (HL)

    uint16_t ld_036(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x36 LD (HL), d8

    uint16_t scf_037(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x37 SCF

    uint16_t jr_038(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x38 JR C, r8

    uint16_t add_039(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x39 ADD HL, SP

    uint16_t ldd_03a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x3A LDD A, (HL)

    uint16_t dec_03b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x3B DEC SP

    uint16_t inc_03c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x3C INC A

    uint16_t dec_03d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x3D DEC A

    uint16_t ld_03e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x3E LD A, d8

    uint16_t ccf_03f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x3F CCF

    uint16_t ld_040(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x40 LD B, B

    uint16_t ld_041(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x41 LD B, C

    uint16_t ld_042(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x42 LD B, D

    uint16_t ld_043(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x43 LD B, E

    uint16_t ld_044(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x44 LD B, H

    uint16_t ld_045(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x45 LD B, L

    uint16_t ld_046(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x46 LD B, (HL)

    uint16_t ld_047(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x47 LD B, A

    uint16_t ld_048(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x48 LD C, B

    uint16_t ld_049(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x49 LD C, C

    uint16_t ld_04a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x4A LD C, D

    uint16_t ld_04b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x4B LD C, E

    uint16_t ld_04c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x4C LD C, H

    uint16_t ld_04d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x4D LD C, L

    uint16_t ld_04e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x4E LD C, (HL)

    uint16_t ld_04f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x4F LD C, A

    uint16_t ld_050(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x50 LD D, B

    uint16_t ld_051(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x51 LD D, C

    uint16_t ld_052(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x52 LD D, D

    uint16_t ld_053(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x53 LD D, E

    uint16_t ld_054(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x54 LD D, H

    uint16_t ld_055(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x55 LD D, L

    uint16_t ld_056(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x56 LD D, (HL)

    uint16_t ld_057(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x57 LD D, A

    uint16_t ld_058(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x58 LD E, B

    uint16_t ld_059(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x59 LD E, C

    uint16_t ld_05a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x5A LD E, D

    uint16_t ld_05b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x5B LD E, E

    uint16_t ld_05c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x5C LD E, H

    uint16_t ld_05d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x5D LD E, L

    uint16_t ld_05e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x5E LD E, (HL)

    uint16_t ld_05f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x5F LD E, A

    uint16_t ld_060(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x60 LD H, B

    uint16_t ld_061(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x61 LD H, C

    uint16_t ld_062(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x62 LD H, D

    uint16_t ld_063(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x63 LD H, E

    uint16_t ld_064(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x64 LD H, H

    uint16_t ld_065(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x65 LD H, L

    uint16_t ld_066(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x66 LD H, (HL)

    uint16_t ld_067(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x67 LD H, A

    uint16_t ld_068(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x68 LD L, B

    uint16_t ld_069(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x69 LD L, C

    uint16_t ld_06a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x6A LD L, D

    uint16_t ld_06b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x6B LD L, E

    uint16_t ld_06c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x6C LD L, H

    uint16_t ld_06d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x6D LD L, L

    uint16_t ld_06e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x6E LD L, (HL)

    uint16_t ld_06f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x6F LD L, A

    uint16_t ld_070(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x70 LD (HL), B

    uint16_t ld_071(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x71 LD (HL), C

    uint16_t ld_072(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x72 LD (HL), D

    uint16_t ld_073(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x73 LD (HL), E

    uint16_t ld_074(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x74 LD (HL), H

    uint16_t ld_075(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x75 LD (HL), L

    uint16_t halt_076(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x76 HALT

    uint16_t ld_077(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x77 LD (HL), A

    uint16_t ld_078(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x78 LD A, B

    uint16_t ld_079(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x79 LD A, C

    uint16_t ld_07a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x7A LD A, D

    uint16_t ld_07b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x7B LD A, E

    uint16_t ld_07c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x7C LD A, H

    uint16_t ld_07d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x7D LD A, L

    uint16_t ld_07e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x7E LD A, (HL)

    uint16_t ld_07f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x7F LD A, A

    uint16_t add_080(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x80 ADD A, B

    uint16_t add_081(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x81 ADD A, C

    uint16_t add_082(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x82 ADD A, D

    uint16_t add_083(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x83 ADD A, E

    uint16_t add_084(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x84 ADD A, H

    uint16_t add_085(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x85 ADD A, L

    uint16_t add_086(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x86 ADD A, (HL)

    uint16_t add_087(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x87 ADD A, A

    uint16_t adc_088(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x88 ADC A, B

    uint16_t adc_089(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x89 ADC A, C

    uint16_t adc_08a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x8A ADC A, D

    uint16_t adc_08b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x8B ADC A, E

    uint16_t adc_08c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x8C ADC A, H

    uint16_t adc_08d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x8D ADC A, L

    uint16_t adc_08e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x8E ADC A, (HL)

    uint16_t adc_08f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x8F ADC A, A

    uint16_t sub_090(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x90 SUB A, B

    uint16_t sub_091(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x91 SUB A, C

    uint16_t sub_092(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x92 SUB A, D

    uint16_t sub_093(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x93 SUB A, E

    uint16_t sub_094(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x94 SUB A, H

    uint16_t sub_095(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x95 SUB A, L

    uint16_t sub_096(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x96 SUB A, (HL)

    uint16_t sub_097(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x97 SUB A, A

    uint16_t sbc_098(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x98 SBC A, B

    uint16_t sbc_099(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x99 SBC A, C

    uint16_t sbc_09a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x9A SBC A, D

    uint16_t sbc_09b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x9B SBC A, E

    uint16_t sbc_09c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x9C SBC A, H

    uint16_t sbc_09d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x9D SBC A, L

    uint16_t sbc_09e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x9E SBC A, (HL)

    uint16_t sbc_09f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x9F SBC A, A

    uint16_t xor_0a8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA8 XOR B

    uint16_t xor_0a9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA9 XOR C

    uint16_t xor_0aa(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xAA XOR D

    uint16_t xor_0ab(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xAB XOR E

    uint16_t xor_0ac(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xAC XOR H

    uint16_t xor_0ad(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xAD XOR L

    uint16_t xor_0ae(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xAE XOR (HL)

    uint16_t xor_0af(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xAF XOR A

    uint16_t or_0b0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB0 OR B

    uint16_t or_0b1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB1 OR C

    uint16_t or_0b2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB2 OR D

    uint16_t or_0b3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB3 OR E

    uint16_t or_0b4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB4 OR H

    uint16_t or_0b5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB5 OR L

    uint16_t or_0b6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB6 OR (HL)

    uint16_t or_0b7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB7 OR A

    uint16_t cp_0b8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB8 CP A, B

    uint16_t cp_0b9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xB9 CP A, C

    uint16_t cp_0ba(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xBA CP A, D

    uint16_t cp_0bb(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xBB CP A, E

    uint16_t cp_0bc(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xBC CP A, H

    uint16_t cp_0bd(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xBD CP A, L

    uint16_t cp_0be(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xBE CP A, (HL)

    uint16_t cp_0bf(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xBF CP A, A

    uint16_t ret_0c0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC0 RET NZ

    uint16_t pop_0c1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC1 POP BC

    uint16_t jp_0c2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC2 JP NZ, a16

    uint16_t jp_0c3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC3 JP a16

    uint16_t call_0c4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC4 CALL NZ, a16

    uint16_t push_0c5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC5 PUSH BC

    uint16_t add_0c6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC6 ADD A, d8

    uint16_t rst_0c7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC7 RST 00H

    uint16_t ret_0c8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC8 RET Z

    uint16_t ret_0c9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xC9 RET

    uint16_t jp_0ca(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xCA JP Z, a16

    uint16_t prefix_0cb(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xCB PREFIX CB

    uint16_t call_0cc(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xCC CALL Z, a16

    uint16_t call_0cd(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xCD CALL a16

    uint16_t adc_0ce(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xCE ADC A, d8

    uint16_t rst_0cf(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xCF RST 08H

    uint16_t ret_0d0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD0 RET NC

    uint16_t pop_0d1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD1 POP DE

    uint16_t jp_0d2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD2 JP NC, a16

    uint16_t unknown_0d3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD3 UNKNOWN

    uint16_t call_0d4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD4 CALL NC, a16

    uint16_t push_0d5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD5 PUSH DE

    uint16_t sub_0d6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD6 SUB A, d8

    uint16_t rst_0d7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD7 RST 10H

    uint16_t ret_0d8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD8 RET C

    uint16_t jp_0da(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xDA JP C, a16

    uint16_t unknown_0db(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xDB UNKNOWN

    uint16_t call_0dc(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xDC CALL C, a16

    uint16_t unknown_0dd(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xDD UNKNOWN

    uint16_t sbc_0de(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xDE SBC A, d8

    uint16_t rst_0df(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xDF RST 18H

    uint16_t ldh_0e0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE0 LDH (a8), A

    uint16_t pop_0e1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE1 POP HL

    uint16_t ldspecialc_0e2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE2 LDSpecialC (C), A

    uint16_t unknown_0e3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE3 UNKNOWN

    uint16_t unknown_0e4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE4 UNKNOWN

    uint16_t push_0e5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE5 PUSH HL

    uint16_t rst_0e7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE7 RST 20H

    uint16_t add_0e8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE8 ADD SP, r8

    uint16_t jp_0e9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE9 JP HL

    uint16_t ld_0ea(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xEA LD (a16), A

    uint16_t unknown_0eb(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xEB UNKNOWN

    uint16_t unknown_0ec(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xEC UNKNOWN

    uint16_t unknown_0ed(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xED UNKNOWN

    uint16_t xor_0ee(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xEE XOR d8

    uint16_t rst_0ef(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xEF RST 28H

    uint16_t ldh_0f0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF0 LDH A, (a8)

    uint16_t pop_0f1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF1 POP AF

    uint16_t ldspecialc_0f2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF2 LDSpecialC A, (C)

    uint16_t di_0f3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF3 DI

    uint16_t unknown_0f4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF4 UNKNOWN

    uint16_t push_0f5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF5 PUSH AF

    uint16_t or_0f6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF6 OR d8

    uint16_t rst_0f7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF7 RST 30H

    uint16_t ldhl_0f8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF8 LDHL SP, r8

    uint16_t ld_0f9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xF9 LD SP, HL

    uint16_t ld_0fa(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xFA LD A, (a16)

    uint16_t ei_0fb(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xFB EI

    uint16_t unknown_0fc(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xFC UNKNOWN

    uint16_t unknown_0fd(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xFD UNKNOWN

    uint16_t cp_0fe(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xFE CP A, d8

    uint16_t rst_0ff(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xFF RST 38H

    uint16_t rlc_100(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x100 RLC B

    uint16_t rlc_101(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x101 RLC C

    uint16_t rlc_102(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x102 RLC D

    uint16_t rlc_103(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x103 RLC E

    uint16_t rlc_104(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x104 RLC H

    uint16_t rlc_105(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x105 RLC L

    uint16_t rlc_106(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x106 RLC (HL)

    uint16_t rlc_107(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x107 RLC A

    uint16_t rrc_108(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x108 RRC B

    uint16_t rrc_109(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x109 RRC C

    uint16_t rrc_10a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x10A RRC D

    uint16_t rrc_10b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x10B RRC E

    uint16_t rrc_10c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x10C RRC H

    uint16_t rrc_10d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x10D RRC L

    uint16_t rrc_10e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x10E RRC (HL)

    uint16_t rrc_10f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x10F RRC A

    uint16_t rl_110(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x110 RL B

    uint16_t rl_111(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x111 RL C

    uint16_t rl_112(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x112 RL D

    uint16_t rl_113(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x113 RL E

    uint16_t rl_114(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x114 RL H

    uint16_t rl_115(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x115 RL L

    uint16_t rl_116(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x116 RL (HL)

    uint16_t rl_117(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x117 RL A

    uint16_t rr_118(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x118 RR B

    uint16_t rr_119(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x119 RR C

    uint16_t rr_11a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x11A RR D

    uint16_t rr_11b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x11B RR E

    uint16_t rr_11c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x11C RR H

    uint16_t rr_11d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x11D RR L

    uint16_t rr_11e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x11E RR (HL)

    uint16_t rr_11f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x11F RR A

    uint16_t sla_120(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x120 SLA B

    uint16_t sla_121(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x121 SLA C

    uint16_t sla_122(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x122 SLA D

    uint16_t sla_123(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x123 SLA E

    uint16_t sla_124(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x124 SLA H

    uint16_t sla_125(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x125 SLA L

    uint16_t sla_126(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x126 SLA (HL)

    uint16_t sla_127(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x127 SLA A

    uint16_t sra_128(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x128 SRA B

    uint16_t sra_129(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x129 SRA C

    uint16_t sra_12a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x12A SRA D

    uint16_t sra_12b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x12B SRA E

    uint16_t sra_12c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x12C SRA H

    uint16_t sra_12d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x12D SRA L

    uint16_t sra_12e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x12E SRA (HL)

    uint16_t sra_12f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x12F SRA A

    uint16_t srl_138(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x138 SRL B

    uint16_t srl_139(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x139 SRL C

    uint16_t srl_13a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13A SRL D

    uint16_t srl_13b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13B SRL E

    uint16_t srl_13c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13C SRL H

    uint16_t srl_13d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13D SRL L

    uint16_t srl_13e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13E SRL (HL)

    uint16_t srl_13f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13F SRL A

    const InstructionFunction INSTRUCTION_FUNCTIONS[] = {
        &nop_000,
        &ld_001,
        &ld_002,
        &inc_003,
        &inc_004,
        &dec_005,
        &ld_006,
        &rlca_007,
        &ld_008,
        &add_009,
        &ld_00a,
        &dec_00b,
        &inc_00c,
        &dec_00d,
        &ld_00e,
        &rrca_00f,
        &ld_011,
        &ld_012,
        &inc_013,
        &inc_014,
        &dec_015,
        &ld_016,
        &rla_017,
        &jr_018,
        &add_019,
        &ld_01a,
        &dec_01b,
        &inc_01c,
        &dec_01d,
        &ld_01e,
        &rra_01f,
        &jr_020,
        &ld_021,
        &ldi_022,
        &inc_023,
        &inc_024,
        &dec_025,
        &ld_026,
        &daa_027,
        &jr_028,
        &add_029,
        &ldi_02a,
        &dec_02b,
        &inc_02c,
        &dec_02d,
        &ld_02e,
        &cpl_02f,
        &jr_030,
        &ld_031,
        &ldd_032,
        &inc_033,
        &inc_034,
        &dec_035,
        &ld_036,
        &scf_037,
        &jr_038,
        &add_039,
        &ldd_03a,
        &dec_03b,
        &inc_03c,
        &dec_03d,
        &ld_03e,
        &ccf_03f,
        &ld_040,
        &ld_041,
        &ld_042,
        &ld_043,
        &ld_044,
        &ld_045,
        &ld_046,
        &ld_047,
        &ld_048,
        &ld_049,
        &ld_04a,
        &ld_04b,
        &ld_04c,
        &ld_04d,
        &ld_04e,
        &ld_04f,
        &ld_050,
        &ld_051,
        &ld_052,
        &ld_053,
        &ld_054,
        &ld_055,
        &ld_056,
        &ld_057,
        &ld_058,
        &ld_059,
        &ld_05a,
        &ld_05b,
        &ld_05c,
        &ld_05d,
        &ld_05e,
        &ld_05f,
        &ld_060,
        &ld_061,
        &ld_062,
        &ld_063,
        &ld_064,
        &ld_065,
        &ld_066,
        &ld_067,
        &ld_068,
        &ld_069,
        &ld_06a,
        &ld_06b,
        &ld_06c,
        &ld_06d,
        &ld_06e,
        &ld_06f,
        &ld_070,
        &ld_071,
        &ld_072,
        &ld_073,
        &ld_074,
        &ld_075,
        &halt_076,
        &ld_077,
        &ld_078,
        &ld_079,
        &ld_07a,
        &ld_07b,
        &ld_07c,
        &ld_07d,
        &ld_07e,
        &ld_07f,
        &add_080,
        &add_081,
        &add_082,
        &add_083,
        &add_084,
        &add_085,
        &add_086,
        &add_087,
        &adc_088,
        &adc_089,
        &adc_08a,
        &adc_08b,
        &adc_08c,
        &adc_08d,
        &adc_08e,
        &adc_08f,
        &sub_090,
        &sub_091,
        &sub_092,
        &sub_093,
        &sub_094,
        &sub_095,
        &sub_096,
        &sub_097,
        &sbc_098,
        &sbc_099,
        &sbc_09a,
        &sbc_09b,
        &sbc_09c,
        &sbc_09d,
        &sbc_09e,
        &sbc_09f,
        &xor_0a8,
        &xor_0a9,
        &xor_0aa,
        &xor_0ab,
        &xor_0ac,
        &xor_0ad,
        &xor_0ae,
        &xor_0af,
        &or_0b0,
        &or_0b1,
        &or_0b2,
        &or_0b3,
        &or_0b4,
        &or_0b5,
        &or_0b6,
        &or_0b7,
        &cp_0b8,
        &cp_0b9,
        &cp_0ba,
        &cp_0bb,
        &cp_0bc,
        &cp_0bd,
        &cp_0be,
        &cp_0bf,
        &ret_0c0,
        &pop_0c1,
        &jp_0c2,
        &jp_0c3,
        &call_0c4,
        &push_0c5,
        &add_0c6,
        &rst_0c7,
        &ret_0c8,
        &ret_0c9,
        &jp_0ca,
        &prefix_0cb,
        &call_0cc,
        &call_0cd,
        &adc_0ce,
        &rst_0cf,
        &ret_0d0,
        &pop_0d1,
        &jp_0d2,
        &unknown_0d3,
        &call_0d4,
        &push_0d5,
        &sub_0d6,
        &rst_0d7,
        &ret_0d8,
        &jp_0da,
        &unknown_0db,
        &call_0dc,
        &unknown_0dd,
        &sbc_0de,
        &rst_0df,
        &ldh_0e0,
        &pop_0e1,
        &ldspecialc_0e2,
        &unknown_0e3,
        &unknown_0e4,
        &push_0e5,
        &rst_0e7,
        &add_0e8,
        &jp_0e9,
        &ld_0ea,
        &unknown_0eb,
        &unknown_0ec,
        &unknown_0ed,
        &xor_0ee,
        &rst_0ef,
        &ldh_0f0,
        &pop_0f1,
        &ldspecialc_0f2,
        &di_0f3,
        &unknown_0f4,
        &push_0f5,
        &or_0f6,
        &rst_0f7,
        &ldhl_0f8,
        &ld_0f9,
        &ld_0fa,
        &ei_0fb,
        &unknown_0fc,
        &unknown_0fd,
        &cp_0fe,
        &rst_0ff,
        &rlc_100,
        &rlc_101,
        &rlc_102,
        &rlc_103,
        &rlc_104,
        &rlc_105,
        &rlc_106,
        &rlc_107,
        &rrc_108,
        &rrc_109,
        &rrc_10a,
        &rrc_10b,
        &rrc_10c,
        &rrc_10d,
        &rrc_10e,
        &rrc_10f,
        &rl_110,
        &rl_111,
        &rl_112,
        &rl_113,
        &rl_114,
        &rl_115,
        &rl_116,
        &rl_117,
        &rr_118,
        &rr_119,
        &rr_11a,
        &rr_11b,
        &rr_11c,
        &rr_11d,
        &rr_11e,
        &rr_11f,
        &sla_120,
        &sla_121,
        &sla_122,
        &sla_123,
        &sla_124,
        &sla_125,
        &sla_126,
        &sla_127,
        &sra_128,
        &sra_129,
        &sra_12a,
        &sra_12b,
        &sra_12c,
        &sra_12d,
        &sra_12e,
        &sra_12f,
        &srl_138,
        &srl_139,
        &srl_13a,
        &srl_13b,
        &srl_13c,
        &srl_13d,
        &srl_13e,
        &srl_13f
    };

    const ArgumentType INSTRUCTION_ARGUMENT_TYPES[] = {
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::int8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::int8,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::int8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::int8,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::int8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::uint16,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::uint16,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::int8,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::int8,
        ArgumentType::none,
        ArgumentType::uint16,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::uint8,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none,
        ArgumentType::none
    };
}