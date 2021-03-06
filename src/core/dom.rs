#![allow(dead_code)]
#![allow(unused_imports)]

pub(crate) mod window_events;

use super::gfxbuffer::GfxBuffer;
use super::{geometry::*, gfxbuffer};
use core::fmt;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::{error::Error, fmt::Display};

use super::color::{Color, Colors};

use log::{debug, error, info, warn};

use mtree::*;

use crate::fixme;

use super::dom::window_events::*;

#[derive(Debug, Copy, Clone)]
enum DomElementType {
    Root,
    Span,
}

#[derive(Debug)]
struct DomElementChild {}

#[derive(Debug)]
struct DomElement {
    bounds: Rect,
    node_type: DomElementType,
    initialized: bool,
    paint_order: u32,
}

#[derive(Debug)]
pub struct DomRoot<'a> {
    tree: Tree<DomElement>,
    window: Option<&'a winit::window::Window>,
    gfx_buffer: GfxBuffer<'a>,
    invalidated_rects: Vec<Rect>,
    last_window_size: Size,
    redraw_requested_invoked: bool,
}

impl<'a> DomRoot<'a> {
    pub fn new(window: &'a winit::window::Window) -> Self {
        let wsize = window.inner_size();
        let mut tree = Tree::new();
        if let Err(err) = tree.add_node(TreeNodeType::Root, || DomElement {
            bounds: Rect::new(0.0, 0.0, wsize.width as f64, wsize.height as f64),
            node_type: DomElementType::Root,
            initialized: false,
            paint_order: 0,
        }) {
            panic!("Error while get root of tree {}", err);
        }

        let mut root = DomRoot {
            tree,
            last_window_size: Size {
                width: wsize.width as f64,
                height: wsize.height as f64,
            },
            window: Some(window),
            gfx_buffer: GfxBuffer::new(window.clone()),
            invalidated_rects: Vec::new(),
            redraw_requested_invoked:false
        };

        root.tree.get_node_mut(0).unwrap().init();

        root.load_initial_state();

        root
    }

    fn on_window_resize(&mut self) {
        let wsize = self.window.as_ref().unwrap().inner_size();
        debug!("Resize window to {}x{}", wsize.width, wsize.height);
        if self.last_window_size == (wsize.width, wsize.height) {
            debug!("Window size is the same, no need to resize");
            return; 
        }
        
        if let Err(err) = self.tree.get_node(0, |node, _| {
            node.bounds = Rect::new(0.0, 0.0, wsize.width as f64, wsize.height as f64);
            debug!("Resize root to {:?}", node.bounds);
            Ok(())
        }) {
            panic!("Error while get root of tree {}", err);
        }
        self.gfx_buffer.resize();

        self.last_window_size = Size {
            width: wsize.width as f64,
            height: wsize.height as f64,
        };
        //Smooth paint on resize
        self.invalidate_rect(&Rect::new(0.0, 0.0, wsize.width as f64, wsize.height as f64));
    }

    fn invalidate_rect(&mut self, rect: &Rect) {
        // limit rect to actual size.
        debug!("Invalidate rect {:?}", rect);
        let new_rect = self
            .tree
            .get_node(0, |node, _| {
                Ok(Rect::new(
                    rect.left().max(0.0).min(node.bounds.right()),
                    rect.top().max(0.0).min(node.bounds.bottom()),
                    rect.width().max(0.0).min(node.bounds.right() - rect.left()),
                    rect.height()
                        .max(0.0)
                        .min(node.bounds.bottom() - rect.top()),
                ))
            })
            .unwrap();
        debug!("Invalidate rebounded rect {:?}", new_rect);
        if new_rect.size == self.last_window_size {
            self.invalidated_rects.clear();
        }
        if !self.invalidated_rects.contains(&new_rect) {
            self.invalidated_rects.push(new_rect);
            self.invalidated_rects = Rect::update_region(&mut self.invalidated_rects);
        }
    }
    pub fn redraw_requested(&mut self) {
        //A full windows repain is invoked... can we trust it?
        let rect = self
            .tree
            .get_node(0, |node, _| Ok(node.bounds.clone()))
            .unwrap();        
        self.invalidate_rect(&rect);
    }

    fn paint_rect(&mut self, rect: &Rect) {
        debug!("Begin paint of {:?}", rect);
        let position = self.tree.get_node_mut(0).unwrap().bounds.position();

        //fixme!("Should check all nodes and arrange them using potizioning and z-index");
        if let Err(err) = self.tree.trasverse_sorted_children(
            0,
            |a: &DomElement, b: &DomElement| a.paint_order.cmp(&b.paint_order),
            &mut |node, _, _| {
                debug!("Paint {:?} on {:?}", rect, node.bounds);
                let rect = node.bounds.rebound(&rect.add_pos(&position));

                if !rect.is_empty() {
                    debug!("Paint rect after rebound {:?}", rect);
                    node.paint(&mut self.gfx_buffer, &rect);
                } else {
                    debug!("Skip rect after rebound {:?}", rect);
                }
                //self.paint_rect(index_in_tree, &rect);
            },
        ) {
            panic!("Error while tree.foreach_children {}", err);
        }

        let result = self.gfx_buffer.render();
        if result.is_err() {
            error!("Error while rendering on framebuffer: {:?}", result.err());
        }
    }
    fn paint(&mut self) {
        let mut someinvalidated = 0;

        let rects = std::mem::replace(&mut self.invalidated_rects, vec![]);

        for rect in rects {
            // Recoursivly paint rect on all children
            self.paint_rect(&rect);
            someinvalidated += 1;
        }

        if someinvalidated > 0 {
            debug!(
                "Painted {} rects. Remains: {}",
                someinvalidated,
                self.invalidated_rects.len()
            );
        }
    }

    pub fn on_frame(&mut self, _: f64) {
        //main Frame is here

        if self.invalidated_rects.len() > 0 {
            self.paint();
        }
    }

    fn load_initial_state(&mut self) {
        self.create_dom_element_at(DomElementType::Span, 0, Rect::new(30.0, 50.0, 47.0, 22.0));
    }

    fn create_dom_element_at(
        &mut self,
        node_type: DomElementType,
        parent_index_in_tree: usize,
        bounds: Rect,
    ) -> usize {
        let index_in_tree = self
            .tree
            .add_node(TreeNodeType::Child(parent_index_in_tree), || DomElement {
                bounds,
                node_type,
                initialized: false,
                paint_order: 0,
            })
            .unwrap();
        self.tree.get_node_mut(index_in_tree).unwrap().paint_order = self.tree.len() as u32;
        index_in_tree
    }
}

impl DomElement {
    fn paint(&self, gfx: &mut GfxBuffer, rect: &Rect) {
        match self.node_type {
            DomElementType::Root => {
                gfx.clear(rect, Colors::BLACK);
            }
            DomElementType::Span => {
                fixme!("Span Paint todo");
                gfx.clear(rect, Colors::WHITE);
            }
        }
    }
    fn init(&mut self) {
        debug!("Init {:?}", self);
        self.initialized = true;
    }
}
