#pragma once

#include <string>
#include "emulator/memory/registers.h"
#include "emulator/generated/instructions.h"

namespace emulator::utils
{
    std::string to_string(const emulator::memory::Registers& registers);
    std::string to_string(const emulator::generated::Arguments& arguments);
}