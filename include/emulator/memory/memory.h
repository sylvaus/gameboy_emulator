#pragma once

#include <cstdint>
#include <memory>
#include <vector>

#include "emulator/memory/mbc.h"
#include "emulator/video/controller.h"
#include "emulator/logging.h"
#include "emulator/memory/ram.h"
#include "emulator/sound/controller.h"
#include "emulator/io/joypad.h"
#include "emulator/time/timer.h"
#include "emulator/io/serial.h"

namespace emulator::memory
{
    class Memory
    {
    public:
        virtual void set(uint16_t address, uint8_t value) = 0;

        void set16bits(uint16_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }

        [[nodiscard]] virtual uint8_t get(uint16_t address) const = 0;

        virtual ~Memory() = default;
    };

    class MemoryNoCGB: public Memory
    {
    public:
        MemoryNoCGB(MemoryBankControllerPtr mbc, std::shared_ptr<emulator::video::VideoController> video,
               std::shared_ptr<RamController> ram, std::shared_ptr<emulator::io::Joypad> joypad,
               std::shared_ptr<emulator::io::SerialTransfer> serial,
               std::shared_ptr<emulator::sound::SoundController> sound,
               std::shared_ptr<emulator::time::Timer> timer);

        void set(uint16_t address, uint8_t value) override;
        [[nodiscard]] uint8_t get(uint16_t address) const override;

    private:
        MemoryBankControllerPtr mbc_;
        std::shared_ptr<emulator::video::VideoController> video_;
        std::shared_ptr<RamController> ram_;
        std::shared_ptr<emulator::io::Joypad> joypad_;
        std::shared_ptr<emulator::io::SerialTransfer> serial_;
        std::shared_ptr<emulator::sound::SoundController> sound_;
        std::shared_ptr<emulator::time::Timer> timer_;

        uint8_t oam_dma_high_bits_{};
        uint8_t interrupt_enable_register_{};
        uint8_t boot_rom_disabled_{};

        void set_io_register(uint16_t address, uint8_t value);
        [[nodiscard]] uint8_t get_io_register(uint16_t address) const;
        void set_lcd(uint16_t address, uint8_t value);
        [[nodiscard]] uint8_t get_lcd(uint16_t address) const;
        void set_oam_dma(uint8_t value);
        [[nodiscard]] uint8_t get_oam_dma() const;
        void handle_vram_dma(uint16_t address, uint8_t value);
    };
}
