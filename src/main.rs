use sdl2::pixels::Color; 
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub mod graphics;
pub mod raytracing;

use crate::graphics::*;
use crate::raytracing::*;

fn main() {
    
    let mut screen: PixelBuf = Vec::with_capacity(SCREEN_HEIGHT * SCREEN_WIDTH);
    unsafe { screen.set_len(SCREEN_HEIGHT * SCREEN_WIDTH) };

    let scene = Scene::init();
    update(&mut screen, &scene);

    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    let window = video.window("SDL2 Test", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .expect("Unable to build window");

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 25));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    for (i, colour) in screen.iter().enumerate() {

        let x = i % SCREEN_WIDTH;
        let y = i / SCREEN_WIDTH;

        canvas.set_draw_color(Color::RGB(colour[0], colour[1], colour[2]));
        canvas.draw_point((x as i32, y as i32)).unwrap();

    }

    'main: loop {
        /*
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - 1));
        canvas.clear();
        */

        canvas.set_draw_color(Color::WHITE);
        //canvas.clear();

        for event in event_pump.poll_iter() {
            match  event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                }
                _ => {}
            }
        }

        
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

}