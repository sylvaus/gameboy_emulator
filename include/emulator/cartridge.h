#pragma once

#include <string>
#include <memory>

#include "emulator/memory/mbc.h"

namespace emulator
{
    std::unique_ptr<memory::MemoryBankController> load_cartridge(const std::string &filename);
}

