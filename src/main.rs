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

mod platform_window;
use platform_window::MainWindow;

fn main() {
    env_logger::init();
    let args = Cli::parse();
    /*info!("This is an Info!");
    warn!("This is a Warning!");
    error!("This is an Error!");*/
    debug!("Args are {:?}", args);
    //let url = Url::parse(&args.url).map_err(|err| CriticalError(format!("{:?}", err)))?;
    //info!("Url {:?}", url.as_str());
    let main_window = MainWindow {};

    main_window.run(|| {
        debug!("Application exit");
    });
}
