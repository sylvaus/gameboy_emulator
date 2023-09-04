/// Interrupts are listed here: https://gbdev.io/pandocs/Interrupts.html#ffff--ie-interrupt-enable
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}
