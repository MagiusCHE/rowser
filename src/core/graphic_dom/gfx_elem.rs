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

use std::cell::RefCell;
use std::rc::Rc;

pub fn CreateDomElement(parent: &dyn DomElement) -> GfxDomElement {
    GfxDomElement {
        dimension: Rect::new(10, 10, 100, 100),
        parent: Rc::new(parent)
    }
}
pub trait DomElement {}
pub trait PaintableGfxDomElement: DomElement {
    /// Execute pating on invalidated rect
    fn paint(&self, rect: &Rect, gfx: &RefCell<GfxBuffer>);
    fn intersect_rect(&self, rect: &Rect) -> bool {
        self.get_dimension().intersect_rect(&rect)
    }
    fn get_dimension(&self) -> &Rect;
    fn get_bounds(&self) -> Bounds {
        self.get_dimension().get_bounds()
    }
}

pub struct GfxDomElement<'a> {
    dimension: Rect,
    parent: Rc<&'a dyn DomElement>
}

impl<'a> PaintableGfxDomElement for GfxDomElement<'a> {
    fn get_dimension(&self) -> &Rect {
        &self.dimension
    }
    fn paint(&self, rect: &Rect, gfx: &RefCell<GfxBuffer>) {
        GfxDomElement::paint_common(self, rect, gfx);
    }
}

impl<'a> DomElement for GfxDomElement<'a> {}

impl<'a> GfxDomElement<'a> {
    pub fn paint_common(
        _elem: &dyn PaintableGfxDomElement,
        _rect: &Rect,
        gfx: &RefCell<GfxBuffer>,
    ) {
        let result = gfx.borrow_mut().render();
        if result.is_err() {
            error!("Error while rendering on framebuffer: {:?}", result.err());
        }
    }
}
