use raylib::prelude::*;

/// Buffer de píxeles en memoria. Aquí dibujamos nosotros mismos;
/// Raylib solo se usa al final para mostrar el resultado en la ventana.
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pixels: Vec<Color>,
    background: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background: Color) -> Self {
        Framebuffer {
            width,
            height,
            pixels: vec![background; (width * height) as usize],
            background,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(self.background);
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Pinta un píxel. Ignora posiciones fuera del framebuffer.
    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = self.current_color;
        }
    }

    /// Copia el contenido del framebuffer a una Image de Raylib.
    pub fn to_image(&self) -> Image {
        let mut image = Image::gen_image_color(self.width, self.height, self.background);
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixels[(y * self.width + x) as usize];
                if color != self.background {
                    image.draw_pixel(x, y, color);
                }
            }
        }
        image
    }

    /// Guarda el framebuffer como PNG (no necesita abrir la ventana).
    pub fn save_png(&self, path: &str) {
        let image = self.to_image();
        image.export_image(path);
    }
}
