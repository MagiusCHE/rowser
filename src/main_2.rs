use log::{error, info, warn};


struct pippo<'s>{
    val: &'s String
}

fn main() {
    env_logger::init();

    let a = String::from("Hello");

    let c = pippo { val: &a } ;

    let b = &a;

    info!("OK");
}
