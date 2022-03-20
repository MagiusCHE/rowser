#![allow(dead_code)]
#![allow(unused_imports)]
mod gbuffer;
pub struct MainWindow {}

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::{ptr::NonNull, rc::Rc};

use gbuffer::GfBuffer;

use log::{debug, error, info, warn};

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

        let mut gfx_buffer = GfBuffer::new(window.clone());
        info!("Begin loop {:?}", event_loop);
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    window_id: _,
                } => {
                    gfx_buffer.resize();
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
                }
                Event::RedrawRequested(_) => {
                    info!("RedrawRequested");
                    if gfx_buffer.draw().is_err() {
                        error!("Error while drawing");
                        *control_flow = ControlFlow::Exit;
                    }
                    //let window_size = window.inner_size();
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
                _ => (),
            }
        });
    }
}
