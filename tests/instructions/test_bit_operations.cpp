/*
Testing all the inc functions

The pages referenced in this file are pointing to [GameBoyProgManVer1.1.pdf](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
 */

#include <tuple>

#include "gtest/gtest.h"

#include "emulator/memory/registers.h"
#include "emulator/generated/instructions.h"

#include "fixtures/instruction.h"
#include "constants.h"

using emulator::generated::Arguments;
using emulator::memory::Registers;
using emulator::instructions::REGISTER_8_BITS_VALUES;
using emulator::instructions::REGISTER_8_BITS_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_8_BITS_VALUE_NAME_MAP;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    const auto BIT_INDEXES = ::testing::Values(0, 1, 2, 3, 4, 5, 6, 7);

    class NamePrinter
    {
    public:
        explicit NamePrinter(const std::unordered_map<uint16_t, std::string>& name_map): name_map_(name_map) {};
        std::string operator()(testing::TestParamInfo<std::tuple<uint16_t, uint16_t>> info) const
        {
            return name_map_.at(std::get<0>(info.param)) + "_bit_" + std::to_string(std::get<1>(info.param));
        }
    private:
        const std::unordered_map<uint16_t, std::string>& name_map_;
    };

    class BitOpTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<std::tuple<uint16_t, uint16_t>> {};

    TEST_P(BitOpTestFixture, BITValueZero)
    {
        // Test from Chapter 4: page 114
        auto [register_index, bit_index] = GetParam();
        uint16_t instruction_index = register_index + (bit_index << 3) + 0b1'0100'0000;
        uint16_t register_value = 0xFF - (1 << bit_index);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, register_value);
        registers.F = emulator::memory::make_flag(false, true, false, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, register_value);
        expected_registers.F = emulator::memory::make_flag(true, false, true, true);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(BitOpTestFixture, BITValueOne)
    {
        // Test from Chapter 4: page 114
        auto [register_index, bit_index] = GetParam();
        uint16_t instruction_index = register_index + (bit_index << 3) + 0b1'0100'0000;
        uint16_t register_value = 1 << bit_index;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, register_value);
        registers.F = emulator::memory::make_flag(true, true, false, false);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, register_value);
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(BitOpTestFixture, RES)
    {
        // Test from Chapter 4: page 114
        auto [register_index, bit_index] = GetParam();
        uint16_t instruction_index = register_index + (bit_index << 3) + 0b1'1000'0000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0xFF);
        registers.F = emulator::memory::make_flag(true, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0xFF - (1 << bit_index));
        expected_registers.F = emulator::memory::make_flag(true, true, true, true);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        BitOpTest, BitOpTestFixture,
        ::testing::Combine(REGISTER_8_BITS_VALUES, BIT_INDEXES), NamePrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );

    class BitOpAddressTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(BitOpAddressTestFixture, BITValueZero)
    {
        // Test from Chapter 4: page 114
        uint16_t bit_index = GetParam();
        uint16_t instruction_index = (bit_index << 3) + 0b1'0100'0110;
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0xFF - (1 << bit_index)));
        registers.F = emulator::memory::make_flag(false, true, false, true);
        expected_registers.set_HL(0xD1C7);
        expected_registers.F = emulator::memory::make_flag(true, false, true, true);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    TEST_P(BitOpAddressTestFixture, BITValueOne)
    {
        // Test from Chapter 4: page 114
        uint16_t bit_index = GetParam();
        uint16_t instruction_index =  (bit_index << 3) + 0b1'0100'0110;
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(1 << bit_index));
        registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.set_HL(0xD1C7);
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    TEST_P(BitOpAddressTestFixture, RES)
    {
        // Test from Chapter 4: page 114
        uint16_t bit_index = GetParam();
        uint16_t instruction_index =  (bit_index << 3) + 0b1'1000'0110;
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0xFF));
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0xFF - (1 << bit_index))).Times(1);
        expected_registers.F = emulator::memory::make_flag(true, true, true, true);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(BitOpAddressTest, BitOpAddressTestFixture, BIT_INDEXES);
}
