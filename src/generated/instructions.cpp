#pragma once

#include "emulator/generated/instructions.h"

constexpr uint8_t FLAG_CARRY = 4;
constexpr uint8_t FLAG_HALF_CARRY = 5;
constexpr uint8_t FLAG_AD_SUB = 6;
constexpr uint8_t FLAG_ZERO = 7;


namespace emulator::generated
{
    uint16_t nop_000(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x0 NOP
    {
        // Nothing to be done
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_001(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x1 LD BC, d16
    {
        registers.BC = arguments.uint16;
        registers.PC += 3;
        return 12;
    }
    
    uint16_t ld_002(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x2 LD (BC), A
    {
        controller.set(controller.get(registers.BC), registers.A);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_006(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x6 LD B, d8
    {
        registers.B = arguments.uint8;
        registers.PC += 2;
        return 8;
    }
    
    uint16_t ld_008(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x8 LD (a16), SP
    {
        controller.set16bits(controller.get(arguments.uint16), registers.SP);
        registers.PC += 3;
        return 20;
    }
    
    uint16_t ld_00a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xA LD A, (BC)
    {
        registers.A = controller.get(registers.BC);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_00e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xE LD C, d8
    {
        registers.C = arguments.uint8;
        registers.PC += 2;
        return 8;
    }
    
    uint16_t ld_011(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x11 LD DE, d16
    {
        registers.DE = arguments.uint16;
        registers.PC += 3;
        return 12;
    }
    
    uint16_t ld_012(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x12 LD (DE), A
    {
        controller.set(controller.get(registers.DE), registers.A);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_016(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x16 LD D, d8
    {
        registers.D = arguments.uint8;
        registers.PC += 2;
        return 8;
    }
    
    uint16_t ld_01a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x1A LD A, (DE)
    {
        registers.A = controller.get(registers.DE);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_01e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x1E LD E, d8
    {
        registers.E = arguments.uint8;
        registers.PC += 2;
        return 8;
    }
    
    uint16_t ld_021(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x21 LD HL, d16
    {
        registers.HL = arguments.uint16;
        registers.PC += 3;
        return 12;
    }
    
    uint16_t ldi_022(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x22 LDI (HL), A
    {
        controller.set(controller.get(registers.HL), registers.A);
        ++registers.HL;
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_026(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x26 LD H, d8
    {
        registers.H = arguments.uint8;
        registers.PC += 2;
        return 8;
    }
    
    uint16_t ldi_02a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x2A LDI A, (HL)
    {
        registers.A = controller.get(registers.HL);
        ++registers.HL;
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_02e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x2E LD L, d8
    {
        registers.L = arguments.uint8;
        registers.PC += 2;
        return 8;
    }
    
    uint16_t ld_031(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x31 LD SP, d16
    {
        registers.SP = arguments.uint16;
        registers.PC += 3;
        return 12;
    }
    
    uint16_t ldd_032(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x32 LDD (HL), A
    {
        controller.set(controller.get(registers.HL), registers.A);
        --registers.HL;
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_036(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x36 LD (HL), d8
    {
        controller.set(controller.get(registers.HL), arguments.uint8);
        registers.PC += 2;
        return 12;
    }
    
    uint16_t ldd_03a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x3A LDD A, (HL)
    {
        registers.A = controller.get(registers.HL);
        --registers.HL;
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_03e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x3E LD A, d8
    {
        registers.A = arguments.uint8;
        registers.PC += 2;
        return 8;
    }
    
    uint16_t ld_040(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x40 LD B, B
    {
        registers.B = registers.B;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_041(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x41 LD B, C
    {
        registers.B = registers.C;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_042(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x42 LD B, D
    {
        registers.B = registers.D;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_043(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x43 LD B, E
    {
        registers.B = registers.E;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_044(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x44 LD B, H
    {
        registers.B = registers.H;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_045(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x45 LD B, L
    {
        registers.B = registers.L;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_046(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x46 LD B, (HL)
    {
        registers.B = controller.get(registers.HL);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_047(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x47 LD B, A
    {
        registers.B = registers.A;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_048(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x48 LD C, B
    {
        registers.C = registers.B;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_049(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x49 LD C, C
    {
        registers.C = registers.C;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_04a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x4A LD C, D
    {
        registers.C = registers.D;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_04b(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x4B LD C, E
    {
        registers.C = registers.E;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_04c(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x4C LD C, H
    {
        registers.C = registers.H;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_04d(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x4D LD C, L
    {
        registers.C = registers.L;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_04e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x4E LD C, (HL)
    {
        registers.C = controller.get(registers.HL);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_04f(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x4F LD C, A
    {
        registers.C = registers.A;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_050(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x50 LD D, B
    {
        registers.D = registers.B;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_051(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x51 LD D, C
    {
        registers.D = registers.C;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_052(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x52 LD D, D
    {
        registers.D = registers.D;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_053(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x53 LD D, E
    {
        registers.D = registers.E;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_054(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x54 LD D, H
    {
        registers.D = registers.H;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_055(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x55 LD D, L
    {
        registers.D = registers.L;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_056(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x56 LD D, (HL)
    {
        registers.D = controller.get(registers.HL);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_057(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x57 LD D, A
    {
        registers.D = registers.A;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_058(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x58 LD E, B
    {
        registers.E = registers.B;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_059(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x59 LD E, C
    {
        registers.E = registers.C;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_05a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x5A LD E, D
    {
        registers.E = registers.D;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_05b(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x5B LD E, E
    {
        registers.E = registers.E;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_05c(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x5C LD E, H
    {
        registers.E = registers.H;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_05d(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x5D LD E, L
    {
        registers.E = registers.L;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_05e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x5E LD E, (HL)
    {
        registers.E = controller.get(registers.HL);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_05f(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x5F LD E, A
    {
        registers.E = registers.A;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_060(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x60 LD H, B
    {
        registers.H = registers.B;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_061(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x61 LD H, C
    {
        registers.H = registers.C;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_062(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x62 LD H, D
    {
        registers.H = registers.D;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_063(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x63 LD H, E
    {
        registers.H = registers.E;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_064(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x64 LD H, H
    {
        registers.H = registers.H;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_065(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x65 LD H, L
    {
        registers.H = registers.L;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_066(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x66 LD H, (HL)
    {
        registers.H = controller.get(registers.HL);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_067(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x67 LD H, A
    {
        registers.H = registers.A;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_068(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x68 LD L, B
    {
        registers.L = registers.B;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_069(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x69 LD L, C
    {
        registers.L = registers.C;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_06a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x6A LD L, D
    {
        registers.L = registers.D;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_06b(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x6B LD L, E
    {
        registers.L = registers.E;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_06c(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x6C LD L, H
    {
        registers.L = registers.H;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_06d(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x6D LD L, L
    {
        registers.L = registers.L;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_06e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x6E LD L, (HL)
    {
        registers.L = controller.get(registers.HL);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_06f(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x6F LD L, A
    {
        registers.L = registers.A;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_070(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x70 LD (HL), B
    {
        controller.set(controller.get(registers.HL), registers.B);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_071(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x71 LD (HL), C
    {
        controller.set(controller.get(registers.HL), registers.C);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_072(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x72 LD (HL), D
    {
        controller.set(controller.get(registers.HL), registers.D);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_073(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x73 LD (HL), E
    {
        controller.set(controller.get(registers.HL), registers.E);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_074(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x74 LD (HL), H
    {
        controller.set(controller.get(registers.HL), registers.H);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_075(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x75 LD (HL), L
    {
        controller.set(controller.get(registers.HL), registers.L);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_077(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x77 LD (HL), A
    {
        controller.set(controller.get(registers.HL), registers.A);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_078(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x78 LD A, B
    {
        registers.A = registers.B;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_079(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x79 LD A, C
    {
        registers.A = registers.C;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_07a(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x7A LD A, D
    {
        registers.A = registers.D;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_07b(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x7B LD A, E
    {
        registers.A = registers.E;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_07c(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x7C LD A, H
    {
        registers.A = registers.H;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_07d(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x7D LD A, L
    {
        registers.A = registers.L;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t ld_07e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x7E LD A, (HL)
    {
        registers.A = controller.get(registers.HL);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_07f(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x7F LD A, A
    {
        registers.A = registers.A;
        registers.PC += 1;
        return 4;
    }
    
    uint16_t unknown_0d3(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xD3 UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xD3");
    }
    
    uint16_t unknown_0db(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xDB UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xDB");
    }
    
    uint16_t unknown_0dd(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xDD UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xDD");
    }
    
    uint16_t ldh_0e0(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xE0 LDH (a8), A
    {
        controller.set((controller.get(arguments.uint8)) + 0xFF00, registers.A);
        registers.PC += 2;
        return 12;
    }
    
    uint16_t ldspecialc_0e2(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xE2 LDSpecialC (C), A
    {
        controller.set((controller.get(registers.C)) + 0xFF00, registers.A);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t unknown_0e3(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xE3 UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xE3");
    }
    
    uint16_t unknown_0e4(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xE4 UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xE4");
    }
    
    uint16_t ld_0ea(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xEA LD (a16), A
    {
        controller.set(controller.get(arguments.uint16), registers.A);
        registers.PC += 3;
        return 16;
    }
    
    uint16_t unknown_0eb(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xEB UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xEB");
    }
    
    uint16_t unknown_0ec(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xEC UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xEC");
    }
    
    uint16_t unknown_0ed(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xED UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xED");
    }
    
    uint16_t ldh_0f0(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xF0 LDH A, (a8)
    {
        registers.A = controller.get((arguments.uint8) + 0xFF00);
        registers.PC += 2;
        return 12;
    }
    
    uint16_t ldspecialc_0f2(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xF2 LDSpecialC A, (C)
    {
        registers.A = controller.get((registers.C) + 0xFF00);
        registers.PC += 1;
        return 8;
    }
    
    uint16_t unknown_0f4(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xF4 UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xF4");
    }
    
    uint16_t ldhl_0f8(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xF8 LDHL SP, r8
    {
        int32_t lhs = registers.SP;
        int32_t rhs = arguments.uint8;
        int32_t result = lhs + rhs;
        registers.F &= 0;
        registers.F |= 00000000 +
            ((((lhs & 0xFFF) + (rhs & 0xFFF)) > 0xFFF) << FLAG_HALF_CARRY) +
            ((result > 0xFFFF) << FLAG_CARRY);
        registers.HL = result & 0xFFFF;
        registers.PC += 2;
        return 12;
    }
    
    uint16_t ld_0f9(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xF9 LD SP, HL
    {
        registers.SP = registers.HL;
        registers.PC += 1;
        return 8;
    }
    
    uint16_t ld_0fa(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xFA LD A, (a16)
    {
        registers.A = controller.get(arguments.uint16);
        registers.PC += 3;
        return 16;
    }
    
    uint16_t unknown_0fc(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xFC UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xFC");
    }
    
    uint16_t unknown_0fd(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xFD UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xFD");
    }
}