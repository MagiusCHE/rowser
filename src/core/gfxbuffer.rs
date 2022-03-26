#![allow(unused_imports)]
//use pixels::raw_window_handle::HasRawWindowHandle;

use log::{debug, error, info, warn};

use pixels::{Pixels, SurfaceTexture};

use super::simple_error::SimpleError;

use std::rc::Rc;
//use std::cell::RefCell;

use super::geometry::{Rect, Size};

#[derive(Debug)]
pub struct GfxBuffer {
    //surface_texture: &'a SurfaceTexture<'a, winit::window::Window>,
    pixels: Pixels,
    window: Rc<winit::window::Window>,
    window_size: Size,
}
use super::color::Color;

impl GfxBuffer {
    pub fn new(window: Rc<winit::window::Window>) -> Self {
        let window_size = window.inner_size();
        
        assert!(
            window_size.width < u16::MAX as u32 || window_size.height < u16::MAX as u32,
            "window_size({:?}) > {}",
            window_size,
            u16::MAX
        );

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        let pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

        Self {
            window: window,
            //surface_texture: &surface_texture,
            pixels: pixels,
            window_size: Size {
                width: window_size.width as u16,
                height: window_size.height as u16,
            },
        }
    }
    pub fn resize(&mut self) {
        let window_size = self.window.inner_size();

        assert!(
            window_size.width < u16::MAX as u32 || window_size.height < u16::MAX as u32,
            "window_size({:?}) > {}",
            window_size,
            u16::MAX
        );

        self.window_size = Size {
            width: window_size.width as u16,
            height: window_size.height as u16,
        };

        self.pixels
            .resize_surface(window_size.width, window_size.height);
        self.pixels
            .resize_buffer(window_size.width, window_size.height);
    }
    pub fn render(&self) -> Result<(), SimpleError> {
        let ret = self.pixels.render();
        if ret.is_err() {
            return Err(SimpleError::new(format!("{}", ret.unwrap_err()).as_str()));
        }
        Ok(ret.unwrap())
    }
    pub fn clear(&mut self, color: Color) {
        let frame = self.pixels.get_frame();
        for (_, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&color.as_u8_ref());
        }
    }
    #[allow(dead_code)]
    pub fn draw(&mut self) -> Result<(), SimpleError> {
        let frame = self.pixels.get_frame();
        let window_size = &self.window_size;

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % window_size.width as usize;
            let y = i / window_size.width as usize;

            let inside = x >= 10 && x < 110 && y > 20 && y < 120;

            let rgba = if inside {
                [0x5e, 0x99, 0x39, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }

        self.render()
    }
}
