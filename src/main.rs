#![allow(unused)]
use clap::Parser;

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

fn main() -> Result<(), CriticalError> {
    env_logger::init();
    let args = Cli::parse();
    /*info!("This is an Info!");
    warn!("This is a Warning!");
    error!("This is an Error!");*/
    info!("Args are {:?}", args);
    
    //let url = Url::parse(&args.url).map_err(|err| CriticalError(format!("{:?}", err)))?;
    //info!("Url {:?}", url.as_str());

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });

    Ok(())
}
