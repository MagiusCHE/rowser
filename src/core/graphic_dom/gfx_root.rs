#[path = "../gfxbuffer.rs"]
mod gfxbuffer;
use crate::core::gfxbuffer::GfxBuffer;

#[path = "../geometry.rs"]
mod geometry;
use crate::core::geometry::{Bounds, Rect};

use log::{debug, error, info, warn};

#[path = "../color.rs"]
mod color;
use crate::core::color::{Color, Colors};

#[path = "./gfx_elem.rs"]
mod gfx_elem;

use crate::core::graphic_dom::gfx_elem::{PaintableGfxDomElement,GfxDomElement,DomElement};

use std::cell::RefCell;
use std::rc::Rc;

pub struct GfxRoot<'a> {
    window: Rc<winit::window::Window>,
    dimension: Rect,
    invalidated_rects: RefCell<Vec<Rect>>,
    children: RefCell<Vec<GfxDomElement<'a>>>,
    gfx_buffer: RefCell<GfxBuffer>,
}

impl<'a> PaintableGfxDomElement for GfxRoot<'a> {
    fn get_dimension(&self) -> &Rect {
        &self.dimension
    }
    fn paint(&self, rect: &Rect, gfx: &RefCell<GfxBuffer>) {
        debug!("Begin pain of {:?}", rect);
        gfx.borrow_mut().clear(Colors::BLACK);
        GfxDomElement::paint_common(self, rect, gfx);
        self.children.borrow().iter().for_each(|child| {
            if child.intersect_rect(rect) {
                GfxDomElement::paint(&child, rect, gfx);
            }
        });
    }
}
impl<'a> DomElement for GfxRoot<'a> {}

impl<'a> GfxRoot<'a> {
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