#include <iostream>

#include <emulator/logging.h>
#include <emulator/cartridge.h>

int main()
{
    emulator::Logging::set_logging_level(emulator::DEBUG_LEVEL);

    emulator::load_cartridge("rom_path");
    return 0;
}
