//! Demo of how to setup a blank window using the `Ash` crate for Vulkan.
//!
//! Be aware that several simplifications have been made that impact
//! performance and robustness.

use beryllium::*;

use ash;
use ash::vk;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::version::DeviceV1_0;

use std::os::raw::c_char;

fn main() -> Result<(), String> {
  //
  // INITIALIZATION CODE START
  //

  let sdl = beryllium::init()?;

  let sdl_window = sdl.create_window(
    "Extern Crate: `ash`",                    // title
    WINDOW_POSITION_CENTERED,                 // x
    WINDOW_POSITION_CENTERED,                 // y
    800,                                      // width
    600,                                      // height
    WindowFlags::default().with_vulkan(true), // flags
  )?;

  // We use the default Ash Entry for simplicity.
  //
  // TODO: Add example in comments on how to use Beryllium's Vulkan procAddress
  let ash_entry = ash::Entry::new().unwrap();

  let ash_instance = create_ash_instance(&ash_entry, &sdl_window);

  // Creates the opaque Vulkan surface handle (VkSurfaceKHR) for which we wish to render onto.
  let vk_surface = sdl_window.create_vk_surface(ash_instance.handle()).unwrap();

  // Loads the function pointers related to VkSurfaceKHR operations.
  let surface_khr_loader = ash::extensions::khr::Surface::new(&ash_entry, &ash_instance);

  let vk_physical_device = pick_physical_device(&ash_entry, &ash_instance, vk_surface);

  // Create the logical VkDevice from our VkPhysicalDevice.
  let ash_device = create_ash_device(&ash_instance, vk_physical_device);



  // Find out what image format we want to use for the VkSwapchainKHR based on which ones are available.
  let available_surface_formats = unsafe { surface_khr_loader.get_physical_device_surface_formats(vk_physical_device, vk_surface).unwrap() };
  let surface_format_to_use = find_surface_format(&available_surface_formats).unwrap();
  let vk_surface_capabilities = unsafe { surface_khr_loader.get_physical_device_surface_capabilities(vk_physical_device, vk_surface).unwrap() };

  // Grab the dimensions for our drawable surface from the `Window` handle
  //
  // This information can ordinarily also be found in the
  // `VkSurfaceCapabilitiesKHR::currentExtent` field after querying
  // from the VkSurfaceKHR handle, like we did directly above.
  let surface_dimensions = sdl_window.vk_drawable_size();

  let swapchain_info = vk::SwapchainCreateInfoKHR{
    surface: vk_surface,
    min_image_count: 2,
    image_format: surface_format_to_use.format,
    image_color_space: surface_format_to_use.color_space,
    image_extent: surface_dimensions,
    image_array_layers: 1,
    image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
    image_sharing_mode: vk::SharingMode::EXCLUSIVE,
    pre_transform: vk_surface_capabilities.current_transform,
    // TODO: Find a vk::CompositeAlphaFlagsKHR bitflag that's available in surface capabilities.
    composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
    // FIFO is guaranteed to be available per Vulkan spec.
    present_mode: vk::PresentModeKHR::FIFO,
    clipped: 1,
    .. Default::default()
  };
  // Loads all functions pointers related to VkSwapchainKHR operations
  let swapchain_khr_loader = ash::extensions::khr::Swapchain::new(&ash_instance, &ash_device);
  let vk_swapchain = unsafe { swapchain_khr_loader.create_swapchain(&swapchain_info, None).unwrap() };
  let vk_swapchain_images = unsafe { swapchain_khr_loader.get_swapchain_images(vk_swapchain).unwrap() };
  let vk_swapchain_image_views = create_swapchain_image_views(&ash_device, &vk_swapchain_images, swapchain_info.image_format);

  let vk_render_pass = create_basic_render_pass(&ash_device, swapchain_info.image_format);


  // Create a command pool for our command buffer
  let cmd_pool_info = vk::CommandPoolCreateInfo{
    queue_family_index: 0,
    flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
    .. Default::default()
  };
  let cmd_pool = unsafe { ash_device.create_command_pool(&cmd_pool_info, None).unwrap() };
  // We allocate 1 command buffer from our pool. This will be used to clear the image.
  let cmd_buffer_alloc_info = vk::CommandBufferAllocateInfo{
    command_pool: cmd_pool,
    level: vk::CommandBufferLevel::PRIMARY,
    command_buffer_count: 1,
    .. Default::default()
  };
  let cmd_buffer = unsafe { ash_device.allocate_command_buffers(&cmd_buffer_alloc_info).unwrap()[0] };

  // Synchronization object (VkFence) that lets ut determine when our next swapchain is ready.
  let image_ready_fence = unsafe { ash_device.create_fence(&Default::default(), None).unwrap() };
  // We also need one for our graphics command buffer.
  let fence_info = vk::FenceCreateInfo{
    flags: vk::FenceCreateFlags::SIGNALED,
    .. Default::default()
  };
  let cmd_buffer_ready_fence = unsafe { ash_device.create_fence(&fence_info, None).unwrap() };

  // Grab our one and only device queue
  let queue = unsafe { ash_device.get_device_queue(0, 0) };

  // We need one framebuffer for each swapchain image,
  // since we want to render to each of the images in turn
  // and then present them.
  let vk_swapchain_framebuffers = create_swapchain_framebuffers(
    &ash_device,
    vk_render_pass,
    &vk_swapchain_image_views,
    swapchain_info.image_extent);

  //
  // INITIALIZATION CODE END
  //


  //
  // MAIN LOOP START
  //
  'game_loop: loop {
    // Poll SDL for events.
    while let Some(event) = sdl.poll_event() {
      #[allow(clippy::single_match)]
      match event {
        Event::Quit { timestamp } => {
          println!("Quitting the program after {} milliseconds.", timestamp);
          break 'game_loop;
        }
        _ => (),
      }
    }


    // Query which swapchain image should be presented next
    let acquire_image_result = unsafe { swapchain_khr_loader.acquire_next_image(
      vk_swapchain,
      std::u64::MAX,
      vk::Semaphore::null(),
      image_ready_fence)
    };
    assert!(acquire_image_result.is_ok(), "Error. Example not coded to handle errors when acquiring swapchain image.");
    assert!(!acquire_image_result.unwrap().1, "Error. Example not coded to handle suboptimal swapchains.");
    let next_swapchain_index = acquire_image_result.unwrap().0;

    // Wait for the command buffer and next swapchain image to be ready to be used, and reset the VkFences
    unsafe {
      ash_device.wait_for_fences(&[image_ready_fence, cmd_buffer_ready_fence], true, std::u64::MAX).unwrap();
      ash_device.reset_fences(&[image_ready_fence, cmd_buffer_ready_fence]).unwrap();
    }

    // Rebuild the command buffer
    build_command_buffer(
      &ash_device,
      cmd_buffer,
      vk_render_pass,
      vk_swapchain_framebuffers[next_swapchain_index as usize],
      vk_surface_capabilities.current_extent
    );

    // Submit our command buffer execution, and signal the fence when the command buffer
    // is no longer in use.
    let submit_info = vk::SubmitInfo{
      command_buffer_count: 1,
      p_command_buffers: &cmd_buffer,
      .. Default::default()
    };
    unsafe {
      ash_device.queue_submit(queue, &[submit_info], cmd_buffer_ready_fence).unwrap();
    }

    let present_info = vk::PresentInfoKHR{
      swapchain_count: 1,
      p_swapchains: &vk_swapchain,
      p_image_indices: &next_swapchain_index,
      .. Default::default()
    };

    unsafe {
      // Queue image for presentation, and check the result of submit
      let swapchain_is_suboptimal = swapchain_khr_loader.queue_present(queue, &present_info).unwrap();
      assert!(!swapchain_is_suboptimal, "Error. Example not coded to handle suboptimal swapchains.");
    }


  }
  //
  // MAIN LOOP END
  //



  // Vulkan resource cleanup omitted for simplicity.



  Ok(())
}

fn create_ash_instance(ash_entry: &ash::Entry, sdl_window: &beryllium::Window) -> ash::Instance {
  let vk_app_info = vk::ApplicationInfo{
    p_application_name: std::ffi::CStr::from_bytes_with_nul(b"Beryllium - Ash example\0").unwrap().as_ptr(),
    application_version: ash::vk_make_version!(1, 0, 0),
    p_engine_name: std::ffi::CStr::from_bytes_with_nul(b"Beryllium - Ash example\0").unwrap().as_ptr(),
    engine_version: ash::vk_make_version!(1, 0, 0),
    api_version: ash::vk_make_version!(1, 0, 0),
    .. Default::default()
  };

  // We need to query which instance extensions are available, and which are required by the Beryllium window.
  let available_instance_extensions = ash_entry.enumerate_instance_extension_properties().unwrap();
  let mut instance_extensions_to_use = sdl_window.required_vk_extensions();

  // Validation layers get enabled if the debug utils extension
  // and the Khronos standard layer is present.
  let mut use_validation_layers = false;
  if check_if_instance_extension_present(ash::extensions::ext::DebugUtils::name().as_ptr(), available_instance_extensions.as_slice()) {
    instance_extensions_to_use.push(ash::extensions::ext::DebugUtils::name().as_ptr());
    use_validation_layers = true;
  }
  assert!(validate_instance_extensions(instance_extensions_to_use.as_slice(), &available_instance_extensions),
          "Error. Not all required extensions are available.");

  // Look for Khrons standard validation layer, enable it found
  let mut validation_layers_to_use = Vec::<*const c_char>::new();
  if use_validation_layers {
    let available_layers = ash_entry.enumerate_instance_layer_properties().unwrap();
    let khronos_layer_name = std::ffi::CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap();
    if check_if_layer_present(khronos_layer_name.as_ptr(), available_layers.as_slice()) {
      validation_layers_to_use.push(khronos_layer_name.as_ptr());
    }
  }

  let vk_instance_create_info = vk::InstanceCreateInfo{
    p_application_info: &vk_app_info,
    enabled_extension_count: instance_extensions_to_use.len() as u32,
    pp_enabled_extension_names: instance_extensions_to_use.as_ptr(),
    enabled_layer_count: validation_layers_to_use.len() as u32,
    pp_enabled_layer_names: validation_layers_to_use.as_ptr(),
    .. Default::default()
  };
  let ash_instance = unsafe { ash_entry.create_instance(&vk_instance_create_info, None).unwrap() };

  // Set up validation layer debug callback if possible
  if use_validation_layers {
    let debug_utils = ash::extensions::ext::DebugUtils::new(ash_entry, &ash_instance);

    let messenger_info = vk::DebugUtilsMessengerCreateInfoEXT{
      message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
      message_type: vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
      pfn_user_callback: Some(debug_utils_callback),
      .. Default::default()
    };

    let _debug_messenger = unsafe { debug_utils.create_debug_utils_messenger(&messenger_info, None) };

  }

  ash_instance
}

/// Returns true if all required extensions are present in available extensions.
fn validate_instance_extensions(required_extensions: &[*const c_char], available_extensions: &[vk::ExtensionProperties]) -> bool {
  required_extensions.iter()
      .map(|required| unsafe { std::ffi::CStr::from_ptr(*required) })
      .all(|required| {
        available_extensions.iter()
            .map(|available| unsafe { std::ffi::CStr::from_ptr(available.extension_name.as_ptr()) })
            .any(|available| available == required)
      })
}

/// Returns true if wanted extension is present in available extensions.
fn check_if_instance_extension_present(wanted_extension: *const c_char, available_extensions: &[vk::ExtensionProperties]) -> bool {
  for available_ext in available_extensions {
    let wanted_ext_c_str = unsafe { std::ffi::CStr::from_ptr(wanted_extension) };
    let available_ext_c_str = unsafe {std::ffi::CStr::from_ptr(available_ext.extension_name.as_ptr()) };
    if wanted_ext_c_str == available_ext_c_str {
      return true;
    }
  }
  false
}

/// Returns true if wanted layer is present in available layers.
fn check_if_layer_present(wanted_layer: *const c_char, available_layers: &[vk::LayerProperties]) -> bool {
  for available_layer in available_layers {
    let wanted_layer_c_str = unsafe { std::ffi::CStr::from_ptr(wanted_layer) };
    let available_layer_c_cstr = unsafe { std::ffi::CStr::from_ptr(available_layer.layer_name.as_ptr()) };
    if wanted_layer_c_str == available_layer_c_cstr {
      return true;
    }
  }
  false
}

/// Callback for the Vulkan validation layer
unsafe extern "system" fn debug_utils_callback(
  _message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  _message_types: vk::DebugUtilsMessageTypeFlagsEXT,
  p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  _p_user_data: *mut core::ffi::c_void,
) -> vk::Bool32 {
  let c_str = std::ffi::CStr::from_ptr((*p_callback_data).p_message);

  println!("Vulkan validation layer:\n{}", c_str.to_str().unwrap());

  0
}

fn pick_physical_device(ash_entry: &ash::Entry, ash_instance: &ash::Instance, vk_surface: vk::SurfaceKHR) -> vk::PhysicalDevice {
  // We list the available physical devices (VkPhysicalDevice) with Vulkan support. For simplicity we pick the first one we find.
  // TODO: Pick physical device based on the device's properties and capabilites.
  let vk_physical_device = unsafe { ash_instance.enumerate_physical_devices().unwrap()[0] };

  // Loads the function pointers related to VkSurfaceKHR operations.
  let surface_khr_loader = ash::extensions::khr::Surface::new(ash_entry, ash_instance);
  // Check if our VkPhysicalDevice can present to the VkSurfaceKHR
  let device_surface_support = unsafe { surface_khr_loader.get_physical_device_surface_support(vk_physical_device, 0, vk_surface) };
  assert!(device_surface_support, "Error. Physical Device 0 cannot present to surface. Example not set up to test other physical devices.");

  vk_physical_device
}

fn create_ash_device(ash_instance: &ash::Instance, vk_physical_device: vk::PhysicalDevice) -> ash::Device {
  // Set up information to request one queue from the physical device.
  let device_queue_priority: f32 = 1.0;
  let vk_device_queue_info = vk::DeviceQueueCreateInfo{
    queue_family_index: 0,
    queue_count: 1,
    p_queue_priorities: &device_queue_priority,
    .. Default::default()
  };
  // Use no physical device feature (other than the ones guaranteed by Vulkan specification)
  let vk_phys_device_features = vk::PhysicalDeviceFeatures::default();

  // We only need one device extension; VK_KHR_swapchain. This is required for a device to be able to present to a VkSurfaceKHR handle.
  // For simplicity we assume this is present on the physical device
  // TODO: Check that VK_KHR_swapchain is available for our chosen VkPhysicalDevice.
  let required_device_extension = std::ffi::CStr::from_bytes_with_nul(b"VK_KHR_swapchain\0").unwrap().as_ptr();

  let vk_device_info = vk::DeviceCreateInfo{
    queue_create_info_count: 1,
    p_queue_create_infos: &vk_device_queue_info,
    enabled_extension_count: 1,
    pp_enabled_extension_names: &required_device_extension,
    p_enabled_features: &vk_phys_device_features,
    .. Default::default()
  };

  unsafe {
    ash_instance.create_device(vk_physical_device, &vk_device_info, None).unwrap()
  }
}

const WANTED_SURFACE_FORMATS: [vk::SurfaceFormatKHR; 2] = [
  // Most used one in Windows
  vk::SurfaceFormatKHR{
    format: vk::Format::R8G8B8A8_UNORM,
    color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR
  },
  // Most used one in UNIX
  vk::SurfaceFormatKHR{
    format: vk::Format::B8G8R8A8_UNORM,
    color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR
  },
];

fn find_surface_format(available_formats: &[vk::SurfaceFormatKHR]) -> Option<vk::SurfaceFormatKHR> {
  let mut surface_format_to_use = vk::SurfaceFormatKHR{
    format: vk::Format::UNDEFINED,
    color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
  };
  'outer_loop: for wanted_format in WANTED_SURFACE_FORMATS.iter() {
    for available_format in available_formats {
      if available_format.format == wanted_format.format && available_format.color_space == wanted_format.color_space {
        surface_format_to_use = *wanted_format;
        break 'outer_loop;
      }
    }
  }
  if surface_format_to_use.format == vk::Format::UNDEFINED {
    return None;
  } else {
    Some(surface_format_to_use)
  }
}

fn create_swapchain_image_views(ash_device: &ash::Device, images: &[vk::Image], image_format: vk::Format) -> Vec<vk::ImageView> {
  let mut image_views = Vec::<vk::ImageView>::new();
  for image in images {
    let view_info = vk::ImageViewCreateInfo{
      image: *image,
      view_type: vk::ImageViewType::TYPE_2D,
      format: image_format,
      subresource_range: vk::ImageSubresourceRange{
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1
      },
      .. Default::default()
    };

    image_views.push(unsafe { ash_device.create_image_view(&view_info, None).unwrap() });
  }
  image_views
}

fn create_basic_render_pass(ash_device: &ash::Device, image_format: vk::Format) -> vk::RenderPass {
  let attach_descr = vk::AttachmentDescription{
    format: image_format,
    samples: vk::SampleCountFlags::TYPE_1,
    load_op: vk::AttachmentLoadOp::CLEAR,
    store_op: vk::AttachmentStoreOp::STORE,
    stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
    stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
    initial_layout: vk::ImageLayout::UNDEFINED,
    final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
    .. Default::default()
  };

  let attach_ref = vk::AttachmentReference{
    attachment: 0,
    layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
  };

  let subpass_descr = vk::SubpassDescription{
    pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
    color_attachment_count: 1,
    p_color_attachments: &attach_ref,
    .. Default::default()
  };

  let render_pass_info = vk::RenderPassCreateInfo{
    attachment_count: 1,
    p_attachments: &attach_descr,
    subpass_count: 1,
    p_subpasses: &subpass_descr,
    .. Default::default()
  };

  unsafe {
    ash_device.create_render_pass(&render_pass_info, None).unwrap()
  }
}

fn create_swapchain_framebuffers(
  ash_device: &ash::Device,
  render_pass: vk::RenderPass,
  swapchain_img_views: &[vk::ImageView],
  extent: vk::Extent2D,
) -> Vec<vk::Framebuffer> {
  let mut framebuffers = Vec::<vk::Framebuffer>::new();
  for image_view in swapchain_img_views {
    let fb_info = vk::FramebufferCreateInfo{
      render_pass,
      attachment_count: 1,
      p_attachments: image_view,
      width: extent.width,
      height: extent.height,
      layers: 1,
      .. Default::default()
    };
    framebuffers.push(unsafe { ash_device.create_framebuffer(&fb_info, None).unwrap() } );
  }

  framebuffers
}

fn build_command_buffer(
  ash_device: &ash::Device,
  cmd_buffer: vk::CommandBuffer,
  render_pass: vk::RenderPass,
  framebuffer: vk::Framebuffer,
  extent: vk::Extent2D) {
  let begin_info = vk::CommandBufferBeginInfo::default();

  unsafe {
    ash_device.begin_command_buffer(cmd_buffer, &begin_info).unwrap();

    let mut clear_value = vk::ClearValue::default();
    clear_value.color.float32 = [1.0, 0.5, 0.0, 1.0];
    let rp_begin_info = vk::RenderPassBeginInfo{
      render_pass,
      framebuffer,
      render_area: vk::Rect2D{
        extent,
        .. Default::default()
      },
      clear_value_count: 1,
      p_clear_values: &clear_value,
      .. Default::default()
    };

    ash_device.cmd_begin_render_pass(cmd_buffer, &rp_begin_info, vk::SubpassContents::INLINE);

    ash_device.cmd_end_render_pass(cmd_buffer);

    ash_device.end_command_buffer(cmd_buffer).unwrap();
  }

}