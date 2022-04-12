#![allow(dead_code)]
#![allow(unused_imports)]

use log::{debug, error, info, warn};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Position {
    pub left: f64,
    pub top: f64,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Limits {
    pub right: f64,
    pub bottom: f64,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Bounds {
    pub begin: Position,
    pub end: Limits,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Rect {
    pub position: Position,
    pub size: Size,
}

use std::{cmp::Eq, fmt, ops};

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
    pub fn from_u32(left: u32, top: u32, width: u32, height: u32) -> Rect {
        Rect {
            position: Position {
                left: left as f64,
                top: top as f64,
            },
            size: Size {
                width: width as f64,
                height: height as f64,
            },
        }
    }
    pub fn get_bounds(&self) -> Bounds {
        Bounds {
            begin: self.position(),
            end: self.limits(),
        }
    }

    pub fn rebound(&self, tocheck: &Rect) -> Self {
        if self == tocheck {
            return self.clone();
        }
        Self {
            position: Position {
                left: self.left().max(tocheck.left()),
                top: self.top().max(tocheck.top()),
            },
            size: Size {
                width: self.width().min(tocheck.right() - self.left()).max(0.0),
                height: self.height().min(tocheck.height() - self.top()).max(0.0),
            },
        }
    }
    pub fn limits(&self) -> Limits {
        Limits {
            right: self.position.left + (self.size.width),
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
        self.position.left + self.size.width as f64
    }
    pub fn bottom(&self) -> f64 {
        self.position.top + self.size.height as f64
    }
    pub fn width(&self) -> f64 {
        self.size.width
    }
    pub fn height(&self) -> f64 {
        self.size.height
    }

    pub fn get_intersection_rect(&self, rect: &Rect) -> Option<Rect> {
        if self.intersect_rect(rect) {
            let ret = Rect {
                position: Position {
                    left: self.left().max(rect.left()),
                    top: self.top().max(rect.top()),
                },
                size: Size {
                    width: self.right().min(rect.right()) - self.left().max(rect.left()),
                    height: self.bottom().min(rect.bottom()) - self.top().max(rect.top()),
                },
            };
            if ret.is_empty() {
                return None;
            }
            Some(ret)
        } else {
            None
        }
    }
    pub fn intersect_rect(&self, rect: &Rect) -> bool {
        !(rect.left() > self.right()
            || rect.right() < self.left()
            || rect.top() > self.bottom()
            || rect.bottom() < self.top())
    }
    pub fn contains_point_f64(&self, x: f64, y: f64) -> bool {
        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }

    pub fn contains_point(&self, pt: &Position) -> bool {
        self.contains_point_f64(pt.left, pt.top)
    }
    pub fn contains_point_u32(&self, x: u32, y: u32) -> bool {
        self.contains_point_f64(x as f64, y as f64)
    }

    pub fn add_pos(&self, pos: &Position) -> Rect {
        Rect {
            position: Position {
                left: self.left() + pos.left,
                top: self.top() + pos.top,
            },
            size: self.size.clone(),
        }
    }
    pub fn sub_pos(&self, pos: &Position) -> Rect {
        Rect {
            position: Position {
                left: self.left() - pos.left,
                top: self.top() - pos.top,
            },
            size: self.size.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size.width == 0.0 || self.size.height == 0.0
    }

    /*
    -------------------------
    |          A            |
    |                       |
    |-----------------------|
    |  B  |   hole    |  C  |
    |-----------------------|
    |                       |
    |          D            |
    -------------------------
    */
    pub fn subtract_rect(&self, rect: &Rect) -> Vec<Rect> {
        // Subtract rectangle from self and returns a list of non overlapping rectangles
        //  rectangle clipping algorithm
        let mut region: Vec<Rect> = vec![];

        if let Some(intersection) = rect.get_intersection_rect(self) {
            //A
            let height_a = intersection.top() - self.top();
            if height_a > 0.0 {
                region.push(Rect::new(self.left(), self.top(), self.width(), height_a));
            }
            //B
            let width_b = intersection.left() - self.left();
            if width_b > 0.0 {
                region.push(Rect::new(
                    self.left(),
                    intersection.top(),
                    width_b,
                    intersection.height(),
                ));
            }
            //C
            let width_c = self.right() - intersection.right();
            if width_c > 0.0 {
                region.push(Rect::new(
                    intersection.right(),
                    intersection.top(),
                    width_c,
                    intersection.height(),
                ));
            }
            //D
            let height_d = self.bottom() - intersection.bottom();
            if height_d > 0.0 {
                region.push(Rect::new(
                    self.left(),
                    intersection.bottom(),
                    self.width(),
                    height_d,
                ));
            }
        }

        region
    }

    pub fn union(&self, r2: &Self) -> Self {
        Self {
            position: Position {
                left: self.left().min(r2.left()),
                top: self.top().min(r2.top()),
            },
            size: Size {
                width: if self.width() > r2.width() {
                    self.width()
                } else {
                    r2.width() + (r2.left() - self.left())
                },
                height: if self.height() > r2.height() {
                    self.height()
                } else {
                    r2.height() + (r2.top() - self.top())
                },
            },
        }
    }

    /// Returns a list of non overlapping rectangles.
    ///   For performance reasons, the input vector will be modified and become invalid.
    pub fn update_region(region: &mut Vec<Rect>) -> Vec<Rect> {
        //search for intersaction between rectangles
        let mut new_region: Vec<Rect> = vec![];
        let mut i = 0;
        while i < region.len() {
            //println!("{} < {}",i,region.len());
            if new_region.is_empty() {
                new_region.push(region[i]);
            } else {
                /*
                ####
                ####
                ##òòòò
                ##òòòò
                  òòòò
                  òòòò
                */
                let mut modified = false;
                for j in 0..new_region.len() {
                    if let Some(intersection) = region[i].get_intersection_rect(&new_region[j]) {
                        region.append(&mut region[i].subtract_rect(&new_region[j]));

                        //println!("Intersect: {:?} with {:?}", new_region[j],region[i]);
                        //println!("Intersection: {:?}", intersection);
                        //new_region[j] = region[i];
                        //println!("Region: {:?}", region);
                        //println!("New Region: {:?}", new_region);
                        //if region.len() > 4 {
                        //    panic!();
                        //}
                        // Original region is now modified.
                        //   Subtracted portion is added to the end.
                        //   Only the intersection is added to the new region.
                        modified = true;
                    }
                }
                if !modified {
                    new_region.push(region[i]);
                }
            }
            i += 1;
        }
        new_region
    }
}

impl PartialEq<(u32, u32, u32, u32)> for Rect {
    fn eq(&self, other: &(u32, u32, u32, u32)) -> bool {
        self.left() == other.0 as f64
            && self.top() == other.1 as f64
            && self.width() == other.2 as f64
            && self.height() == other.3 as f64
    }
}

impl PartialEq<(i32, i32, i32, i32)> for Rect {
    fn eq(&self, other: &(i32, i32, i32, i32)) -> bool {
        self.left() == other.0 as f64
            && self.top() == other.1 as f64
            && self.width() == other.2 as f64
            && self.height() == other.3 as f64
    }
}

impl PartialEq<(f64, f64, u32, u32)> for Rect {
    fn eq(&self, other: &(f64, f64, u32, u32)) -> bool {
        self.left() == other.0
            && self.top() == other.1
            && self.width() == other.2 as f64
            && self.height() == other.3 as f64
    }
}

impl ops::Add<Position> for Rect {
    type Output = Self;

    fn add(self, src: Position) -> Self {
        Rect {
            position: Position {
                left: self.left() + src.left,
                top: self.top() + src.top,
            },
            size: self.size,
        }
    }
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Self { width, height }
    }
}

impl PartialEq<(u32, u32)> for Size {
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.width == other.0 as f64 && self.height == other.1 as f64
    }
}
/*
impl PartialEq<Size> for Size {
    fn eq(&self, other: &Size) -> bool {
        self.width == other.width && self.height == other.height
    }
}*/

pub fn print_region_ascii(region: &Vec<Rect>) {
    //Find bounds on the region
    let mut bounds = Rect::new(0.0, 0.0, 0.0, 0.0);

    region.iter().for_each(|r| {
        bounds = bounds.union(r);
    });
    //println!("Bounds {:?}", bounds);
    let mut buffer = vec![' '; ((bounds.width() + 1.0) * bounds.height()) as usize];

    let mut char = ('A' as u8 - 1) as char;
    region.iter().for_each(|r| {
        char = (char as u8 + 1) as char;
        for y in 0..bounds.height() as u32 {
            for x in 0..bounds.width() as u32 {
                if r.contains_point_f64(x as f64, y as f64) {
                    buffer[(x + y * (1+bounds.width() as u32)) as usize] = char;
                }
            }
        }
    });
    for y in 0..bounds.height() as u32 {
        buffer[(bounds.width() as u32 + y * (1 + bounds.width() as u32)) as usize] = '\n';
    }
    let s: String = buffer.into_iter().collect();

    println!("{}", s.as_str());
}

#[cfg(test)]
mod tests {
    use assert2::check;

    use super::*;

    #[test]
    fn get_intersection() {
        let r1 = Rect::from_u32(0, 0, 10, 10);
        let r2 = Rect::from_u32(2, 2, 6, 6);
        let result = r1.get_intersection_rect(&r2).unwrap();
        println!("{:?} intersect {:?} = {:?}", r1, r2, result);
        check!(result == (2, 2, 6, 6));
    }
    #[test]
    fn subtract_rect() {
        let r1 = Rect::from_u32(0, 0, 10, 10);
        let r2 = Rect::from_u32(2, 2, 6, 6);
        let result = r1.subtract_rect(&r2);
        println!("{:?} intersect {:?} = {:#?}", r1, r2, result);
        check!(result.len() == 4);
        check!(result[0] == (0, 0, 10, 2));
        check!(result[1] == (0, 2, 2, 6));
        check!(result[2] == (8, 2, 2, 6));
        check!(result[3] == (0, 8, 10, 2));
    }
    #[test]
    fn subtract_rect_2() {
        let r1 = Rect::from_u32(0, 0, 4, 4);
        let r2 = Rect::from_u32(2, 2, 4, 4);
        let result = r1.subtract_rect(&r2);
        println!("{:?} intersect {:?} = {:#?}", r1, r2, result);
        check!(result.len() == 2);
        check!(result[0] == (0, 0, 4, 2));
        check!(result[1] == (0, 2, 2, 2));
    }
    #[test]
    fn update_region() {
        let mut region: Vec<Rect> = Vec::new();
        region.push(Rect::from_u32(0, 0, 4, 4));
        region.push(Rect::from_u32(2, 2, 4, 4));
        region.push(Rect::from_u32(0, 2, 3, 3));

        println!("Was {:?}", &region);
        print_region_ascii(&region);

        let new_region = Rect::update_region(&mut region);

        println!("Updated {:?}", &new_region);

        print_region_ascii(&new_region);

        check!(new_region.len() == 5);
    }
}
