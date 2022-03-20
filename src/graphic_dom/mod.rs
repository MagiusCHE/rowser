#![allow(dead_code)]
#![allow(unused_imports)]

use log::{debug, error, info, warn};

pub struct Position {
    left: i32,
    top: i32,
}

pub struct Limit {
    right: i32,
    bottom: i32,
}

pub struct Boundary {
    begin: Position,
    end: Limit,
}

pub struct Size {
    width: u16,
    height: u16,
}

pub struct Rect {
    position: Position,
    size: Size,
}

impl Rect {
    pub fn get_bounds(&self) -> Boundary {
        Boundary {
            begin: Position{
                left: self.position.left,
                top: self.position.top,
            },
            end: Limit {
                right: self.position.left + (self.size.width as i32),
                bottom: self.position.top + (self.size.height as i32),
            },
        }
    }
}

pub trait TGfxDomElement {
    fn on_frame(&self, delta: f64) {}
}

pub struct GfxDomElement {
    rect: Rect,
}

impl GfxDomElement {}

use std::rc::Rc;
pub struct GfxRoot {
    window: Rc<winit::window::Window>,
}

impl TGfxDomElement for GfxRoot {
    fn on_frame(&self, delta: f64) {}
}

impl TGfxDomElement for GfxDomElement {
    fn on_frame(&self, delta: f64) {
        //GfxDomElement::common_on_frame(self, delta)
    }
}

impl GfxRoot {
    pub fn new(window: Rc<winit::window::Window>) -> Self {
        Self { window: window }
    }
    pub fn on_window_resize(&self) {
        debug!("on_window_resize");
    }
    pub fn on_full_redraw_requested(&self) {
        debug!("on_full_redraw_requested");
        //TGfxDomElement::invalidate_rect()
    }
}
