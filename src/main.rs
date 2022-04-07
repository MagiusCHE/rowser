#![feature(type_name_of_val)]
use clap::Parser;

//use url::{ParseError, Url};

/// Full browser experience
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    //URL to browse
    #[clap(default_value_t)]
    url: String,
}

#[allow(unused_imports)]
use log::{debug, error, info, warn};

//#[path = "core/platform_window/mod.rs"]
//mod platform_window;
mod core;
use crate::core::platform_window::MainWindow;
use mtree::*;

fn main() {
    let tree:Tree<i32> = Tree::new(); 
    env_logger::init();
    let args = Cli::parse();
    /*info!("This is an Info!");
    warn!("This is a Warning!");
    error!("This is an Error!");*/
    debug!("Args are {:?}", args);
    //let url = Url::parse(&args.url).map_err(|err| CriticalError(format!("{:?}", err)))?;
    //info!("Url {:?}", url.as_str());
    let main_window = MainWindow {};

    main_window.run();
    
    debug!("Application exit");
}

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    //#[test]    
    fn helper_1() {
        let hexstrcolor = "0FFF";
        let u = u8::from_str_radix(&hexstrcolor[2..4], 16);        
        assert!(u.is_ok());
        let val = u.unwrap();
        assert!(val==255,"Expected {}, found {}", 255,val );
    }
}