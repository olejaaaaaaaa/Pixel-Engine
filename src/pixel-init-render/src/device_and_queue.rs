#![allow(warnings)]
use ash::{khr::swapchain, vk::*, Entry};
use log::*;

use crate::PixelEngine;


pub struct ComponentDevice {
    pub logical_device:             ash::Device,
}

pub struct ComponentQueue {
    pub graphics: Queue,
    pub transfer: Queue,
    pub compute:  Queue,
}

impl PixelEngine {
    pub unsafe fn create_device_and_queues<'s>(instance: &ash::Instance, phys_dev: &PhysicalDevice, graphics: &Vec<u32>, transfer: &Vec<u32>, compute: &Vec<u32>) -> (ComponentDevice, ComponentQueue) {
        info!("Графика: {:?} Перемещения: {:?} Вычисления: {:?}", graphics, transfer, compute);

        let index_graphics = graphics[0];
        let mut index_transfer = transfer[0];
        for i in transfer {
            if *i != index_graphics {
                index_transfer = *i;
            }
        }

        let mut index_compute = compute[0];
        for i in compute {

            if *i != index_graphics {
                index_compute = *i;
            }

            if *i != index_graphics && *i != index_transfer {
                index_compute = *i;
                break
            }
        }

        info!("Лучший выбор: {} {} {}", index_graphics, index_transfer, index_compute);
        let prior = [1.0f32];
        let queue_graphics_info = DeviceQueueCreateInfo::default()
            .queue_family_index(index_graphics)
            .queue_priorities(&prior);
        
        let queue_transfer_info = DeviceQueueCreateInfo::default()
            .queue_family_index(index_transfer)
            .queue_priorities(&prior);

        let queue_compute_info = DeviceQueueCreateInfo::default()
            .queue_family_index(index_compute)
            .queue_priorities(&prior);

        let list = [queue_graphics_info, queue_transfer_info, queue_compute_info];

        let device_prop = instance.enumerate_device_extension_properties(*phys_dev).expect("Ошибка получения расширений устройства");
        let mut device_ext = vec![];
        for i in device_prop {
            if let Ok(ext) = i.extension_name_as_c_str() {
                device_ext.push(ext.to_str().unwrap().to_string());
            }
        }

        info!("Количество расширений устройства: {:?}", device_ext.len());

        let device_extension_names_raw = [
             swapchain::NAME.as_ptr(),
        ];

        let features = PhysicalDeviceFeatures {
            shader_clip_distance: 1,
             ..Default::default()
        };

        let device_info = DeviceCreateInfo::default()
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device = instance.create_device(*phys_dev, &device_info, None).expect("Ошибка создания устройства");
        let queue_graphics = device.get_device_queue(index_graphics, 0);
        let queue_transer = device.get_device_queue(index_transfer, 0);
        let queue_compute = device.get_device_queue(index_compute, 0);

        return ( ComponentDevice{ logical_device: device }, ComponentQueue { graphics: queue_graphics, transfer: queue_transer, compute: queue_compute } )
    }
}