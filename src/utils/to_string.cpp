#include <sstream>
#include "emulator/utils/to_string.h"

namespace emulator::utils
{
    std::string to_string(const emulator::memory::Registers& registers)
    {
        std::stringstream stream;
        stream << "Registers(F: " << std::hex << static_cast<uint32_t>(registers.F)
               << ", A: 0x" << std::hex << static_cast<uint16_t>(registers.A)
               << ", C: 0x" << std::hex << static_cast<uint16_t>(registers.C)
               << ", B: 0x" << std::hex << static_cast<uint16_t>(registers.B)
               << ", E: 0x" << std::hex << static_cast<uint16_t>(registers.E)
               << ", D: 0x" << std::hex << static_cast<uint16_t>(registers.D)
               << ", L: 0x" << std::hex << static_cast<uint16_t>(registers.L)
               << ", H: 0x" << std::hex << static_cast<uint16_t>(registers.H)
               << ", PC: 0x" << std::hex << registers.PC
               << ", SP: 0x" << std::hex << registers.SP
               << ",halted: 0x" << static_cast<int16_t>(registers.halted)
               << ",stopped: 0x" << static_cast<int16_t>(registers.stopped)
               << ",ime_flag: 0x" << static_cast<int16_t>(registers.ime_flag)
               << ")";
        return stream.str();
    }

    std::string to_string(const generated::Arguments &arguments)
    {
        std::stringstream stream;
        stream << "Arguments(uint16: " << arguments.uint16
               << " ,uint8: " << static_cast<uint16_t>(arguments.uint8)
               << " ,int8: " << static_cast<int16_t>(arguments.int8)
               << ")";
        return stream.str();
    }
}