#pragma once

#include <string>
#include <memory>

#include "emulator/memory_controller.h"

namespace emulator
{
    std::unique_ptr<MemoryController> load_cartridge(const std::string &filename);
}

