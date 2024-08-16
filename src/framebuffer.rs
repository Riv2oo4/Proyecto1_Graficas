
use image::GenericImageView;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
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

    pub fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let dx = (x1 as isize - x0 as isize).abs();
        let dy = -(y1 as isize - y0 as isize).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x0 = x0;
        let mut y0 = y0;

        loop {
            if x0 < self.width && y0 < self.height {
                self.point(x0, y0);
            }
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 = (x0 as isize + sx) as usize; }
            if e2 <= dx { err += dx; y0 = (y0 as isize + sy) as usize; }
        }
    }

    pub fn triangle(
        &mut self,
        x1: isize, y1: isize,
        x2: isize, y2: isize,
        x3: isize, y3: isize
    ) {
        let mut x1 = x1 as f32;
        let mut y1 = y1 as f32;
        let mut x2 = x2 as f32;
        let mut y2 = y2 as f32;
        let mut x3 = x3 as f32;
        let mut y3 = y3 as f32;

        // Ordena los puntos por su coordenada y (y1 <= y2 <= y3)
        if y1 > y2 { std::mem::swap(&mut x1, &mut x2); std::mem::swap(&mut y1, &mut y2); }
        if y1 > y3 { std::mem::swap(&mut x1, &mut x3); std::mem::swap(&mut y1, &mut y3); }
        if y2 > y3 { std::mem::swap(&mut x2, &mut x3); std::mem::swap(&mut y2, &mut y3); }

        let total_height = y3 - y1;
        let mut segment_height;
        let mut alpha;
        let mut beta;

        for i in 0..total_height as usize {
            let second_half = i as f32 > y2 - y1 || y2 == y1;
            segment_height = if second_half { y3 - y2 } else { y2 - y1 };
            alpha = i as f32 / total_height;
            beta = (i as f32 - if second_half { y2 - y1 } else { 0.0 }) / segment_height;

            let ax = x1 + (x3 - x1) * alpha;
            let bx = if second_half {
                x2 + (x3 - x2) * beta
            } else {
                x1 + (x2 - x1) * beta
            };

            let (min_x, max_x) = if ax > bx { (bx, ax) } else { (ax, bx) };
            for j in min_x as usize..max_x as usize {
                self.point(j, (y1 + i as f32) as usize);
            }
        }
    }
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
