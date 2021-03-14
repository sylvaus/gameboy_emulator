#pragma once

#include <cstdint>
#include <functional>
#include <array>
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
    using InstructionFunction = uint16_t (*)(const Arguments& arguments, Registers& registers, MemoryController& controller);

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

    uint16_t stop_010(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x10 STOP 0

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

    uint16_t and_0a0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA0 AND B

    uint16_t and_0a1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA1 AND C

    uint16_t and_0a2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA2 AND D

    uint16_t and_0a3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA3 AND E

    uint16_t and_0a4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA4 AND H

    uint16_t and_0a5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA5 AND L

    uint16_t and_0a6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA6 AND (HL)

    uint16_t and_0a7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xA7 AND A

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

    uint16_t reti_0d9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xD9 RETI

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

    uint16_t and_0e6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0xE6 AND d8

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

    uint16_t swap_130(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x130 SWAP B

    uint16_t swap_131(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x131 SWAP C

    uint16_t swap_132(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x132 SWAP D

    uint16_t swap_133(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x133 SWAP E

    uint16_t swap_134(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x134 SWAP H

    uint16_t swap_135(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x135 SWAP L

    uint16_t swap_136(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x136 SWAP (HL)

    uint16_t swap_137(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x137 SWAP A

    uint16_t srl_138(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x138 SRL B

    uint16_t srl_139(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x139 SRL C

    uint16_t srl_13a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13A SRL D

    uint16_t srl_13b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13B SRL E

    uint16_t srl_13c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13C SRL H

    uint16_t srl_13d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13D SRL L

    uint16_t srl_13e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13E SRL (HL)

    uint16_t srl_13f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x13F SRL A

    uint16_t bit_140(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x140 BIT 0, B

    uint16_t bit_141(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x141 BIT 0, C

    uint16_t bit_142(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x142 BIT 0, D

    uint16_t bit_143(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x143 BIT 0, E

    uint16_t bit_144(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x144 BIT 0, H

    uint16_t bit_145(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x145 BIT 0, L

    uint16_t bit_146(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x146 BIT 0, (HL)

    uint16_t bit_147(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x147 BIT 0, A

    uint16_t bit_148(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x148 BIT 1, B

    uint16_t bit_149(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x149 BIT 1, C

    uint16_t bit_14a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x14A BIT 1, D

    uint16_t bit_14b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x14B BIT 1, E

    uint16_t bit_14c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x14C BIT 1, H

    uint16_t bit_14d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x14D BIT 1, L

    uint16_t bit_14e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x14E BIT 1, (HL)

    uint16_t bit_14f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x14F BIT 1, A

    uint16_t bit_150(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x150 BIT 2, B

    uint16_t bit_151(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x151 BIT 2, C

    uint16_t bit_152(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x152 BIT 2, D

    uint16_t bit_153(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x153 BIT 2, E

    uint16_t bit_154(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x154 BIT 2, H

    uint16_t bit_155(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x155 BIT 2, L

    uint16_t bit_156(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x156 BIT 2, (HL)

    uint16_t bit_157(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x157 BIT 2, A

    uint16_t bit_158(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x158 BIT 3, B

    uint16_t bit_159(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x159 BIT 3, C

    uint16_t bit_15a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x15A BIT 3, D

    uint16_t bit_15b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x15B BIT 3, E

    uint16_t bit_15c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x15C BIT 3, H

    uint16_t bit_15d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x15D BIT 3, L

    uint16_t bit_15e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x15E BIT 3, (HL)

    uint16_t bit_15f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x15F BIT 3, A

    uint16_t bit_160(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x160 BIT 4, B

    uint16_t bit_161(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x161 BIT 4, C

    uint16_t bit_162(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x162 BIT 4, D

    uint16_t bit_163(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x163 BIT 4, E

    uint16_t bit_164(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x164 BIT 4, H

    uint16_t bit_165(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x165 BIT 4, L

    uint16_t bit_166(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x166 BIT 4, (HL)

    uint16_t bit_167(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x167 BIT 4, A

    uint16_t bit_168(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x168 BIT 5, B

    uint16_t bit_169(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x169 BIT 5, C

    uint16_t bit_16a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x16A BIT 5, D

    uint16_t bit_16b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x16B BIT 5, E

    uint16_t bit_16c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x16C BIT 5, H

    uint16_t bit_16d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x16D BIT 5, L

    uint16_t bit_16e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x16E BIT 5, (HL)

    uint16_t bit_16f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x16F BIT 5, A

    uint16_t bit_170(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x170 BIT 6, B

    uint16_t bit_171(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x171 BIT 6, C

    uint16_t bit_172(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x172 BIT 6, D

    uint16_t bit_173(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x173 BIT 6, E

    uint16_t bit_174(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x174 BIT 6, H

    uint16_t bit_175(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x175 BIT 6, L

    uint16_t bit_176(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x176 BIT 6, (HL)

    uint16_t bit_177(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x177 BIT 6, A

    uint16_t bit_178(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x178 BIT 7, B

    uint16_t bit_179(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x179 BIT 7, C

    uint16_t bit_17a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x17A BIT 7, D

    uint16_t bit_17b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x17B BIT 7, E

    uint16_t bit_17c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x17C BIT 7, H

    uint16_t bit_17d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x17D BIT 7, L

    uint16_t bit_17e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x17E BIT 7, (HL)

    uint16_t bit_17f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x17F BIT 7, A

    uint16_t res_180(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x180 RES 0, B

    uint16_t res_181(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x181 RES 0, C

    uint16_t res_182(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x182 RES 0, D

    uint16_t res_183(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x183 RES 0, E

    uint16_t res_184(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x184 RES 0, H

    uint16_t res_185(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x185 RES 0, L

    uint16_t res_186(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x186 RES 0, (HL)

    uint16_t res_187(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x187 RES 0, A

    uint16_t res_188(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x188 RES 1, B

    uint16_t res_189(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x189 RES 1, C

    uint16_t res_18a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x18A RES 1, D

    uint16_t res_18b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x18B RES 1, E

    uint16_t res_18c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x18C RES 1, H

    uint16_t res_18d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x18D RES 1, L

    uint16_t res_18e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x18E RES 1, (HL)

    uint16_t res_18f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x18F RES 1, A

    uint16_t res_190(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x190 RES 2, B

    uint16_t res_191(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x191 RES 2, C

    uint16_t res_192(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x192 RES 2, D

    uint16_t res_193(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x193 RES 2, E

    uint16_t res_194(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x194 RES 2, H

    uint16_t res_195(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x195 RES 2, L

    uint16_t res_196(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x196 RES 2, (HL)

    uint16_t res_197(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x197 RES 2, A

    uint16_t res_198(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x198 RES 3, B

    uint16_t res_199(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x199 RES 3, C

    uint16_t res_19a(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x19A RES 3, D

    uint16_t res_19b(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x19B RES 3, E

    uint16_t res_19c(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x19C RES 3, H

    uint16_t res_19d(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x19D RES 3, L

    uint16_t res_19e(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x19E RES 3, (HL)

    uint16_t res_19f(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x19F RES 3, A

    uint16_t res_1a0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A0 RES 4, B

    uint16_t res_1a1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A1 RES 4, C

    uint16_t res_1a2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A2 RES 4, D

    uint16_t res_1a3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A3 RES 4, E

    uint16_t res_1a4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A4 RES 4, H

    uint16_t res_1a5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A5 RES 4, L

    uint16_t res_1a6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A6 RES 4, (HL)

    uint16_t res_1a7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A7 RES 4, A

    uint16_t res_1a8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A8 RES 5, B

    uint16_t res_1a9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1A9 RES 5, C

    uint16_t res_1aa(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1AA RES 5, D

    uint16_t res_1ab(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1AB RES 5, E

    uint16_t res_1ac(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1AC RES 5, H

    uint16_t res_1ad(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1AD RES 5, L

    uint16_t res_1ae(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1AE RES 5, (HL)

    uint16_t res_1af(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1AF RES 5, A

    uint16_t res_1b0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B0 RES 6, B

    uint16_t res_1b1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B1 RES 6, C

    uint16_t res_1b2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B2 RES 6, D

    uint16_t res_1b3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B3 RES 6, E

    uint16_t res_1b4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B4 RES 6, H

    uint16_t res_1b5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B5 RES 6, L

    uint16_t res_1b6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B6 RES 6, (HL)

    uint16_t res_1b7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B7 RES 6, A

    uint16_t res_1b8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B8 RES 7, B

    uint16_t res_1b9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1B9 RES 7, C

    uint16_t res_1ba(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1BA RES 7, D

    uint16_t res_1bb(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1BB RES 7, E

    uint16_t res_1bc(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1BC RES 7, H

    uint16_t res_1bd(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1BD RES 7, L

    uint16_t res_1be(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1BE RES 7, (HL)

    uint16_t res_1bf(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1BF RES 7, A

    uint16_t set_1c0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C0 SET 0, B

    uint16_t set_1c1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C1 SET 0, C

    uint16_t set_1c2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C2 SET 0, D

    uint16_t set_1c3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C3 SET 0, E

    uint16_t set_1c4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C4 SET 0, H

    uint16_t set_1c5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C5 SET 0, L

    uint16_t set_1c6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C6 SET 0, (HL)

    uint16_t set_1c7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C7 SET 0, A

    uint16_t set_1c8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C8 SET 1, B

    uint16_t set_1c9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1C9 SET 1, C

    uint16_t set_1ca(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1CA SET 1, D

    uint16_t set_1cb(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1CB SET 1, E

    uint16_t set_1cc(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1CC SET 1, H

    uint16_t set_1cd(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1CD SET 1, L

    uint16_t set_1ce(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1CE SET 1, (HL)

    uint16_t set_1cf(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1CF SET 1, A

    uint16_t set_1d0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D0 SET 2, B

    uint16_t set_1d1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D1 SET 2, C

    uint16_t set_1d2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D2 SET 2, D

    uint16_t set_1d3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D3 SET 2, E

    uint16_t set_1d4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D4 SET 2, H

    uint16_t set_1d5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D5 SET 2, L

    uint16_t set_1d6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D6 SET 2, (HL)

    uint16_t set_1d7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D7 SET 2, A

    uint16_t set_1d8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D8 SET 3, B

    uint16_t set_1d9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1D9 SET 3, C

    uint16_t set_1da(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1DA SET 3, D

    uint16_t set_1db(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1DB SET 3, E

    uint16_t set_1dc(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1DC SET 3, H

    uint16_t set_1dd(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1DD SET 3, L

    uint16_t set_1de(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1DE SET 3, (HL)

    uint16_t set_1df(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1DF SET 3, A

    uint16_t set_1e0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E0 SET 4, B

    uint16_t set_1e1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E1 SET 4, C

    uint16_t set_1e2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E2 SET 4, D

    uint16_t set_1e3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E3 SET 4, E

    uint16_t set_1e4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E4 SET 4, H

    uint16_t set_1e5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E5 SET 4, L

    uint16_t set_1e6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E6 SET 4, (HL)

    uint16_t set_1e7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E7 SET 4, A

    uint16_t set_1e8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E8 SET 5, B

    uint16_t set_1e9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1E9 SET 5, C

    uint16_t set_1ea(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1EA SET 5, D

    uint16_t set_1eb(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1EB SET 5, E

    uint16_t set_1ec(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1EC SET 5, H

    uint16_t set_1ed(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1ED SET 5, L

    uint16_t set_1ee(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1EE SET 5, (HL)

    uint16_t set_1ef(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1EF SET 5, A

    uint16_t set_1f0(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F0 SET 6, B

    uint16_t set_1f1(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F1 SET 6, C

    uint16_t set_1f2(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F2 SET 6, D

    uint16_t set_1f3(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F3 SET 6, E

    uint16_t set_1f4(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F4 SET 6, H

    uint16_t set_1f5(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F5 SET 6, L

    uint16_t set_1f6(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F6 SET 6, (HL)

    uint16_t set_1f7(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F7 SET 6, A

    uint16_t set_1f8(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F8 SET 7, B

    uint16_t set_1f9(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1F9 SET 7, C

    uint16_t set_1fa(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1FA SET 7, D

    uint16_t set_1fb(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1FB SET 7, E

    uint16_t set_1fc(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1FC SET 7, H

    uint16_t set_1fd(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1FD SET 7, L

    uint16_t set_1fe(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1FE SET 7, (HL)

    uint16_t set_1ff(const Arguments& arguments, Registers& registers, MemoryController& controller); // 0x1FF SET 7, A

    const std::array<InstructionFunction, 512> INSTRUCTION_FUNCTIONS = {
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
        &stop_010,
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
        &and_0a0,
        &and_0a1,
        &and_0a2,
        &and_0a3,
        &and_0a4,
        &and_0a5,
        &and_0a6,
        &and_0a7,
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
        &reti_0d9,
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
        &and_0e6,
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
        &swap_130,
        &swap_131,
        &swap_132,
        &swap_133,
        &swap_134,
        &swap_135,
        &swap_136,
        &swap_137,
        &srl_138,
        &srl_139,
        &srl_13a,
        &srl_13b,
        &srl_13c,
        &srl_13d,
        &srl_13e,
        &srl_13f,
        &bit_140,
        &bit_141,
        &bit_142,
        &bit_143,
        &bit_144,
        &bit_145,
        &bit_146,
        &bit_147,
        &bit_148,
        &bit_149,
        &bit_14a,
        &bit_14b,
        &bit_14c,
        &bit_14d,
        &bit_14e,
        &bit_14f,
        &bit_150,
        &bit_151,
        &bit_152,
        &bit_153,
        &bit_154,
        &bit_155,
        &bit_156,
        &bit_157,
        &bit_158,
        &bit_159,
        &bit_15a,
        &bit_15b,
        &bit_15c,
        &bit_15d,
        &bit_15e,
        &bit_15f,
        &bit_160,
        &bit_161,
        &bit_162,
        &bit_163,
        &bit_164,
        &bit_165,
        &bit_166,
        &bit_167,
        &bit_168,
        &bit_169,
        &bit_16a,
        &bit_16b,
        &bit_16c,
        &bit_16d,
        &bit_16e,
        &bit_16f,
        &bit_170,
        &bit_171,
        &bit_172,
        &bit_173,
        &bit_174,
        &bit_175,
        &bit_176,
        &bit_177,
        &bit_178,
        &bit_179,
        &bit_17a,
        &bit_17b,
        &bit_17c,
        &bit_17d,
        &bit_17e,
        &bit_17f,
        &res_180,
        &res_181,
        &res_182,
        &res_183,
        &res_184,
        &res_185,
        &res_186,
        &res_187,
        &res_188,
        &res_189,
        &res_18a,
        &res_18b,
        &res_18c,
        &res_18d,
        &res_18e,
        &res_18f,
        &res_190,
        &res_191,
        &res_192,
        &res_193,
        &res_194,
        &res_195,
        &res_196,
        &res_197,
        &res_198,
        &res_199,
        &res_19a,
        &res_19b,
        &res_19c,
        &res_19d,
        &res_19e,
        &res_19f,
        &res_1a0,
        &res_1a1,
        &res_1a2,
        &res_1a3,
        &res_1a4,
        &res_1a5,
        &res_1a6,
        &res_1a7,
        &res_1a8,
        &res_1a9,
        &res_1aa,
        &res_1ab,
        &res_1ac,
        &res_1ad,
        &res_1ae,
        &res_1af,
        &res_1b0,
        &res_1b1,
        &res_1b2,
        &res_1b3,
        &res_1b4,
        &res_1b5,
        &res_1b6,
        &res_1b7,
        &res_1b8,
        &res_1b9,
        &res_1ba,
        &res_1bb,
        &res_1bc,
        &res_1bd,
        &res_1be,
        &res_1bf,
        &set_1c0,
        &set_1c1,
        &set_1c2,
        &set_1c3,
        &set_1c4,
        &set_1c5,
        &set_1c6,
        &set_1c7,
        &set_1c8,
        &set_1c9,
        &set_1ca,
        &set_1cb,
        &set_1cc,
        &set_1cd,
        &set_1ce,
        &set_1cf,
        &set_1d0,
        &set_1d1,
        &set_1d2,
        &set_1d3,
        &set_1d4,
        &set_1d5,
        &set_1d6,
        &set_1d7,
        &set_1d8,
        &set_1d9,
        &set_1da,
        &set_1db,
        &set_1dc,
        &set_1dd,
        &set_1de,
        &set_1df,
        &set_1e0,
        &set_1e1,
        &set_1e2,
        &set_1e3,
        &set_1e4,
        &set_1e5,
        &set_1e6,
        &set_1e7,
        &set_1e8,
        &set_1e9,
        &set_1ea,
        &set_1eb,
        &set_1ec,
        &set_1ed,
        &set_1ee,
        &set_1ef,
        &set_1f0,
        &set_1f1,
        &set_1f2,
        &set_1f3,
        &set_1f4,
        &set_1f5,
        &set_1f6,
        &set_1f7,
        &set_1f8,
        &set_1f9,
        &set_1fa,
        &set_1fb,
        &set_1fc,
        &set_1fd,
        &set_1fe,
        &set_1ff
    };

    const std::array<ArgumentType, 512> INSTRUCTION_ARGUMENT_TYPES = {
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
        ArgumentType::none
    };
}