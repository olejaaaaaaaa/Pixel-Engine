use std::ffi::CStr;

use crate::PixelEngine;

use ash::vk::*;
use log::*;

pub struct ComponentPhysicalDevice {
    pub phys_device_name:   &'static str,
    pub phys_device:        PhysicalDevice,
    pub memory:             usize,
    pub phys_mem_prop:      PhysicalDeviceMemoryProperties,
    pub phys_features:      PhysicalDeviceFeatures,
    pub queue_family_prop:  Vec<QueueFamilyProperties>
}

impl PixelEngine {
    pub unsafe fn create_physical_device(instance: &ash::Instance) -> ComponentPhysicalDevice {

        let phys_devs: Vec<PhysicalDevice> = instance.enumerate_physical_devices().expect("Ошибка: Не получилось получить список физических устройств");
        let mut phys_mem:           Vec<PhysicalDeviceMemoryProperties>         = vec![];
        let mut phys_prop:          Vec<PhysicalDeviceProperties>               = vec![];
        let mut phys_queue_prop:    Vec<Vec<QueueFamilyProperties>>             = vec![];
        let mut phys_features:      Vec<PhysicalDeviceFeatures>                 = vec![];

        info!("Количество видеокарт: {}", phys_devs.len());
        let mut index = 0;
        let mut n = 0;
        let mut mem = 0;

        for i in &phys_devs {
 
            let properties: PhysicalDeviceProperties        = instance.get_physical_device_properties(*i);
            let memory:     PhysicalDeviceMemoryProperties  = instance.get_physical_device_memory_properties(*i);
            let features:   PhysicalDeviceFeatures          = instance.get_physical_device_features(*i);
            let queue:      Vec<QueueFamilyProperties>      = instance.get_physical_device_queue_family_properties(*i);
            info!("Видеокарта: {:?}", CStr::from_ptr(properties.device_name.as_ptr()));

            for i in memory.memory_heaps {
                if i.flags.contains(MemoryHeapFlags::DEVICE_LOCAL) {
                    mem += i.size;
                }
            }

            info!("Общее количество видеопамяти: {:?} Мб", mem / (1024 * 1024));
            info!("Количество семейств очередей: {:?}", queue.len());
            let s = queue.iter().map(|x| x.queue_count.to_string()).collect::<Vec<_>>().join("-");

            if properties.device_type == PhysicalDeviceType::DISCRETE_GPU {
                index = n;
            }

            info!("Количество очередей: {:?}", s);

            phys_prop.push(properties);
            phys_mem.push(memory);
            phys_features.push(features);
            phys_queue_prop.push(queue);
            n += 1;
        }

        let phys_dev = phys_devs[index];
        let phys_queue_prop = phys_queue_prop[index].clone();
        let phys_features = phys_features[index];
        let phys_mem_prop = phys_mem[index];
        let phys_prop = phys_prop[index];

        ComponentPhysicalDevice {
            phys_device_name: CStr::from_ptr(phys_prop.device_name.as_ptr()).to_str().unwrap(),
            phys_device: phys_dev,
            memory: mem as usize,
            phys_mem_prop: phys_mem_prop,
            phys_features: phys_features,
            queue_family_prop: phys_queue_prop,
        }
    }
    
}
