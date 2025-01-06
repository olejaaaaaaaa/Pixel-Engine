#![allow(warnings)]


use std::ptr;

use ash::{khr::swapchain, vk::*, Entry};
use log::*;

use crate::PixelEngine;

pub struct ComponentSwapchain {
    pub swapchain:              SwapchainKHR,
    pub swapchain_fn:           ash::khr::swapchain::Device,
    pub swapchain_images:        Vec<Image>,
    pub format:                 Format,
    pub present:                PresentModeKHR,
    pub surface_capabilities:   SurfaceCapabilitiesKHR,
}

impl PixelEngine {
    pub unsafe fn create_swapchain(
        entry: &Entry, 
        instance: &ash::Instance, 
        phys_dev: &PhysicalDevice, 
        device: &ash::Device, 
        surface: &SurfaceKHR,
        count_family_queue: usize
    ) -> ComponentSwapchain {

        let FORMAT = Format::B8G8R8A8_SRGB;
        let COLOR_SPACE = ColorSpaceKHR::SRGB_NONLINEAR;

        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
        let caps = surface_loader.get_physical_device_surface_capabilities(*phys_dev, *surface).unwrap();
        let formats = surface_loader.get_physical_device_surface_formats(*phys_dev, *surface).unwrap();
        let present = surface_loader.get_physical_device_surface_present_modes(*phys_dev, *surface).unwrap();

        let mut family_queue_index = vec![];
        for i in 0..count_family_queue as u32 {
            let n = unsafe { surface_loader.get_physical_device_surface_support(*phys_dev, i, *surface).unwrap() };
            if n ==  true {
                family_queue_index.push(i as u32);
            }
        }

        let mut available_formats = vec![];
        for i in formats {
            if i.format == FORMAT && i.color_space == COLOR_SPACE {
                available_formats.push(i);
            }
        }

        info!("Формат: {:?} c цветовым пространством: {:?} доступно: {}", available_formats[0].format, available_formats[0].color_space , available_formats.len() >= 1);
        let mut available_presents = vec![];
        for i in present {
            if i == PresentModeKHR::FIFO {
                available_presents.push(i);
            }
        }

        info!("Выбран режим представления {:?}", PresentModeKHR::FIFO);
        let swapchain_create_info = SwapchainCreateInfoKHR::default()
            .present_mode(available_presents[0])
            .pre_transform(caps.current_transform)
            .image_color_space(available_formats[0].color_space)
            .image_format(available_formats[0].format)
            .surface(*surface)
            .image_array_layers(1)
            .clipped(true)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .queue_family_indices(&family_queue_index)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .min_image_count(caps.min_image_count)
            .image_sharing_mode(SharingMode::EXCLUSIVE);

        let swapchain_loader = ash::khr::swapchain::Device::new(instance, device);
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Не получилось создать swapchain!")
        };

        let swapchain_images = unsafe {
            swapchain_loader
                .get_swapchain_images(swapchain)
                .expect("Не получилось получить swapchain images")
        };

        info!("{:?}", caps.current_extent);

        ComponentSwapchain { 
            swapchain:              swapchain, 
            swapchain_fn:           swapchain_loader,
            swapchain_images:       swapchain_images, 
            format:                 available_formats[0].format, 
            present:                available_presents[0], 
            surface_capabilities:   caps,
        }
    }

}