#pragma once

#include <cstdint>
#include <memory>
#include <vector>
#include <array>

#include "memory_map.h"
#include "emulator/logging.h"

namespace emulator::memory
{
    using std::vector;
    using std::unique_ptr;

    class RamController
    {
    public:

        virtual void set(uint16_t address, uint8_t value) = 0;
        void set16bits(uint16_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }
        [[nodiscard]] virtual uint8_t get(uint16_t  address) const = 0;

        virtual ~RamController() = default;
    };

    class RamControllerBase : public RamController
    {
    public:
        HighRam high_ram;

        explicit RamControllerBase(uint8_t nb_ram_banks);

        virtual void set(uint16_t address, uint8_t value) override;
        [[nodiscard]]  virtual uint8_t get(uint16_t  address) const override;

    protected:
        vector<RamBank> external_ram_banks_;

        virtual void set_work_ram(uint16_t address, uint8_t value) = 0;
        [[nodiscard]]  virtual uint8_t get_work_ram(uint16_t  address) const = 0;
    };

    class RamControllerNoCGBOnly : public RamControllerBase
    {
    public:
        static std::unique_ptr<memory::RamController> create(uint8_t nb_ram_banks);

        HighRam high_ram;

        explicit RamControllerNoCGBOnly(uint8_t nb_ram_banks);

        void set(uint16_t address, uint8_t value) override;
        [[nodiscard]] uint8_t get(uint16_t  address) const override;

    protected:
        WorkRamBank ram_bank_0_;
        WorkRamBank ram_bank_1_;

        void set_work_ram(uint16_t address, uint8_t value) override;
        [[nodiscard]]  uint8_t get_work_ram(uint16_t  address) const override;
    };

    class RamControllerCGBOnly : public RamControllerBase
    {
    public:
        static std::unique_ptr<memory::RamController> create(uint8_t nb_ram_banks);

        HighRam high_ram;

        explicit RamControllerCGBOnly(uint8_t nb_ram_banks);

        void set(uint16_t address, uint8_t value) override;
        [[nodiscard]] uint8_t get(uint16_t  address) const override;

    protected:
        WorkRamBank ram_bank_0_;
        std::array<WorkRamBank, 7> switchable_ram_banks_;

        void set_work_ram(uint16_t address, uint8_t value) override;
        [[nodiscard]]  uint8_t get_work_ram(uint16_t  address) const override;
    };
}


