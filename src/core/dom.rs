#![allow(dead_code)]
#![allow(unused_imports)]

pub(crate) mod window_events;

use crate::fixme;

use super::geometry::*;
use super::gfxbuffer::GfxBuffer;

use log::{debug, error, info, warn};

use super::color::{Color, Colors};

use core::fmt;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::num::NonZeroU8;
use std::rc::Rc;

#[derive(Debug)]
pub enum DomElementType<'a> {
    Root(&'a winit::window::Window),
    //Literal,
    Span,
}

pub struct DomElement<'a> {
    node_type: DomElementType<'a>,
    dimension: Rect,
    parent: Option<&'a DomElement<'a>>,
    children: RefCell<Vec<DomElement<'a>>>,
    window: Option<&'a winit::window::Window>,
    invalidated_rects: Option<RefCell<Vec<Rect>>>,
    gfx_buffer: Option<RefCell<GfxBuffer<'a>>>,
    initialized: bool,
}

pub trait PaintableDomElement {
    fn paint(&self, rect: &Rect, gfx: &mut GfxBuffer);
    fn invalidate(&self);
}

pub trait BaseDomElement {
    fn init(&mut self);
}

impl<'a> BaseDomElement for DomElement<'a> {
    fn init(&mut self) {
        debug!("Initialized DomElement");
        self.initialized = true
    }
}

pub fn create_dom_element<'a>(
    parent: Option<&'a DomElement<'a>>,
    _type: DomElementType<'a>,
) -> DomElement<'a> {
    let mut elem = DomElement {
        initialized: false,
        node_type: _type,
        dimension: Rect::new(0.0, 0.0, 0.0, 0.0),
        parent: parent,
        children: RefCell::new(Vec::new()),
        //Theese are Root'a fields
        window: None,
        invalidated_rects: None,
        gfx_buffer: None,
    };
    match elem.node_type {
        DomElementType::Root(win) => {
            elem.window = Some(win);
            let wsize = win.inner_size();
            elem.dimension = Rect::new(0.0, 0.0, wsize.width as f64, wsize.height as f64);
            elem.invalidated_rects = Some(RefCell::new(Vec::new()));
            elem.gfx_buffer = Some(RefCell::new(GfxBuffer::new(win)));
            RootDomElement::init(&mut elem);
        }

        _ => {
            BaseDomElement::init(&mut elem);
        }
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
        debug!("invalidate {:?}", self.dimension);
        self.invalidated_rects
            .as_ref()
            .unwrap()
            .borrow_mut()
            .push(self.dimension.clone());
    }
}

pub trait RootDomElement<'a>: PaintableDomElement {
    fn paint(&self, rect: &Rect, gfx: &mut GfxBuffer);
    fn init(&mut self) {}
    fn on_frame(&self, _: f64);
    fn on_window_resize(&mut self);
}

impl<'a> RootDomElement<'a> for DomElement<'a> {
    fn paint(&self, rect: &Rect, gfx: &mut GfxBuffer) {
        debug!("Begin paint of {:?}", rect);
        if !self.intersect_rect(rect) {
            () //return!
        }
        gfx.clear(rect, Colors::BLACK);
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
        let mut rects = self.invalidated_rects.as_ref().unwrap().borrow_mut();

        rects.retain(|rect| {
            RootDomElement::paint(
                self,
                rect,
                &mut self.gfx_buffer.as_ref().unwrap().borrow_mut(),
            );
            someinvalidated += 1;
            false
        });
        if someinvalidated > 0 {
            debug!(
                "Painted {} rects. Remains: {}",
                someinvalidated,
                rects.len()
            );
        }
    }
    fn on_window_resize(&mut self) {
        let wsize = self.window.as_ref().unwrap().inner_size();
        self.dimension = Rect::new(0.0, 0.0, wsize.width as f64, wsize.height as f64);
        self.gfx_buffer.as_ref().unwrap().borrow_mut().resize();
    }
    fn init(&mut self) {
        debug!("Initialized ROOT");
    }
}
