#![allow(warnings)]

use std::ptr;

use ash::vk::SurfaceKHR;
use ash::Entry;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use ash::vk::*;

use crate::PixelEngine;

impl PixelEngine {
    pub unsafe fn create_image_views(device: &ash::Device, images: &Vec<Image>, format: &Format) -> Vec<ImageView> {
 
        let mut image_views = vec![];
        for i in images {

            let image_view_info = ImageViewCreateInfo::default()
            .components(ComponentMapping {
                r: ComponentSwizzle::R,
                g: ComponentSwizzle::G,
                b: ComponentSwizzle::B,
                a: ComponentSwizzle::B
            })
            .image(*i)
            .format(*format)
            .subresource_range(ImageSubresourceRange {
                aspect_mask:        ImageAspectFlags::COLOR,
                base_mip_level:     0,
                level_count:        1,
                base_array_layer:   0,
                layer_count:        1,
            })
            .view_type(ImageViewType::TYPE_2D);
        
            let image_view = device.create_image_view(&image_view_info, None).expect("Ошибка создания image views");
            image_views.push(image_view);
        }

        return image_views
    }

}