#![allow(unused)]
#![deny(elided_lifetimes_in_paths)]
use clap::Parser;

#[macro_use]
extern crate throw;

use url::{ParseError, Url};

/// Full browser experience
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    //URL to browse
    #[clap(default_value_t)]
    url: String,
}

use log::{error, info, warn};

#[derive(Debug)]
struct CriticalError(String);

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod gbuffer;

use gbuffer::GfBuffer;

fn main() {
    env_logger::init();
    let args = Cli::parse();
    /*info!("This is an Info!");
    warn!("This is a Warning!");
    error!("This is an Error!");*/
    info!("Args are {:?}", args);
    //let url = Url::parse(&args.url).map_err(|err| CriticalError(format!("{:?}", err)))?;
    //info!("Url {:?}", url.as_str());

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rowser")
        .build(&event_loop)
        .unwrap();

    let mut gfx_buffer = GfBuffer::new(&window);
    info!("Begin loop");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                window_id,
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
            Event::RedrawRequested(windowid) => {
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
                info!("Exit from loop REQUESTED");
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });

    info!("Exit from loop {:?}", window);
    println!("Exit from loop");
}
