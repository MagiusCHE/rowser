pub(crate) mod color;
pub(crate) mod dom;
pub(crate) mod geometry;
pub(crate) mod gfxbuffer;
pub mod platform_window;
pub(crate) mod simple_error;


#[macro_export]
macro_rules! fixme {
    ($a:expr) => {
        warn!("Fixme({}): {}:{},{}", $a, file!(), line!(), column!())
    };
    () => {
        warn!("Fixme: {}:{},{}", file!(), line!(), column!())
    };
}
