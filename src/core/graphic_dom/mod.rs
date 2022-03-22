#![allow(dead_code)]
#![allow(unused_imports)]

#[path = "../geometry.rs"]
mod geometry;
use crate::core::geometry::{Bounds, Rect};

use log::{debug, error, info, warn};

#[path = "../color.rs"]
mod color;
use crate::core::color::{Color, Colors};

pub trait PaintableGfxDomElement {
    /// Execute pating on invalidated rect
    fn paint(&self, rect: &Rect, gfx: &RefCell<GfxBuffer>);
    fn contains_rect(&self, rect: &Rect) -> bool {
        false
    }
    fn get_bounds(&self) -> Bounds;
}

pub struct GfxDomElement {
    dimension: Rect,
}

impl PaintableGfxDomElement for GfxDomElement {
    fn get_bounds(&self) -> Bounds {
        self.dimension.get_bounds()
    }
    fn paint(&self, rect: &Rect, gfx: &RefCell<GfxBuffer>) {
        GfxDomElement::paint_common( self, rect, gfx); 
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[path = "../gfxbuffer.rs"]
mod gfxbuffer;
use gfxbuffer::GfxBuffer;

pub struct GfxRoot {
    window: Rc<winit::window::Window>,
    dimension: Rect,
    invalidated_rects: RefCell<Vec<Rect>>,
    children: RefCell<Vec<GfxDomElement>>,
    gfx_buffer: RefCell<GfxBuffer>,
}

impl PaintableGfxDomElement for GfxRoot {
    fn get_bounds(&self) -> Bounds {
        self.dimension.get_bounds()
    }
    fn paint(&self, rect: &Rect, gfx: &RefCell<GfxBuffer>) {
        debug!("Begin pain of {:?}",rect);
        gfx.borrow_mut().clear(Colors::BLACK);
        GfxDomElement::paint_common(self, rect, gfx);
        self.children.borrow().iter().for_each(|child| {
            if child.contains_rect(rect) {
                GfxDomElement::paint(&child, rect, gfx);
            }
        });
    }
    fn contains_rect(&self, rect: &Rect) -> bool {
        let wsize = self.window.inner_size();
        rect.left() >= 0
            && rect.left() < wsize.width as i32
            && rect.top() >= 0
            && rect.top() < wsize.height as i32
    }
}

impl GfxRoot {
    pub fn new(window: Rc<winit::window::Window>) -> Self {
        let wsize = window.inner_size();
        let ret = Self {
            window: window.clone(),
            dimension: Rect::new(0, 0, wsize.width as u16, wsize.height as u16),
            invalidated_rects: RefCell::new(Vec::new()),
            children: RefCell::new(Vec::new()),
            gfx_buffer: RefCell::new(GfxBuffer::new(window.clone())),
        };
        //ret.gfx_buffer.clear(Color::BLACK);
        ret
    }
    pub fn on_frame(&self, _: f64) {
        //main Frame is here
        //check is there are invalidated rects
        let mut someinvalidated = 0;
        loop {
            match self.invalidated_rects.borrow_mut().pop() {
                None => break,
                Some(rect) => {
                    self.paint(&rect, &self.gfx_buffer);
                    someinvalidated += 1;
                }
            }
        }
        if someinvalidated > 0 {
            debug!("Painted {} rects.", someinvalidated);
        }
    }
    pub fn on_window_resize(&mut self) {
        let wsize = self.window.inner_size();
        self.dimension = Rect::new(0, 0, wsize.width as u16, wsize.height as u16);
        self.gfx_buffer.borrow_mut().resize();
    }
    pub fn invalidate(&self) {
        debug!("invalidate");
        self.invalidated_rects
            .borrow_mut()
            .push(self.dimension.clone());
    }
}

impl GfxDomElement {
    fn paint_common(_elem: &dyn PaintableGfxDomElement, _rect: &Rect, gfx: &RefCell<GfxBuffer>) {
        gfx.borrow_mut().render();
    }
}
