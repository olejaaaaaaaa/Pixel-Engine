
use std::{ffi::{CStr, CString}, fs::File, io::Read, ptr, str::FromStr};
use ash::vk::*;
use log::info;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
const shader_main: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };
use crate::swapchain;

pub unsafe fn debug_draw(window: &winit::window::Window) {
    let entry = ash::Entry::load().unwrap();

    let extension = vec![
        CString::from_str("VK_KHR_surface").unwrap(),
        CString::from_str("VK_KHR_win32_surface").unwrap(),
    ];

    let mut layers = vec![
        CString::new("VK_LAYER_KHRONOS_profiles").unwrap(),
        CString::new("VK_LAYER_KHRONOS_validation").unwrap(),
        CString::new("VK_LAYER_AMD_switchable_graphics").unwrap(),
    ];

    let layer_p = layers.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();
    let exten_p = extension.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();
        //let surface_exten_p = surface_exten.iter().map(|x| x.as_ptr() as *const i8 ).collect::<Vec<_>>();

        let app_info = ApplicationInfo::default()
            .api_version(make_api_version(0, 1, 3, 0));

        //let mut debug = Self::setup_debug_utils();
        let instance_info = InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_layer_names: layer_p.as_ptr(),
            enabled_extension_count: exten_p.len() as u32,
            pp_enabled_extension_names: exten_p.as_ptr(),
            //p_next: &debug as *const _ as *const std::ffi::c_void, // Включение отладочного колбека
            ..Default::default()
        };
    
        let instance = entry.create_instance(&instance_info, None).expect("Ошибка: создание экземпляра не удалось");

        use ash_window;
        let surface = unsafe { ash_window::create_surface(&entry, &instance, window.display_handle().unwrap().into(), window.window_handle().unwrap().into(), None).expect("Не получилось создать поверхность") };

        let phys_devs: Vec<PhysicalDevice> = unsafe { instance.enumerate_physical_devices().expect("Ошибка: Не получилось получить список физических устройств") };
        let mut phys_mem:           Vec<PhysicalDeviceMemoryProperties>         = vec![];
        let mut phys_prop:          Vec<PhysicalDeviceProperties>               = vec![];
        let mut phys_queue_prop:    Vec<Vec<QueueFamilyProperties>>             = vec![];
        let mut phys_features:      Vec<PhysicalDeviceFeatures>                 = vec![];

        info!("Количество видеокарт: {}", phys_devs.len());
        let mut index = 0;
        let mut n = 0;
        let mut mem = 0;

        for i in &phys_devs {
 
            let properties: PhysicalDeviceProperties        = unsafe { instance.get_physical_device_properties(*i) };
            let memory:     PhysicalDeviceMemoryProperties  = unsafe { instance.get_physical_device_memory_properties(*i) };
            let features:   PhysicalDeviceFeatures          = unsafe { instance.get_physical_device_features(*i) };
            let queue:      Vec<QueueFamilyProperties>      = unsafe { instance.get_physical_device_queue_family_properties(*i) };
            info!("Видеокарта: {:?}", unsafe { CStr::from_ptr(properties.device_name.as_ptr()) });

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

        let prior = [1.0f32];
        let queue_graphics_info = DeviceQueueCreateInfo {
            queue_count: 1,
            queue_family_index: 0,
            p_queue_priorities: prior.as_ptr(),
            ..Default::default()
        };
        
        let queue_transfer_info = DeviceQueueCreateInfo {
            queue_count: 1,
            queue_family_index: 2,
            p_queue_priorities: prior.as_ptr(),
            ..Default::default()
        };

        let queue_compute_info = DeviceQueueCreateInfo {
            queue_count: 1,
            queue_family_index: 1,
            p_queue_priorities: prior.as_ptr(),
            ..Default::default()
        };

        let list = [queue_graphics_info, queue_transfer_info, queue_compute_info];

        let device_extension_names_raw = [
             ash::khr::swapchain::NAME.as_ptr(),
        ];

        let features = PhysicalDeviceFeatures {
            shader_clip_distance: 1,
             ..Default::default()
        };

        let device_info = DeviceCreateInfo {
            queue_create_info_count: list.len() as u32,
            p_queue_create_infos: list.as_ptr(),
            enabled_extension_count: device_extension_names_raw.len() as u32,
            pp_enabled_extension_names: device_extension_names_raw.as_ptr(),
            ..Default::default()
        };

        let device = unsafe { instance.create_device(phys_dev, &device_info, None).expect("Ошибка создания устройства") };
        let queue_graphics = unsafe { device.get_device_queue(0, 0) };
        let queue_transer = unsafe { device.get_device_queue(2, 0) };
        let queue_compute = unsafe { device.get_device_queue(1, 0) };

        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
        let caps = unsafe { surface_loader.get_physical_device_surface_capabilities(phys_dev, surface).unwrap() };
        let formats = unsafe { surface_loader.get_physical_device_surface_formats(phys_dev, surface).unwrap() };
        let present = unsafe { surface_loader.get_physical_device_surface_present_modes(phys_dev, surface).unwrap() };

        let mut nformats = vec![];
        for i in formats {
            if i.format == Format::R8G8B8A8_SRGB && i.color_space == ColorSpaceKHR::SRGB_NONLINEAR {
                nformats.push(i);
            }
        }

        info!("Формат: {:?} c цветовым пространством: {:?} доступно: {}", nformats[0].format, nformats[0].color_space , nformats.len() >= 1);
        let mut npresent = vec![];
        for i in present {
            if i == PresentModeKHR::FIFO {
                npresent.push(i);
            }
        }

        let s = [0u32];
        info!("Выбран режим представления {:?}", PresentModeKHR::FIFO);
        let swapchain_create_info = SwapchainCreateInfoKHR {
             s_type:                    StructureType::SWAPCHAIN_CREATE_INFO_KHR,
             p_next:                    ptr::null(),
             flags:                     SwapchainCreateFlagsKHR::empty(),
             surface:                   surface,
             min_image_count:           1,
             image_color_space:         nformats[0].color_space,
             image_format:              nformats[0].format,
             image_extent:              caps.current_extent,
             image_usage:               ImageUsageFlags::COLOR_ATTACHMENT,
             image_sharing_mode:        SharingMode::EXCLUSIVE,
             p_queue_family_indices:    s.as_ptr(),
             queue_family_index_count:  s.len() as u32,
             pre_transform:             caps.current_transform,
             composite_alpha:           CompositeAlphaFlagsKHR::OPAQUE,
             present_mode:              npresent[0],
             clipped:                   TRUE,
             old_swapchain:             SwapchainKHR::null(),
             image_array_layers:        1,
             _marker:                   std::marker::PhantomData,
        };

        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &device);
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Failed to create Swapchain!")
        };

        let swapchain_images = unsafe {
            swapchain_loader
                .get_swapchain_images(swapchain)
                .expect("Failed to get Swapchain Images.")
        };

        info!("{:?}", caps.current_extent);

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
            image: swapchain_images[0],
            _marker: std::marker::PhantomData,
            };
        
        let image_view = unsafe { device.create_image_view(&image_view_info, None).expect("Ошибка создания image views") };

        let shader = File::open("C:/Users/Oleja/Desktop/Pixel-Engine/src/pixel-init-render/vert.spv").expect("Не удалось найти файл spv");
        let source = shader.bytes().filter_map(|byte| byte.ok()).collect::<Vec<u8>>();
        
        let shader_module_create_info = ShaderModuleCreateInfo {
            code_size:  source.len(),
            p_code:     source.as_ptr() as *const u32,
            ..Default::default()
        };

        let vertex_shader = unsafe { device.create_shader_module(&shader_module_create_info, None).expect("Не получилось создать шейдерный модуль") };

        let shader = File::open("C:/Users/Oleja/Desktop/Pixel-Engine/src/pixel-init-render/frag.spv").expect("Не удалось найти файл spv");
        let source = shader.bytes().filter_map(|byte| byte.ok()).collect::<Vec<u8>>();
        
        let shader_module_create_info = ShaderModuleCreateInfo {
            code_size:  source.len(),
            p_code:     source.as_ptr() as *const u32,
            ..Default::default()
        };

        let fragment_shader = unsafe { device.create_shader_module(&shader_module_create_info, None).expect("Не получилось создать шейдерный модуль") };
        let color_attachment = AttachmentDescription {
            flags:              AttachmentDescriptionFlags::empty(),
            format:             Format::R8G8B8A8_UNORM,
            samples:            SampleCountFlags::TYPE_1,
            load_op:            AttachmentLoadOp::CLEAR,
            store_op:           AttachmentStoreOp::STORE,
            stencil_load_op:    AttachmentLoadOp::DONT_CARE,
            stencil_store_op:   AttachmentStoreOp::DONT_CARE,
            initial_layout:     ImageLayout::UNDEFINED,
            final_layout:       ImageLayout::PRESENT_SRC_KHR,
        };

        let color_attachment_ref = AttachmentReference {
            attachment: 0,
            layout:     ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let subpass = SubpassDescription {
            flags:                      SubpassDescriptionFlags::empty(),
            pipeline_bind_point:        PipelineBindPoint::GRAPHICS,
            input_attachment_count:     0,
            p_input_attachments:        ptr::null(),
            color_attachment_count:     1,
            p_color_attachments:        &color_attachment_ref,
            p_resolve_attachments:      ptr::null(),
            p_depth_stencil_attachment: ptr::null(),
            preserve_attachment_count:  0,
            p_preserve_attachments:     ptr::null(),
            _marker:                    std::marker::PhantomData,
        };

        let render_pass_attachments = vec![color_attachment];

        let renderpass_create_info = RenderPassCreateInfo {
            s_type:                 StructureType::RENDER_PASS_CREATE_INFO,
            flags:                  RenderPassCreateFlags::empty(),
            p_next:                 ptr::null(),
            attachment_count:       render_pass_attachments.len() as u32,
            p_attachments:          render_pass_attachments.as_ptr(),
            subpass_count:          1,
            p_subpasses:            &subpass,
            dependency_count:       0,
            p_dependencies:         ptr::null(),
            _marker:                std::marker::PhantomData,
        };

        let render_pass = device
                .create_render_pass(&renderpass_create_info, None)
                .expect("Failed to create render pass!");

                let main_function_name = CString::new("main").unwrap(); // the beginning function name in shader code.

                let vertex_input_state_create_info = PipelineVertexInputStateCreateInfo {
                    s_type: StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: PipelineVertexInputStateCreateFlags::empty(),
                    vertex_attribute_description_count: 0,
                    p_vertex_attribute_descriptions: ptr::null(),
                    vertex_binding_description_count: 0,
                    p_vertex_binding_descriptions: ptr::null(),
                    _marker: std::marker::PhantomData,
                };
        
                let vertex_input_assembly_state_info = PipelineInputAssemblyStateCreateInfo {
                    s_type: StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                    flags: PipelineInputAssemblyStateCreateFlags::empty(),
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
        
                let viewport_state_create_info = PipelineViewportStateCreateInfo {
                    s_type: StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: PipelineViewportStateCreateFlags::empty(),
                    scissor_count: scissors.len() as u32,
                    p_scissors: scissors.as_ptr(),
                    viewport_count: viewports.len() as u32,
                    p_viewports: viewports.as_ptr(),
                    _marker: std::marker::PhantomData,
                };
        
                let rasterization_statue_create_info = PipelineRasterizationStateCreateInfo {
                    s_type: StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: PipelineRasterizationStateCreateFlags::empty(),
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
                let multisample_state_create_info = PipelineMultisampleStateCreateInfo {
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
        
                let depth_state_create_info = PipelineDepthStencilStateCreateInfo {
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
        
                let color_blend_state = PipelineColorBlendStateCreateInfo {
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
        
                let pipeline_layout = unsafe {
                    device
                        .create_pipeline_layout(&pipeline_layout_create_info, None)
                        .expect("Failed to create pipeline layout!")
                };

                let shader_stages = [



                PipelineShaderStageCreateInfo {
                    // Vertex Shader
                    s_type:                 StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                    p_next:                 ptr::null(),
                    flags:                  PipelineShaderStageCreateFlags::empty(),
                    module:                 vertex_shader,
                    p_name:                 shader_main.as_ptr(),
                    p_specialization_info:  ptr::null(),
                    stage:                  ShaderStageFlags::VERTEX,
                    _marker:                std::marker::PhantomData,
                },
    
                PipelineShaderStageCreateInfo {
                    // Fragment Shader
                    s_type:                 StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                    p_next:                 ptr::null(),
                    flags:                  PipelineShaderStageCreateFlags::empty(),
                    module:                 fragment_shader,
                    p_name:                 shader_main.as_ptr(),
                    p_specialization_info:  ptr::null(),
                    stage:                  ShaderStageFlags::FRAGMENT,
                    _marker:                std::marker::PhantomData,
                },
            ];
        
                let graphic_pipeline_create_infos = [GraphicsPipelineCreateInfo {
                    s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: PipelineCreateFlags::empty(),
                    stage_count: shader_stages.len() as u32,
                    p_stages: shader_stages.as_ptr(),
                    p_vertex_input_state: &vertex_input_state_create_info,
                    p_input_assembly_state: &vertex_input_assembly_state_info,
                    p_tessellation_state: ptr::null(),
                    p_viewport_state: &viewport_state_create_info,
                    p_rasterization_state: &rasterization_statue_create_info,
                    p_multisample_state: &multisample_state_create_info,
                    p_depth_stencil_state: &depth_state_create_info,
                    p_color_blend_state: &color_blend_state,
                    p_dynamic_state: ptr::null(),
                    layout: pipeline_layout,
                    render_pass: render_pass,
                    subpass: 0,
                    base_pipeline_handle: Pipeline::null(),
                    base_pipeline_index: -1,
                    _marker: std::marker::PhantomData,
                }];
        
                let graphics_pipelines = unsafe {
                    device
                        .create_graphics_pipelines(
                            PipelineCache::null(),
                            &graphic_pipeline_create_infos,
                            None,
                        )
                        .expect("Failed to create Graphics Pipeline!.")
                };

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
                    image: swapchain_images[0],
                    _marker: std::marker::PhantomData,
                    };
                
                let image_view = unsafe { device.create_image_view(&image_view_info, None).expect("Ошибка создания image views") };

                let mut framebuffers = vec![];


                let attachments = [image_view];
        
                let framebuffer_create_info = FramebufferCreateInfo {
                    s_type: StructureType::FRAMEBUFFER_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: FramebufferCreateFlags::empty(),
                    render_pass,
                    attachment_count: 1,
                    p_attachments: &image_view,
                    width: caps.current_extent.width,
                    height: caps.current_extent.height,
                    layers: 1,
                    _marker: std::marker::PhantomData,
                };
        
                let framebuffer = unsafe {
                    device
                        .create_framebuffer(&framebuffer_create_info, None)
                        .expect("Failed to create Framebuffer!")
                };
        
                framebuffers.push(framebuffer);

                let command_pool_create_info = CommandPoolCreateInfo {
                    s_type: StructureType::COMMAND_POOL_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: CommandPoolCreateFlags::empty(),
                    queue_family_index: 0,
                    _marker: std::marker::PhantomData,
                };
        
                let command_pool = unsafe {
                    device
                        .create_command_pool(&command_pool_create_info, None)
                        .expect("Failed to create Command Pool!")
                };

                let command_buffer_allocate_info = CommandBufferAllocateInfo {
                    s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                    p_next: ptr::null(),
                    command_buffer_count: framebuffers.len() as u32,
                    command_pool,
                    level: CommandBufferLevel::PRIMARY,
                    _marker: std::marker::PhantomData,
                };
        
                let command_buffers = unsafe {
                    device
                        .allocate_command_buffers(&command_buffer_allocate_info)
                        .expect("Failed to allocate Command Buffers!")
                };
        
        
                    let command_buffer_begin_info = CommandBufferBeginInfo {
                        s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
                        p_next: ptr::null(),
                        p_inheritance_info: ptr::null(),
                        flags: CommandBufferUsageFlags::SIMULTANEOUS_USE,
                        _marker: std::marker::PhantomData,
                    };
        
                    unsafe {
                        device
                            .begin_command_buffer(command_buffers[0], &command_buffer_begin_info)
                            .expect("Failed to begin recording Command Buffer at beginning!");
                    }
        
                    let clear_values = [ClearValue {
                        color: ClearColorValue {
                            float32: [0.0, 1.0, 0.0, 1.0],
                        },
                    }];
        
                    let render_pass_begin_info = RenderPassBeginInfo {
                        s_type: StructureType::RENDER_PASS_BEGIN_INFO,
                        p_next: ptr::null(),
                        render_pass,
                        framebuffer: framebuffers[0],
                        render_area: Rect2D {
                            offset: Offset2D { x: 0, y: 0 },
                            extent: Extent2D { width: 720, height: 480 },
                        },
                        clear_value_count: clear_values.len() as u32,
                        p_clear_values: clear_values.as_ptr(),
                        _marker: std::marker::PhantomData,
                    };
        
                    unsafe {
                        device.cmd_begin_render_pass(
                            command_buffers[0],
                            &render_pass_begin_info,
                            SubpassContents::INLINE,
                        );
                        device.cmd_bind_pipeline(
                            command_buffers[0],
                            PipelineBindPoint::GRAPHICS,
                            graphics_pipelines[0],
                        );
                        device.cmd_draw(command_buffers[0], 3, 1, 0, 0);
        
                        device.cmd_end_render_pass(command_buffers[0]);
        
                        device
                            .end_command_buffer(command_buffers[0])
                            .expect("Failed to record Command Buffer at Ending!");
        
                        let submit_info = SubmitInfo::default()
                            .command_buffers(&command_buffers);
                            
                        unsafe {
                            device
                                .queue_submit(queue_graphics, &[submit_info], Fence::null())
                                .expect("Не удалось отправить команду на выполнение");
                            device.queue_wait_idle(queue_graphics).expect("Не удалось дождаться завершения очереди");
                        }
                }
                
        
              
}