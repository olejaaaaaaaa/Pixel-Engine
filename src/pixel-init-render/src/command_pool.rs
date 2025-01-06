
use std::ptr;

use crate::PixelEngine;
use ash::vk::*;
use log::info;

impl PixelEngine {
    pub unsafe fn create_command_pool(
        device: &ash::Device,
        queue_families: u32,
    ) -> CommandPool {
        let command_pool_create_info = CommandPoolCreateInfo::default()
            .queue_family_index(queue_families)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let command_pool = device
            .create_command_pool(&command_pool_create_info, None)
            .expect("Failed to create Command Pool!");
        
        info!("Создан command_pool");
        return command_pool;
    }
}