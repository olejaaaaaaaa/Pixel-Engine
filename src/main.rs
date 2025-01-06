#![allow(warnings)]
#![feature(duration_millis_float)]

/*
    Copyright 2024 Oleg Pavlenko
    This is a fucking Russian program
*/


extern crate tokio;
use image::codecs::png::FilterType;
use log::{debug, info};
use rodio::Source;
use winit::window::{Fullscreen, Icon};
use ash::vk;
use std::{error::Error, ffi::CString, fs::File, io::{BufReader, Read}, time::Instant};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::WindowBuilder,
};


extern crate pixel_init_render;
use pixel_init_render::{debug_draw, PixelEngine};

use tokio::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    unsafe { 
        std::env::set_var("RUST_LOG", "Debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    };

    env_logger::init();

    // debug_udp_client();
    // debug_sound_play();
    // debug_exec_script();

    // let mut icon_image = image::open("src/assets/icons/game_icon.png")
    //     .expect("Не удалось открыть изображение")
    //     .to_rgba8();

    // icon_image = image::imageops::resize(&icon_image, 256, 256, image::imageops::FilterType::Lanczos3);
    // let icon = Icon::from_rgba(icon_image.as_raw().to_vec(), icon_image.width(), icon_image.height()).unwrap();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        //.with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_title("Cube game")
        .with_inner_size(PhysicalSize::new(720, 480))
        //.with_window_icon(Some(icon))
        .build(&event_loop)?;

    let mut engine = unsafe { PixelEngine::new(&window) };

    
    let mut time_begin = Instant::now();
    event_loop.run(move |event, elwp| { 
        
        match event {

            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },

            winit::event::Event::WindowEvent { window_id, event } => {
                match event {

                    winit::event::WindowEvent::CloseRequested => {
                        elwp.exit();
                    },

                    winit::event::WindowEvent::RedrawRequested => {
                        let dt =  time_begin.elapsed().as_millis_f64();
                        //debug!("{:?} fps", 1.0 / dt * 1000.0);
                        time_begin = Instant::now();
                    }

                    winit::event::WindowEvent::Resized(size) => {
                        let width = size.width as usize;
                        let height = size.height as usize;
                        //engine.resize(width, height);
                    },

                    winit::event::WindowEvent::Destroyed => {
                            
                    },

                     _ => ()
                }
            },

            _ => {}
        }
    
    })?;

    Ok(())

}