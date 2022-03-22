#![allow(unused_imports)]
#![allow(dead_code)]

use log::{debug, error, info, warn};

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_u8_arr(u8arr: &[u8; 4]) -> Color {
        Color {
            r: u8arr[0],
            g: u8arr[1],
            b: u8arr[2],
            a: u8arr[3],
        }
    }

    pub fn from_str(hexstrcolor: &str) -> Color {
        let mut u8arr: [u8; 4] = [0, 0, 0, 0];
        //let mut err = false;
        (0..hexstrcolor.len()).step_by(2).for_each(|i| {
            let u = u8::from_str_radix(&hexstrcolor[i..i + 2], 16);

            assert!(
                u.is_ok(),
                "Error while parsing object {:?} at position {} of array {:?}",
                u,
                i,
                hexstrcolor
            );
            u8arr[i / 2] = u.unwrap_or(0);
        });
        debug!("Color from {} is {:?}", hexstrcolor, u8arr);
        /*if err {
            return Err(SimpleError::new(format!("Invalid hexadecimal string '{hexstrcolor}'. Expected something like '23F4AAFF'.").as_str()));
        }*/
        //let hex = String::from(hexstrcolor);

        Color::from_u8_arr(&u8arr)
    }

    pub fn as_u8_ref(&self) -> [u8;4] {
        [self.r, self.g, self.b, self.a]
    }
}

pub struct Colors {}
impl Colors {
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
}

#[cfg(test)]
mod tests {
    //use std::borrow::Borrow;

    use super::Color;
    #[test]
    fn color_parsing() {
        let color = Color::from_str("3422FFFE");
        assert!(color.r == 52, "Red '{}' != 52", color.r);
        assert!(color.g == 34, "Red '{}' != 34", color.g);
        assert!(color.b == 255, "Red '{}' != 52", color.b);
        assert!(color.a == 254, "Red '{}' != 52", color.a);
    }
}
