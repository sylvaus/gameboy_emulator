use crate::gui::Gui;
use crate::joypad::{InputProvider, JoypadInput};
use crate::video::controller::VideoController;
use crate::video::renderer::{VideoRenderer, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum::ARGB8888;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::{EventPump, Sdl};

const NB_BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH_BYTES: usize = (SCREEN_WIDTH as usize) * NB_BYTES_PER_PIXEL;
const NB_PIXELS: usize = ((SCREEN_WIDTH * SCREEN_HEIGHT) as usize) * NB_BYTES_PER_PIXEL;

/// Information from: https://gbdev.io/pandocs/Tile_Maps.html
const TILE_MAP_WIDTH: usize = 32;
const TILE_MAP_HEIGHT: usize = 32;
const TILE_MAP_TOTAL_SIZE: usize = TILE_MAP_WIDTH * TILE_MAP_HEIGHT;

/// Information from: https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data
const TILE_SIZE: usize = 16;
const TILE_BLOCK_1_OFFSET: usize = 0x800;

/// Information from https://gbdev.io/pandocs/Palettes.html#ff47--bgp-non-cgb-mode-only-bg-palette-data
const WHITE: Color = Color {
    alpha: 0,
    red: 0xFF,
    green: 0xFF,
    blue: 0xFF,
};

const LIGHT_GRAY: Color = Color {
    alpha: 0,
    red: 170,
    green: 170,
    blue: 170,
};

const DARK_GRAY: Color = Color {
    alpha: 0,
    red: 85,
    green: 85,
    blue: 85,
};

const BLACK: Color = Color {
    alpha: 0,
    red: 0,
    green: 0,
    blue: 0,
};

#[derive(Debug, Clone, Copy)]
struct Color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Information from https://gbdev.io/pandocs/Palettes.html#ff47--bgp-non-cgb-mode-only-bg-palette-data
fn get_non_cgb_color(index: u8) -> Color {
    match index {
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

/// Returns the offset from the start of the VRAM memory area based on the bit value given
///
/// Information from:
/// * https://gbdev.io/pandocs/LCDC.html#lcdc6--window-tile-map-area
/// * https://gbdev.io/pandocs/LCDC.html#lcdc3--bg-tile-map-area
fn get_vram_tile_offset_from_area(bit_value: u8) -> usize {
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
fn get_tile_address(mut tile_index: usize, tile_data_area: u8) -> usize {
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

    let low_bit = (low_bits >> (7 -x)) & 0b1;
    let high_bit = (high_bits >> (7 -x)) & 0b1;
    (high_bit << 1) + low_bit
}

pub struct Sdl2Gui<'a> {
    // TODO: split graphic and input into two sub structs
    // Graphic Fields
    canvas: &'a mut WindowCanvas,
    texture: Texture<'a>,
    pixels: [u8; NB_PIXELS],

    // Input fields
    events: EventPump,
    quit_pressed: bool,
}

impl<'a> Sdl2Gui<'a> {
    pub fn new(canvas: &'a mut WindowCanvas, texture: Texture<'a>, events: EventPump) -> Self {
        Self {
            canvas,
            texture,
            pixels: [0; NB_PIXELS],
            events,
            quit_pressed: false,
        }
    }

    fn render_background_window(&mut self, video: &VideoController) {
        // Information from: https://gbdev.io/pandocs/pixel_fifo.html#get-tile
        let y = (video.coordinate_y - 1) as usize;
        for x in 0..(SCREEN_WIDTH as usize) {
            let color = if video.control.read_window_enable() != 0 {
                self.get_window_pixel(video, x, y)
            } else if video.control.read_bg_window_enable() != 0 {
                self.get_background_pixel(video, x, y)
            } else {
                WHITE
            };
            self.write_pixel(x, y, &color);
        }
    }

    fn get_window_pixel(&mut self, video: &VideoController, x: usize, y: usize) -> Color {
        // TODO: implement
        let tile_offset = get_vram_tile_offset_from_area(video.control.read_window_tile_map_area());
        WHITE
    }

    fn get_background_pixel(&mut self, video: &VideoController, x: usize, y: usize) -> Color {
        let tile_map_offset = get_vram_tile_offset_from_area(video.control.read_bg_tile_map_area());
        let background_x = video.scroll_x as usize + x;
        let background_y = video.scroll_y as usize + y;

        let tile_map_x = background_x / 8;
        let tile_map_y = background_y / 8;

        let tile_map_index =
            ((tile_map_y * 32) % TILE_MAP_TOTAL_SIZE) + (tile_map_x % TILE_MAP_WIDTH);

        let tile_index = video.vram[tile_map_offset + tile_map_index];
        let tile_address = get_tile_address(
            tile_index as usize,
            video.control.read_bg_window_tile_data_area(),
        );

        let tile_x = background_x % 8;
        let tile_y = background_y % 8;
        let color_index = get_pixel_value_from_tile(&video.vram, tile_address, tile_x, tile_y);

        get_non_cgb_color(color_index)
    }

    fn render_sprites(&mut self, video: &VideoController) {
        // TODO: implement
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        let start_position = y * SCREEN_WIDTH_BYTES + x * NB_BYTES_PER_PIXEL;
        self.pixels[start_position] = color.blue;
        self.pixels[start_position + 1] = color.green;
        self.pixels[start_position + 2] = color.red;
        self.pixels[start_position + 3] = color.alpha;
    }
}

impl<'a> Gui for Sdl2Gui<'a> {}

fn create_screen_rect() -> Rect {
    Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)
}

impl<'a> VideoRenderer for Sdl2Gui<'a> {
    fn scanline(&mut self, video: &VideoController) {
        if video.control.read_lcd_enable() == 0 {
            return;
        }

        self.render_background_window(video);
        if video.control.read_obj_enable() != 0 {
            self.render_sprites(video);
        }
    }

    fn update_frame(&mut self) {
        self.canvas.clear();
        let screen = create_screen_rect();
        self.texture
            .update(screen, &self.pixels, SCREEN_WIDTH_BYTES)
            .unwrap();
        self.canvas.copy(&self.texture, None, Some(screen)).unwrap();
        self.canvas.present()
    }
}

impl<'a> InputProvider for Sdl2Gui<'a> {
    fn update_inputs(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.quit_pressed = true;
                    break;
                }
                _ => {}
            }
        }
    }

    fn set_inputs(&self, input: &mut JoypadInput) -> bool {
        // TODO: implement
        false
    }

    fn is_quit_pressed(&self) -> bool {
        self.quit_pressed
    }
}

pub struct Sdl2Context {
    pub context: Sdl,
}

impl Sdl2Context {
    pub fn new(context: Sdl) -> Self {
        Self { context }
    }
}

pub struct Sdl2GuiProvider {
    context: Sdl,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
}

impl Sdl2GuiProvider {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            context,
            canvas,
            texture_creator,
        }
    }

    pub fn get(&mut self) -> Sdl2Gui {
        let texture = self
            .texture_creator
            .create_texture_streaming(ARGB8888, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
        Sdl2Gui::new(
            &mut self.canvas,
            texture,
            self.context.event_pump().unwrap(),
        )
    }
}
