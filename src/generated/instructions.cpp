#include "emulator/generated/instructions.h"

namespace emulator::generated
{
    uint16_t nop_000(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x0 NOP
    {
        // Nothing to be done
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_001(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x1 LD BC, d16
    {
        registers.set_BC(arguments.uint16);
        registers.PC += 3;
        return 12;
    }

    uint16_t ld_002(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x2 LD (BC), A
    {
        controller.set(registers.get_BC(), registers.A);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_003(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x3 INC BC
    {
        int32_t lhs = registers.get_BC();
        int32_t rhs = 1;
        int32_t result = lhs + rhs;
        // No flag operation
        registers.set_BC(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_004(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x4 INC B
    {
        int32_t lhs = registers.B;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xF) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= zero_flag + half_carry_flag;
        registers.B = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_005(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x5 DEC B
    {
        int32_t lhs = registers.B;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result < 0) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + zero_flag + half_carry_flag;
        registers.B = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_006(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x6 LD B, d8
    {
        registers.B = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t rlca_007(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x7 RLCA
    {
        uint8_t carry_flag = (registers.A >> 7) & 0b1;
        registers.A = (registers.A << 1) + carry_flag;
        registers.F &= 0b00000000;
        registers.F |= carry_flag;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_008(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x8 LD (a16), SP
    {
        controller.set16bits(arguments.uint16, registers.SP);
        registers.PC += 3;
        return 20;
    }

    uint16_t add_009(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x9 ADD HL, BC
    {
        int32_t lhs = registers.get_HL();
        int32_t rhs = registers.get_BC();
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b10000000;
        registers.F |= half_carry_flag + carry_flag;
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_00a(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0xA LD A, (BC)
    {
        registers.A = controller.get(registers.get_BC());
        registers.PC += 1;
        return 8;
    }

    uint16_t dec_00b(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0xB DEC BC
    {
        int32_t lhs = registers.get_BC();
        int32_t rhs = 1;
        int32_t result = lhs - rhs;
        // No flag operation
        registers.set_BC(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_00c(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0xC INC C
    {
        int32_t lhs = registers.C;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xF) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= zero_flag + half_carry_flag;
        registers.C = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_00d(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0xD DEC C
    {
        int32_t lhs = registers.C;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result < 0) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + zero_flag + half_carry_flag;
        registers.C = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_00e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0xE LD C, d8
    {
        registers.C = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t rrca_00f(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0xF RRCA
    {
        uint8_t carry_flag = registers.A & 0b1;
        registers.A = (registers.A >> 1) + (carry_flag << 7);
        registers.F &= 0b00000000;
        registers.F |= carry_flag;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_011(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x11 LD DE, d16
    {
        registers.set_DE(arguments.uint16);
        registers.PC += 3;
        return 12;
    }

    uint16_t ld_012(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x12 LD (DE), A
    {
        controller.set(registers.get_DE(), registers.A);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_013(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x13 INC DE
    {
        int32_t lhs = registers.get_DE();
        int32_t rhs = 1;
        int32_t result = lhs + rhs;
        // No flag operation
        registers.set_DE(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_014(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x14 INC D
    {
        int32_t lhs = registers.D;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xF) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= zero_flag + half_carry_flag;
        registers.D = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_015(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x15 DEC D
    {
        int32_t lhs = registers.D;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result < 0) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + zero_flag + half_carry_flag;
        registers.D = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_016(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x16 LD D, d8
    {
        registers.D = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t rla_017(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x17 RLA
    {
        uint8_t carry_flag = (registers.A >> 7) & 0b1;
        registers.A = (registers.A << 1) + registers.get_carry_flag();
        registers.F &= 0b00000000;
        registers.F |= carry_flag;
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_018(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x18 JR r8
    {
        registers.PC += 2 + arguments.int8;
        return 12;
    }

    uint16_t add_019(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x19 ADD HL, DE
    {
        int32_t lhs = registers.get_HL();
        int32_t rhs = registers.get_DE();
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b10000000;
        registers.F |= half_carry_flag + carry_flag;
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_01a(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x1A LD A, (DE)
    {
        registers.A = controller.get(registers.get_DE());
        registers.PC += 1;
        return 8;
    }

    uint16_t dec_01b(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x1B DEC DE
    {
        int32_t lhs = registers.get_DE();
        int32_t rhs = 1;
        int32_t result = lhs - rhs;
        // No flag operation
        registers.set_DE(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_01c(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x1C INC E
    {
        int32_t lhs = registers.E;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xF) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= zero_flag + half_carry_flag;
        registers.E = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_01d(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x1D DEC E
    {
        int32_t lhs = registers.E;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result < 0) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + zero_flag + half_carry_flag;
        registers.E = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_01e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x1E LD E, d8
    {
        registers.E = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t rra_01f(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x1F RRA
    {
        uint8_t carry_flag = registers.A & 0b1;
        registers.A = (registers.A >> 1) + (registers.get_carry_flag() << 7);
        registers.F &= 0b00000000;
        registers.F |= carry_flag;
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_020(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x20 JR NZ, r8
    {
        registers.PC += 2;
        if (registers.get_non_zero_flag())
            return 8;
        registers.PC += arguments.int8;
        return 12;
    }

    uint16_t ld_021(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x21 LD HL, d16
    {
        registers.set_HL(arguments.uint16);
        registers.PC += 3;
        return 12;
    }

    uint16_t ldi_022(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x22 LDI (HL), A
    {
        controller.set(registers.get_HL(), registers.A);
        registers.set_HL(registers.get_HL() + 1);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_023(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x23 INC HL
    {
        int32_t lhs = registers.get_HL();
        int32_t rhs = 1;
        int32_t result = lhs + rhs;
        // No flag operation
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_024(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x24 INC H
    {
        int32_t lhs = registers.H;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xF) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= zero_flag + half_carry_flag;
        registers.H = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_025(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x25 DEC H
    {
        int32_t lhs = registers.H;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result < 0) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + zero_flag + half_carry_flag;
        registers.H = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_026(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x26 LD H, d8
    {
        registers.H = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t daa_027(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x27 DAA
    {
        uint8_t carry_flag;
        if (registers.get_add_sub_flag())
        {
            carry_flag = registers.get_carry_flag();
            registers.A -= (0x6 * registers.get_half_carry_flag()) + (0x60 * carry_flag);
        }
        else
        {
            carry_flag = (0x99 < registers.A) || registers.get_carry_flag();
            registers.A += (0x6 * ((0xA < (registers.A & 0xF)) || registers.get_half_carry_flag())) +(0x60 * carry_flag);
        }
        uint8_t zero_flag = (registers.A == 0x00);
        registers.F &= 0b01000000;
        registers.F |= zero_flag + carry_flag;
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_028(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x28 JR Z, r8
    {
        registers.PC += 2;
        if (registers.get_zero_flag())
            return 8;
        registers.PC += arguments.int8;
        return 12;
    }

    uint16_t add_029(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x29 ADD HL, HL
    {
        int32_t lhs = registers.get_HL();
        int32_t rhs = registers.get_HL();
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b10000000;
        registers.F |= half_carry_flag + carry_flag;
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t ldi_02a(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x2A LDI A, (HL)
    {
        registers.A = controller.get(registers.get_HL());
        registers.set_HL(registers.get_HL() + 1);
        registers.PC += 1;
        return 8;
    }

    uint16_t dec_02b(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x2B DEC HL
    {
        int32_t lhs = registers.get_HL();
        int32_t rhs = 1;
        int32_t result = lhs - rhs;
        // No flag operation
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_02c(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x2C INC L
    {
        int32_t lhs = registers.L;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xF) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= zero_flag + half_carry_flag;
        registers.L = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_02d(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x2D DEC L
    {
        int32_t lhs = registers.L;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result < 0) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + zero_flag + half_carry_flag;
        registers.L = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_02e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x2E LD L, d8
    {
        registers.L = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t cpl_02f(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x2F CPL
    {
        registers.A = ~registers.A;
        registers.F &= 0b10010000;
        registers.F |= 0b01100000;
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_030(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x30 JR NC, r8
    {
        registers.PC += 2;
        if (registers.get_non_carry_flag())
            return 8;
        registers.PC += arguments.int8;
        return 12;
    }

    uint16_t ld_031(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x31 LD SP, d16
    {
        registers.SP = arguments.uint16;
        registers.PC += 3;
        return 12;
    }

    uint16_t ldd_032(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x32 LDD (HL), A
    {
        controller.set(registers.get_HL(), registers.A);
        registers.set_HL(registers.get_HL() - 1);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_033(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x33 INC SP
    {
        int32_t lhs = registers.SP;
        int32_t rhs = 1;
        int32_t result = lhs + rhs;
        // No flag operation
        registers.SP = result & 0xFFFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_034(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x34 INC (HL)
    {
        int32_t lhs = controller.get(registers.get_HL());
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xF) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= zero_flag + half_carry_flag;
        controller.set(registers.get_HL(), result & 0xFF);
        registers.PC += 1;
        return 12;
    }

    uint16_t dec_035(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x35 DEC (HL)
    {
        int32_t lhs = controller.get(registers.get_HL());
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result < 0) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + zero_flag + half_carry_flag;
        controller.set(registers.get_HL(), result & 0xFF);
        registers.PC += 1;
        return 12;
    }

    uint16_t ld_036(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0x36 LD (HL), d8
    {
        controller.set(registers.get_HL(), arguments.uint8);
        registers.PC += 2;
        return 12;
    }

    uint16_t scf_037(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x37 SCF
    {
        registers.F &= 0b10000000;
        registers.F |= 0b00010000;
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_038(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x38 JR C, r8
    {
        registers.PC += 2;
        if (registers.get_carry_flag())
            return 8;
        registers.PC += arguments.int8;
        return 12;
    }

    uint16_t add_039(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x39 ADD HL, SP
    {
        int32_t lhs = registers.get_HL();
        int32_t rhs = registers.SP;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b10000000;
        registers.F |= half_carry_flag + carry_flag;
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t ldd_03a(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x3A LDD A, (HL)
    {
        registers.A = controller.get(registers.get_HL());
        registers.set_HL(registers.get_HL() - 1);
        registers.PC += 1;
        return 8;
    }

    uint16_t dec_03b(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x3B DEC SP
    {
        int32_t lhs = registers.SP;
        int32_t rhs = 1;
        int32_t result = lhs - rhs;
        // No flag operation
        registers.SP = result & 0xFFFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_03c(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x3C INC A
    {
        int32_t lhs = registers.A;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xF) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= zero_flag + half_carry_flag;
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_03d(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x3D DEC A
    {
        int32_t lhs = registers.A;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result < 0) << OFFSET_HALF_CARRY_FLAG);
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + zero_flag + half_carry_flag;
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_03e(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0x3E LD A, d8
    {
        registers.A = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t ccf_03f(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x3F CCF
    {
        uint8_t carry_flag = !registers.get_carry_flag();
        registers.F &= 0b10000000;
        registers.F |= carry_flag;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_040(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x40 LD B, B
    {
        registers.B = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_041(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x41 LD B, C
    {
        registers.B = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_042(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x42 LD B, D
    {
        registers.B = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_043(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x43 LD B, E
    {
        registers.B = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_044(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x44 LD B, H
    {
        registers.B = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_045(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x45 LD B, L
    {
        registers.B = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_046(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x46 LD B, (HL)
    {
        registers.B = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_047(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x47 LD B, A
    {
        registers.B = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_048(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x48 LD C, B
    {
        registers.C = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_049(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x49 LD C, C
    {
        registers.C = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04a(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x4A LD C, D
    {
        registers.C = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04b(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x4B LD C, E
    {
        registers.C = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04c(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x4C LD C, H
    {
        registers.C = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04d(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x4D LD C, L
    {
        registers.C = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04e(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x4E LD C, (HL)
    {
        registers.C = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_04f(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x4F LD C, A
    {
        registers.C = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_050(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x50 LD D, B
    {
        registers.D = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_051(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x51 LD D, C
    {
        registers.D = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_052(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x52 LD D, D
    {
        registers.D = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_053(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x53 LD D, E
    {
        registers.D = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_054(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x54 LD D, H
    {
        registers.D = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_055(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x55 LD D, L
    {
        registers.D = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_056(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x56 LD D, (HL)
    {
        registers.D = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_057(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x57 LD D, A
    {
        registers.D = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_058(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x58 LD E, B
    {
        registers.E = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_059(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x59 LD E, C
    {
        registers.E = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05a(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x5A LD E, D
    {
        registers.E = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05b(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x5B LD E, E
    {
        registers.E = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05c(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x5C LD E, H
    {
        registers.E = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05d(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x5D LD E, L
    {
        registers.E = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05e(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x5E LD E, (HL)
    {
        registers.E = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_05f(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x5F LD E, A
    {
        registers.E = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_060(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x60 LD H, B
    {
        registers.H = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_061(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x61 LD H, C
    {
        registers.H = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_062(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x62 LD H, D
    {
        registers.H = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_063(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x63 LD H, E
    {
        registers.H = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_064(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x64 LD H, H
    {
        registers.H = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_065(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x65 LD H, L
    {
        registers.H = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_066(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x66 LD H, (HL)
    {
        registers.H = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_067(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x67 LD H, A
    {
        registers.H = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_068(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x68 LD L, B
    {
        registers.L = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_069(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x69 LD L, C
    {
        registers.L = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06a(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x6A LD L, D
    {
        registers.L = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06b(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x6B LD L, E
    {
        registers.L = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06c(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x6C LD L, H
    {
        registers.L = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06d(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x6D LD L, L
    {
        registers.L = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06e(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x6E LD L, (HL)
    {
        registers.L = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_06f(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x6F LD L, A
    {
        registers.L = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_070(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x70 LD (HL), B
    {
        controller.set(registers.get_HL(), registers.B);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_071(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x71 LD (HL), C
    {
        controller.set(registers.get_HL(), registers.C);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_072(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x72 LD (HL), D
    {
        controller.set(registers.get_HL(), registers.D);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_073(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x73 LD (HL), E
    {
        controller.set(registers.get_HL(), registers.E);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_074(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x74 LD (HL), H
    {
        controller.set(registers.get_HL(), registers.H);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_075(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x75 LD (HL), L
    {
        controller.set(registers.get_HL(), registers.L);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_077(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x77 LD (HL), A
    {
        controller.set(registers.get_HL(), registers.A);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_078(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x78 LD A, B
    {
        registers.A = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_079(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x79 LD A, C
    {
        registers.A = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07a(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x7A LD A, D
    {
        registers.A = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07b(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x7B LD A, E
    {
        registers.A = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07c(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x7C LD A, H
    {
        registers.A = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07d(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x7D LD A, L
    {
        registers.A = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07e(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x7E LD A, (HL)
    {
        registers.A = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_07f(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x7F LD A, A
    {
        registers.A = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_080(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x80 ADD A, B
    {
        int32_t lhs = registers.A;
        int32_t rhs = registers.B;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_081(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x81 ADD A, C
    {
        int32_t lhs = registers.A;
        int32_t rhs = registers.C;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_082(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x82 ADD A, D
    {
        int32_t lhs = registers.A;
        int32_t rhs = registers.D;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_083(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x83 ADD A, E
    {
        int32_t lhs = registers.A;
        int32_t rhs = registers.E;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_084(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x84 ADD A, H
    {
        int32_t lhs = registers.A;
        int32_t rhs = registers.H;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_085(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x85 ADD A, L
    {
        int32_t lhs = registers.A;
        int32_t rhs = registers.L;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_086(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0x86 ADD A, (HL)
    {
        int32_t lhs = registers.A;
        int32_t rhs = controller.get(registers.get_HL());
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t add_087(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0x87 ADD A, A
    {
        int32_t lhs = registers.A;
        int32_t rhs = registers.A;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_0c6(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0xC6 ADD A, d8
    {
        int32_t lhs = registers.A;
        int32_t rhs = arguments.uint8;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = ((result == 0) << OFFSET_ZERO_FLAG);
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= zero_flag + half_carry_flag + carry_flag;
        registers.A = result & 0xFFFF;
        registers.PC += 2;
        return 8;
    }

    uint16_t unknown_0d3(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xD3 UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xD3");
    }

    uint16_t unknown_0db(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xDB UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xDB");
    }

    uint16_t unknown_0dd(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xDD UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xDD");
    }

    uint16_t ldh_0e0(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xE0 LDH (a8), A
    {
        controller.set((arguments.uint8) + 0xFF00, registers.A);
        registers.PC += 2;
        return 12;
    }

    uint16_t ldspecialc_0e2(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0xE2 LDSpecialC (C), A
    {
        controller.set((registers.C) + 0xFF00, registers.A);
        registers.PC += 1;
        return 8;
    }

    uint16_t unknown_0e3(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xE3 UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xE3");
    }

    uint16_t unknown_0e4(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xE4 UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xE4");
    }

    uint16_t add_0e8(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0xE8 ADD SP, r8
    {
        int32_t lhs = registers.SP;
        int32_t rhs = arguments.int8;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= half_carry_flag + carry_flag;
        registers.SP = result & 0xFFFF;
        registers.PC += 2;
        return 16;
    }

    uint16_t ld_0ea(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xEA LD (a16), A
    {
        controller.set(arguments.uint16, registers.A);
        registers.PC += 3;
        return 16;
    }

    uint16_t unknown_0eb(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xEB UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xEB");
    }

    uint16_t unknown_0ec(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xEC UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xEC");
    }

    uint16_t unknown_0ed(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xED UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xED");
    }

    uint16_t ldh_0f0(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xF0 LDH A, (a8)
    {
        registers.A = controller.get((arguments.uint8) + 0xFF00);
        registers.PC += 2;
        return 12;
    }

    uint16_t ldspecialc_0f2(const Arguments& , emulator::Registers& registers, emulator::MemoryController& controller) // 0xF2 LDSpecialC A, (C)
    {
        registers.A = controller.get((registers.C) + 0xFF00);
        registers.PC += 1;
        return 8;
    }

    uint16_t unknown_0f4(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xF4 UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xF4");
    }

    uint16_t ldhl_0f8(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& ) // 0xF8 LDHL SP, r8
    {
        int32_t lhs = registers.SP;
        int32_t rhs = arguments.int8;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = ((half_result > 0xFFF) << OFFSET_HALF_CARRY_FLAG);
        uint8_t carry_flag = ((result > 0xFFFF) << OFFSET_CARRY_FLAG);
        registers.F &= 0b00000000;
        registers.F |= half_carry_flag + carry_flag;
        registers.set_HL(result & 0xFFFF);
        registers.PC += 2;
        return 12;
    }

    uint16_t ld_0f9(const Arguments& , emulator::Registers& registers, emulator::MemoryController& ) // 0xF9 LD SP, HL
    {
        registers.SP = registers.get_HL();
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_0fa(const Arguments& arguments, emulator::Registers& registers, emulator::MemoryController& controller) // 0xFA LD A, (a16)
    {
        registers.A = controller.get(arguments.uint16);
        registers.PC += 3;
        return 16;
    }

    uint16_t unknown_0fc(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xFC UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xFC");
    }

    uint16_t unknown_0fd(const Arguments& , emulator::Registers& , emulator::MemoryController& ) // 0xFD UNKNOWN
    {
        throw std::runtime_error("Unknown opcode 0xFD");
    }
}