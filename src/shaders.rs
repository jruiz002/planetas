use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct ShaderColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ShaderColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        ShaderColor { r, g, b, a }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        ShaderColor {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn to_raylib_color(&self) -> raylib::prelude::Color {
        raylib::prelude::Color {
            r: (self.r * 255.0) as u8,
            g: (self.g * 255.0) as u8,
            b: (self.b * 255.0) as u8,
            a: (self.a * 255.0) as u8,
        }
    }

    pub const WHITE: ShaderColor = ShaderColor { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: ShaderColor = ShaderColor { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const YELLOW: ShaderColor = ShaderColor { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
}

pub struct ShaderUniforms {
    pub time: f32,
    pub light_direction: Vector3,
    pub camera_position: Vector3,
}

pub trait PlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3);
    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor;
}

// Funciones de ruido para efectos procedurales
fn simple_noise(x: f32, y: f32) -> f32 {
    let seed = ((x * 12.9898 + y * 78.233) * 43758.5453).sin().abs();
    (seed * 1000.0).fract()
}

fn fbm(mut x: f32, mut y: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    
    for _ in 0..octaves {
        value += amplitude * simple_noise(x, y);
        x *= 2.0;
        y *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn mix_color(a: ShaderColor, b: ShaderColor, t: f32) -> ShaderColor {
    ShaderColor::new(
        mix(a.r, b.r, t),
        mix(a.g, b.g, t),
        mix(a.b, b.b, t),
        mix(a.a, b.a, t),
    )
}

// Shader para planeta rocoso
pub struct RockyPlanetShader;

impl PlanetShader for RockyPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), _uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Deformación sutil para simular montañas y cráteres
        let noise_val = fbm(position.x * 3.0, position.z * 3.0, 4);
        let displacement = noise_val * 0.05;
        let new_position = position + normal * displacement;
        
        (new_position, normal)
    }

    fn fragment_shader(&self, _position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // Colores base del planeta rocoso
        let rock_color = ShaderColor::from_rgb(139, 69, 19);  // Marrón rocoso
        let dirt_color = ShaderColor::from_rgb(160, 82, 45);  // Tierra
        let mountain_color = ShaderColor::from_rgb(105, 105, 105); // Gris montaña
        
        // Ruido para variación de superficie
        let surface_noise = fbm(uv.0 * 8.0, uv.1 * 8.0, 4);
        let height_noise = fbm(uv.0 * 2.0, uv.1 * 2.0, 3);
        
        // Mezclar colores basado en el ruido
        let base_color = if surface_noise > 0.6 {
            mountain_color
        } else if surface_noise > 0.3 {
            dirt_color
        } else {
            rock_color
        };
        
        // Iluminación básica
        let light_intensity = normal.dot(&uniforms.light_direction).max(0.0);
        let ambient = 0.2;
        let final_intensity = (ambient + light_intensity * 0.8).min(1.0);
        
        // Aplicar variación de altura
        let height_factor = (height_noise * 0.3 + 0.7).clamp(0.4, 1.0);
        
        ShaderColor::new(
            base_color.r * final_intensity * height_factor,
            base_color.g * final_intensity * height_factor,
            base_color.b * final_intensity * height_factor,
            1.0,
        )
    }
}

// Shader para gigante gaseoso
pub struct GasGiantShader;

impl PlanetShader for GasGiantShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), _uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        (position, normal)
    }

    fn fragment_shader(&self, _position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // Colores para las bandas del gigante gaseoso
        let band1 = ShaderColor::from_rgb(255, 140, 0);   // Naranja
        let band2 = ShaderColor::from_rgb(255, 165, 0);   // Naranja claro
        let band3 = ShaderColor::from_rgb(139, 69, 19);   // Marrón
        let band4 = ShaderColor::from_rgb(255, 215, 0);   // Dorado
        
        // Crear bandas horizontales con ruido
        let band_frequency = 8.0;
        let band_position = (uv.1 * band_frequency + uniforms.time * 0.1).sin();
        
        // Añadir turbulencia
        let turbulence = fbm(uv.0 * 6.0 + uniforms.time * 0.05, uv.1 * 4.0, 3) * 0.3;
        let distorted_band = band_position + turbulence;
        
        // Seleccionar color basado en la banda
        let base_color = if distorted_band > 0.5 {
            band1
        } else if distorted_band > 0.0 {
            band2
        } else if distorted_band > -0.5 {
            band3
        } else {
            band4
        };
        
        // Añadir remolinos
        let swirl_x = uv.0 * 4.0 + uniforms.time * 0.02;
        let swirl_y = uv.1 * 4.0;
        let swirl = (swirl_x.sin() * swirl_y.cos()) * 0.2;
        
        // Iluminación
        let light_intensity = normal.dot(&uniforms.light_direction).max(0.0);
        let ambient = 0.3;
        let final_intensity = (ambient + light_intensity * 0.7).min(1.0);
        
        // Mezclar con efecto de remolino
        let swirl_factor = (swirl + 1.0) * 0.5;
        let final_color = mix_color(base_color, ShaderColor::from_rgb(255, 255, 200), swirl_factor * 0.2);
        
        ShaderColor::new(
            final_color.r * final_intensity,
            final_color.g * final_intensity,
            final_color.b * final_intensity,
            1.0,
        )
    }
}

// Shader para planeta de cristal (personalizado)
pub struct CrystalPlanetShader;

impl PlanetShader for CrystalPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Deformación cristalina
        let crystal_noise = fbm(position.x * 4.0, position.y * 4.0, 3);
        let displacement = (crystal_noise * 0.1 + uniforms.time.sin() * 0.02).abs();
        let new_position = position + normal * displacement;
        
        (new_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // Colores cristalinos
        let crystal_blue = ShaderColor::from_rgb(173, 216, 230);
        let crystal_purple = ShaderColor::from_rgb(147, 112, 219);
        let crystal_white = ShaderColor::from_rgb(240, 248, 255);
        let crystal_cyan = ShaderColor::from_rgb(0, 255, 255);
        
        // Patrón de cristal
        let crystal_pattern = fbm(uv.0 * 12.0, uv.1 * 12.0, 4);
        let time_factor = (uniforms.time * 2.0).sin() * 0.5 + 0.5;
        
        // Seleccionar color basado en el patrón
        let base_color = if crystal_pattern > 0.7 {
            crystal_white
        } else if crystal_pattern > 0.4 {
            crystal_cyan
        } else if crystal_pattern > 0.2 {
            crystal_blue
        } else {
            crystal_purple
        };
        
        // Efecto de brillo interno
        let glow_intensity = (position.length() * 3.0 + uniforms.time).sin().abs();
        let glow_color = ShaderColor::from_rgb(255, 255, 255);
        
        // Iluminación especular para efecto cristalino
        let view_dir = (uniforms.camera_position - position).normalize();
        let reflect_dir = normal * (2.0 * normal.dot(&uniforms.light_direction)) - uniforms.light_direction;
        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(32.0);
        
        // Iluminación difusa
        let light_intensity = normal.dot(&uniforms.light_direction).max(0.0);
        let ambient = 0.4;
        
        // Combinar todos los efectos
        let final_color = mix_color(base_color, glow_color, glow_intensity * 0.3 * time_factor);
        let final_intensity = (ambient + light_intensity * 0.6 + specular * 0.8).min(1.5);
        
        ShaderColor::new(
            final_color.r * final_intensity,
            final_color.g * final_intensity,
            final_color.b * final_intensity,
            0.9, // Ligeramente transparente para efecto cristalino
        )
    }
}