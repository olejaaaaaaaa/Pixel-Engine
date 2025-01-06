#![allow(warnings)]
use std::{fs::File, io::Read};
use ash::{khr::swapchain, vk::*, Entry};
use log::*;

use crate::PixelEngine;

impl PixelEngine {
    pub fn create_shader_module(path: &str, device: &ash::Device) -> ShaderModule {
        let shader = File::open(path).expect("Не удалось найти файл spv");
        let source = shader.bytes().filter_map(|byte| byte.ok()).collect::<Vec<u8>>();
        
        let shader_module_create_info = ShaderModuleCreateInfo {
             code_size:  source.len(),
             p_code:     source.as_ptr() as *const u32,
             ..Default::default()
        };

        let shader_module = unsafe { device.create_shader_module(&shader_module_create_info, None).expect("Не получилось создать шейдерный модуль") };

        std::mem::forget(source);
        return shader_module
    }
}