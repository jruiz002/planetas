use raylib::prelude::*;

pub struct Framebuffer {
    pub pixels: Vec<Color>,
    pub width: u32,
    pub height: u32,
    pub current_color: Color,
    pub background_color: Color,
    texture: Option<Texture2D>,
    pub zbuffer: Vec<f32>, // Z-buffer para profundidad
}

impl Framebuffer {
    /// Crear el framebuffer con ancho y alto específicos
    pub fn new(width: u32, height: u32) -> Self {
        let total_pixels = (width * height) as usize;
        Self {
            pixels: vec![Color::BLACK; total_pixels],
            width,
            height,
            current_color: Color::WHITE,
            background_color: Color::BLACK,
            texture: None,
            zbuffer: vec![f32::INFINITY; total_pixels],
        }
    }

    /// Limpia el framebuffer con un color específico
    pub fn clear(&mut self, color: Color) {
        self.pixels.fill(color);
        self.zbuffer.fill(f32::INFINITY);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Establecer un pixel con el color actual
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = self.current_color;
        }
    }

    /// Establecer un pixel con un color específico
    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }

    /// Establecer un pixel con profundidad (z-buffer test)
    pub fn set_pixel_with_depth(&mut self, x: u32, y: u32, color: Color, depth: f32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            if depth < self.zbuffer[index] {
                self.pixels[index] = color;
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index]
        } else {
            Color::BLACK
        }
    }

    /// Dibujar una línea usando el algoritmo de Bresenham
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.set_pixel(x0 as u32, y0 as u32);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Método auxiliar para dibujar un punto (usado en rasterización)
    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.set_pixel_color(x as u32, y as u32, color);
        }
    }

    /// Método para dibujar un punto con profundidad
    pub fn point_with_depth(&mut self, x: i32, y: i32, color: Color, depth: f32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.set_pixel_with_depth(x as u32, y as u32, color, depth);
        }
    }

    /// Actualizar la textura de Raylib con los datos del framebuffer
    pub fn swap_buffers(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Crear una nueva imagen
        let mut image = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);
        
        // Copiar píxeles del framebuffer a la imagen
        unsafe {
            let image_ptr = image.data as *mut Color;
            for (i, pixel) in self.pixels.iter().enumerate() {
                *image_ptr.add(i) = *pixel;
            }
        }

        // Solo liberar la textura anterior si existe
        if let Some(_old_texture) = self.texture.take() {
            // La textura se libera automáticamente al salir de scope
        }

        // Crear nueva textura desde la imagen
        match rl.load_texture_from_image(thread, &image) {
            Ok(texture) => self.texture = Some(texture),
            Err(_) => {
                eprintln!("Error cargando textura del framebuffer");
            }
        }
    }

    /// Dibujar el framebuffer a la pantalla
    pub fn draw_to_screen(&self, d: &mut RaylibDrawHandle) {
        if let Some(ref texture) = self.texture {
            d.draw_texture_rec(
                texture,
                Rectangle::new(0.0, 0.0, self.width as f32, -(self.height as f32)),
                Vector2::new(0.0, 0.0),
                Color::WHITE,
            );
        }
    }
}
