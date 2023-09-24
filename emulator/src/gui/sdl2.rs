use crate::gui::Gui;
use crate::joypad::{InputProvider, JoypadInput};
use crate::video::controller::VideoController;
use crate::video::renderer::{VideoRenderer, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::video::sprite::{
    get_intersected_sprites, get_pixel_value_from_sprite, SpriteSize, SPRITE_Y_OFFSET,
};
use crate::video::tile::{
    get_pixel_value_from_tile, get_tile_address, get_vram_tile_offset_from_area,
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum::ARGB8888;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::{EventPump, Sdl};
use std::cmp::min;

const INIT_SCALE: u32 = 3;

const NB_BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH_BYTES: usize = (SCREEN_WIDTH as usize) * NB_BYTES_PER_PIXEL;
const NB_PIXELS: usize = ((SCREEN_WIDTH * SCREEN_HEIGHT) as usize) * NB_BYTES_PER_PIXEL;

/// Information from: https://gbdev.io/pandocs/Tile_Maps.html
const TILE_MAP_WIDTH: usize = 32;
const TILE_MAP_HEIGHT: usize = 32;
const TILE_MAP_TOTAL_SIZE: usize = TILE_MAP_WIDTH * TILE_MAP_HEIGHT;

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

        get_non_cgb_color(color_index, video.bg_palette_data.value)
    }

    fn render_sprites(&mut self, video: &VideoController) {
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
                self.write_pixel(x, y, &color);
            }
        }
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
            .window(
                "Emulator",
                SCREEN_WIDTH * INIT_SCALE,
                SCREEN_HEIGHT * INIT_SCALE,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        // TODO: implement dynamic resizing.
        canvas
            .set_scale(INIT_SCALE as f32, INIT_SCALE as f32)
            .expect("Scaling should not fail");
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
