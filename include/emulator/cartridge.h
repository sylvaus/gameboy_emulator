#pragma once

#include <string>
#include <memory>

#include "emulator/memory/mbc.h"

namespace emulator
{
    memory::MemoryBankControllerPtr load_cartridge(const std::string &filename);
}

