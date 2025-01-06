use std::ffi::{CStr, CString};

use ash::{vk::{make_api_version, ApplicationInfo, InstanceCreateInfo}, Instance};
use log::info;

use crate::PixelEngine;



pub struct ComponentInstance {
    pub instance: Instance,
    layers: Vec<CString>,
    extensions: Vec<CString>,
    surface_exyensions: Vec<CString>,
}

impl PixelEngine {
    pub unsafe fn create_instance(entry: &ash::Entry) -> ComponentInstance {

        let engine_name = CString::new("Pixel").unwrap();

        let layer = Self::default_layers();
        info!("Количество instance layers: {:?}", layer.len());

        let mut exten = Self::available_instance_extensions(entry, &layer).unwrap();
        info!("Количество instance extensions: {:?}", exten.len());

        let default_layer = Self::default_layers();
        for i in &default_layer {
            if !layer.contains(i) {
                panic!("Ошибка слоев! Нет нужных слоев на устройстве");
            }
        }

        let mut surface_exten = Self::necessary_surface_extensions();
        surface_exten.push(CString::new("VK_EXT_debug_utils").unwrap());
        surface_exten.push(CString::new("VK_EXT_debug_report").unwrap());
        surface_exten.push(CString::new("VK_KHR_device_group_creation").unwrap());

        let layer_p = layer.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();
        let exten_p = exten.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();
        let surface_exten_p = surface_exten.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();

        let app_info = ApplicationInfo::default()
            .engine_name(CStr::from_ptr(engine_name.as_ptr() as *const i8))
            .api_version(make_api_version(0, 1, 2, 0));

        let instance_info = InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&surface_exten_p)
            .enabled_layer_names(&layer_p);
    
        let instance = entry.create_instance(&instance_info, None).expect("Ошибка: создание экземпляра не удалось");
        ComponentInstance {
            instance,
            layers: default_layer,
            extensions: exten,
            surface_exyensions: surface_exten,
        }
        
    }
}