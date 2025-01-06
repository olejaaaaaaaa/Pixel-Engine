
use std::ptr;

use crate::PixelEngine;
use ash::vk::*;
use log::info;

impl PixelEngine {
    // pub unsafe fn create_command_buffers(
    //     device: &ash::Device,
    //     command_pool: CommandPool,
    //     count_buffers: usize,
    // ) -> Vec<CommandBuffer> {

    //     let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
    //         .command_pool(command_pool)
    //         .command_buffer_count(count_buffers as u32)
    //         .level(CommandBufferLevel::PRIMARY);

    //     let command_buffers = 
    //         device
    //             .allocate_command_buffers(&command_buffer_allocate_info)
    //             .expect("Failed to allocate Command Buffers!");

    //     return command_buffers
    // }
}