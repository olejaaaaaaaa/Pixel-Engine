#![allow(warnings)]

use std::ffi::{CStr, CString, FromBytesUntilNulError};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr::{self, null};
use std::str::FromStr;

use ash::khr::swapchain;
use ash::prelude::VkResult;
use ash::vk::*;
use ash::Entry;
use log::{debug, info};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
pub struct PixelEngine {
    pub entry:              ash::Entry,
    pub instance:           ash::Instance,
    pub phys_device:        PhysicalDevice,
    pub logical_device:     ash::Device,
}

impl PixelEngine {

    pub unsafe fn new(window: &winit::window::Window) {
        let entry = ash::Entry::load().unwrap();

        let instance = Self::create_instance(&entry);
        let surface = Self::create_surface(&entry, &instance, window);
        let (phys_dev, queue_prop) = Self::create_physical_device(&instance);
        let qgraphics = Self::get_index_family_queue_graphics(&queue_prop);
        let qtransfer = Self::get_index_family_queue_transfer(&queue_prop);
        let (device, qgraphics, qtransfer) = Self::create_device_and_queue(&instance, &phys_dev, qgraphics[0], *qtransfer.last().unwrap());
        let (swapchain, images) = Self::create_swapchain(&entry, &instance, &phys_dev, &device, &surface);
        let image_view = Self::create_image_views(&device, &images);
        //let vertex_shader = Self::create_shader_module("..src/shaders/spv/vert.spv", &device);
        //let fragment_shader = Self::create_shader_module("../src/assets/shaders/spv/frag.spv", &device);
        //let render_pass = Self::create_render_pass(&device);
       // let graphics_pipeline = Self::create_graphics_pipeline_layout(&device, &vertex_shader, &fragment_shader);
    }

    unsafe fn create_render_pass(device: &ash::Device) -> RenderPass {
        let color_attachment = AttachmentDescription {
            flags: AttachmentDescriptionFlags::empty(),
            format: Format::R8G8B8A8_UNORM,
            samples: SampleCountFlags::TYPE_1,
            load_op: AttachmentLoadOp::CLEAR,
            store_op: AttachmentStoreOp::STORE,
            stencil_load_op: AttachmentLoadOp::DONT_CARE,
            stencil_store_op: AttachmentStoreOp::DONT_CARE,
            initial_layout: ImageLayout::UNDEFINED,
            final_layout: ImageLayout::PRESENT_SRC_KHR,
        };

        let color_attachment_ref = AttachmentReference {
            attachment: 0,
            layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let subpass = SubpassDescription {
            flags: SubpassDescriptionFlags::empty(),
            pipeline_bind_point: PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            p_resolve_attachments: ptr::null(),
            p_depth_stencil_attachment: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
            _marker: std::marker::PhantomData,
        };

        let render_pass_attachments = [color_attachment];

        let renderpass_create_info = RenderPassCreateInfo {
            s_type: StructureType::RENDER_PASS_CREATE_INFO,
            flags: RenderPassCreateFlags::empty(),
            p_next: ptr::null(),
            attachment_count: render_pass_attachments.len() as u32,
            p_attachments: render_pass_attachments.as_ptr(),
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: 0,
            p_dependencies: ptr::null(),
            _marker: std::marker::PhantomData,
        };

        let render_pass = device
                .create_render_pass(&renderpass_create_info, None)
                .expect("Failed to create render pass!");
        
        return render_pass;
    }


    // unsafe fn create_graphics_pipeline(device: &ash::Device, pipeline_layout: &PipelineLayout) -> Pipeline {
    //     let graphic_pipeline_create_infos = [GraphicsPipelineCreateInfo {
    //         s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
    //         p_next: ptr::null(),
    //         flags:  PipelineCreateFlags::empty(),
    //         stage_count: shader_stages.len() as u32,
    //         p_stages: shader_stages.as_ptr(),
    //         p_vertex_input_state: &vertex_input_state_create_info,
    //         p_input_assembly_state: &vertex_input_assembly_state_info,
    //         p_tessellation_state: ptr::null(),
    //         p_viewport_state: &viewport_state_create_info,
    //         p_rasterization_state: &rasterization_statue_create_info,
    //         p_multisample_state: &multisample_state_create_info,
    //         p_depth_stencil_state: &depth_state_create_info,
    //         p_color_blend_state: &color_blend_state,
    //         p_dynamic_state: ptr::null(),
    //         layout: pipeline_layout,
    //         render_pass,
    //         subpass: 0,
    //         base_pipeline_handle: Pipeline::null(),
    //         base_pipeline_index: -1,
    //         _marker: std::marker::PhantomData,
    //     }];

    //     let graphics_pipelines = 
    //         device
    //             .create_graphics_pipelines(
    //                 PipelineCache::null(),
    //                 &graphic_pipeline_create_infos,
    //                 None,
    //             )
    //             .expect("Failed to create Graphics Pipeline!.");
        
    //     return graphics_pipelines
    // }

    unsafe fn create_graphics_pipeline_layout(device: &ash::Device, vshader: &ShaderModule, fshader: &ShaderModule) -> PipelineLayout {

        let main_function_name = CString::new("main").unwrap();

        let shader_stages = [

            PipelineShaderStageCreateInfo {
                // Vertex Shader
                s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags:  PipelineShaderStageCreateFlags::empty(),
                module: *vshader,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage:  ShaderStageFlags::VERTEX,
                _marker: std::marker::PhantomData,
            },

            PipelineShaderStageCreateInfo {
                // Fragment Shader
                s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags:  PipelineShaderStageCreateFlags::empty(),
                module: *fshader,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage:   ShaderStageFlags::FRAGMENT,
                _marker: std::marker::PhantomData,
            },
        ];

        let _vertex_input_state_create_info = PipelineVertexInputStateCreateInfo {
            s_type: StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags:  PipelineVertexInputStateCreateFlags::empty(),
            vertex_attribute_description_count: 0,
            p_vertex_attribute_descriptions: ptr::null(),
            vertex_binding_description_count: 0,
            p_vertex_binding_descriptions: ptr::null(),
            _marker: std::marker::PhantomData,
        };
        let _vertex_input_assembly_state_info = PipelineInputAssemblyStateCreateInfo {
            s_type: StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            flags:  PipelineInputAssemblyStateCreateFlags::empty(),
            p_next: ptr::null(),
            primitive_restart_enable: FALSE,
            topology: PrimitiveTopology::TRIANGLE_LIST,
            _marker: std::marker::PhantomData,
        };

        let viewports = [Viewport {
            x: 0.0,
            y: 0.0,
            width: 720.0 as f32,
            height: 480.0 as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissors = [Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: Extent2D { width: 720, height: 480 },
        }];

        let _viewport_state_create_info = PipelineViewportStateCreateInfo {
            s_type: StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags:  PipelineViewportStateCreateFlags::empty(),
            scissor_count: scissors.len() as u32,
            p_scissors: scissors.as_ptr(),
            viewport_count: viewports.len() as u32,
            p_viewports: viewports.as_ptr(),
            _marker: std::marker::PhantomData,
        };

        let _rasterization_statue_create_info = PipelineRasterizationStateCreateInfo {
            s_type: StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags:  PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: FALSE,
            cull_mode: CullModeFlags::BACK,
            front_face: FrontFace::CLOCKWISE,
            line_width: 1.0,
            polygon_mode: PolygonMode::FILL,
            rasterizer_discard_enable: FALSE,
            depth_bias_clamp: 0.0,
            depth_bias_constant_factor: 0.0,
            depth_bias_enable: FALSE,
            depth_bias_slope_factor: 0.0,
            _marker: std::marker::PhantomData,
        };
        
        let _multisample_state_create_info = PipelineMultisampleStateCreateInfo {
            s_type: StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            flags: PipelineMultisampleStateCreateFlags::empty(),
            p_next: ptr::null(),
            rasterization_samples: SampleCountFlags::TYPE_1,
            sample_shading_enable: FALSE,
            min_sample_shading: 0.0,
            p_sample_mask: ptr::null(),
            alpha_to_one_enable: FALSE,
            alpha_to_coverage_enable: FALSE,
            _marker: std::marker::PhantomData,
        };

        let stencil_state = StencilOpState {
            fail_op: StencilOp::KEEP,
            pass_op: StencilOp::KEEP,
            depth_fail_op: StencilOp::KEEP,
            compare_op: CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };

        let _depth_state_create_info = PipelineDepthStencilStateCreateInfo {
            s_type: StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable: FALSE,
            depth_write_enable: FALSE,
            depth_compare_op: CompareOp::LESS_OR_EQUAL,
            depth_bounds_test_enable: FALSE,
            stencil_test_enable: FALSE,
            front: stencil_state,
            back: stencil_state,
            max_depth_bounds: 1.0,
            min_depth_bounds: 0.0,
            _marker: std::marker::PhantomData,
        };

        let color_blend_attachment_states = [PipelineColorBlendAttachmentState {
            blend_enable: FALSE,
            color_write_mask: ColorComponentFlags::RGBA,
            src_color_blend_factor: BlendFactor::ONE,
            dst_color_blend_factor: BlendFactor::ZERO,
            color_blend_op: BlendOp::ADD,
            src_alpha_blend_factor: BlendFactor::ONE,
            dst_alpha_blend_factor: BlendFactor::ZERO,
            alpha_blend_op: BlendOp::ADD,
        }];

        let _color_blend_state = PipelineColorBlendStateCreateInfo {
            s_type: StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: FALSE,
            logic_op: LogicOp::COPY,
            attachment_count: color_blend_attachment_states.len() as u32,
            p_attachments: color_blend_attachment_states.as_ptr(),
            blend_constants: [0.0, 0.0, 0.0, 0.0],
            _marker: std::marker::PhantomData,
        };

        let pipeline_layout_create_info = PipelineLayoutCreateInfo {
            s_type: StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: PipelineLayoutCreateFlags::empty(),
            set_layout_count: 0,
            p_set_layouts: ptr::null(),
            push_constant_range_count: 0,
            p_push_constant_ranges: ptr::null(),
            _marker: std::marker::PhantomData,
        };

        let pipeline_layout = device
                .create_pipeline_layout(&pipeline_layout_create_info, None)
                .expect("Failed to create pipeline layout!");

        return pipeline_layout;
    }

    /// Ahtung! Format surface R8G8B8A8_UNORM
    unsafe fn create_image_views(device: &ash::Device, images: &Vec<Image>) -> ImageView {
 
        let image_view_info = ImageViewCreateInfo {
            s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: ptr::null(),
            flags: ImageViewCreateFlags::empty(),
            view_type: ImageViewType::TYPE_2D,
            format: Format::R8G8B8A8_UNORM,
            components: ComponentMapping {
                r: ComponentSwizzle::IDENTITY,
                g: ComponentSwizzle::IDENTITY,
                b: ComponentSwizzle::IDENTITY,
                a: ComponentSwizzle::IDENTITY,
            },
            subresource_range: ImageSubresourceRange {
                aspect_mask: ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            image: images[0],
            _marker: std::marker::PhantomData,
            };
        
        let image_view = device.create_image_view(&image_view_info, None).expect("Ошибка создания image views");
        return image_view
    }


    pub unsafe fn available_instance_layers(entry: &Entry) -> VkResult<Vec<CString>> {
        let layers = entry.enumerate_instance_layer_properties()?;
        let layer_names = layers
            .iter()
            .map(|layer| {
                let name = CStr::from_ptr(layer.layer_name.as_ptr());
                CString::new(name.to_bytes()).unwrap()
            })
            .collect();
        Ok(layer_names)
    }

    fn default_layers() -> Vec<CString> {
        let mut layers = vec![
            CString::new("VK_LAYER_KHRONOS_profiles").unwrap(),
            CString::new("VK_LAYER_KHRONOS_profiles").unwrap(),
            CString::new("VK_LAYER_AMD_switchable_graphics").unwrap(),
        ];

        return layers;
    }

    unsafe fn available_instance_extensions(entry: &Entry, layer_names: &Vec<CString>) -> VkResult<Vec<CString>> {
        let mut extension_names = Vec::new();
        // Enumerate global extensions (no layer specified)
        let global_extensions = entry.enumerate_instance_extension_properties(None)?;
        for extension in global_extensions {
            let name = CStr::from_ptr(extension.extension_name.as_ptr());
            extension_names.push(CString::new(name.to_bytes()).unwrap());
        }
        
        // Enumerate layer-specific extensions
        for layer_name in layer_names {
            let layer_extensions = entry.enumerate_instance_extension_properties(Some(&layer_name))?;
            for extension in layer_extensions {
                let name = CStr::from_ptr(extension.extension_name.as_ptr());
                if !extension_names.contains(&CString::new(name.to_bytes()).unwrap()) {
                    extension_names.push(CString::new(name.to_bytes()).unwrap());
                }
            }
        }
        Ok(extension_names)
    }

    #[cfg(target_os = "windows")]
    fn necessary_surface_extensions() -> Vec<CString> {

        let extension = vec![
            CString::from_str("VK_KHR_surface").unwrap(),
            CString::from_str("VK_KHR_win32_surface").unwrap(),
        ];

        return extension;
    }

    unsafe fn create_shader_module(path: &str, device: &ash::Device) -> ShaderModule {
        let shader = File::open(path).expect("Не удалось найти файл spv");
        let mut source = shader.bytes().filter_map(|byte| byte.ok()).collect::<Vec<u8>>();
        
        let shader_module_create_info = ShaderModuleCreateInfo {
            code_size:  source.len(),
            p_code:     source.as_ptr() as *const u32,
            ..Default::default()
        };

        let shader_module = device.create_shader_module(&shader_module_create_info, None).expect("Не получилось создать шейдерный модуль");
        return shader_module
    }

    unsafe fn create_surface(entry: &Entry, instance: &ash::Instance, window: &winit::window::Window) -> SurfaceKHR {
        use ash_window;
        let surface = ash_window::create_surface(entry, instance, window.display_handle().unwrap().into(), window.window_handle().unwrap().into(), None).expect("Не получилось создать поверхность");
        return surface;
    }

    unsafe fn create_swapchain(entry: &Entry, instance: &ash::Instance, phys_dev: &PhysicalDevice, device: &ash::Device, surface: &SurfaceKHR) 
    -> (SwapchainKHR, Vec<Image>) {
        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
        let caps = surface_loader.get_physical_device_surface_capabilities(*phys_dev, *surface).unwrap();
        info!("Актуальное разрешение экрана: {} {}", caps.current_extent.width, caps.current_extent.height);
        let formats = surface_loader.get_physical_device_surface_formats(*phys_dev, *surface).unwrap();
        let present = surface_loader.get_physical_device_surface_present_modes(*phys_dev, *surface).unwrap();
        let n = surface_loader.get_physical_device_surface_support(*phys_dev, 0, *surface).unwrap();
 
        let swapchain_loader = swapchain::Device::new(instance, device);

        let format = formats[0];
        let swapchain_create_info = SwapchainCreateInfoKHR::default()
            .surface(*surface)
            .min_image_count(caps.min_image_count)
            .image_color_space(format.color_space)
            .image_format(format.format)
            .image_extent(caps.max_image_extent)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(SharingMode::EXCLUSIVE)
            .pre_transform(caps.current_transform)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present[0])
            .clipped(true)
            .image_array_layers(1);

        let swapchain = swapchain_loader
            .create_swapchain(&swapchain_create_info, None)
            .unwrap();

        let images = swapchain_loader.get_swapchain_images(swapchain).unwrap();

        return (swapchain, images)
    }

    unsafe fn create_physical_device(instance: &ash::Instance) -> (PhysicalDevice, Vec<QueueFamilyProperties>) {

        let phys_devs: Vec<PhysicalDevice> = instance.enumerate_physical_devices().expect("Ошибка: Не получилось получить список физических устройств");
        //let mut phys_mem:           Vec<PhysicalDeviceMemoryProperties>         = vec![];
        //let mut phys_prop:          Vec<PhysicalDeviceProperties>               = vec![];
        let mut phys_queue_prop:    Vec<Vec<QueueFamilyProperties>>             = vec![];
        //let mut phys_features:      Vec<PhysicalDeviceFeatures>                 = vec![];

        info!("Количество видеокарт: {}", phys_devs.len());
        let mut index = 0;
        let mut n = 0;
        for i in &phys_devs {
            
            let properties: PhysicalDeviceProperties        = instance.get_physical_device_properties(*i);
            let memory:     PhysicalDeviceMemoryProperties  = instance.get_physical_device_memory_properties(*i);
            let features:   PhysicalDeviceFeatures          = instance.get_physical_device_features(*i);
            let queue:      Vec<QueueFamilyProperties>      = instance.get_physical_device_queue_family_properties(*i);
            info!("Видеокарта: {:?}", CStr::from_ptr(properties.device_name.as_ptr()));

            let mut mem = 0;
            for i in memory.memory_heaps {
                mem += i.size;
            }

            info!("Общее количество видеопамяти: {:?} Мб", mem / (1024 * 1024));
            info!("Количество семейств очередей: {:?}", queue.len());
            let s = queue.iter().map(|x| x.queue_count.to_string()).collect::<Vec<_>>().join("-");

            if properties.device_type == PhysicalDeviceType::DISCRETE_GPU {
                index = n;
            }

            info!("Количество очередей: {:?}", s);

            //phys_prop.push(properties);
            //phys_mem.push(memory);
            //phys_features.push(features);
            phys_queue_prop.push(queue);
            n += 1;
        }

        let phys_dev = phys_devs[index];
        let phys_queue_prop = phys_queue_prop[index].clone();

        return (phys_dev, phys_queue_prop);
    }

    fn create_callback_function() {
        

    }

    pub fn update_swapchain(&mut self, width: usize, height: usize) {

    }

    unsafe fn get_index_family_queue_graphics(queue_prop: &Vec<QueueFamilyProperties>) -> Vec<u32>{

        // Ищем семейство со свойством graphics
        let mut index_queue_family_graphics = vec![];
        let mut n = 0u32;
        for i in queue_prop {
            if i.queue_flags.contains(QueueFlags::GRAPHICS) {
                index_queue_family_graphics.push(n);
            }
            n += 1;
        }

        return index_queue_family_graphics;
    }

    unsafe fn get_index_family_queue_transfer(queue_prop: &Vec<QueueFamilyProperties>) -> Vec<u32>{

        // Ищем семейство со свойством transfer
        let mut index_queue_family_transfer = vec![];
        let mut n = 0u32;
        for i in queue_prop {
            if i.queue_flags.contains(QueueFlags::TRANSFER) {
                index_queue_family_transfer.push(n);
            }
            n += 1;
        }

        return index_queue_family_transfer;
    }

    unsafe fn create_device_and_queue(instance: &ash::Instance, phys_dev: &PhysicalDevice, igraphics: u32, itransfer: u32) -> (ash::Device, Queue, Queue) {
        let prior = [1.0f32];

        let queue_graphics_info = DeviceQueueCreateInfo::default()
            .queue_family_index(igraphics)
            .queue_priorities(&prior);
        
        let queue_transfer_info = DeviceQueueCreateInfo::default()
            .queue_family_index(itransfer)
            .queue_priorities(&prior);

        let que_list = [queue_graphics_info, queue_transfer_info];

        let device_extension_names_raw = [
            swapchain::NAME.as_ptr(),
        ];

        let features = PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };

        let device_info = DeviceCreateInfo::default()
            .queue_create_infos(&que_list)
            .enabled_features(&features)
            .enabled_extension_names(&device_extension_names_raw);

        let device = instance.create_device(*phys_dev, &device_info, None).expect("Ошибка создания устройства");
        let queue_graphics = device.get_device_queue(igraphics, 0);
        let queue_transer = device.get_device_queue(itransfer, 0);

        return (device, queue_graphics, queue_transer)
    }

    unsafe fn create_instance(entry: &ash::Entry) -> ash::Instance {

        let engine_name = CString::new("Pixel").unwrap();

        let layer = Self::available_instance_layers(&entry).unwrap();
        let mut exten = Self::available_instance_extensions(entry, &layer).unwrap();
        let default_layer = Self::default_layers();
        for i in &default_layer {
            if !layer.contains(i) {
                panic!("Ошибка слоев! Нет нужных слоев на устройстве");
            }
        }

        let mut surface_exten = Self::necessary_surface_extensions();

        let layer = default_layer.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();
        let exten = exten.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();
        let surface_exten = surface_exten.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();

        let app_info = ApplicationInfo::default()
            .engine_name(CStr::from_ptr(engine_name.as_ptr() as *const i8))
            .api_version(make_api_version(0, 1, 3, 0));

        let instance_info = InstanceCreateInfo::default()
            .enabled_extension_names(&surface_exten)
            .enabled_layer_names(&layer)
            .application_info(&app_info);
    
        let instance = entry.create_instance(&instance_info, None).expect("Ошибка: создание экземпляра не удалось");
        return instance;
        
    }


}


impl Drop for PixelEngine {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}