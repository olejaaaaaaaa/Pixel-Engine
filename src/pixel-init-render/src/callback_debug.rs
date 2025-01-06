
use std::ffi::c_void;
use std::ffi::CStr;
use std::ptr;
use log::*;
use ash::vk::*;

use crate::PixelEngine;

pub unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> Bool32 {
    let severity = match message_severity {
        DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);

    match severity {
        "[Verbose]" => debug!("{:?}", message),
        "[Warning]" => warn!("{:?}", message),
        "[Error]" => error!("{:?}", message),
        "[Info]" => info!("{:?}", message),
        _ => (),
    }


    FALSE
}


impl PixelEngine {
    pub fn setup_debug_utils(
    ) -> DebugUtilsMessengerCreateInfoEXT<'static> {

        let mut debug_utils_create_info = DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            DebugUtilsMessageSeverityFlagsEXT::WARNING |
            DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            DebugUtilsMessageSeverityFlagsEXT::INFO |
            DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
                  DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        )
        .pfn_user_callback(Some(vulkan_debug_utils_callback));
    
        return debug_utils_create_info
    }
}


