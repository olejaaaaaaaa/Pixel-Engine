#![allow(warnings)]

use std::{ffi::CString, ptr};

use crate::PixelEngine;
use crate::swapchain::ComponentSwapchain;
use ash::khr::swapchain;
use ash::vk::*;
use std::ffi::CStr;

const shader_main: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };

#[derive(Debug)]
pub struct ComponentGraphicsPipelineLayout {
    pub pipeline_layout:            PipelineLayout,
    pub shader_stages:              [PipelineShaderStageCreateInfo<'static>; 2],
    pub rasterization_info:         PipelineRasterizationStateCreateInfo<'static>,
    pub vertex_info:                PipelineVertexInputStateCreateInfo<'static>,
    pub assembly_info:              PipelineInputAssemblyStateCreateInfo<'static>,
    pub viewport_info:              PipelineViewportStateCreateInfo<'static>,
    pub multisample_info:           PipelineMultisampleStateCreateInfo<'static>,
    pub depth_stencil_info:         PipelineDepthStencilStateCreateInfo<'static>,
    pub color_blend_info:           PipelineColorBlendStateCreateInfo<'static>,
    pub scissors:                   Vec<Rect2D>,
    pub viewport:                   Vec<Viewport>
}

const color_blend_attachment_states: [PipelineColorBlendAttachmentState; 1] = [PipelineColorBlendAttachmentState {
    blend_enable:           FALSE,
    color_write_mask:       ColorComponentFlags::RGBA,
    src_color_blend_factor: BlendFactor::ONE,
    dst_color_blend_factor: BlendFactor::ZERO,
    color_blend_op:         BlendOp::ADD,
    src_alpha_blend_factor: BlendFactor::ONE,
    dst_alpha_blend_factor: BlendFactor::ZERO,
    alpha_blend_op:         BlendOp::ADD,
}];

impl PixelEngine {
    pub fn create_graphics_pipeline_layout(
        device: &ash::Device, 
        vshader: &ShaderModule, 
        fshader: &ShaderModule,
        current_extent2d: Extent2D
    ) -> ComponentGraphicsPipelineLayout {

        let extent2d = current_extent2d;

        let shader_stages = [
            PipelineShaderStageCreateInfo::default()
                .name(&shader_main)
                .module(*vshader)
                .stage(ShaderStageFlags::VERTEX),

            PipelineShaderStageCreateInfo::default()
                .name(&shader_main)
                .module(*fshader)
                .stage(ShaderStageFlags::FRAGMENT)
        ];

        let vertex_input_state_create_info = PipelineVertexInputStateCreateInfo::default();

        let vertex_input_assembly_state_info = PipelineInputAssemblyStateCreateInfo::default()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = vec![Viewport {
            x:          0.0,
            y:          0.0,
            width:      extent2d.width as f32,
            height:     extent2d.height as f32,
            min_depth:  0.0,
            max_depth:  1.0,
        }];

        let scissors = vec![Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: extent2d,
        }];

        let viewport_state_create_info = PipelineViewportStateCreateInfo {
            s_type:         StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next:         ptr::null(),
            flags:          PipelineViewportStateCreateFlags::empty(),
            scissor_count:  scissors.len() as u32,
            p_scissors:     scissors.as_ptr(),
            viewport_count: viewport.len() as u32,
            p_viewports:    viewport.as_ptr(),
            _marker:        std::marker::PhantomData,
        };

        let rasterization_status_create_info = PipelineRasterizationStateCreateInfo {
            s_type:                     StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next:                     ptr::null(),
            flags:                      PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable:         FALSE,
            cull_mode:                  CullModeFlags::BACK,
            front_face:                 FrontFace::CLOCKWISE,
            line_width:                 1.0,
            polygon_mode:               PolygonMode::FILL,
            rasterizer_discard_enable:  FALSE,
            depth_bias_clamp:           0.0,
            depth_bias_constant_factor: 0.0,
            depth_bias_enable:          FALSE,
            depth_bias_slope_factor:    0.0,
            _marker:                    std::marker::PhantomData,
        };
        
        let multisample_state_create_info = PipelineMultisampleStateCreateInfo {
            s_type:                     StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            flags:                      PipelineMultisampleStateCreateFlags::empty(),
            p_next:                     ptr::null(),
            rasterization_samples:      SampleCountFlags::TYPE_1,
            sample_shading_enable:      FALSE,
            min_sample_shading:         0.0,
            p_sample_mask:              ptr::null(),
            alpha_to_one_enable:        FALSE,
            alpha_to_coverage_enable:   FALSE,
            _marker:                    std::marker::PhantomData,
        };

        let stencil_state = StencilOpState {
            fail_op:        StencilOp::KEEP,
            pass_op:        StencilOp::KEEP,
            depth_fail_op:  StencilOp::KEEP,
            compare_op:     CompareOp::ALWAYS,
            compare_mask:   0,
            write_mask:     0,
            reference:      0,
        };

        let depth_state_create_info = PipelineDepthStencilStateCreateInfo {
            s_type:                     StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next:                     ptr::null(),
            flags:                      PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable:          FALSE,
            depth_write_enable:         FALSE,
            depth_compare_op:           CompareOp::LESS_OR_EQUAL,
            depth_bounds_test_enable:   FALSE,
            stencil_test_enable:        FALSE,
            front:                      stencil_state,
            back:                       stencil_state,
            max_depth_bounds:           1.0,
            min_depth_bounds:           0.0,
            _marker:                    std::marker::PhantomData,
        };


        let color_blend_state = PipelineColorBlendStateCreateInfo::default()
            .blend_constants([0.0, 0.0, 0.0, 0.0])
            .attachments(&color_blend_attachment_states)
            .logic_op_enable(true);
        //     s_type:             StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        //     p_next:             ptr::null(),
        //     flags:              PipelineColorBlendStateCreateFlags::empty(),
        //     logic_op_enable:    FALSE,
        //     logic_op:           LogicOp::COPY,
        //     attachment_count:   color_blend_attachment_states.len() as u32,
        //     p_attachments:      color_blend_attachment_states.as_ptr(),
        //     blend_constants:    [0.0, 0.0, 0.0, 0.0],
        //     _marker:            std::marker::PhantomData,
        // };

        let pipeline_layout_create_info = PipelineLayoutCreateInfo::default();

        let pipeline_layout = unsafe { device
                .create_pipeline_layout(&pipeline_layout_create_info, None)
                .expect("Failed to create pipeline layout!") };

        ComponentGraphicsPipelineLayout {
            scissors,
            viewport,
            pipeline_layout:    pipeline_layout,
            shader_stages:      shader_stages,
            rasterization_info: rasterization_status_create_info,
            vertex_info:        vertex_input_state_create_info,
            assembly_info:      vertex_input_assembly_state_info,
            viewport_info:      viewport_state_create_info,
            multisample_info:   multisample_state_create_info,
            depth_stencil_info: depth_state_create_info,
            color_blend_info:   color_blend_state,
        }
    }
}