use std::collections::HashMap;
use std::time::Instant;

use pixels::wgpu::Color;
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;

use winit_input_helper::WinitInputHelper;

use pixels::{Error, Pixels, SurfaceTexture};

use crate::graphics::*;
use crate::raytracing::*;

pub mod graphics;
pub mod raytracing;

// TODO make backend switchable

fn main() {
    
    // this array is too big and causes a stack overflow
    let mut screen: PixelBuf = Vec::with_capacity(SCREEN_HEIGHT * SCREEN_WIDTH);
    unsafe { screen.set_len(SCREEN_HEIGHT * SCREEN_WIDTH) };

    // time for fps counter
    let mut time: Instant = Instant::now();
    let mut old_time: Instant = Instant::now();

    // setup winit
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Raytracer")
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32))
        .with_resizable(true)
        .build(&event_loop)
        .expect("Unable to create window.");

    let mut winit_input = WinitInputHelper::new();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap()
    };

    // raytracing init
    let scene = Scene::init();

    event_loop.run(move |event, _, control_flow| {

        //control_flow.set_poll();
        if winit_input.update(&event) {

            let delta_time = (time.elapsed().as_secs_f64() - old_time.elapsed().as_secs_f64()).abs();

            //input(&mut player, &winit_input, delta_time);

            if winit_input.key_pressed(VirtualKeyCode::Escape) {
                control_flow.set_exit();
            }

        }

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {

                time = Instant::now();

                let delta_time = (time.elapsed().as_secs_f64() - old_time.elapsed().as_secs_f64()).abs();

                // clear frame so theres no ghosting (like what you see when you noclip through the map in half-life)
                //screen.iter_mut().for_each(|x| *x = [0xff, 0xff, 0xff, 0xff]);

                //update(&mut screen, &mut player, delta_time);

                update(&mut screen, &scene);

                //screen.iter_mut().map(|x| *x = pack_colour(0xff, 0x8a, 0x8a));

                render(&screen, pixels.get_frame_mut());

                

                /*
                for pixel in pixels.get_frame_mut().chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]);
                }
                */

                pixels.render().expect("Render failed");

                old_time = time;

                //println!("FPS: {}", 1.0 / (delta_time * -1.0));

            },
            Event::WindowEvent { event: window_event, .. } => match window_event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                //WindowEvent::Resized(new_size) => pixels.resize_surface(new_size.width, new_size.height).unwrap(),
                _ => ()
            },
            _ => ()
        }

    })

}

fn render(framebuffer: &PixelBuf, render_buffer: &mut [u8]) {

    //println!("render");

    for (i, pixel) in render_buffer.chunks_exact_mut(4).enumerate() {
        let x = (i % SCREEN_WIDTH) as usize;
        let y = (i / SCREEN_HEIGHT) as usize;

        let index = x + y * SCREEN_WIDTH;

        if index >= framebuffer.len() {
            continue;
        }

        let pixel_colour = framebuffer[index];

        pixel.copy_from_slice(&pixel_colour);

    }

}
