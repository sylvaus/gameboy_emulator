#pragma once

#include <cstdint>
#include <memory>
#include <vector>

#include "emulator/memory/mbc.h"
#include "emulator/video/controller.h"
#include "emulator/logging.h"
#include "ram.h"

namespace emulator::memory
{
    class Memory
    {
    public:
        explicit Memory(MemoryBankControllerPtr mbc) : mbc_(std::move(mbc))
        {}

        virtual void set(uint16_t address, uint8_t value);

        void set16bits(uint16_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }

        [[nodiscard]] virtual uint8_t get(uint16_t address) const;

        virtual ~Memory() = default;

    private:
        MemoryBankControllerPtr mbc_;
        std::shared_ptr<emulator::video::VideoController> video_;
        std::shared_ptr<RamController> ram_;

        void set_io_register(uint16_t address, uint8_t value);
        [[nodiscard]] uint8_t get_io_register(uint16_t address) const;
    };
}
