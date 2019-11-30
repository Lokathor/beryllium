use super::*;

/// A controller was added, removed, or had the buttons remapped.
#[derive(Debug, Clone, Copy)]
pub enum ControllerDeviceEvent {
  /// Controller added
  Added {
    /// When?
    timestamp: u32,
    /// What's the new index?
    joystick_index: i32,
  },
  /// A controller was removed from the system.
  Removed {
    /// When?
    timestamp: u32,
    /// Which instance ID went away?
    instance_id: i32,
  },
  /// A controller had the buttons remapped.
  Remapped {
    /// When?
    timestamp: u32,
    /// Which instance ID changed button mapping?
    instance_id: i32,
  },
}

impl TryFrom<fermium::SDL_ControllerDeviceEvent> for ControllerDeviceEvent {
  type Error = ();
  fn try_from(ev: fermium::SDL_ControllerDeviceEvent) -> Result<Self, ()> {
    Ok(match ev.type_ as SDL_EventType {
      fermium::SDL_CONTROLLERDEVICEADDED => ControllerDeviceEvent::Added {
        timestamp: ev.timestamp,
        joystick_index: ev.which,
      },
      fermium::SDL_CONTROLLERDEVICEREMOVED => ControllerDeviceEvent::Removed {
        timestamp: ev.timestamp,
        instance_id: ev.which,
      },
      fermium::SDL_CONTROLLERDEVICEREMAPPED => {
        ControllerDeviceEvent::Remapped {
          timestamp: ev.timestamp,
          instance_id: ev.which,
        }
      }
      _ => return Err(()),
    })
  }
}
impl From<ControllerDeviceEvent> for fermium::SDL_ControllerDeviceEvent {
  fn from(ev: ControllerDeviceEvent) -> Self {
    match ev {
      ControllerDeviceEvent::Added { timestamp, joystick_index } => {
        fermium::SDL_ControllerDeviceEvent {
          type_: fermium::SDL_CONTROLLERDEVICEADDED as u32,
          timestamp,
          which: joystick_index,
        }
      }
      ControllerDeviceEvent::Removed { timestamp, instance_id } => {
        fermium::SDL_ControllerDeviceEvent {
          type_: fermium::SDL_CONTROLLERDEVICEREMOVED as u32,
          timestamp,
          which: instance_id,
        }
      }
      ControllerDeviceEvent::Remapped { timestamp, instance_id } => {
        fermium::SDL_ControllerDeviceEvent {
          type_: fermium::SDL_CONTROLLERDEVICEREMAPPED as u32,
          timestamp,
          which: instance_id,
        }
      }
    }
  }
}
