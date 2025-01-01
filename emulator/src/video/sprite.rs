use crate::video::tile::get_pixel_value_from_tile;
use macros::BitAccessor;

/// Information from: https://gbdev.io/pandocs/OAM.html
pub const NB_SPRITES: usize = 40;
pub const SPRITE_X_OFFSET: usize = 8;
pub const SPRITE_Y_OFFSET: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SpriteSize {
    Size8x8,
    Size8x16,
}

impl SpriteSize {
    pub fn get_height(&self) -> usize {
        match self {
            SpriteSize::Size8x8 => 8,
            SpriteSize::Size8x16 => 16,
        }
    }

    pub fn get_tile_size(&self) -> usize {
        match self {
            SpriteSize::Size8x8 => 16,
            SpriteSize::Size8x16 => 16 * 2,
        }
    }
}

#[derive(BitAccessor, Debug, Copy, Clone)]
pub struct Sprite {
    pub x: usize,
    pub y: usize,
    pub index: usize,

    #[bit_offset_size(bg_window_over_obj, 7, 1)]
    #[bit_offset_size(y_flip, 6, 1)]
    #[bit_offset_size(x_flip, 5, 1)]
    #[bit_offset_size(non_cgb_palette, 4, 1)]
    #[bit_offset_size(tile_vram_bank, 3, 1)]
    #[bit_offset_size(cgb_palette, 0, 3)]
    pub flags: u8,
    pub size: SpriteSize,
}

impl Sprite {
    pub fn get_tile_address(&self) -> usize {
        self.index * self.size.get_tile_size()
    }
}

pub fn get_intersected_sprites(oam: &[u8], mut y: usize, size: SpriteSize) -> Vec<Sprite> {
    y += SPRITE_Y_OFFSET;

    let height = size.get_height();
    let mut sprites = Vec::new();
    for i in 0..NB_SPRITES {
        let offset = 4 * i;
        let sprite_y = oam[offset] as usize;
        let sprite_x = oam[offset + 1] as usize;
        if y < (sprite_y + height) && sprite_y <= y {
            sprites.push(Sprite {
                x: sprite_x,
                y: sprite_y,
                index: oam[offset + 2] as usize,
                flags: oam[offset + 3],
                size,
            });

            // Information from: https://gbdev.io/pandocs/OAM.html#object-priority-and-conflicts
            // Only the first 10 Sprites are kept.
            if sprites.len() == 10 {
                break;
            }
        }
    }

    sprites
}

pub fn get_pixel_value_from_sprite(vram: &[u8], sprite: &Sprite, mut x: usize, mut y: usize) -> u8 {
    let tile_address = sprite.get_tile_address();
    if sprite.read_x_flip() == 1 {
        x = 7 - x;
    }
    if sprite.read_y_flip() == 1 {
        y = sprite.size.get_tile_size() - y;
    }

    get_pixel_value_from_tile(vram, tile_address, x, y)
}
