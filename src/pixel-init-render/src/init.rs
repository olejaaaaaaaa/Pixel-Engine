#![allow(warnings)]

use std::ffi::{CStr, CString, FromBytesUntilNulError};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr::{self, null};
use std::str::FromStr;

use ash::khr::swapchain;
use ash::prelude::VkResult;
use ash::vk::*;
use ash::Entry;
use log::{debug, info, warn};


use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
pub struct PixelEngine {

}

impl PixelEngine {

    pub unsafe fn new(window: &winit::window::Window) {
        let entry = ash::Entry::load().unwrap();
        
        let instance = Self::create_instance(&entry);
        let debug_info = Self::setup_debug_utils();
        let debug_loader = ash::ext::debug_utils::Instance::new(&entry, &instance.instance);
        let debug_callback = debug_loader.create_debug_utils_messenger(&debug_info, None).expect("Не получилось создать debug utils messenger");
        let surface = Self::create_surface(&entry, &instance.instance, window);
        let phys = Self::create_physical_device(&instance.instance);
        let graphics = Self::get_index_family_queues(&phys.queue_family_prop, QueueFlags::GRAPHICS);
        let transfer = Self::get_index_family_queues(&phys.queue_family_prop, QueueFlags::TRANSFER);
        let compute = Self::get_index_family_queues(&phys.queue_family_prop, QueueFlags::COMPUTE);
        let (device, queue) = Self::create_device_and_queues(&instance.instance, &phys.phys_device, &graphics, &transfer, &compute);
        let count = phys.queue_family_prop.len();
        let swapchain = Self::create_swapchain(&entry, &instance.instance, &phys.phys_device, &device.logical_device, &surface, count);
        let image_views = Self::create_image_views(&device.logical_device, &swapchain.swapchain_images, &swapchain.format);
        info!("Доступных изображений для вывода: {:?}", image_views.len());

        let vertex_shader = Self::create_shader_module("C:/Users/Oleja/Desktop/Pixel-Engine/src/pixel-init-render/vert.spv", &device.logical_device);
        let fragment_shader = Self::create_shader_module("C:/Users/Oleja/Desktop/Pixel-Engine/src/pixel-init-render/frag.spv", &device.logical_device);
        
        let render_pass = Self::create_render_pass(&device.logical_device, &swapchain.format);
        let pipeline_layout = Self::create_graphics_pipeline_layout(&device.logical_device, &vertex_shader, &fragment_shader, swapchain.surface_capabilities.current_extent);
        let pipeline = Self::create_graphics_pipeline(&device.logical_device, &render_pass.render_pass, &pipeline_layout);
        let frame_buffers = Self::create_framebuffer(&device.logical_device, render_pass.render_pass, &image_views, &swapchain.surface_capabilities.current_extent);
        let command_pool = Self::create_command_pool(&device.logical_device, 0);
        let command_buffers = Self::create_command_buffers(&device.logical_device, command_pool, pipeline, &frame_buffers, render_pass.render_pass, swapchain.surface_capabilities.current_extent);
        let sync = Self::create_sync_objects(&device.logical_device);
        let mut current_frame = 0;

        Self::draw_frame(&device.logical_device, command_buffers, sync.inflight_fences, current_frame, swapchain, sync.image_available_semaphores, sync.render_finished_semaphores, queue.graphics, queue.transfer);
    }

    pub fn available_instance_layers(entry: &Entry) -> VkResult<Vec<CString>> {
        let layers = unsafe { entry.enumerate_instance_layer_properties() }?;
        let layer_names = layers
            .iter()
            .map(|layer| {
                let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
                CString::new(name.to_bytes()).unwrap()
            })
            .collect();
        Ok(layer_names)
    }

    pub fn default_layers() -> Vec<CString> {
        let mut layers = vec![
            CString::new("VK_LAYER_KHRONOS_profiles").unwrap(),
            CString::new("VK_LAYER_KHRONOS_validation").unwrap(),
            CString::new("VK_LAYER_AMD_switchable_graphics").unwrap(),
        ];

        return layers;
    }

    pub fn available_instance_extensions(entry: &Entry, layer_names: &Vec<CString>) -> VkResult<Vec<CString>> {
        let mut extension_names = Vec::new();
        let global_extensions = unsafe { entry.enumerate_instance_extension_properties(None) }?;
        for extension in global_extensions {
            let name = unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) };
            extension_names.push(CString::new(name.to_bytes()).unwrap());
        }
        
        for layer_name in layer_names {
            let layer_extensions = unsafe { entry.enumerate_instance_extension_properties(Some(&layer_name)) }?;
            for extension in layer_extensions {
                let name = unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) };
                if !extension_names.contains(&CString::new(name.to_bytes()).unwrap()) {
                    extension_names.push(CString::new(name.to_bytes()).unwrap());
                }
            }
        }
        Ok(extension_names)
    }

    #[cfg(target_os = "windows")]
    pub fn necessary_surface_extensions() -> Vec<CString> {

        let extension = vec![
            CString::from_str("VK_KHR_surface").unwrap(),
            CString::from_str("VK_KHR_win32_surface").unwrap(),
        ];

        return extension;
    }

    unsafe fn get_index_family_queues(queue_prop: &Vec<QueueFamilyProperties>, flags: QueueFlags) -> Vec<u32>{

        let mut index_queue_family_graphics = vec![];
        let mut n = 0u32;
        for i in queue_prop {
            if i.queue_flags.contains(flags) {
                index_queue_family_graphics.push(n);
            }
            n += 1;
        }

        return index_queue_family_graphics;
    }

}


impl Drop for PixelEngine {
    fn drop(&mut self) {

    }
}