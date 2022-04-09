#![allow(dead_code)]
#![allow(unused_imports)]
use core::fmt;
use std::{error::Error, fmt::Display};

use log::{debug, error, info, warn};

use mtree::*;

use crate::fixme;

use super::dom::window_events::Event;

#[derive(Debug)]
pub enum DomError{
    TreeManipulationError(TreeError),
    GenericError
}

impl From<TreeError> for DomError{
    fn from(error: TreeError) -> Self{
        DomError::TreeManipulationError(error)
    }
}

impl Display for DomError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomError::TreeManipulationError(error) => write!(f, "{}{:?}", self, error),
            DomError::GenericError => write!(f, "{}", self),
        }
    }
}

impl Error for DomError{
    fn source(&self) -> Option<&(dyn Error + 'static)>{
        Some(self)
    }
}

#[derive(Debug)]
struct DomElement {}
pub struct DomRoot {
    tree: Tree<DomElement>,
    window: winit::window::Window,
}

impl DomRoot {
    pub fn new(window: winit::window::Window) -> Result<Self,DomError> {
        let mut tree: Tree<DomElement> = Tree::new();
        tree.add_node(TreeNodeType::Root, || DomElement {})?;
        Ok(DomRoot {
            tree,
            window,
        })
    }
    pub fn on_window_event(&mut self, event: &Event){
        fixme!();
    }
    pub fn on_frame(&mut self, delta: f64){
        fixme!();
    }
    pub fn invalidate(&mut self){
        fixme!();
    }
}


