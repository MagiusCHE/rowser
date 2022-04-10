use core::fmt;

use crate::core::geometry::*;

use super::*;

#[derive(Debug)]
pub struct DeviceId {
    id: String,
}

impl DeviceId {
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Resized,
    CloseRequested,
    AxisMotion {
        device_id: DeviceId,
        axis: u32,
        value: f64,
    },
    CursorMoved {
        device_id: DeviceId,
        position: Position,
    },
    CursorLeft {
        device_id: DeviceId,
    },
    CursorEntered {
        device_id: DeviceId,
    },
    Focused {
        focus: bool,
    },
    Moved {
        position: Position,
    },
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", std::any::type_name_of_val(self))
    }
}

pub trait EventHandler {
    fn on_window_event(&mut self, _: &Event);
    fn on_window_resize(&mut self) {}
}

impl<'a> EventHandler for DomRoot<'a> {
    fn on_window_event(&mut self, event: &Event) {
        match event {
            Event::Resized => self.on_window_resize(),
            Event::CloseRequested => (),
            #[allow(unused)]
            Event::AxisMotion {
                device_id,
                axis,
                value,
            } => (),
            #[allow(unused)]
            Event::CursorMoved {
                device_id,
                position,
            } => (),
            _ => {
                fixme!(format!("{:?}", event))
            }
        }
    }
}
