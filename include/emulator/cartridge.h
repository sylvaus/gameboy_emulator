#pragma once

#include <string>
#include <memory>

#include "emulator/romram_controller.h"

namespace emulator
{
    std::unique_ptr<RomRamController> load_cartridge(const std::string &filename);
}

