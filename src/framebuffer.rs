// src/framebuffer.rs
use image::GenericImageView;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn load_texture(path: &str) -> Result<Texture, String> {
    let img = image::open(path).map_err(|_| "Error cargando la textura".to_string())?;
    let (width, height) = img.dimensions();
    let texture = Texture {
        width,
        height,
        data: img.to_rgba8().into_raw(),
    };
    Ok(texture)
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }
}