#pragma once

#include <cstdint>
#include <memory>
#include <vector>
#include "emulator/logging.h"
#include "emulator/memory/memory_map.h"

namespace emulator::memory
{
    class MemoryBankController
    {
    public:
        explicit MemoryBankController(uint8_t nb_external_ram_banks);

        virtual void set_rom(uint16_t address, uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_rom(uint16_t  address) const = 0;
        virtual void set_ext_ram(uint16_t address, uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_ext_ram(uint16_t  address) const = 0;
        virtual ~MemoryBankController() = default;

    protected:
        std::vector<RamBank> external_ram_banks_;
    };

    using MemoryBankControllerPtr = std::shared_ptr<MemoryBankController>;

    class NoMemoryBankController : public MemoryBankController
    {
    public:
        static MemoryBankControllerPtr create(std::vector<memory::RomBank>&, uint8_t nb_external_ram_banks);

        NoMemoryBankController(std::vector<RomBank> &rom_banks, uint8_t nb_external_ram_banks);

        void set_rom(uint16_t address, uint8_t value) override;
        [[nodiscard]]  uint8_t get_rom(uint16_t  address) const override;
        void set_ext_ram(uint16_t address, uint8_t value) override;
        [[nodiscard]]  uint8_t get_ext_ram(uint16_t  address) const override;

    protected:
        std::vector<RomBank> rom_banks_;
    };
}


