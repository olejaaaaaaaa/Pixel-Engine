


use std::ptr;

use crate::PixelEngine;
use ash::vk::*;
use log::info;

impl PixelEngine {
    pub unsafe fn create_framebuffer(
        device: &ash::Device,
        render_pass: RenderPass,
        image_views: &Vec<ImageView>,
        swapchain_extent: &Extent2D,
    ) -> Vec<Framebuffer> {

        let mut frame_buffers = vec![];

        for i in image_views {
            let attachments = [*i];
            let framebuffer_create_info = FramebufferCreateInfo::default()
                .attachments(attachments.as_slice()) 
                .attachment_count(1)
                .render_pass(render_pass)
                .width(swapchain_extent.width)
                .height(swapchain_extent.height)
                .layers(1);

            let framebuffer =
                device
                    .create_framebuffer(&framebuffer_create_info, None)
                    .expect("Failed to create Framebuffer!");

            info!("Создан framebuffer");
            frame_buffers.push(framebuffer);
        }
        
        frame_buffers
    }
}