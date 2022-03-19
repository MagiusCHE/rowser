use pixels::raw_window_handle::HasRawWindowHandle;
use pixels::{Error, Pixels, SurfaceTexture};

use std::cell::RefCell;

#[derive(Debug)]
pub struct GfBuffer<'win> {
    surface_texture: &'win RefCell<SurfaceTexture<'win, winit::window::Window>>,
    pixels: &'win RefCell<Pixels>,
    window: &'win winit::window::Window,
}
impl<'win> GfBuffer<'win> {
    pub fn new(window: &'win winit::window::Window) -> Self {
        let window_size = window.inner_size();
        let surface_texture = RefCell::new(SurfaceTexture::new(window_size.width, window_size.height, window));
        let pixels = RefCell::new(Pixels::new(window_size.width, window_size.height, surface_texture.into_inner()).unwrap());
        Self {
            window: window,
            surface_texture: &surface_texture,
            pixels: &pixels,
        }
    }
    pub fn resize(&mut self, window: &winit::window::Window) {
        let window_size = window.inner_size();
        self.pixels.borrow()
            .resize_surface(window_size.width, window_size.height);
        self.pixels.borrow()
            .resize_buffer(window_size.width, window_size.height);
    }
    pub fn draw(&mut self) -> Result<(), Error> {
        let frame = self.pixels.borrow().get_frame();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % 320 as usize) as i16;
            let y = (i / 320 as usize) as i16;

            let inside = x >= 10 && x < 110 && y > 20 && y < 120;

            let rgba = if inside {
                [0x5e, 0x99, 0x39, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }

        self.pixels.borrow().render()
    }
}
