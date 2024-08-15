// src/texture.rs
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
impl Texture {
    pub fn get_pixel(&self, index: usize) -> u32 {
        let r = self.data[index * 4] as u32;
        let g = self.data[index * 4 + 1] as u32;
        let b = self.data[index * 4 + 2] as u32;
        let a = self.data[index * 4 + 3] as u32;

        // Convertir RGBA a un valor de color hexadecimal
        (a << 24) | (r << 16) | (g << 8) | b
    }
}
