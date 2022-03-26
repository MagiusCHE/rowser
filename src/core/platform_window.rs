#![allow(dead_code)]
#![allow(unused_imports)]

pub struct MainWindow {}

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::{error::Error, ptr::NonNull, rc::Rc};

use super::dom::{DomElement, RootDomElement, PaintableDomElement};

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
        let mut gfx_root = crate::core::dom::create_dom_element(None,  crate::core::dom::DomElementType::Root(window.clone()));

        //let mut sampl_elem = dom::create_dom_element(Some(&gfx_root),dom::DomElementType::Span);

        //let mut gfx_buffer = GfBuffer::new(window.clone());
        info!("Begin loop {:?}", event_loop);
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    window_id: _,
                } => {
                    gfx_root.on_window_resize();
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
                    info!("Full RedrawRequested");
                    gfx_root.invalidate();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    debug!("Exit from loop REQUESTED");
                    debug!("Window refcount: {}", Rc::strong_count(&window));
                    exit_handler();
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: _,
                    window_id: _,
                } => {
                    //debug!("Event::WindowEvent {:?}", event);
                }
                _ => (),
            }
        });
    }
}
