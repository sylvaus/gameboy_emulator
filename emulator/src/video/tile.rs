/// Information from: https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data
const TILE_SIZE: usize = 16;
const TILE_BLOCK_1_OFFSET: usize = 0x800;


/// Returns the offset from the start of the VRAM memory area based on the bit value given
///
/// Information from:
/// * https://gbdev.io/pandocs/LCDC.html#lcdc6--window-tile-map-area
/// * https://gbdev.io/pandocs/LCDC.html#lcdc3--bg-tile-map-area
pub fn get_vram_tile_offset_from_area(bit_value: u8) -> usize {
    if bit_value > 0 {
        0x1C00 // 0x9C00 - 0x8000
    } else {
        0x1800 // 0x9800 - 0x8000
    }
}

/// Returns the tile address from the tile_index and the flag BG and Window tile data area
///
/// Information from:
/// * https://gbdev.io/pandocs/LCDC.html#lcdc4--bg-and-window-tile-data-area
/// * https://gbdev.io/pandocs/Tile_Data.html
pub fn get_tile_address(mut tile_index: usize, tile_data_area: u8) -> usize {
    let offset = if tile_data_area == 0 {
        // Convert 128 - 255 followed by 0-127 mapping to 0-255 mapping
        tile_index = (tile_index + 128) % 256;
        TILE_BLOCK_1_OFFSET
    } else {
        0
    };

    return offset + (tile_index * TILE_SIZE);
}

/// Returns the color index of the pixel at coordinate x, y on the tile
///
/// Information from https://gbdev.io/pandocs/Tile_Data.html
pub fn get_pixel_value_from_tile(vram: &[u8], tile_address: usize, x: usize, y: usize) -> u8 {
    // Information from https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data
    let low_bits = vram[tile_address + y * 2];
    let high_bits = vram[tile_address + y * 2 + 1];

    let low_bit = (low_bits >> (7 - x)) & 0b1;
    let high_bit = (high_bits >> (7 - x)) & 0b1;
    (high_bit << 1) + low_bit
}