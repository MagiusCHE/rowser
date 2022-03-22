#![allow(dead_code)]
#![allow(unused_imports)]

use log::{debug, error, info, warn};

#[derive(Debug,Clone)]
pub struct Position {
    pub left: i32,
    pub top: i32,
}

#[derive(Debug,Clone)]
pub struct Limits {
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug,Clone)]
pub struct Bounds {
    pub begin: Position,
    pub end: Limits,
}

#[derive(Debug,Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug,Clone)]
pub struct Rect {
    pub position: Position,
    pub size: Size,
}

impl Rect {
    pub fn new(left: i32, top: i32, width: u16, height: u16) -> Rect {
        Rect {
            position: Position { left, top },
            size: Size { width, height },
        }
    }
    pub fn get_bounds(&self) -> Bounds {
        Bounds {
            begin: self.position(),
            end: self.limits(),
        }
    }
    pub fn limits(&self) -> Limits {
        Limits {
            right: self.position.left + (self.size.width as i32),
            bottom: self.position.top + (self.size.height as i32),
        }
    }
    pub fn position(&self) -> Position {
        Position {
            left: self.left(),
            top: self.top(),
        }
    }
    pub fn left(&self) -> i32 {
        self.position.left
    }
    pub fn top(&self) -> i32 {
        self.position.top
    }
    pub fn right(&self) -> i32 {
        self.position.top + self.size.height as i32
    }
    pub fn bottom(&self) -> i32 {
        self.position.left + self.size.width as i32
    }
}
impl Size {
    pub fn new(width: u16, height: u16) -> Size {
        Self { width, height }
    }
}
