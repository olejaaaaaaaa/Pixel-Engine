#![allow(warnings)]

use std::ptr;

use ash::vk::SurfaceKHR;
use ash::Entry;
use log::info;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use ash::vk::*;

use crate::PixelEngine;

pub struct ComponentRenderPass {
    pub attachment: Vec<AttachmentDescription>,
    pub render_pass: RenderPass
}

impl PixelEngine {
    pub unsafe fn create_render_pass(device: &ash::Device, format: &Format) -> ComponentRenderPass {
        let color_attachment = AttachmentDescription::default()
            .format(*format)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR);

        let color_attachment_ref = AttachmentReference {
            attachment: 0,
            layout:     ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let binding = [color_attachment_ref];
        let subpass = SubpassDescription::default()
            .color_attachments(&binding)
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS);

        let render_pass_attachments = vec![color_attachment];

        let binding = [subpass];
        let renderpass_create_info = RenderPassCreateInfo::default()
            .attachments(&render_pass_attachments)
            .subpasses(&binding);

        let render_pass = device
                .create_render_pass(&renderpass_create_info, None)
                .expect("Failed to create render pass!");
        
        info!("Создан render_pass");
        ComponentRenderPass {
            attachment: render_pass_attachments,
            render_pass,
        }
    }


}