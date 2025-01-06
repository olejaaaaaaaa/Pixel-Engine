#![allow(warnings)]
#![feature(unwrap_infallible)]

use std::{ffi::CString, ptr};

use crate::{graphics_pipeline_layout::{self, ComponentGraphicsPipelineLayout}, PixelEngine};
use ash::vk::*;
use log::info;

impl PixelEngine {
    pub fn create_graphics_pipeline(device: &ash::Device, render_pass: &RenderPass, pipeline_layout_component: &ComponentGraphicsPipelineLayout) -> Pipeline {

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

        let graphic_pipeline_create_infos = [GraphicsPipelineCreateInfo {
            s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: ptr::null(),
            flags: PipelineCreateFlags::empty(),
            stage_count: pipeline_layout_component.shader_stages.len() as u32,
            p_stages: pipeline_layout_component.shader_stages.as_ptr(),
            p_vertex_input_state: &pipeline_layout_component.vertex_info,
            p_input_assembly_state: &pipeline_layout_component.assembly_info,
            p_tessellation_state: ptr::null(),
            p_viewport_state: &pipeline_layout_component.viewport_info,
            p_rasterization_state: &rasterization_statue_create_info,
            p_multisample_state: &pipeline_layout_component.multisample_info,
            p_depth_stencil_state: &pipeline_layout_component.depth_stencil_info,
            p_color_blend_state: &color_blend_state,
            p_dynamic_state: ptr::null(),
            layout: pipeline_layout,
            render_pass: *render_pass,
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
        
        info!("Создан pipeline");
        return graphics_pipelines[0]
    }
}