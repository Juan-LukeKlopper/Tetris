extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

use std::time::{Duration, SystemTime};
use std::thread::sleep;

const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor, 
    size: u32) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) =
        texture_creator.create_texture_target(None, size, size) {
            canvas.with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green =>
                        texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue =>
                        texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            }).expect("Failed to color a texture");
            Some(square_texture)
        } else {
            None
        }
}

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

    let window = video_subsystem.window("Tetris in Rust!", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let green_square = create_texture_rect(&mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE).expect("failed to create a texture");

    let blue_square = create_texture_rect(&mut canvas,
        &texture_creator,
        TextureColor::Blue,
        TEXTURE_SIZE).expect("failed to create a texture");

    let timer = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => {
                true
            }
        };

        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };

        canvas.copy(square_texture,
            None,
            Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
