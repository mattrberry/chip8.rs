use std::mem;

use bytemuck::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
struct RGB(u8, u8, u8);
unsafe impl Zeroable for RGB {}
unsafe impl Pod for RGB {}

const OFF: RGB = RGB(0x7C, 0x3F, 0x58);
const ON: RGB = RGB(0xFF, 0xF6, 0xD3);
const EMPTY_BUFFER: [RGB; (WIDTH * HEIGHT) as usize] = [OFF; (WIDTH * HEIGHT) as usize];

pub struct Display {
    canvas: Canvas<Window>,
    framebuffer: [RGB; (WIDTH * HEIGHT) as usize],
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Display {
        let video_subsystem = sdl_context
            .video()
            .expect("Failed to initialize SDL2 video");
        let window = video_subsystem
            .window("Chip-8 - Rust", WIDTH * SCALE, HEIGHT * SCALE)
            .position_centered()
            .build()
            .expect("Failed to create window");
        let canvas = window
            .into_canvas()
            .build()
            .expect("Failed to create canvas");
        Display {
            canvas: canvas,
            framebuffer: EMPTY_BUFFER,
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer = EMPTY_BUFFER;
        self.render();
    }

    pub fn add_sprite(&mut self, x: u8, y: u8, data: &[u8]) -> u8 {
        let mut collision: u8 = 0;
        for (row, pixel) in data.iter().enumerate() {
            for col in 0..8 {
                if pixel & (0x80 >> col) > 0 {
                    let y_pos = (row + y as usize) % HEIGHT as usize;
                    let x_pos = (col + x as usize) % WIDTH as usize;
                    let idx = y_pos as usize * WIDTH as usize + x_pos;
                    if self.framebuffer[idx] == ON {
                        collision = 1;
                        self.framebuffer[idx] = OFF;
                    } else {
                        self.framebuffer[idx] = ON;
                    }
                }
            }
        }
        self.render();
        collision
    }

    fn render(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
            .expect("Failed to create texture");
        texture
            .update(
                None,
                bytes_of(&self.framebuffer),
                WIDTH as usize * mem::size_of::<RGB>(),
            )
            .expect("Failed to update texture");
        self.canvas.clear();
        self.canvas
            .copy(&texture, None, None)
            .expect("Failed to copy texture");
        self.canvas.present();
    }
}
