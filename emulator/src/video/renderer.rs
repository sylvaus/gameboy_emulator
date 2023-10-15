use crate::video::controller::VideoController;
use crate::video::sprite::{
    get_intersected_sprites, get_pixel_value_from_sprite, SpriteSize, SPRITE_Y_OFFSET,
};
use crate::video::tile::{
    get_pixel_value_from_tile, get_tile_address, get_vram_tile_offset_from_area,
};
use std::cmp::min;
use std::marker::PhantomData;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
const NB_BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH_BYTES: usize = (SCREEN_WIDTH as usize) * NB_BYTES_PER_PIXEL;
const NB_PIXELS: usize = ((SCREEN_WIDTH * SCREEN_HEIGHT) as usize) * NB_BYTES_PER_PIXEL;

/// Information from: https://gbdev.io/pandocs/Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7
pub const WINDOW_X_OFFSET: usize = 7;
/// Information from: https://gbdev.io/pandocs/Tile_Maps.html
const TILE_MAP_WIDTH: usize = 32;
const TILE_MAP_HEIGHT: usize = 32;
const TILE_MAP_TOTAL_SIZE: usize = TILE_MAP_WIDTH * TILE_MAP_HEIGHT;

/// Information from https://gbdev.io/pandocs/Palettes.html#ff47--bgp-non-cgb-mode-only-bg-palette-data
pub const WHITE: Color = Color {
    alpha: 0,
    red: 0xFF,
    green: 0xFF,
    blue: 0xFF,
};

pub const LIGHT_GRAY: Color = Color {
    alpha: 0,
    red: 170,
    green: 170,
    blue: 170,
};

pub const DARK_GRAY: Color = Color {
    alpha: 0,
    red: 85,
    green: 85,
    blue: 85,
};

pub const BLACK: Color = Color {
    alpha: 0,
    red: 0,
    green: 0,
    blue: 0,
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub trait VideoRenderer {
    fn scanline(&mut self, video: &VideoController);
    fn update_frame(&mut self);
}

/// This class provides the common implementation of the video renderer for non-cgb mode.
///
/// The final renderer only has to provide a write_pixel function and update the frame.
pub struct CoreNonCgbRenderer {
    window_y: usize,
}

impl CoreNonCgbRenderer {
    pub fn new() -> Self {
        Self { window_y: 0 }
    }

    pub fn scanline<PixelWriter>(&mut self, video: &VideoController, mut writer: PixelWriter)
    where
        PixelWriter: FnMut(usize, usize, &Color),
    {
        if video.control.read_lcd_enable() == 0 {
            return;
        }

        if video.control.read_bg_window_enable() != 0 {
            self.render_background_window(video, &mut writer);
        } else {
            self.render_blank(video, &mut writer);
        }
        if video.control.read_obj_enable() != 0 {
            self.render_sprites(video, &mut writer);
        }
    }

    fn render_blank<PixelWriter>(&mut self, video: &VideoController, writer: &mut PixelWriter)
    where
        PixelWriter: FnMut(usize, usize, &Color),
    {
        let y = (video.coordinate_y - 1) as usize;
        for x in 0..(SCREEN_WIDTH as usize) {
            writer(x, y, &WHITE);
        }
    }

    fn render_background_window<PixelWriter>(
        &mut self,
        video: &VideoController,
        writer: &mut PixelWriter,
    ) where
        PixelWriter: FnMut(usize, usize, &Color),
    {
        // Information from: https://gbdev.io/pandocs/pixel_fifo.html#get-tile
        let y = (video.coordinate_y - 1) as usize;
        // Information from: https://gbdev.io/pandocs/Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7
        let window_enabled = (video.control.read_window_enable() != 0)
            && ((video.window_position_y as u32) < SCREEN_HEIGHT)
            && ((video.window_position_x as u32) < (SCREEN_WIDTH + WINDOW_X_OFFSET as u32))
            && (6 < video.window_position_x); // 0-6 are not valid values: page 30 https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
        let window_enabled_for_y = window_enabled && ((video.window_position_y as usize) <= y);
        let background_enabled = video.control.read_bg_window_enable() != 0;

        for x in 0..(SCREEN_WIDTH as usize) {
            let window_x = (video.window_position_x as usize).saturating_sub(WINDOW_X_OFFSET);
            let color = if window_enabled_for_y && (window_x <= x) {
                self.get_window_pixel(video, x - window_x, self.window_y)
            } else if background_enabled {
                self.get_background_pixel(
                    video,
                    x + video.scroll_x as usize,
                    y + video.scroll_y as usize,
                )
            } else {
                WHITE
            };
            writer(x, y, &color);
        }

        if window_enabled {
            self.window_y += 1;
            if self.window_y == SCREEN_HEIGHT as usize {
                self.window_y = 0;
            }
        }
    }

    fn get_window_pixel(&mut self, video: &VideoController, x: usize, y: usize) -> Color {
        let tile_map_offset =
            get_vram_tile_offset_from_area(video.control.read_window_tile_map_area());

        self.get_tile_pixel(video, x, y, tile_map_offset)
    }

    fn get_background_pixel(&mut self, video: &VideoController, x: usize, y: usize) -> Color {
        let tile_map_offset = get_vram_tile_offset_from_area(video.control.read_bg_tile_map_area());

        self.get_tile_pixel(video, x, y, tile_map_offset)
    }

    fn get_tile_pixel(
        &mut self,
        video: &VideoController,
        x: usize,
        y: usize,
        tile_map_offset: usize,
    ) -> Color {
        let tile_map_x = x / 8;
        let tile_map_y = y / 8;

        let tile_map_index =
            ((tile_map_y * 32) % TILE_MAP_TOTAL_SIZE) + (tile_map_x % TILE_MAP_WIDTH);

        let tile_index = video.vram[tile_map_offset + tile_map_index];
        let tile_address = get_tile_address(
            tile_index as usize,
            video.control.read_bg_window_tile_data_area(),
        );

        let tile_x = x % 8;
        let tile_y = y % 8;
        let color_index = get_pixel_value_from_tile(&video.vram, tile_address, tile_x, tile_y);

        get_non_cgb_color(color_index, video.bg_palette_data.value)
    }

    fn render_sprites<PixelWriter>(&mut self, video: &VideoController, writer: &mut PixelWriter)
    where
        PixelWriter: FnMut(usize, usize, &Color),
    {
        let y = (video.coordinate_y - 1) as usize;

        // Information from: https://gbdev.io/pandocs/LCDC.html#lcdc2--obj-size
        let object_size = if video.control.read_obj_size() == 1 {
            SpriteSize::Size8x16
        } else {
            SpriteSize::Size8x8
        };

        let mut sprites = get_intersected_sprites(&video.oam, y, object_size);
        // Information from: https://gbdev.io/pandocs/OAM.html#drawing-priority
        sprites.sort_by_key(|sprite| sprite.x);

        let mut y_colors: [Option<Color>; SCREEN_WIDTH as usize] = [None; SCREEN_WIDTH as usize];
        for sprite in sprites.iter().rev() {
            let palette = match sprite.read_non_cgb_palette() {
                0 => video.obj_palette_data_0,
                1 => video.obj_palette_data_1,
                _ => panic!("This should never happen."),
            };
            let min_x: usize = 8usize.saturating_sub(sprite.x);
            let max_x = min(8usize, (SCREEN_WIDTH as usize).saturating_sub(sprite.x));
            let sprite_y = (y + SPRITE_Y_OFFSET) - sprite.y;

            for sprite_x in min_x..max_x {
                let color_index =
                    get_pixel_value_from_sprite(&video.vram, sprite, sprite_x, sprite_y);
                let color = if color_index == 0 {
                    None
                } else {
                    Some(get_non_cgb_color(color_index, palette.value))
                };
                y_colors[(sprite.x + sprite_x) - 8] = color;
            }
        }

        for (x, color) in y_colors.iter().enumerate() {
            if let Some(color) = color {
                writer(x, y, &color);
            }
        }
    }
}

/// Information from https://gbdev.io/pandocs/Palettes.html#ff47--bgp-non-cgb-mode-only-bg-palette-data
fn get_non_cgb_color(index: u8, palette: u8) -> Color {
    let color = (palette >> (index * 2)) & 0b11;

    match color {
        0 => WHITE,
        1 => LIGHT_GRAY,
        2 => DARK_GRAY,
        3 => BLACK,
        _ => panic!(
            "Non CGB color only supports value in range [0..=3], got: {}",
            index
        ),
    }
}
