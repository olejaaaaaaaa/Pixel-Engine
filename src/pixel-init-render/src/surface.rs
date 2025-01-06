
use ash::vk::SurfaceKHR;
use ash::Entry;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::PixelEngine;

impl PixelEngine {
    pub fn create_surface(entry: &Entry, instance: &ash::Instance, window: &winit::window::Window) -> SurfaceKHR {
        use ash_window;
        let surface = unsafe { ash_window::create_surface(entry, instance, window.display_handle().unwrap().into(), window.window_handle().unwrap().into(), None).expect("Не получилось создать поверхность") };
        return surface;
    }
}