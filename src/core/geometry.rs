#![allow(dead_code)]
#![allow(unused_imports)]

use log::{debug, error, info, warn};

#[derive(Debug, Clone)]
pub struct Position {
    pub left: f64,
    pub top: f64,
}

#[derive(Debug, Clone)]
pub struct Limits {
    pub right: f64,
    pub bottom: f64,
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub begin: Position,
    pub end: Limits,
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct Rect {
    pub position: Position,
    pub size: Size,
}

use std::fmt;

impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(
            f,
            "(l:{}, t:{}, w:{}, h:{})",
            self.left(),
            self.top(),
            self.width(),
            self.height()
        )
    }
}

impl Rect {
    pub fn new(left: f64, top: f64, width: f64, height: f64) -> Rect {
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
            right: self.position.left + (self.size.width ),
            bottom: self.position.top + (self.size.height),
        }
    }
    pub fn position(&self) -> Position {
        Position {
            left: self.left(),
            top: self.top(),
        }
    }
    pub fn left(&self) -> f64 {
        self.position.left
    }
    pub fn top(&self) -> f64 {
        self.position.top
    }
    pub fn right(&self) -> f64 {
        self.position.top + self.size.height as f64
    }
    pub fn bottom(&self) -> f64 {
        self.position.left + self.size.width as f64
    }
    pub fn width(&self) -> f64 {
        self.size.height
    }
    pub fn height(&self) -> f64 {
        self.size.width
    }

    pub fn intersect_rect(&self, rect: &Rect) -> bool {
        !(rect.left() > self.right()
            || rect.right() < self.left()
            || rect.top() > self.bottom()
            || rect.bottom() < self.top())
    }
    pub fn contains_xy(&self, x: f64, y: f64) -> bool {
        self.left() < x && x < self.right() && self.top() < y && y < self.bottom()
    }

    pub fn contains_point(&self, pt: &Position) -> bool {
        self.contains_xy(pt.left, pt.top)
    }
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Self { width, height }
    }
}
