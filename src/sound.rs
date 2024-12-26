
use std::{fs::File, thread::{self, JoinHandle}, time::Duration};
use log::info;
use rodio::{Decoder, OutputStream, Sink, Source};
use image::codecs::png::FilterType;
use tokio::task;
use winit::window::{Fullscreen, Icon};
use ash::vk;
use std::{error::Error, ffi::CString, io::{BufReader, Read}};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::WindowBuilder,
};


pub fn debug_sound_play() {
    thread::spawn(|| {
        info!("Начинаю воспроизводить музыку");
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let file = File::open("src/assets/sounds/back2.mp3").unwrap();
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples()).unwrap();
        thread::sleep(Duration::from_secs(99)); 
    });
}