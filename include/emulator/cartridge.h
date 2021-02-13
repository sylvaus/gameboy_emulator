#pragma once

#include <string>
#include <memory>

#include "emulator/memory/romram_controller.h"

namespace emulator
{
    std::unique_ptr<memory::RomRamController> load_cartridge(const std::string &filename);
}

