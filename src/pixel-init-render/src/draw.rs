
use std::ptr;

use ash::vk::*;

use crate::{swapchain::ComponentSwapchain, PixelEngine};

pub struct SyncObjects {
    pub image_available_semaphores: Vec<Semaphore>,
    pub render_finished_semaphores: Vec<Semaphore>,
    pub inflight_fences: Vec<Fence>,
}

impl PixelEngine {
    
    pub fn create_command_buffers(
        device: &ash::Device,
        command_pool: CommandPool,
        graphics_pipeline: Pipeline,
        framebuffers: &Vec<Framebuffer>,
        render_pass: RenderPass,
        surface_extent: Extent2D,
    ) -> Vec<CommandBuffer> {
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

        for (i, &command_buffer) in command_buffers.iter().enumerate() {
            let command_buffer_begin_info = CommandBufferBeginInfo {
                s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: ptr::null(),
                p_inheritance_info: ptr::null(),
                flags: CommandBufferUsageFlags::SIMULTANEOUS_USE,
                _marker: std::marker::PhantomData,
            };

            unsafe {
                device
                    .begin_command_buffer(command_buffer, &command_buffer_begin_info)
                    .expect("Failed to begin recording Command Buffer at beginning!");
            }

            let clear_values = [ClearValue {
                color: ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];

            let render_pass_begin_info = RenderPassBeginInfo {
                s_type: StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: ptr::null(),
                render_pass,
                framebuffer: framebuffers[i],
                render_area: Rect2D {
                    offset: Offset2D { x: 0, y: 0 },
                    extent: surface_extent,
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
                _marker: std::marker::PhantomData,
            };

            unsafe {
                device.cmd_begin_render_pass(
                    command_buffer,
                    &render_pass_begin_info,
                    SubpassContents::INLINE,
                );
                device.cmd_bind_pipeline(
                    command_buffer,
                    PipelineBindPoint::GRAPHICS,
                    graphics_pipeline,
                );
                device.cmd_draw(command_buffer, 3, 1, 0, 0);

                device.cmd_end_render_pass(command_buffer);

                device
                    .end_command_buffer(command_buffer)
                    .expect("Failed to record Command Buffer at Ending!");
            }
        }

        command_buffers
    }

    pub fn draw_frame(device: &ash::Device, command_buffers: Vec<CommandBuffer>, in_flight_fences: Vec<Fence>, mut current_frame: usize, swapchain: ComponentSwapchain, image_available_semaphores: Vec<Semaphore>, render_finished_semaphores: Vec<Semaphore>, graphics: Queue, present: Queue) {

        let wait_fences = [in_flight_fences[current_frame]];

        let (image_index, _is_sub_optimal) = unsafe {
            device
                .wait_for_fences(&wait_fences, true, std::u64::MAX)
                .expect("Failed to wait for Fence!");

            swapchain.swapchain_fn
                .acquire_next_image(
                    swapchain.swapchain,
                    std::u64::MAX,
                    image_available_semaphores[current_frame],
                    Fence::null(),
                )
                .expect("Failed to acquire next image.")
        };

        let wait_semaphores = [image_available_semaphores[current_frame]];
        let wait_stages = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [render_finished_semaphores[current_frame]];

        let submit_infos = [SubmitInfo {
            s_type: StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &command_buffers[image_index as usize],
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
            _marker: std::marker::PhantomData,
        }];

        unsafe {
            device
                .reset_fences(&wait_fences)
                .expect("Failed to reset Fence!");

            device
                .queue_submit(
                    graphics,
                    &submit_infos,
                    in_flight_fences[current_frame],
                )
                .expect("Failed to execute queue submit.");
        }

        let swapchains = [swapchain.swapchain];

        let present_info = PresentInfoKHR {
            s_type: StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: ptr::null_mut(),
            _marker: std::marker::PhantomData,
        };

        unsafe {
            swapchain.swapchain_fn
                .queue_present(present, &present_info)
                .expect("Failed to execute queue present.");
        }

        current_frame = (current_frame + 1) % 60;
    }

    pub fn create_sync_objects(device: &ash::Device) -> SyncObjects {
        let mut sync_objects = SyncObjects {
            image_available_semaphores: vec![],
            render_finished_semaphores: vec![],
            inflight_fences: vec![],
        };

        let semaphore_create_info = SemaphoreCreateInfo {
            s_type: StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: SemaphoreCreateFlags::empty(),
            _marker: std::marker::PhantomData,
        };

        let fence_create_info = FenceCreateInfo {
            s_type: StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: FenceCreateFlags::SIGNALED,
            _marker: std::marker::PhantomData,
        };

        for _ in 0..2 {
            unsafe {
                let image_available_semaphore = device
                    .create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let render_finished_semaphore = device
                    .create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let inflight_fence = device
                    .create_fence(&fence_create_info, None)
                    .expect("Failed to create Fence Object!");

                sync_objects
                    .image_available_semaphores
                    .push(image_available_semaphore);
                sync_objects
                    .render_finished_semaphores
                    .push(render_finished_semaphore);
                sync_objects.inflight_fences.push(inflight_fence);
            }
        }

        sync_objects
    }
       
}