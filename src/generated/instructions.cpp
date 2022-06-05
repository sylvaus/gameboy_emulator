#include <stdexcept>

#include "emulator/logging.h"
#include "emulator/utils/to_string.h"
#include "emulator/generated/instructions.h"

namespace emulator::generated
{
    using emulator::memory::OFFSET_CARRY_FLAG;
    using emulator::memory::OFFSET_HALF_CARRY_FLAG;
    using emulator::memory::OFFSET_ADD_SUB_FLAG;
    using emulator::memory::OFFSET_ZERO_FLAG;


    constexpr bool TRACE_LOG = false;
    auto LOGGER = Logging::get_logger("Instruction");

    uint16_t nop_000(const Arguments& arguments, Registers& registers, Memory& ) // 0x0 NOP
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x0 NOP");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        // Nothing to be done
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_001(const Arguments& arguments, Registers& registers, Memory& ) // 0x1 LD BC, d16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1 LD BC, d16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.set_BC(arguments.uint16);
        registers.PC += 3;
        return 12;
    }

    uint16_t ld_002(const Arguments& arguments, Registers& registers, Memory& controller) // 0x2 LD (BC), A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x2 LD (BC), A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_BC(), registers.A);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_003(const Arguments& arguments, Registers& registers, Memory& ) // 0x3 INC BC
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x3 INC BC");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_BC();
        int32_t rhs = 1;
        int32_t result = lhs + rhs;
        // No flag operation
        registers.set_BC(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_004(const Arguments& arguments, Registers& registers, Memory& ) // 0x4 INC B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x4 INC B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.B;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        registers.F &= 0b00010000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.B = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_005(const Arguments& arguments, Registers& registers, Memory& ) // 0x5 DEC B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x5 DEC B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.B;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.B = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_006(const Arguments& arguments, Registers& registers, Memory& ) // 0x6 LD B, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x6 LD B, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t rlca_007(const Arguments& arguments, Registers& registers, Memory& ) // 0x7 RLCA
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x7 RLCA");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_008(const Arguments& arguments, Registers& registers, Memory& controller) // 0x8 LD (a16), SP
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x8 LD (a16), SP");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set16bits(arguments.uint16, registers.SP);
        registers.PC += 3;
        return 20;
    }

    uint16_t add_009(const Arguments& arguments, Registers& registers, Memory& ) // 0x9 ADD HL, BC
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x9 ADD HL, BC");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_HL();
        int32_t rhs = registers.get_BC();
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = half_result > 0xFFF;
        uint8_t carry_flag = result > 0xFFFF;
        registers.F &= 0b10000000;
        registers.F |= (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_00a(const Arguments& arguments, Registers& registers, Memory& controller) // 0xA LD A, (BC)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA LD A, (BC)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = controller.get(registers.get_BC());
        registers.PC += 1;
        return 8;
    }

    uint16_t dec_00b(const Arguments& arguments, Registers& registers, Memory& ) // 0xB DEC BC
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB DEC BC");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_BC();
        int32_t rhs = 1;
        int32_t result = lhs - rhs;
        // No flag operation
        registers.set_BC(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_00c(const Arguments& arguments, Registers& registers, Memory& ) // 0xC INC C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC INC C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.C;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        registers.F &= 0b00010000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.C = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_00d(const Arguments& arguments, Registers& registers, Memory& ) // 0xD DEC C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD DEC C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.C;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.C = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_00e(const Arguments& arguments, Registers& registers, Memory& ) // 0xE LD C, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE LD C, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t rrca_00f(const Arguments& arguments, Registers& registers, Memory& ) // 0xF RRCA
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF RRCA");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t stop_010(const Arguments& arguments, Registers& registers, Memory& ) // 0x10 STOP 0
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x10 STOP 0");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.stopped = true;
        registers.PC += 2;
        return 4;
    }

    uint16_t ld_011(const Arguments& arguments, Registers& registers, Memory& ) // 0x11 LD DE, d16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x11 LD DE, d16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.set_DE(arguments.uint16);
        registers.PC += 3;
        return 12;
    }

    uint16_t ld_012(const Arguments& arguments, Registers& registers, Memory& controller) // 0x12 LD (DE), A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x12 LD (DE), A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_DE(), registers.A);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_013(const Arguments& arguments, Registers& registers, Memory& ) // 0x13 INC DE
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x13 INC DE");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_DE();
        int32_t rhs = 1;
        int32_t result = lhs + rhs;
        // No flag operation
        registers.set_DE(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_014(const Arguments& arguments, Registers& registers, Memory& ) // 0x14 INC D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x14 INC D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.D;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        registers.F &= 0b00010000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.D = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_015(const Arguments& arguments, Registers& registers, Memory& ) // 0x15 DEC D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x15 DEC D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.D;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.D = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_016(const Arguments& arguments, Registers& registers, Memory& ) // 0x16 LD D, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x16 LD D, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t rla_017(const Arguments& arguments, Registers& registers, Memory& ) // 0x17 RLA
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x17 RLA");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_018(const Arguments& arguments, Registers& registers, Memory& ) // 0x18 JR r8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x18 JR r8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 2 + arguments.int8;
        return 12;
    }

    uint16_t add_019(const Arguments& arguments, Registers& registers, Memory& ) // 0x19 ADD HL, DE
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x19 ADD HL, DE");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_HL();
        int32_t rhs = registers.get_DE();
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = half_result > 0xFFF;
        uint8_t carry_flag = result > 0xFFFF;
        registers.F &= 0b10000000;
        registers.F |= (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_01a(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1A LD A, (DE)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A LD A, (DE)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = controller.get(registers.get_DE());
        registers.PC += 1;
        return 8;
    }

    uint16_t dec_01b(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B DEC DE
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B DEC DE");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_DE();
        int32_t rhs = 1;
        int32_t result = lhs - rhs;
        // No flag operation
        registers.set_DE(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_01c(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C INC E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C INC E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.E;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        registers.F &= 0b00010000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.E = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_01d(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D DEC E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D DEC E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.E;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.E = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_01e(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E LD E, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E LD E, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t rra_01f(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F RRA
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F RRA");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_020(const Arguments& arguments, Registers& registers, Memory& ) // 0x20 JR NZ, r8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x20 JR NZ, r8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 2;
        if (!registers.get_non_zero_flag())
            return 8;
        registers.PC += arguments.int8;
        return 12;
    }

    uint16_t ld_021(const Arguments& arguments, Registers& registers, Memory& ) // 0x21 LD HL, d16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x21 LD HL, d16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.set_HL(arguments.uint16);
        registers.PC += 3;
        return 12;
    }

    uint16_t ldi_022(const Arguments& arguments, Registers& registers, Memory& controller) // 0x22 LDI (HL), A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x22 LDI (HL), A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.A);
        registers.set_HL(registers.get_HL() + 1);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_023(const Arguments& arguments, Registers& registers, Memory& ) // 0x23 INC HL
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x23 INC HL");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_HL();
        int32_t rhs = 1;
        int32_t result = lhs + rhs;
        // No flag operation
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_024(const Arguments& arguments, Registers& registers, Memory& ) // 0x24 INC H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x24 INC H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.H;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        registers.F &= 0b00010000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.H = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_025(const Arguments& arguments, Registers& registers, Memory& ) // 0x25 DEC H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x25 DEC H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.H;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.H = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_026(const Arguments& arguments, Registers& registers, Memory& ) // 0x26 LD H, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x26 LD H, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t daa_027(const Arguments& arguments, Registers& registers, Memory& ) // 0x27 DAA
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x27 DAA");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
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
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_028(const Arguments& arguments, Registers& registers, Memory& ) // 0x28 JR Z, r8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x28 JR Z, r8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 2;
        if (!registers.get_zero_flag())
            return 8;
        registers.PC += arguments.int8;
        return 12;
    }

    uint16_t add_029(const Arguments& arguments, Registers& registers, Memory& ) // 0x29 ADD HL, HL
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x29 ADD HL, HL");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_HL();
        int32_t rhs = registers.get_HL();
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = half_result > 0xFFF;
        uint8_t carry_flag = result > 0xFFFF;
        registers.F &= 0b10000000;
        registers.F |= (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t ldi_02a(const Arguments& arguments, Registers& registers, Memory& controller) // 0x2A LDI A, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x2A LDI A, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = controller.get(registers.get_HL());
        registers.set_HL(registers.get_HL() + 1);
        registers.PC += 1;
        return 8;
    }

    uint16_t dec_02b(const Arguments& arguments, Registers& registers, Memory& ) // 0x2B DEC HL
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x2B DEC HL");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_HL();
        int32_t rhs = 1;
        int32_t result = lhs - rhs;
        // No flag operation
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_02c(const Arguments& arguments, Registers& registers, Memory& ) // 0x2C INC L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x2C INC L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.L;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        registers.F &= 0b00010000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.L = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_02d(const Arguments& arguments, Registers& registers, Memory& ) // 0x2D DEC L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x2D DEC L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.L;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.L = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_02e(const Arguments& arguments, Registers& registers, Memory& ) // 0x2E LD L, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x2E LD L, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t cpl_02f(const Arguments& arguments, Registers& registers, Memory& ) // 0x2F CPL
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x2F CPL");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = ~registers.A;
        registers.F &= 0b10010000;
        registers.F |= 0b01100000;
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_030(const Arguments& arguments, Registers& registers, Memory& ) // 0x30 JR NC, r8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x30 JR NC, r8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 2;
        if (!registers.get_non_carry_flag())
            return 8;
        registers.PC += arguments.int8;
        return 12;
    }

    uint16_t ld_031(const Arguments& arguments, Registers& registers, Memory& ) // 0x31 LD SP, d16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x31 LD SP, d16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.SP = arguments.uint16;
        registers.PC += 3;
        return 12;
    }

    uint16_t ldd_032(const Arguments& arguments, Registers& registers, Memory& controller) // 0x32 LDD (HL), A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x32 LDD (HL), A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.A);
        registers.set_HL(registers.get_HL() - 1);
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_033(const Arguments& arguments, Registers& registers, Memory& ) // 0x33 INC SP
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x33 INC SP");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.SP;
        int32_t rhs = 1;
        int32_t result = lhs + rhs;
        // No flag operation
        registers.SP = result & 0xFFFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_034(const Arguments& arguments, Registers& registers, Memory& controller) // 0x34 INC (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x34 INC (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = controller.get(registers.get_HL());
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        registers.F &= 0b00010000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        controller.set(registers.get_HL(), result & 0xFF);
        registers.PC += 1;
        return 12;
    }

    uint16_t dec_035(const Arguments& arguments, Registers& registers, Memory& controller) // 0x35 DEC (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x35 DEC (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = controller.get(registers.get_HL());
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        controller.set(registers.get_HL(), result & 0xFF);
        registers.PC += 1;
        return 12;
    }

    uint16_t ld_036(const Arguments& arguments, Registers& registers, Memory& controller) // 0x36 LD (HL), d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x36 LD (HL), d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), arguments.uint8);
        registers.PC += 2;
        return 12;
    }

    uint16_t scf_037(const Arguments& arguments, Registers& registers, Memory& ) // 0x37 SCF
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x37 SCF");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.F &= 0b10000000;
        registers.F |= 0b00010000;
        registers.PC += 1;
        return 4;
    }

    uint16_t jr_038(const Arguments& arguments, Registers& registers, Memory& ) // 0x38 JR C, r8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x38 JR C, r8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 2;
        if (!registers.get_carry_flag())
            return 8;
        registers.PC += arguments.int8;
        return 12;
    }

    uint16_t add_039(const Arguments& arguments, Registers& registers, Memory& ) // 0x39 ADD HL, SP
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x39 ADD HL, SP");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.get_HL();
        int32_t rhs = registers.SP;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = half_result > 0xFFF;
        uint8_t carry_flag = result > 0xFFFF;
        registers.F &= 0b10000000;
        registers.F |= (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.set_HL(result & 0xFFFF);
        registers.PC += 1;
        return 8;
    }

    uint16_t ldd_03a(const Arguments& arguments, Registers& registers, Memory& controller) // 0x3A LDD A, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x3A LDD A, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = controller.get(registers.get_HL());
        registers.set_HL(registers.get_HL() - 1);
        registers.PC += 1;
        return 8;
    }

    uint16_t dec_03b(const Arguments& arguments, Registers& registers, Memory& ) // 0x3B DEC SP
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x3B DEC SP");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.SP;
        int32_t rhs = 1;
        int32_t result = lhs - rhs;
        // No flag operation
        registers.SP = result & 0xFFFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t inc_03c(const Arguments& arguments, Registers& registers, Memory& ) // 0x3C INC A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x3C INC A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        registers.F &= 0b00010000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t dec_03d(const Arguments& arguments, Registers& registers, Memory& ) // 0x3D DEC A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x3D DEC A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = 1;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        registers.F &= 0b00010000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_03e(const Arguments& arguments, Registers& registers, Memory& ) // 0x3E LD A, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x3E LD A, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = arguments.uint8;
        registers.PC += 2;
        return 8;
    }

    uint16_t ccf_03f(const Arguments& arguments, Registers& registers, Memory& ) // 0x3F CCF
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x3F CCF");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t carry_flag = !registers.get_carry_flag();
        registers.F &= 0b10000000;
        registers.F |= (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_040(const Arguments& arguments, Registers& registers, Memory& ) // 0x40 LD B, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x40 LD B, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_041(const Arguments& arguments, Registers& registers, Memory& ) // 0x41 LD B, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x41 LD B, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_042(const Arguments& arguments, Registers& registers, Memory& ) // 0x42 LD B, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x42 LD B, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_043(const Arguments& arguments, Registers& registers, Memory& ) // 0x43 LD B, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x43 LD B, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_044(const Arguments& arguments, Registers& registers, Memory& ) // 0x44 LD B, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x44 LD B, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_045(const Arguments& arguments, Registers& registers, Memory& ) // 0x45 LD B, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x45 LD B, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_046(const Arguments& arguments, Registers& registers, Memory& controller) // 0x46 LD B, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x46 LD B, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_047(const Arguments& arguments, Registers& registers, Memory& ) // 0x47 LD B, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x47 LD B, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_048(const Arguments& arguments, Registers& registers, Memory& ) // 0x48 LD C, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x48 LD C, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_049(const Arguments& arguments, Registers& registers, Memory& ) // 0x49 LD C, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x49 LD C, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04a(const Arguments& arguments, Registers& registers, Memory& ) // 0x4A LD C, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x4A LD C, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04b(const Arguments& arguments, Registers& registers, Memory& ) // 0x4B LD C, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x4B LD C, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04c(const Arguments& arguments, Registers& registers, Memory& ) // 0x4C LD C, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x4C LD C, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04d(const Arguments& arguments, Registers& registers, Memory& ) // 0x4D LD C, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x4D LD C, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_04e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x4E LD C, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x4E LD C, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_04f(const Arguments& arguments, Registers& registers, Memory& ) // 0x4F LD C, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x4F LD C, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_050(const Arguments& arguments, Registers& registers, Memory& ) // 0x50 LD D, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x50 LD D, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_051(const Arguments& arguments, Registers& registers, Memory& ) // 0x51 LD D, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x51 LD D, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_052(const Arguments& arguments, Registers& registers, Memory& ) // 0x52 LD D, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x52 LD D, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_053(const Arguments& arguments, Registers& registers, Memory& ) // 0x53 LD D, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x53 LD D, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_054(const Arguments& arguments, Registers& registers, Memory& ) // 0x54 LD D, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x54 LD D, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_055(const Arguments& arguments, Registers& registers, Memory& ) // 0x55 LD D, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x55 LD D, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_056(const Arguments& arguments, Registers& registers, Memory& controller) // 0x56 LD D, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x56 LD D, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_057(const Arguments& arguments, Registers& registers, Memory& ) // 0x57 LD D, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x57 LD D, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_058(const Arguments& arguments, Registers& registers, Memory& ) // 0x58 LD E, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x58 LD E, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_059(const Arguments& arguments, Registers& registers, Memory& ) // 0x59 LD E, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x59 LD E, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05a(const Arguments& arguments, Registers& registers, Memory& ) // 0x5A LD E, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x5A LD E, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05b(const Arguments& arguments, Registers& registers, Memory& ) // 0x5B LD E, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x5B LD E, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05c(const Arguments& arguments, Registers& registers, Memory& ) // 0x5C LD E, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x5C LD E, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05d(const Arguments& arguments, Registers& registers, Memory& ) // 0x5D LD E, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x5D LD E, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_05e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x5E LD E, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x5E LD E, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_05f(const Arguments& arguments, Registers& registers, Memory& ) // 0x5F LD E, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x5F LD E, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_060(const Arguments& arguments, Registers& registers, Memory& ) // 0x60 LD H, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x60 LD H, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_061(const Arguments& arguments, Registers& registers, Memory& ) // 0x61 LD H, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x61 LD H, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_062(const Arguments& arguments, Registers& registers, Memory& ) // 0x62 LD H, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x62 LD H, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_063(const Arguments& arguments, Registers& registers, Memory& ) // 0x63 LD H, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x63 LD H, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_064(const Arguments& arguments, Registers& registers, Memory& ) // 0x64 LD H, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x64 LD H, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_065(const Arguments& arguments, Registers& registers, Memory& ) // 0x65 LD H, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x65 LD H, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_066(const Arguments& arguments, Registers& registers, Memory& controller) // 0x66 LD H, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x66 LD H, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_067(const Arguments& arguments, Registers& registers, Memory& ) // 0x67 LD H, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x67 LD H, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_068(const Arguments& arguments, Registers& registers, Memory& ) // 0x68 LD L, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x68 LD L, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_069(const Arguments& arguments, Registers& registers, Memory& ) // 0x69 LD L, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x69 LD L, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06a(const Arguments& arguments, Registers& registers, Memory& ) // 0x6A LD L, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x6A LD L, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06b(const Arguments& arguments, Registers& registers, Memory& ) // 0x6B LD L, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x6B LD L, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06c(const Arguments& arguments, Registers& registers, Memory& ) // 0x6C LD L, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x6C LD L, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06d(const Arguments& arguments, Registers& registers, Memory& ) // 0x6D LD L, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x6D LD L, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_06e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x6E LD L, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x6E LD L, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_06f(const Arguments& arguments, Registers& registers, Memory& ) // 0x6F LD L, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x6F LD L, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_070(const Arguments& arguments, Registers& registers, Memory& controller) // 0x70 LD (HL), B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x70 LD (HL), B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.B);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_071(const Arguments& arguments, Registers& registers, Memory& controller) // 0x71 LD (HL), C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x71 LD (HL), C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.C);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_072(const Arguments& arguments, Registers& registers, Memory& controller) // 0x72 LD (HL), D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x72 LD (HL), D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.D);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_073(const Arguments& arguments, Registers& registers, Memory& controller) // 0x73 LD (HL), E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x73 LD (HL), E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.E);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_074(const Arguments& arguments, Registers& registers, Memory& controller) // 0x74 LD (HL), H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x74 LD (HL), H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.H);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_075(const Arguments& arguments, Registers& registers, Memory& controller) // 0x75 LD (HL), L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x75 LD (HL), L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.L);
        registers.PC += 1;
        return 8;
    }

    uint16_t halt_076(const Arguments& arguments, Registers& registers, Memory& ) // 0x76 HALT
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x76 HALT");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.halted = true;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_077(const Arguments& arguments, Registers& registers, Memory& controller) // 0x77 LD (HL), A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x77 LD (HL), A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), registers.A);
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_078(const Arguments& arguments, Registers& registers, Memory& ) // 0x78 LD A, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x78 LD A, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.B;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_079(const Arguments& arguments, Registers& registers, Memory& ) // 0x79 LD A, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x79 LD A, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.C;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07a(const Arguments& arguments, Registers& registers, Memory& ) // 0x7A LD A, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x7A LD A, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.D;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07b(const Arguments& arguments, Registers& registers, Memory& ) // 0x7B LD A, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x7B LD A, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.E;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07c(const Arguments& arguments, Registers& registers, Memory& ) // 0x7C LD A, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x7C LD A, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.H;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07d(const Arguments& arguments, Registers& registers, Memory& ) // 0x7D LD A, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x7D LD A, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.L;
        registers.PC += 1;
        return 4;
    }

    uint16_t ld_07e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x7E LD A, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x7E LD A, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = controller.get(registers.get_HL());
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_07f(const Arguments& arguments, Registers& registers, Memory& ) // 0x7F LD A, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x7F LD A, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_080(const Arguments& arguments, Registers& registers, Memory& ) // 0x80 ADD A, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x80 ADD A, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.B;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_081(const Arguments& arguments, Registers& registers, Memory& ) // 0x81 ADD A, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x81 ADD A, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.C;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_082(const Arguments& arguments, Registers& registers, Memory& ) // 0x82 ADD A, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x82 ADD A, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.D;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_083(const Arguments& arguments, Registers& registers, Memory& ) // 0x83 ADD A, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x83 ADD A, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.E;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_084(const Arguments& arguments, Registers& registers, Memory& ) // 0x84 ADD A, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x84 ADD A, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.H;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_085(const Arguments& arguments, Registers& registers, Memory& ) // 0x85 ADD A, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x85 ADD A, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.L;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t add_086(const Arguments& arguments, Registers& registers, Memory& controller) // 0x86 ADD A, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x86 ADD A, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = controller.get(registers.get_HL());
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t add_087(const Arguments& arguments, Registers& registers, Memory& ) // 0x87 ADD A, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x87 ADD A, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.A;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t adc_088(const Arguments& arguments, Registers& registers, Memory& ) // 0x88 ADC A, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x88 ADC A, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.B;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t adc_089(const Arguments& arguments, Registers& registers, Memory& ) // 0x89 ADC A, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x89 ADC A, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.C;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t adc_08a(const Arguments& arguments, Registers& registers, Memory& ) // 0x8A ADC A, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x8A ADC A, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.D;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t adc_08b(const Arguments& arguments, Registers& registers, Memory& ) // 0x8B ADC A, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x8B ADC A, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.E;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t adc_08c(const Arguments& arguments, Registers& registers, Memory& ) // 0x8C ADC A, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x8C ADC A, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.H;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t adc_08d(const Arguments& arguments, Registers& registers, Memory& ) // 0x8D ADC A, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x8D ADC A, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.L;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t adc_08e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x8E ADC A, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x8E ADC A, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = controller.get(registers.get_HL());
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t adc_08f(const Arguments& arguments, Registers& registers, Memory& ) // 0x8F ADC A, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x8F ADC A, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.A;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sub_090(const Arguments& arguments, Registers& registers, Memory& ) // 0x90 SUB A, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x90 SUB A, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.B;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sub_091(const Arguments& arguments, Registers& registers, Memory& ) // 0x91 SUB A, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x91 SUB A, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.C;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sub_092(const Arguments& arguments, Registers& registers, Memory& ) // 0x92 SUB A, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x92 SUB A, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.D;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sub_093(const Arguments& arguments, Registers& registers, Memory& ) // 0x93 SUB A, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x93 SUB A, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.E;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sub_094(const Arguments& arguments, Registers& registers, Memory& ) // 0x94 SUB A, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x94 SUB A, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.H;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sub_095(const Arguments& arguments, Registers& registers, Memory& ) // 0x95 SUB A, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x95 SUB A, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.L;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sub_096(const Arguments& arguments, Registers& registers, Memory& controller) // 0x96 SUB A, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x96 SUB A, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = controller.get(registers.get_HL());
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t sub_097(const Arguments& arguments, Registers& registers, Memory& ) // 0x97 SUB A, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x97 SUB A, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.A;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sbc_098(const Arguments& arguments, Registers& registers, Memory& ) // 0x98 SBC A, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x98 SBC A, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.B;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sbc_099(const Arguments& arguments, Registers& registers, Memory& ) // 0x99 SBC A, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x99 SBC A, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.C;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sbc_09a(const Arguments& arguments, Registers& registers, Memory& ) // 0x9A SBC A, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x9A SBC A, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.D;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sbc_09b(const Arguments& arguments, Registers& registers, Memory& ) // 0x9B SBC A, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x9B SBC A, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.E;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sbc_09c(const Arguments& arguments, Registers& registers, Memory& ) // 0x9C SBC A, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x9C SBC A, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.H;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sbc_09d(const Arguments& arguments, Registers& registers, Memory& ) // 0x9D SBC A, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x9D SBC A, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.L;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t sbc_09e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x9E SBC A, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x9E SBC A, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = controller.get(registers.get_HL());
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 8;
    }

    uint16_t sbc_09f(const Arguments& arguments, Registers& registers, Memory& ) // 0x9F SBC A, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x9F SBC A, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.A;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 1;
        return 4;
    }

    uint16_t and_0a0(const Arguments& arguments, Registers& registers, Memory& ) // 0xA0 AND B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA0 AND B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= registers.B;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t and_0a1(const Arguments& arguments, Registers& registers, Memory& ) // 0xA1 AND C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA1 AND C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= registers.C;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t and_0a2(const Arguments& arguments, Registers& registers, Memory& ) // 0xA2 AND D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA2 AND D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= registers.D;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t and_0a3(const Arguments& arguments, Registers& registers, Memory& ) // 0xA3 AND E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA3 AND E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= registers.E;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t and_0a4(const Arguments& arguments, Registers& registers, Memory& ) // 0xA4 AND H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA4 AND H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= registers.H;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t and_0a5(const Arguments& arguments, Registers& registers, Memory& ) // 0xA5 AND L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA5 AND L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= registers.L;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t and_0a6(const Arguments& arguments, Registers& registers, Memory& controller) // 0xA6 AND (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA6 AND (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= controller.get(registers.get_HL());
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 8;
    }

    uint16_t and_0a7(const Arguments& arguments, Registers& registers, Memory& ) // 0xA7 AND A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA7 AND A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= registers.A;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t xor_0a8(const Arguments& arguments, Registers& registers, Memory& ) // 0xA8 XOR B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA8 XOR B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= registers.B;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t xor_0a9(const Arguments& arguments, Registers& registers, Memory& ) // 0xA9 XOR C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xA9 XOR C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= registers.C;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t xor_0aa(const Arguments& arguments, Registers& registers, Memory& ) // 0xAA XOR D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xAA XOR D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= registers.D;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t xor_0ab(const Arguments& arguments, Registers& registers, Memory& ) // 0xAB XOR E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xAB XOR E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= registers.E;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t xor_0ac(const Arguments& arguments, Registers& registers, Memory& ) // 0xAC XOR H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xAC XOR H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= registers.H;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t xor_0ad(const Arguments& arguments, Registers& registers, Memory& ) // 0xAD XOR L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xAD XOR L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= registers.L;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t xor_0ae(const Arguments& arguments, Registers& registers, Memory& controller) // 0xAE XOR (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xAE XOR (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= controller.get(registers.get_HL());
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 8;
    }

    uint16_t xor_0af(const Arguments& arguments, Registers& registers, Memory& ) // 0xAF XOR A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xAF XOR A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= registers.A;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t or_0b0(const Arguments& arguments, Registers& registers, Memory& ) // 0xB0 OR B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB0 OR B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= registers.B;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t or_0b1(const Arguments& arguments, Registers& registers, Memory& ) // 0xB1 OR C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB1 OR C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= registers.C;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t or_0b2(const Arguments& arguments, Registers& registers, Memory& ) // 0xB2 OR D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB2 OR D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= registers.D;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t or_0b3(const Arguments& arguments, Registers& registers, Memory& ) // 0xB3 OR E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB3 OR E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= registers.E;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t or_0b4(const Arguments& arguments, Registers& registers, Memory& ) // 0xB4 OR H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB4 OR H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= registers.H;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t or_0b5(const Arguments& arguments, Registers& registers, Memory& ) // 0xB5 OR L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB5 OR L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= registers.L;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t or_0b6(const Arguments& arguments, Registers& registers, Memory& controller) // 0xB6 OR (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB6 OR (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= controller.get(registers.get_HL());
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 8;
    }

    uint16_t or_0b7(const Arguments& arguments, Registers& registers, Memory& ) // 0xB7 OR A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB7 OR A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= registers.A;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t cp_0b8(const Arguments& arguments, Registers& registers, Memory& ) // 0xB8 CP A, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB8 CP A, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.B;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t cp_0b9(const Arguments& arguments, Registers& registers, Memory& ) // 0xB9 CP A, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xB9 CP A, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.C;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t cp_0ba(const Arguments& arguments, Registers& registers, Memory& ) // 0xBA CP A, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xBA CP A, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.D;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t cp_0bb(const Arguments& arguments, Registers& registers, Memory& ) // 0xBB CP A, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xBB CP A, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.E;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t cp_0bc(const Arguments& arguments, Registers& registers, Memory& ) // 0xBC CP A, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xBC CP A, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.H;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t cp_0bd(const Arguments& arguments, Registers& registers, Memory& ) // 0xBD CP A, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xBD CP A, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.L;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t cp_0be(const Arguments& arguments, Registers& registers, Memory& controller) // 0xBE CP A, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xBE CP A, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = controller.get(registers.get_HL());
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 8;
    }

    uint16_t cp_0bf(const Arguments& arguments, Registers& registers, Memory& ) // 0xBF CP A, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xBF CP A, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = registers.A;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 1;
        return 4;
    }

    uint16_t ret_0c0(const Arguments& arguments, Registers& registers, Memory& controller) // 0xC0 RET NZ
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC0 RET NZ");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        if (registers.get_non_zero_flag())
        {
            registers.PC = controller.get(registers.SP++);
            registers.PC += (controller.get(registers.SP++)) << 8;
            return 20;
        }
        registers.PC += 1;
        return 8;
    }

    uint16_t pop_0c1(const Arguments& arguments, Registers& registers, Memory& controller) // 0xC1 POP BC
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC1 POP BC");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = controller.get(registers.SP++);
        registers.B += controller.get(registers.SP++);
        registers.PC += 1;
        return 12;
    }

    uint16_t jp_0c2(const Arguments& arguments, Registers& registers, Memory& ) // 0xC2 JP NZ, a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC2 JP NZ, a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        if (registers.get_non_zero_flag())
        {
            registers.PC = arguments.uint16;
            return 16;
        }
        registers.PC += 3;
        return 12;
    }

    uint16_t jp_0c3(const Arguments& arguments, Registers& registers, Memory& ) // 0xC3 JP a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC3 JP a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC = arguments.uint16;
        return 16;
    }

    uint16_t call_0c4(const Arguments& arguments, Registers& registers, Memory& controller) // 0xC4 CALL NZ, a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC4 CALL NZ, a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 3;
        if (registers.get_non_zero_flag())
        {
            controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
            controller.set(--registers.SP, registers.PC & 0xFF);
            registers.PC = arguments.uint16;
            return 24;
        }
        return 12;
    }

    uint16_t push_0c5(const Arguments& arguments, Registers& registers, Memory& controller) // 0xC5 PUSH BC
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC5 PUSH BC");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(--registers.SP, registers.B);
        controller.set(--registers.SP, registers.C);
        registers.PC += 1;
        return 16;
    }

    uint16_t add_0c6(const Arguments& arguments, Registers& registers, Memory& ) // 0xC6 ADD A, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC6 ADD A, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = arguments.uint8;
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF);
        int32_t result = lhs + rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 2;
        return 8;
    }

    uint16_t rst_0c7(const Arguments& arguments, Registers& registers, Memory& controller) // 0xC7 RST 00H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC7 RST 00H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 1;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = 0x00;
        return 16;
    }

    uint16_t ret_0c8(const Arguments& arguments, Registers& registers, Memory& controller) // 0xC8 RET Z
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC8 RET Z");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        if (registers.get_zero_flag())
        {
            registers.PC = controller.get(registers.SP++);
            registers.PC += (controller.get(registers.SP++)) << 8;
            return 20;
        }
        registers.PC += 1;
        return 8;
    }

    uint16_t ret_0c9(const Arguments& arguments, Registers& registers, Memory& controller) // 0xC9 RET
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xC9 RET");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC = controller.get(registers.SP++);
        registers.PC += (controller.get(registers.SP++)) << 8;
        return 16;
    }

    uint16_t jp_0ca(const Arguments& arguments, Registers& registers, Memory& ) // 0xCA JP Z, a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xCA JP Z, a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        if (registers.get_zero_flag())
        {
            registers.PC = arguments.uint16;
            return 16;
        }
        registers.PC += 3;
        return 12;
    }

    uint16_t prefix_0cb(const Arguments& arguments, Registers& registers, Memory& ) // 0xCB PREFIX CB
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xCB PREFIX CB");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Opcode 0xCB should be handled separately, something bad must have happened");
    }

    uint16_t call_0cc(const Arguments& arguments, Registers& registers, Memory& controller) // 0xCC CALL Z, a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xCC CALL Z, a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 3;
        if (registers.get_zero_flag())
        {
            controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
            controller.set(--registers.SP, registers.PC & 0xFF);
            registers.PC = arguments.uint16;
            return 24;
        }
        return 12;
    }

    uint16_t call_0cd(const Arguments& arguments, Registers& registers, Memory& controller) // 0xCD CALL a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xCD CALL a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 3;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = arguments.uint16;
        return 24;
    }

    uint16_t adc_0ce(const Arguments& arguments, Registers& registers, Memory& ) // 0xCE ADC A, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xCE ADC A, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = arguments.uint8;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) + (rhs & 0xF) + rrhs;
        int32_t result = lhs + rhs + rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result > 0xF;
        uint8_t carry_flag = result > 0xFF;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 2;
        return 8;
    }

    uint16_t rst_0cf(const Arguments& arguments, Registers& registers, Memory& controller) // 0xCF RST 08H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xCF RST 08H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 1;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = 0x08;
        return 16;
    }

    uint16_t ret_0d0(const Arguments& arguments, Registers& registers, Memory& controller) // 0xD0 RET NC
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD0 RET NC");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        if (registers.get_non_carry_flag())
        {
            registers.PC = controller.get(registers.SP++);
            registers.PC += (controller.get(registers.SP++)) << 8;
            return 20;
        }
        registers.PC += 1;
        return 8;
    }

    uint16_t pop_0d1(const Arguments& arguments, Registers& registers, Memory& controller) // 0xD1 POP DE
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD1 POP DE");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = controller.get(registers.SP++);
        registers.D += controller.get(registers.SP++);
        registers.PC += 1;
        return 12;
    }

    uint16_t jp_0d2(const Arguments& arguments, Registers& registers, Memory& ) // 0xD2 JP NC, a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD2 JP NC, a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        if (registers.get_non_carry_flag())
        {
            registers.PC = arguments.uint16;
            return 16;
        }
        registers.PC += 3;
        return 12;
    }

    uint16_t unknown_0d3(const Arguments& arguments, Registers& registers, Memory& ) // 0xD3 UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD3 UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xD3");
    }

    uint16_t call_0d4(const Arguments& arguments, Registers& registers, Memory& controller) // 0xD4 CALL NC, a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD4 CALL NC, a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 3;
        if (registers.get_non_carry_flag())
        {
            controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
            controller.set(--registers.SP, registers.PC & 0xFF);
            registers.PC = arguments.uint16;
            return 24;
        }
        return 12;
    }

    uint16_t push_0d5(const Arguments& arguments, Registers& registers, Memory& controller) // 0xD5 PUSH DE
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD5 PUSH DE");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(--registers.SP, registers.D);
        controller.set(--registers.SP, registers.E);
        registers.PC += 1;
        return 16;
    }

    uint16_t sub_0d6(const Arguments& arguments, Registers& registers, Memory& ) // 0xD6 SUB A, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD6 SUB A, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = arguments.uint8;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 2;
        return 8;
    }

    uint16_t rst_0d7(const Arguments& arguments, Registers& registers, Memory& controller) // 0xD7 RST 10H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD7 RST 10H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 1;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = 0x10;
        return 16;
    }

    uint16_t ret_0d8(const Arguments& arguments, Registers& registers, Memory& controller) // 0xD8 RET C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD8 RET C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        if (registers.get_carry_flag())
        {
            registers.PC = controller.get(registers.SP++);
            registers.PC += (controller.get(registers.SP++)) << 8;
            return 20;
        }
        registers.PC += 1;
        return 8;
    }

    uint16_t reti_0d9(const Arguments& arguments, Registers& registers, Memory& controller) // 0xD9 RETI
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xD9 RETI");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.ime_flag = true;
        registers.PC = controller.get(registers.SP++);
        registers.PC += (controller.get(registers.SP++)) << 8;
        return 16;
    }

    uint16_t jp_0da(const Arguments& arguments, Registers& registers, Memory& ) // 0xDA JP C, a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xDA JP C, a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        if (registers.get_carry_flag())
        {
            registers.PC = arguments.uint16;
            return 16;
        }
        registers.PC += 3;
        return 12;
    }

    uint16_t unknown_0db(const Arguments& arguments, Registers& registers, Memory& ) // 0xDB UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xDB UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xDB");
    }

    uint16_t call_0dc(const Arguments& arguments, Registers& registers, Memory& controller) // 0xDC CALL C, a16
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xDC CALL C, a16");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 3;
        if (registers.get_carry_flag())
        {
            controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
            controller.set(--registers.SP, registers.PC & 0xFF);
            registers.PC = arguments.uint16;
            return 24;
        }
        return 12;
    }

    uint16_t unknown_0dd(const Arguments& arguments, Registers& registers, Memory& ) // 0xDD UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xDD UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xDD");
    }

    uint16_t sbc_0de(const Arguments& arguments, Registers& registers, Memory& ) // 0xDE SBC A, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xDE SBC A, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = arguments.uint8;
        int32_t rrhs = registers.get_carry_flag();
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF) - rrhs;
        int32_t result = lhs - rhs - rrhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.A = result & 0xFF;
        registers.PC += 2;
        return 8;
    }

    uint16_t rst_0df(const Arguments& arguments, Registers& registers, Memory& controller) // 0xDF RST 18H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xDF RST 18H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 1;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = 0x18;
        return 16;
    }

    uint16_t ldh_0e0(const Arguments& arguments, Registers& registers, Memory& controller) // 0xE0 LDH (a8), A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE0 LDH (a8), A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set((arguments.uint8) + 0xFF00, registers.A);
        registers.PC += 2;
        return 12;
    }

    uint16_t pop_0e1(const Arguments& arguments, Registers& registers, Memory& controller) // 0xE1 POP HL
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE1 POP HL");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = controller.get(registers.SP++);
        registers.H += controller.get(registers.SP++);
        registers.PC += 1;
        return 12;
    }

    uint16_t ldspecialc_0e2(const Arguments& arguments, Registers& registers, Memory& controller) // 0xE2 LDSpecialC (C), A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE2 LDSpecialC (C), A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set((registers.C) + 0xFF00, registers.A);
        registers.PC += 1;
        return 8;
    }

    uint16_t unknown_0e3(const Arguments& arguments, Registers& registers, Memory& ) // 0xE3 UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE3 UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xE3");
    }

    uint16_t unknown_0e4(const Arguments& arguments, Registers& registers, Memory& ) // 0xE4 UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE4 UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xE4");
    }

    uint16_t push_0e5(const Arguments& arguments, Registers& registers, Memory& controller) // 0xE5 PUSH HL
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE5 PUSH HL");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(--registers.SP, registers.H);
        controller.set(--registers.SP, registers.L);
        registers.PC += 1;
        return 16;
    }

    uint16_t and_0e6(const Arguments& arguments, Registers& registers, Memory& ) // 0xE6 AND d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE6 AND d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A &= arguments.uint8;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rst_0e7(const Arguments& arguments, Registers& registers, Memory& controller) // 0xE7 RST 20H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE7 RST 20H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 1;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = 0x20;
        return 16;
    }

    uint16_t add_0e8(const Arguments& arguments, Registers& registers, Memory& ) // 0xE8 ADD SP, r8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE8 ADD SP, r8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.SP;
        int32_t rhs = arguments.int8;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = half_result > 0xFFF;
        uint8_t carry_flag = result > 0xFFFF;
        registers.F &= 0b00000000;
        registers.F |= (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.SP = result & 0xFFFF;
        registers.PC += 2;
        return 16;
    }

    uint16_t jp_0e9(const Arguments& arguments, Registers& registers, Memory& ) // 0xE9 JP HL
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xE9 JP HL");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC = registers.get_HL();
        return 4;
    }

    uint16_t ld_0ea(const Arguments& arguments, Registers& registers, Memory& controller) // 0xEA LD (a16), A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xEA LD (a16), A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(arguments.uint16, registers.A);
        registers.PC += 3;
        return 16;
    }

    uint16_t unknown_0eb(const Arguments& arguments, Registers& registers, Memory& ) // 0xEB UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xEB UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xEB");
    }

    uint16_t unknown_0ec(const Arguments& arguments, Registers& registers, Memory& ) // 0xEC UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xEC UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xEC");
    }

    uint16_t unknown_0ed(const Arguments& arguments, Registers& registers, Memory& ) // 0xED UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xED UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xED");
    }

    uint16_t xor_0ee(const Arguments& arguments, Registers& registers, Memory& ) // 0xEE XOR d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xEE XOR d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A ^= arguments.uint8;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rst_0ef(const Arguments& arguments, Registers& registers, Memory& controller) // 0xEF RST 28H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xEF RST 28H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 1;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = 0x28;
        return 16;
    }

    uint16_t ldh_0f0(const Arguments& arguments, Registers& registers, Memory& controller) // 0xF0 LDH A, (a8)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF0 LDH A, (a8)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = controller.get((arguments.uint8) + 0xFF00);
        registers.PC += 2;
        return 12;
    }

    uint16_t pop_0f1(const Arguments& arguments, Registers& registers, Memory& controller) // 0xF1 POP AF
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF1 POP AF");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.F = controller.get(registers.SP++);
        registers.A += controller.get(registers.SP++);
        registers.PC += 1;
        return 12;
    }

    uint16_t ldspecialc_0f2(const Arguments& arguments, Registers& registers, Memory& controller) // 0xF2 LDSpecialC A, (C)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF2 LDSpecialC A, (C)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = controller.get((registers.C) + 0xFF00);
        registers.PC += 1;
        return 8;
    }

    uint16_t di_0f3(const Arguments& arguments, Registers& registers, Memory& ) // 0xF3 DI
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF3 DI");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.ime_flag = false;
        registers.PC += 1;
        return 4;
    }

    uint16_t unknown_0f4(const Arguments& arguments, Registers& registers, Memory& ) // 0xF4 UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF4 UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xF4");
    }

    uint16_t push_0f5(const Arguments& arguments, Registers& registers, Memory& controller) // 0xF5 PUSH AF
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF5 PUSH AF");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(--registers.SP, registers.A);
        controller.set(--registers.SP, registers.F);
        registers.PC += 1;
        return 16;
    }

    uint16_t or_0f6(const Arguments& arguments, Registers& registers, Memory& ) // 0xF6 OR d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF6 OR d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A |= arguments.uint8;
        uint8_t zero_flag = registers.A == 0;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rst_0f7(const Arguments& arguments, Registers& registers, Memory& controller) // 0xF7 RST 30H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF7 RST 30H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 1;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = 0x30;
        return 16;
    }

    uint16_t ldhl_0f8(const Arguments& arguments, Registers& registers, Memory& ) // 0xF8 LDHL SP, r8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF8 LDHL SP, r8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.SP;
        int32_t rhs = arguments.int8;
        int32_t half_result = (lhs & 0xFFF) + (rhs & 0xFFF);
        int32_t result = lhs + rhs;
        uint8_t half_carry_flag = half_result > 0xFFF;
        uint8_t carry_flag = result > 0xFFFF;
        registers.F &= 0b00000000;
        registers.F |= (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.set_HL(result & 0xFFFF);
        registers.PC += 2;
        return 12;
    }

    uint16_t ld_0f9(const Arguments& arguments, Registers& registers, Memory& ) // 0xF9 LD SP, HL
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xF9 LD SP, HL");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.SP = registers.get_HL();
        registers.PC += 1;
        return 8;
    }

    uint16_t ld_0fa(const Arguments& arguments, Registers& registers, Memory& controller) // 0xFA LD A, (a16)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xFA LD A, (a16)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = controller.get(arguments.uint16);
        registers.PC += 3;
        return 16;
    }

    uint16_t ei_0fb(const Arguments& arguments, Registers& registers, Memory& ) // 0xFB EI
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xFB EI");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.ime_flag = true;
        registers.PC += 1;
        return 4;
    }

    uint16_t unknown_0fc(const Arguments& arguments, Registers& registers, Memory& ) // 0xFC UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xFC UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xFC");
    }

    uint16_t unknown_0fd(const Arguments& arguments, Registers& registers, Memory& ) // 0xFD UNKNOWN
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xFD UNKNOWN");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        throw std::runtime_error("Unknown opcode 0xFD");
    }

    uint16_t cp_0fe(const Arguments& arguments, Registers& registers, Memory& ) // 0xFE CP A, d8
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xFE CP A, d8");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        int32_t lhs = registers.A;
        int32_t rhs = arguments.uint8;
        int32_t half_result = (lhs & 0xF) - (rhs & 0xF);
        int32_t result = lhs - rhs;
        uint8_t zero_flag = (result & 0xFF) == 0;
        uint8_t half_carry_flag = half_result < 0;
        uint8_t carry_flag = result < 0;
        registers.F &= 0b00000000;
        registers.F |= 0b01000000 + (zero_flag << OFFSET_ZERO_FLAG) + (half_carry_flag << OFFSET_HALF_CARRY_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rst_0ff(const Arguments& arguments, Registers& registers, Memory& controller) // 0xFF RST 38H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0xFF RST 38H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.PC += 1;
        controller.set(--registers.SP, (registers.PC >> 8) & 0xFF);
        controller.set(--registers.SP, registers.PC & 0xFF);
        registers.PC = 0x38;
        return 16;
    }

    uint16_t rlc_100(const Arguments& arguments, Registers& registers, Memory& ) // 0x100 RLC B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x100 RLC B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.B;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.B = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rlc_101(const Arguments& arguments, Registers& registers, Memory& ) // 0x101 RLC C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x101 RLC C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.C;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.C = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rlc_102(const Arguments& arguments, Registers& registers, Memory& ) // 0x102 RLC D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x102 RLC D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.D;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.D = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rlc_103(const Arguments& arguments, Registers& registers, Memory& ) // 0x103 RLC E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x103 RLC E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.E;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.E = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rlc_104(const Arguments& arguments, Registers& registers, Memory& ) // 0x104 RLC H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x104 RLC H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.H;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.H = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rlc_105(const Arguments& arguments, Registers& registers, Memory& ) // 0x105 RLC L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x105 RLC L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.L;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.L = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rlc_106(const Arguments& arguments, Registers& registers, Memory& controller) // 0x106 RLC (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x106 RLC (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = controller.get(registers.get_HL());
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        uint8_t zero_flag = (result & 0xFF) == 0;
        controller.set(registers.get_HL(), result);
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 16;
    }

    uint16_t rlc_107(const Arguments& arguments, Registers& registers, Memory& ) // 0x107 RLC A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x107 RLC A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + carry_flag;
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rrc_108(const Arguments& arguments, Registers& registers, Memory& ) // 0x108 RRC B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x108 RRC B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.B;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.B = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rrc_109(const Arguments& arguments, Registers& registers, Memory& ) // 0x109 RRC C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x109 RRC C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.C;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.C = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rrc_10a(const Arguments& arguments, Registers& registers, Memory& ) // 0x10A RRC D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x10A RRC D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.D;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.D = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rrc_10b(const Arguments& arguments, Registers& registers, Memory& ) // 0x10B RRC E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x10B RRC E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.E;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.E = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rrc_10c(const Arguments& arguments, Registers& registers, Memory& ) // 0x10C RRC H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x10C RRC H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.H;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.H = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rrc_10d(const Arguments& arguments, Registers& registers, Memory& ) // 0x10D RRC L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x10D RRC L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.L;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.L = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rrc_10e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x10E RRC (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x10E RRC (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = controller.get(registers.get_HL());
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        controller.set(registers.get_HL(), result);
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 16;
    }

    uint16_t rrc_10f(const Arguments& arguments, Registers& registers, Memory& ) // 0x10F RRC A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x10F RRC A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (carry_flag << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rl_110(const Arguments& arguments, Registers& registers, Memory& ) // 0x110 RL B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x110 RL B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.B;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.B = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rl_111(const Arguments& arguments, Registers& registers, Memory& ) // 0x111 RL C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x111 RL C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.C;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.C = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rl_112(const Arguments& arguments, Registers& registers, Memory& ) // 0x112 RL D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x112 RL D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.D;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.D = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rl_113(const Arguments& arguments, Registers& registers, Memory& ) // 0x113 RL E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x113 RL E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.E;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.E = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rl_114(const Arguments& arguments, Registers& registers, Memory& ) // 0x114 RL H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x114 RL H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.H;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.H = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rl_115(const Arguments& arguments, Registers& registers, Memory& ) // 0x115 RL L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x115 RL L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.L;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.L = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rl_116(const Arguments& arguments, Registers& registers, Memory& controller) // 0x116 RL (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x116 RL (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = controller.get(registers.get_HL());
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        uint8_t zero_flag = (result & 0xFF) == 0;
        controller.set(registers.get_HL(), result);
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 16;
    }

    uint16_t rl_117(const Arguments& arguments, Registers& registers, Memory& ) // 0x117 RL A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x117 RL A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = (value << 1) + registers.get_carry_flag();
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rr_118(const Arguments& arguments, Registers& registers, Memory& ) // 0x118 RR B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x118 RR B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.B;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.B = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rr_119(const Arguments& arguments, Registers& registers, Memory& ) // 0x119 RR C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x119 RR C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.C;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.C = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rr_11a(const Arguments& arguments, Registers& registers, Memory& ) // 0x11A RR D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x11A RR D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.D;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.D = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rr_11b(const Arguments& arguments, Registers& registers, Memory& ) // 0x11B RR E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x11B RR E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.E;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.E = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rr_11c(const Arguments& arguments, Registers& registers, Memory& ) // 0x11C RR H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x11C RR H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.H;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.H = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rr_11d(const Arguments& arguments, Registers& registers, Memory& ) // 0x11D RR L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x11D RR L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.L;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.L = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t rr_11e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x11E RR (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x11E RR (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = controller.get(registers.get_HL());
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        controller.set(registers.get_HL(), result);
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 16;
    }

    uint16_t rr_11f(const Arguments& arguments, Registers& registers, Memory& ) // 0x11F RR A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x11F RR A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (registers.get_carry_flag() << 7);
        uint8_t zero_flag = (result & 0xFF) == 0;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sla_120(const Arguments& arguments, Registers& registers, Memory& ) // 0x120 SLA B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x120 SLA B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.B;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = value << 1;
        uint8_t zero_flag = result == 0;
        registers.B = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sla_121(const Arguments& arguments, Registers& registers, Memory& ) // 0x121 SLA C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x121 SLA C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.C;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = value << 1;
        uint8_t zero_flag = result == 0;
        registers.C = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sla_122(const Arguments& arguments, Registers& registers, Memory& ) // 0x122 SLA D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x122 SLA D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.D;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = value << 1;
        uint8_t zero_flag = result == 0;
        registers.D = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sla_123(const Arguments& arguments, Registers& registers, Memory& ) // 0x123 SLA E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x123 SLA E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.E;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = value << 1;
        uint8_t zero_flag = result == 0;
        registers.E = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sla_124(const Arguments& arguments, Registers& registers, Memory& ) // 0x124 SLA H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x124 SLA H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.H;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = value << 1;
        uint8_t zero_flag = result == 0;
        registers.H = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sla_125(const Arguments& arguments, Registers& registers, Memory& ) // 0x125 SLA L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x125 SLA L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.L;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = value << 1;
        uint8_t zero_flag = result == 0;
        registers.L = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sla_126(const Arguments& arguments, Registers& registers, Memory& controller) // 0x126 SLA (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x126 SLA (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = controller.get(registers.get_HL());
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = value << 1;
        uint8_t zero_flag = result == 0;
        controller.set(registers.get_HL(), result);
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 16;
    }

    uint16_t sla_127(const Arguments& arguments, Registers& registers, Memory& ) // 0x127 SLA A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x127 SLA A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = (value >> 7) & 0b1;
        uint8_t result = value << 1;
        uint8_t zero_flag = result == 0;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sra_128(const Arguments& arguments, Registers& registers, Memory& ) // 0x128 SRA B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x128 SRA B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.B;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (value & 0x80);
        uint8_t zero_flag = result == 0;
        registers.B = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sra_129(const Arguments& arguments, Registers& registers, Memory& ) // 0x129 SRA C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x129 SRA C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.C;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (value & 0x80);
        uint8_t zero_flag = result == 0;
        registers.C = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sra_12a(const Arguments& arguments, Registers& registers, Memory& ) // 0x12A SRA D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x12A SRA D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.D;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (value & 0x80);
        uint8_t zero_flag = result == 0;
        registers.D = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sra_12b(const Arguments& arguments, Registers& registers, Memory& ) // 0x12B SRA E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x12B SRA E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.E;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (value & 0x80);
        uint8_t zero_flag = result == 0;
        registers.E = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sra_12c(const Arguments& arguments, Registers& registers, Memory& ) // 0x12C SRA H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x12C SRA H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.H;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (value & 0x80);
        uint8_t zero_flag = result == 0;
        registers.H = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sra_12d(const Arguments& arguments, Registers& registers, Memory& ) // 0x12D SRA L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x12D SRA L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.L;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (value & 0x80);
        uint8_t zero_flag = result == 0;
        registers.L = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t sra_12e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x12E SRA (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x12E SRA (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = controller.get(registers.get_HL());
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (value & 0x80);
        uint8_t zero_flag = result == 0;
        controller.set(registers.get_HL(), result);
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 16;
    }

    uint16_t sra_12f(const Arguments& arguments, Registers& registers, Memory& ) // 0x12F SRA A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x12F SRA A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = (value >> 1) + (value & 0x80);
        uint8_t zero_flag = result == 0;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t swap_130(const Arguments& arguments, Registers& registers, Memory& ) // 0x130 SWAP B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x130 SWAP B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.B;
        uint8_t result = (value >> 4) | (value  << 4);
        uint8_t zero_flag = result == 0;
        registers.B = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t swap_131(const Arguments& arguments, Registers& registers, Memory& ) // 0x131 SWAP C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x131 SWAP C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.C;
        uint8_t result = (value >> 4) | (value  << 4);
        uint8_t zero_flag = result == 0;
        registers.C = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t swap_132(const Arguments& arguments, Registers& registers, Memory& ) // 0x132 SWAP D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x132 SWAP D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.D;
        uint8_t result = (value >> 4) | (value  << 4);
        uint8_t zero_flag = result == 0;
        registers.D = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t swap_133(const Arguments& arguments, Registers& registers, Memory& ) // 0x133 SWAP E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x133 SWAP E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.E;
        uint8_t result = (value >> 4) | (value  << 4);
        uint8_t zero_flag = result == 0;
        registers.E = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t swap_134(const Arguments& arguments, Registers& registers, Memory& ) // 0x134 SWAP H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x134 SWAP H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.H;
        uint8_t result = (value >> 4) | (value  << 4);
        uint8_t zero_flag = result == 0;
        registers.H = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t swap_135(const Arguments& arguments, Registers& registers, Memory& ) // 0x135 SWAP L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x135 SWAP L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.L;
        uint8_t result = (value >> 4) | (value  << 4);
        uint8_t zero_flag = result == 0;
        registers.L = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t swap_136(const Arguments& arguments, Registers& registers, Memory& controller) // 0x136 SWAP (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x136 SWAP (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = controller.get(registers.get_HL());
        uint8_t result = (value >> 4) | (value  << 4);
        uint8_t zero_flag = result == 0;
        controller.set(registers.get_HL(), result);
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 16;
    }

    uint16_t swap_137(const Arguments& arguments, Registers& registers, Memory& ) // 0x137 SWAP A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x137 SWAP A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t result = (value >> 4) | (value  << 4);
        uint8_t zero_flag = result == 0;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t srl_138(const Arguments& arguments, Registers& registers, Memory& ) // 0x138 SRL B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x138 SRL B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.B;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = value >> 1;
        uint8_t zero_flag = result == 0;
        registers.B = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t srl_139(const Arguments& arguments, Registers& registers, Memory& ) // 0x139 SRL C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x139 SRL C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.C;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = value >> 1;
        uint8_t zero_flag = result == 0;
        registers.C = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t srl_13a(const Arguments& arguments, Registers& registers, Memory& ) // 0x13A SRL D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x13A SRL D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.D;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = value >> 1;
        uint8_t zero_flag = result == 0;
        registers.D = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t srl_13b(const Arguments& arguments, Registers& registers, Memory& ) // 0x13B SRL E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x13B SRL E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.E;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = value >> 1;
        uint8_t zero_flag = result == 0;
        registers.E = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t srl_13c(const Arguments& arguments, Registers& registers, Memory& ) // 0x13C SRL H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x13C SRL H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.H;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = value >> 1;
        uint8_t zero_flag = result == 0;
        registers.H = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t srl_13d(const Arguments& arguments, Registers& registers, Memory& ) // 0x13D SRL L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x13D SRL L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.L;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = value >> 1;
        uint8_t zero_flag = result == 0;
        registers.L = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t srl_13e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x13E SRL (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x13E SRL (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = controller.get(registers.get_HL());
        uint8_t carry_flag = value & 0b1;
        uint8_t result = value >> 1;
        uint8_t zero_flag = result == 0;
        controller.set(registers.get_HL(), result);
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 16;
    }

    uint16_t srl_13f(const Arguments& arguments, Registers& registers, Memory& ) // 0x13F SRL A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x13F SRL A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t value = registers.A;
        uint8_t carry_flag = value & 0b1;
        uint8_t result = value >> 1;
        uint8_t zero_flag = result == 0;
        registers.A = result;
        registers.F &= 0b00000000;
        registers.F |= (zero_flag << OFFSET_ZERO_FLAG) + (carry_flag << OFFSET_CARRY_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_140(const Arguments& arguments, Registers& registers, Memory& ) // 0x140 BIT 0, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x140 BIT 0, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.B >> 0) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_141(const Arguments& arguments, Registers& registers, Memory& ) // 0x141 BIT 0, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x141 BIT 0, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.C >> 0) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_142(const Arguments& arguments, Registers& registers, Memory& ) // 0x142 BIT 0, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x142 BIT 0, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.D >> 0) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_143(const Arguments& arguments, Registers& registers, Memory& ) // 0x143 BIT 0, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x143 BIT 0, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.E >> 0) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_144(const Arguments& arguments, Registers& registers, Memory& ) // 0x144 BIT 0, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x144 BIT 0, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.H >> 0) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_145(const Arguments& arguments, Registers& registers, Memory& ) // 0x145 BIT 0, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x145 BIT 0, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.L >> 0) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_146(const Arguments& arguments, Registers& registers, Memory& controller) // 0x146 BIT 0, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x146 BIT 0, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((controller.get(registers.get_HL()) >> 0) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 12;
    }

    uint16_t bit_147(const Arguments& arguments, Registers& registers, Memory& ) // 0x147 BIT 0, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x147 BIT 0, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.A >> 0) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_148(const Arguments& arguments, Registers& registers, Memory& ) // 0x148 BIT 1, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x148 BIT 1, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.B >> 1) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_149(const Arguments& arguments, Registers& registers, Memory& ) // 0x149 BIT 1, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x149 BIT 1, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.C >> 1) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_14a(const Arguments& arguments, Registers& registers, Memory& ) // 0x14A BIT 1, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x14A BIT 1, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.D >> 1) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_14b(const Arguments& arguments, Registers& registers, Memory& ) // 0x14B BIT 1, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x14B BIT 1, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.E >> 1) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_14c(const Arguments& arguments, Registers& registers, Memory& ) // 0x14C BIT 1, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x14C BIT 1, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.H >> 1) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_14d(const Arguments& arguments, Registers& registers, Memory& ) // 0x14D BIT 1, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x14D BIT 1, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.L >> 1) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_14e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x14E BIT 1, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x14E BIT 1, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((controller.get(registers.get_HL()) >> 1) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 12;
    }

    uint16_t bit_14f(const Arguments& arguments, Registers& registers, Memory& ) // 0x14F BIT 1, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x14F BIT 1, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.A >> 1) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_150(const Arguments& arguments, Registers& registers, Memory& ) // 0x150 BIT 2, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x150 BIT 2, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.B >> 2) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_151(const Arguments& arguments, Registers& registers, Memory& ) // 0x151 BIT 2, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x151 BIT 2, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.C >> 2) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_152(const Arguments& arguments, Registers& registers, Memory& ) // 0x152 BIT 2, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x152 BIT 2, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.D >> 2) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_153(const Arguments& arguments, Registers& registers, Memory& ) // 0x153 BIT 2, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x153 BIT 2, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.E >> 2) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_154(const Arguments& arguments, Registers& registers, Memory& ) // 0x154 BIT 2, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x154 BIT 2, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.H >> 2) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_155(const Arguments& arguments, Registers& registers, Memory& ) // 0x155 BIT 2, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x155 BIT 2, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.L >> 2) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_156(const Arguments& arguments, Registers& registers, Memory& controller) // 0x156 BIT 2, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x156 BIT 2, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((controller.get(registers.get_HL()) >> 2) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 12;
    }

    uint16_t bit_157(const Arguments& arguments, Registers& registers, Memory& ) // 0x157 BIT 2, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x157 BIT 2, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.A >> 2) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_158(const Arguments& arguments, Registers& registers, Memory& ) // 0x158 BIT 3, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x158 BIT 3, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.B >> 3) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_159(const Arguments& arguments, Registers& registers, Memory& ) // 0x159 BIT 3, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x159 BIT 3, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.C >> 3) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_15a(const Arguments& arguments, Registers& registers, Memory& ) // 0x15A BIT 3, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x15A BIT 3, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.D >> 3) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_15b(const Arguments& arguments, Registers& registers, Memory& ) // 0x15B BIT 3, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x15B BIT 3, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.E >> 3) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_15c(const Arguments& arguments, Registers& registers, Memory& ) // 0x15C BIT 3, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x15C BIT 3, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.H >> 3) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_15d(const Arguments& arguments, Registers& registers, Memory& ) // 0x15D BIT 3, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x15D BIT 3, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.L >> 3) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_15e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x15E BIT 3, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x15E BIT 3, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((controller.get(registers.get_HL()) >> 3) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 12;
    }

    uint16_t bit_15f(const Arguments& arguments, Registers& registers, Memory& ) // 0x15F BIT 3, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x15F BIT 3, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.A >> 3) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_160(const Arguments& arguments, Registers& registers, Memory& ) // 0x160 BIT 4, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x160 BIT 4, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.B >> 4) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_161(const Arguments& arguments, Registers& registers, Memory& ) // 0x161 BIT 4, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x161 BIT 4, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.C >> 4) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_162(const Arguments& arguments, Registers& registers, Memory& ) // 0x162 BIT 4, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x162 BIT 4, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.D >> 4) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_163(const Arguments& arguments, Registers& registers, Memory& ) // 0x163 BIT 4, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x163 BIT 4, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.E >> 4) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_164(const Arguments& arguments, Registers& registers, Memory& ) // 0x164 BIT 4, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x164 BIT 4, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.H >> 4) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_165(const Arguments& arguments, Registers& registers, Memory& ) // 0x165 BIT 4, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x165 BIT 4, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.L >> 4) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_166(const Arguments& arguments, Registers& registers, Memory& controller) // 0x166 BIT 4, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x166 BIT 4, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((controller.get(registers.get_HL()) >> 4) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 12;
    }

    uint16_t bit_167(const Arguments& arguments, Registers& registers, Memory& ) // 0x167 BIT 4, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x167 BIT 4, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.A >> 4) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_168(const Arguments& arguments, Registers& registers, Memory& ) // 0x168 BIT 5, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x168 BIT 5, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.B >> 5) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_169(const Arguments& arguments, Registers& registers, Memory& ) // 0x169 BIT 5, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x169 BIT 5, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.C >> 5) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_16a(const Arguments& arguments, Registers& registers, Memory& ) // 0x16A BIT 5, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x16A BIT 5, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.D >> 5) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_16b(const Arguments& arguments, Registers& registers, Memory& ) // 0x16B BIT 5, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x16B BIT 5, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.E >> 5) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_16c(const Arguments& arguments, Registers& registers, Memory& ) // 0x16C BIT 5, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x16C BIT 5, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.H >> 5) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_16d(const Arguments& arguments, Registers& registers, Memory& ) // 0x16D BIT 5, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x16D BIT 5, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.L >> 5) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_16e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x16E BIT 5, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x16E BIT 5, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((controller.get(registers.get_HL()) >> 5) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 12;
    }

    uint16_t bit_16f(const Arguments& arguments, Registers& registers, Memory& ) // 0x16F BIT 5, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x16F BIT 5, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.A >> 5) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_170(const Arguments& arguments, Registers& registers, Memory& ) // 0x170 BIT 6, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x170 BIT 6, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.B >> 6) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_171(const Arguments& arguments, Registers& registers, Memory& ) // 0x171 BIT 6, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x171 BIT 6, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.C >> 6) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_172(const Arguments& arguments, Registers& registers, Memory& ) // 0x172 BIT 6, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x172 BIT 6, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.D >> 6) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_173(const Arguments& arguments, Registers& registers, Memory& ) // 0x173 BIT 6, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x173 BIT 6, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.E >> 6) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_174(const Arguments& arguments, Registers& registers, Memory& ) // 0x174 BIT 6, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x174 BIT 6, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.H >> 6) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_175(const Arguments& arguments, Registers& registers, Memory& ) // 0x175 BIT 6, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x175 BIT 6, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.L >> 6) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_176(const Arguments& arguments, Registers& registers, Memory& controller) // 0x176 BIT 6, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x176 BIT 6, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((controller.get(registers.get_HL()) >> 6) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 12;
    }

    uint16_t bit_177(const Arguments& arguments, Registers& registers, Memory& ) // 0x177 BIT 6, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x177 BIT 6, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.A >> 6) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_178(const Arguments& arguments, Registers& registers, Memory& ) // 0x178 BIT 7, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x178 BIT 7, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.B >> 7) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_179(const Arguments& arguments, Registers& registers, Memory& ) // 0x179 BIT 7, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x179 BIT 7, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.C >> 7) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_17a(const Arguments& arguments, Registers& registers, Memory& ) // 0x17A BIT 7, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x17A BIT 7, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.D >> 7) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_17b(const Arguments& arguments, Registers& registers, Memory& ) // 0x17B BIT 7, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x17B BIT 7, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.E >> 7) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_17c(const Arguments& arguments, Registers& registers, Memory& ) // 0x17C BIT 7, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x17C BIT 7, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.H >> 7) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_17d(const Arguments& arguments, Registers& registers, Memory& ) // 0x17D BIT 7, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x17D BIT 7, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.L >> 7) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t bit_17e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x17E BIT 7, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x17E BIT 7, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((controller.get(registers.get_HL()) >> 7) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 12;
    }

    uint16_t bit_17f(const Arguments& arguments, Registers& registers, Memory& ) // 0x17F BIT 7, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x17F BIT 7, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        uint8_t zero_flag = ((registers.A >> 7) & 0b1) == 0;
        registers.F &= 0b00010000;
        registers.F |= 0b00100000 + (zero_flag << OFFSET_ZERO_FLAG);
        registers.PC += 2;
        return 8;
    }

    uint16_t res_180(const Arguments& arguments, Registers& registers, Memory& ) // 0x180 RES 0, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x180 RES 0, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B & 0b11111110;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_181(const Arguments& arguments, Registers& registers, Memory& ) // 0x181 RES 0, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x181 RES 0, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C & 0b11111110;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_182(const Arguments& arguments, Registers& registers, Memory& ) // 0x182 RES 0, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x182 RES 0, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D & 0b11111110;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_183(const Arguments& arguments, Registers& registers, Memory& ) // 0x183 RES 0, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x183 RES 0, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E & 0b11111110;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_184(const Arguments& arguments, Registers& registers, Memory& ) // 0x184 RES 0, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x184 RES 0, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H & 0b11111110;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_185(const Arguments& arguments, Registers& registers, Memory& ) // 0x185 RES 0, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x185 RES 0, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L & 0b11111110;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_186(const Arguments& arguments, Registers& registers, Memory& controller) // 0x186 RES 0, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x186 RES 0, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) & 0b11111110);
        registers.PC += 2;
        return 16;
    }

    uint16_t res_187(const Arguments& arguments, Registers& registers, Memory& ) // 0x187 RES 0, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x187 RES 0, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A & 0b11111110;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_188(const Arguments& arguments, Registers& registers, Memory& ) // 0x188 RES 1, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x188 RES 1, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B & 0b11111101;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_189(const Arguments& arguments, Registers& registers, Memory& ) // 0x189 RES 1, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x189 RES 1, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C & 0b11111101;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_18a(const Arguments& arguments, Registers& registers, Memory& ) // 0x18A RES 1, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x18A RES 1, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D & 0b11111101;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_18b(const Arguments& arguments, Registers& registers, Memory& ) // 0x18B RES 1, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x18B RES 1, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E & 0b11111101;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_18c(const Arguments& arguments, Registers& registers, Memory& ) // 0x18C RES 1, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x18C RES 1, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H & 0b11111101;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_18d(const Arguments& arguments, Registers& registers, Memory& ) // 0x18D RES 1, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x18D RES 1, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L & 0b11111101;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_18e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x18E RES 1, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x18E RES 1, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) & 0b11111101);
        registers.PC += 2;
        return 16;
    }

    uint16_t res_18f(const Arguments& arguments, Registers& registers, Memory& ) // 0x18F RES 1, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x18F RES 1, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A & 0b11111101;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_190(const Arguments& arguments, Registers& registers, Memory& ) // 0x190 RES 2, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x190 RES 2, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B & 0b11111011;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_191(const Arguments& arguments, Registers& registers, Memory& ) // 0x191 RES 2, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x191 RES 2, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C & 0b11111011;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_192(const Arguments& arguments, Registers& registers, Memory& ) // 0x192 RES 2, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x192 RES 2, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D & 0b11111011;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_193(const Arguments& arguments, Registers& registers, Memory& ) // 0x193 RES 2, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x193 RES 2, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E & 0b11111011;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_194(const Arguments& arguments, Registers& registers, Memory& ) // 0x194 RES 2, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x194 RES 2, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H & 0b11111011;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_195(const Arguments& arguments, Registers& registers, Memory& ) // 0x195 RES 2, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x195 RES 2, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L & 0b11111011;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_196(const Arguments& arguments, Registers& registers, Memory& controller) // 0x196 RES 2, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x196 RES 2, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) & 0b11111011);
        registers.PC += 2;
        return 16;
    }

    uint16_t res_197(const Arguments& arguments, Registers& registers, Memory& ) // 0x197 RES 2, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x197 RES 2, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A & 0b11111011;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_198(const Arguments& arguments, Registers& registers, Memory& ) // 0x198 RES 3, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x198 RES 3, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B & 0b11110111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_199(const Arguments& arguments, Registers& registers, Memory& ) // 0x199 RES 3, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x199 RES 3, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C & 0b11110111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_19a(const Arguments& arguments, Registers& registers, Memory& ) // 0x19A RES 3, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x19A RES 3, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D & 0b11110111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_19b(const Arguments& arguments, Registers& registers, Memory& ) // 0x19B RES 3, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x19B RES 3, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E & 0b11110111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_19c(const Arguments& arguments, Registers& registers, Memory& ) // 0x19C RES 3, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x19C RES 3, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H & 0b11110111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_19d(const Arguments& arguments, Registers& registers, Memory& ) // 0x19D RES 3, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x19D RES 3, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L & 0b11110111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_19e(const Arguments& arguments, Registers& registers, Memory& controller) // 0x19E RES 3, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x19E RES 3, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) & 0b11110111);
        registers.PC += 2;
        return 16;
    }

    uint16_t res_19f(const Arguments& arguments, Registers& registers, Memory& ) // 0x19F RES 3, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x19F RES 3, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A & 0b11110111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a0(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A0 RES 4, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A0 RES 4, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B & 0b11101111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a1(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A1 RES 4, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A1 RES 4, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C & 0b11101111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a2(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A2 RES 4, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A2 RES 4, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D & 0b11101111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a3(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A3 RES 4, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A3 RES 4, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E & 0b11101111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a4(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A4 RES 4, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A4 RES 4, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H & 0b11101111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a5(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A5 RES 4, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A5 RES 4, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L & 0b11101111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a6(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1A6 RES 4, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A6 RES 4, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) & 0b11101111);
        registers.PC += 2;
        return 16;
    }

    uint16_t res_1a7(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A7 RES 4, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A7 RES 4, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A & 0b11101111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a8(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A8 RES 5, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A8 RES 5, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B & 0b11011111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1a9(const Arguments& arguments, Registers& registers, Memory& ) // 0x1A9 RES 5, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1A9 RES 5, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C & 0b11011111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1aa(const Arguments& arguments, Registers& registers, Memory& ) // 0x1AA RES 5, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1AA RES 5, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D & 0b11011111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1ab(const Arguments& arguments, Registers& registers, Memory& ) // 0x1AB RES 5, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1AB RES 5, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E & 0b11011111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1ac(const Arguments& arguments, Registers& registers, Memory& ) // 0x1AC RES 5, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1AC RES 5, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H & 0b11011111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1ad(const Arguments& arguments, Registers& registers, Memory& ) // 0x1AD RES 5, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1AD RES 5, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L & 0b11011111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1ae(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1AE RES 5, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1AE RES 5, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) & 0b11011111);
        registers.PC += 2;
        return 16;
    }

    uint16_t res_1af(const Arguments& arguments, Registers& registers, Memory& ) // 0x1AF RES 5, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1AF RES 5, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A & 0b11011111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b0(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B0 RES 6, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B0 RES 6, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B & 0b10111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b1(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B1 RES 6, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B1 RES 6, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C & 0b10111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b2(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B2 RES 6, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B2 RES 6, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D & 0b10111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b3(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B3 RES 6, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B3 RES 6, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E & 0b10111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b4(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B4 RES 6, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B4 RES 6, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H & 0b10111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b5(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B5 RES 6, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B5 RES 6, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L & 0b10111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b6(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1B6 RES 6, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B6 RES 6, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) & 0b10111111);
        registers.PC += 2;
        return 16;
    }

    uint16_t res_1b7(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B7 RES 6, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B7 RES 6, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A & 0b10111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b8(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B8 RES 7, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B8 RES 7, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B & 0b01111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1b9(const Arguments& arguments, Registers& registers, Memory& ) // 0x1B9 RES 7, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1B9 RES 7, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C & 0b01111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1ba(const Arguments& arguments, Registers& registers, Memory& ) // 0x1BA RES 7, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1BA RES 7, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D & 0b01111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1bb(const Arguments& arguments, Registers& registers, Memory& ) // 0x1BB RES 7, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1BB RES 7, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E & 0b01111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1bc(const Arguments& arguments, Registers& registers, Memory& ) // 0x1BC RES 7, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1BC RES 7, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H & 0b01111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1bd(const Arguments& arguments, Registers& registers, Memory& ) // 0x1BD RES 7, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1BD RES 7, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L & 0b01111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t res_1be(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1BE RES 7, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1BE RES 7, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) & 0b01111111);
        registers.PC += 2;
        return 16;
    }

    uint16_t res_1bf(const Arguments& arguments, Registers& registers, Memory& ) // 0x1BF RES 7, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1BF RES 7, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A & 0b01111111;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c0(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C0 SET 0, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C0 SET 0, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B | 0b00000001;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c1(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C1 SET 0, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C1 SET 0, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C | 0b00000001;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c2(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C2 SET 0, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C2 SET 0, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D | 0b00000001;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c3(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C3 SET 0, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C3 SET 0, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E | 0b00000001;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c4(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C4 SET 0, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C4 SET 0, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H | 0b00000001;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c5(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C5 SET 0, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C5 SET 0, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L | 0b00000001;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c6(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1C6 SET 0, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C6 SET 0, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) | 0b00000001);
        registers.PC += 2;
        return 16;
    }

    uint16_t set_1c7(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C7 SET 0, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C7 SET 0, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A | 0b00000001;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c8(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C8 SET 1, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C8 SET 1, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B | 0b00000010;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1c9(const Arguments& arguments, Registers& registers, Memory& ) // 0x1C9 SET 1, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1C9 SET 1, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C | 0b00000010;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1ca(const Arguments& arguments, Registers& registers, Memory& ) // 0x1CA SET 1, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1CA SET 1, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D | 0b00000010;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1cb(const Arguments& arguments, Registers& registers, Memory& ) // 0x1CB SET 1, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1CB SET 1, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E | 0b00000010;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1cc(const Arguments& arguments, Registers& registers, Memory& ) // 0x1CC SET 1, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1CC SET 1, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H | 0b00000010;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1cd(const Arguments& arguments, Registers& registers, Memory& ) // 0x1CD SET 1, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1CD SET 1, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L | 0b00000010;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1ce(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1CE SET 1, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1CE SET 1, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) | 0b00000010);
        registers.PC += 2;
        return 16;
    }

    uint16_t set_1cf(const Arguments& arguments, Registers& registers, Memory& ) // 0x1CF SET 1, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1CF SET 1, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A | 0b00000010;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d0(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D0 SET 2, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D0 SET 2, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B | 0b00000100;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d1(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D1 SET 2, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D1 SET 2, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C | 0b00000100;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d2(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D2 SET 2, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D2 SET 2, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D | 0b00000100;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d3(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D3 SET 2, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D3 SET 2, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E | 0b00000100;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d4(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D4 SET 2, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D4 SET 2, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H | 0b00000100;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d5(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D5 SET 2, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D5 SET 2, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L | 0b00000100;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d6(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1D6 SET 2, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D6 SET 2, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) | 0b00000100);
        registers.PC += 2;
        return 16;
    }

    uint16_t set_1d7(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D7 SET 2, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D7 SET 2, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A | 0b00000100;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d8(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D8 SET 3, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D8 SET 3, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B | 0b00001000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1d9(const Arguments& arguments, Registers& registers, Memory& ) // 0x1D9 SET 3, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1D9 SET 3, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C | 0b00001000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1da(const Arguments& arguments, Registers& registers, Memory& ) // 0x1DA SET 3, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1DA SET 3, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D | 0b00001000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1db(const Arguments& arguments, Registers& registers, Memory& ) // 0x1DB SET 3, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1DB SET 3, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E | 0b00001000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1dc(const Arguments& arguments, Registers& registers, Memory& ) // 0x1DC SET 3, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1DC SET 3, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H | 0b00001000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1dd(const Arguments& arguments, Registers& registers, Memory& ) // 0x1DD SET 3, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1DD SET 3, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L | 0b00001000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1de(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1DE SET 3, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1DE SET 3, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) | 0b00001000);
        registers.PC += 2;
        return 16;
    }

    uint16_t set_1df(const Arguments& arguments, Registers& registers, Memory& ) // 0x1DF SET 3, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1DF SET 3, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A | 0b00001000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e0(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E0 SET 4, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E0 SET 4, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B | 0b00010000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e1(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E1 SET 4, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E1 SET 4, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C | 0b00010000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e2(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E2 SET 4, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E2 SET 4, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D | 0b00010000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e3(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E3 SET 4, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E3 SET 4, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E | 0b00010000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e4(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E4 SET 4, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E4 SET 4, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H | 0b00010000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e5(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E5 SET 4, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E5 SET 4, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L | 0b00010000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e6(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1E6 SET 4, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E6 SET 4, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) | 0b00010000);
        registers.PC += 2;
        return 16;
    }

    uint16_t set_1e7(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E7 SET 4, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E7 SET 4, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A | 0b00010000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e8(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E8 SET 5, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E8 SET 5, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B | 0b00100000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1e9(const Arguments& arguments, Registers& registers, Memory& ) // 0x1E9 SET 5, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1E9 SET 5, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C | 0b00100000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1ea(const Arguments& arguments, Registers& registers, Memory& ) // 0x1EA SET 5, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1EA SET 5, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D | 0b00100000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1eb(const Arguments& arguments, Registers& registers, Memory& ) // 0x1EB SET 5, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1EB SET 5, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E | 0b00100000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1ec(const Arguments& arguments, Registers& registers, Memory& ) // 0x1EC SET 5, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1EC SET 5, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H | 0b00100000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1ed(const Arguments& arguments, Registers& registers, Memory& ) // 0x1ED SET 5, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1ED SET 5, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L | 0b00100000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1ee(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1EE SET 5, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1EE SET 5, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) | 0b00100000);
        registers.PC += 2;
        return 16;
    }

    uint16_t set_1ef(const Arguments& arguments, Registers& registers, Memory& ) // 0x1EF SET 5, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1EF SET 5, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A | 0b00100000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f0(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F0 SET 6, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F0 SET 6, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B | 0b01000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f1(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F1 SET 6, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F1 SET 6, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C | 0b01000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f2(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F2 SET 6, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F2 SET 6, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D | 0b01000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f3(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F3 SET 6, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F3 SET 6, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E | 0b01000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f4(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F4 SET 6, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F4 SET 6, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H | 0b01000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f5(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F5 SET 6, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F5 SET 6, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L | 0b01000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f6(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1F6 SET 6, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F6 SET 6, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) | 0b01000000);
        registers.PC += 2;
        return 16;
    }

    uint16_t set_1f7(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F7 SET 6, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F7 SET 6, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A | 0b01000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f8(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F8 SET 7, B
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F8 SET 7, B");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.B = registers.B | 0b10000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1f9(const Arguments& arguments, Registers& registers, Memory& ) // 0x1F9 SET 7, C
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1F9 SET 7, C");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.C = registers.C | 0b10000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1fa(const Arguments& arguments, Registers& registers, Memory& ) // 0x1FA SET 7, D
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1FA SET 7, D");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.D = registers.D | 0b10000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1fb(const Arguments& arguments, Registers& registers, Memory& ) // 0x1FB SET 7, E
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1FB SET 7, E");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.E = registers.E | 0b10000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1fc(const Arguments& arguments, Registers& registers, Memory& ) // 0x1FC SET 7, H
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1FC SET 7, H");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.H = registers.H | 0b10000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1fd(const Arguments& arguments, Registers& registers, Memory& ) // 0x1FD SET 7, L
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1FD SET 7, L");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.L = registers.L | 0b10000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t set_1fe(const Arguments& arguments, Registers& registers, Memory& controller) // 0x1FE SET 7, (HL)
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1FE SET 7, (HL)");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        controller.set(registers.get_HL(), controller.get(registers.get_HL()) | 0b10000000);
        registers.PC += 2;
        return 16;
    }

    uint16_t set_1ff(const Arguments& arguments, Registers& registers, Memory& ) // 0x1FF SET 7, A
    {
        if constexpr (TRACE_LOG)
        {
            LOGGER.trace("0x1FF SET 7, A");
            LOGGER.trace(emulator::utils::to_string(arguments));
            LOGGER.trace(emulator::utils::to_string(registers));
        }
        registers.A = registers.A | 0b10000000;
        registers.PC += 2;
        return 8;
    }

    uint16_t execute_next_instruction(Registers& registers, Memory& controller)
    {
        uint16_t pc = registers.PC;
        uint16_t opcode = controller.get(pc);
        if (0xCB == opcode)
            opcode = 0x100 + controller.get(++pc);

        Arguments arguments{};
        switch (INSTRUCTION_ARGUMENT_TYPES[opcode])
        {
            case ArgumentType::int8:
                arguments.int8 = controller.get(++pc);
                break;
            case ArgumentType::uint8:
                arguments.uint8 = controller.get(++pc);
                break;
            case ArgumentType::uint16:
                arguments.uint16 = controller.get(pc + 1) + (controller.get(pc + 2) << 8);
                break;
            default:
                break;
        }
        return INSTRUCTION_FUNCTIONS[opcode](arguments, registers, controller);
    }
}