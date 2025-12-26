use crate::cartridge::{CGBFlag, Cartridge, CartridgeInfo, MBCType, RAMSize, RAMSizeInfo, ROMSize, ROMSizeInfo};
use crate::memory::mbc::common::ROM_BANK_SIZE;
use crate::memory::mbc::mbc::MemoryBankController;
use crate::memory::mbc::no_controller::NoMemoryBankController;

pub(crate) fn create_valid_cartridge() -> Cartridge {
    Cartridge {
        title: "Test Cartridge".to_string(),
        cgb_flag: CGBFlag::NonCgbCompatible,
        info: CartridgeInfo {
            bank_type: MBCType::RomOnly,
            sram: false,
            battery: false,
            rtc: false,
        },
        rom_info: ROMSizeInfo {
            rom_size: ROMSize::S32KB,
            num_banks: 0,
        },
        ram_info: RAMSizeInfo {
            ram_size: RAMSize::NONE,
            num_banks: 0,
        },
        valid_header_checksum: false,
        memory_controller: MemoryBankController::NoController(
            NoMemoryBankController::new(vec![0; 2 * ROM_BANK_SIZE], vec![])
        ),
    }
}