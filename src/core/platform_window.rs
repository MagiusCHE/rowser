#![allow(dead_code)]
#![allow(unused_imports)]

pub struct MainWindow {}

use winit::{
    event::{DeviceId, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::{
    any::Any,
    cell::{Ref, RefCell},
    error::Error,
    ptr::NonNull,
    rc::Rc,
};

use super::dom::{
    self, *, window_events::*
};

use super::geometry::*;

use log::{debug, error, info, warn};

use std::time::{Duration, Instant};

impl MainWindow {
    pub fn run<F>(&self, exit_handler: F)
    where
        F: 'static + Fn(),
    {
        let event_loop = EventLoop::new();
        let window = Rc::new(
            WindowBuilder::new()
                .with_title("Rowser")
                .build(&event_loop)
                .unwrap(),
        );

        let mut last_frame_time = Instant::now();
        let mut gfx_root: Box<DomElement> =
            dom::create_dom_element(None, dom::DomElementType::Root(window.clone()));

        //let mut gfx_buffer = GfBuffer::new(window.clone());
        info!("Begin loop {:?}", event_loop);
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    window_id: _,
                } => {
                    gfx_root
                        .as_mut()
                        .on_window_event(&window_events::Event::Resized);
                }
                #[allow(deprecated)]
                Event::WindowEvent {
                    event:
                        WindowEvent::CursorMoved {
                            device_id,
                            position,
                            modifiers: _,
                        },
                    window_id: _,
                } => {
                    gfx_root
                        .as_mut()
                        .on_window_event(&window_events::Event::CursorMoved {
                            device_id: window_events::DeviceId::new(format!("{:?}", device_id).as_str()), // format!("{}",device_id),
                            position: Position {
                                left: position.x,
                                top: position.y,
                            },
                        });
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::AxisMotion {
                            device_id,
                            axis,
                            value,
                        },
                    window_id: _,
                } => {
                    gfx_root
                        .as_mut()
                        .on_window_event(&window_events::Event::AxisMotion {
                            device_id: window_events::DeviceId::new(format!("{:?}", device_id).as_str()), // format!("{}",device_id),
                            axis,
                            value,
                        });
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorEntered { device_id },
                    window_id: _,
                } => {
                    gfx_root
                        .as_mut()
                        .on_window_event(&window_events::Event::CursorEntered {
                            device_id: window_events::DeviceId::new(format!("{:?}", device_id).as_str()), // format!("{}",device_id),
                        });
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorLeft { device_id },
                    window_id: _,
                } => {
                    gfx_root
                        .as_mut()
                        .on_window_event(&window_events::Event::CursorLeft {
                            device_id: window_events::DeviceId::new(format!("{:?}", device_id).as_str()), // format!("{}",device_id),
                        });
                }
                Event::WindowEvent {
                    event: WindowEvent::Focused(focus),
                    window_id: _,
                } => {
                    gfx_root
                        .as_mut()
                        .on_window_event(&window_events::Event::Focused { focus });
                }
                Event::MainEventsCleared => {
                    // Application update code.

                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.
                    //info!("MainEventsCleared");
                    //window.request_redraw();
                    gfx_root.on_frame(last_frame_time.elapsed().as_secs_f64());
                    last_frame_time = Instant::now();
                }
                Event::RedrawRequested(_) => {
                    debug!("Full RedrawRequested");
                    gfx_root.invalidate();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    gfx_root
                        .as_mut()
                        .on_window_event(&window_events::Event::CloseRequested);
                    //gfx_root.on_window_event(&event, );
                    debug!("Exit from loop REQUESTED");
                    debug!("Window refcount: {}", Rc::strong_count(&window));
                    exit_handler();
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent { event, window_id } if window_id == window.id() => {
                    debug!("Unhandled WindowEvent {:?}", event);
                    //gfx_root.on_window_event(&event);
                }
                _ => (),
            }
        });
    }
}
