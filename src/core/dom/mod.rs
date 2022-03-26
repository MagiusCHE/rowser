#![allow(dead_code)]
#![allow(unused_imports)]

use super::gfxbuffer::GfxBuffer;
use super::geometry::{Bounds, Rect};

use log::{debug, error, info, warn};

use super::color::{Color, Colors};

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub enum DomElementType {
    Root(Rc<winit::window::Window>),
    //Literal,
    Span,
}

pub struct DomElement<'a> {
    node_type: DomElementType,
    dimension: Rect,
    parent: Option<&'a DomElement<'a>>,
    children: RefCell<Vec<DomElement<'a>>>,
    window: Option<Rc<winit::window::Window>>,
    invalidated_rects: Option<RefCell<Vec<Rect>>>,
    gfx_buffer: Option<RefCell<&'a GfxBuffer>>,
}

pub trait PaintableDomElement {
    fn paint(&self, rect: &Rect, gfx: &mut GfxBuffer);
    fn invalidate(&self);
}

pub trait BaseDomElement {
    fn init(&self, _type: DomElementType);
}

impl<'a> BaseDomElement for DomElement<'a> {
    fn init(&self, _type: DomElementType) {}
}

pub fn create_dom_element<'a>(
    parent: Option<&'a DomElement<'a>>,
    _type: DomElementType,
) -> DomElement<'a> {
    let mut elem = DomElement {
        node_type: _type,
        dimension: Rect::new(0, 0, 0, 0),
        parent: parent,
        children: RefCell::new(Vec::new()),
        //Theese are Root'a fields
        window: None,
        invalidated_rects: None,
        gfx_buffer: None,
    };
    match _type {
        DomElementType::Root(win) => {
            elem.window = Some(win.clone());
            let wsize = win.inner_size();
            elem.dimension = Rect::new(0, 0, wsize.width as u16, wsize.height as u16);
            elem.invalidated_rects = Some(RefCell::new(Vec::new()));
            elem.gfx_buffer = Some(RefCell::new(&GfxBuffer::new(win.clone())));
            RootDomElement::init(&elem, _type);
        }

        DomElementType::Span => todo!(),
    }
    elem
}

impl<'a> DomElement<'a> {
    pub fn add_child(&self, child: DomElement<'a>) {
        self.children.borrow_mut().push(child);
    }
    pub fn intersect_rect(&self, rect: &Rect) -> bool {
        self.get_dimension().intersect_rect(&rect)
    }
    pub fn get_dimension(&self) -> &Rect {
        &self.dimension
    }
    pub fn get_bounds(&self) -> Bounds {
        self.get_dimension().get_bounds()
    }
}

fn paint_common(_: &dyn PaintableDomElement, _: &Rect, gfx: &mut GfxBuffer) {
    let result = gfx.render();
    if result.is_err() {
        error!("Error while rendering on framebuffer: {:?}", result.err());
    }
}

impl<'a> PaintableDomElement for DomElement<'a> {
    fn paint(&self, rect: &Rect, gfx: &mut GfxBuffer) {
        paint_common(self, rect, gfx);
    }
    fn invalidate(&self) {
        debug!("invalidate");
        self.invalidated_rects
            .unwrap()
            .borrow_mut()
            .push(self.dimension.clone());
    }
}

pub trait RootDomElement: PaintableDomElement {
    fn paint(&self, rect: &Rect, gfx: &mut GfxBuffer);
    fn init(&self, _type: DomElementType) {}
    fn on_frame(&self, _: f64);
    fn on_window_resize(&mut self);
}

impl<'a> RootDomElement for DomElement<'a> {
    fn paint(&self, rect: &Rect, gfx: &mut GfxBuffer) {
        debug!("Begin pain of {:?}", rect);
        gfx.clear(Colors::BLACK);
        paint_common(self, rect, gfx);
        self.children.borrow_mut().iter().for_each(|child| {
            if child.intersect_rect(rect) {
                paint_common(child, rect, gfx);
            }
        });
    }
    fn on_frame(&self, _: f64) {
        //main Frame is here
        //check is there are invalidated rects
        let mut someinvalidated = 0;
        self.invalidated_rects
            .unwrap()
            .borrow_mut()
            .iter()
            .for_each(|rect| {
                RootDomElement::paint(self, rect,
                    &mut self.gfx_buffer.unwrap().borrow_mut()
                );
                someinvalidated += 1;
            });
        if someinvalidated > 0 {
            debug!("Painted {} rects.", someinvalidated);
        }
    }
    fn on_window_resize(&mut self) {
        let wsize = self.window.unwrap().inner_size();
        self.dimension = Rect::new(0, 0, wsize.width as u16, wsize.height as u16);
        self.gfx_buffer.unwrap().borrow_mut().resize();
    }
}
