use ash::{vk::StaticFn, Entry};
use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};
use fermium::vulkan::SDL_Vulkan_GetVkGetInstanceProcAddr;

fn main() {
  let sdl = Sdl::init(InitFlags::VIDEO);
  let win = sdl
    .create_vk_window(CreateWinArgs { title: "Example Ash Window", ..Default::default() })
    .unwrap();

  let required_extensions = win.get_instance_extensions().unwrap();
  for mut extension_ptr in required_extensions.iter().copied() {
    print!("Required Extension: `");
    while unsafe { *extension_ptr } != 0 {
      print!("{}", unsafe { *extension_ptr } as u8 as char);
      extension_ptr = unsafe { extension_ptr.add(1) };
    }
    println!("`");
  }

  let pfn = unsafe { SDL_Vulkan_GetVkGetInstanceProcAddr() };
  if pfn.is_null() {
    panic!("couldn't begin Vulkan initialization.");
  }

  let entry = unsafe {
    Entry::from_static_fn(StaticFn { get_instance_proc_addr: core::mem::transmute(pfn) })
  };
  let app_info = ash::vk::ApplicationInfo {
    api_version: ash::vk::make_api_version(0, 1, 0, 0),
    ..Default::default()
  };
  let create_info_builder = ash::vk::InstanceCreateInfo::builder()
    .application_info(&app_info)
    .enabled_extension_names(&required_extensions);
  let _instance = unsafe { entry.create_instance(&create_info_builder, None).unwrap() };

  // program "main loop".
  'the_loop: loop {
    // Process events from this frame.
    #[allow(clippy::never_loop)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'the_loop,
        Event::JoystickAxis { .. } | Event::ControllerAxis { .. } | Event::MouseMotion { .. } => (),
        _ => println!("{event:?}"),
      }
    }

    // TODO: post-events drawing

    // TODO: swap buffers.
  }
}
