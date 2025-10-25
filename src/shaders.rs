use crate::vector::Vector3;
use crate::sphere::Vertex;

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

// Funciones de ruido mejoradas para efectos procedurales
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

// Función de ruido Voronoi para efectos de celdas
fn voronoi_noise(x: f32, y: f32) -> f32 {
    let cell_x = x.floor();
    let cell_y = y.floor();
    let mut min_dist = f32::INFINITY;
    
    for i in -1..=1 {
        for j in -1..=1 {
            let neighbor_x = cell_x + i as f32;
            let neighbor_y = cell_y + j as f32;
            let point_x = neighbor_x + simple_noise(neighbor_x, neighbor_y);
            let point_y = neighbor_y + simple_noise(neighbor_y, neighbor_x);
            
            let dist = ((x - point_x).powi(2) + (y - point_y).powi(2)).sqrt();
            min_dist = min_dist.min(dist);
        }
    }
    
    min_dist
}

// Función de ruido ridge para efectos de montañas
fn ridge_noise(x: f32, y: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        let n = simple_noise(x * frequency, y * frequency);
        let ridge = 1.0 - (2.0 * n - 1.0).abs();
        value += ridge * amplitude;
        frequency *= 2.0;
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

// Shader para planeta rocoso mejorado con múltiples capas
pub struct RockyPlanetShader;

impl PlanetShader for RockyPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Capa 1: Deformación base para montañas
        let mountain_noise = ridge_noise(position.x * 2.0, position.z * 2.0, 4);
        let mountain_displacement = mountain_noise * 0.08;
        
        // Capa 2: Cráteres usando Voronoi
        let crater_noise = voronoi_noise(position.x * 8.0, position.z * 8.0);
        let crater_displacement = -(crater_noise * 0.03).max(0.0);
        
        // Capa 3: Rugosidad fina
        let detail_noise = fbm(position.x * 15.0, position.z * 15.0, 3);
        let detail_displacement = detail_noise * 0.01;
        
        let total_displacement = mountain_displacement + crater_displacement + detail_displacement;
        let new_position = position + normal * total_displacement;
        
        (new_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // Capa 1: Colores base del terreno
        let bedrock_color = ShaderColor::from_rgb(101, 67, 33);    // Roca base
        let soil_color = ShaderColor::from_rgb(139, 69, 19);      // Tierra
        let mountain_color = ShaderColor::from_rgb(105, 105, 105); // Montañas
        let crater_color = ShaderColor::from_rgb(64, 64, 64);     // Cráteres
        let mineral_color = ShaderColor::from_rgb(184, 134, 11);  // Minerales
        
        // Capa 2: Mapas de ruido para diferentes características
        let elevation_noise = ridge_noise(uv.0 * 3.0, uv.1 * 3.0, 4);
        let surface_noise = fbm(uv.0 * 8.0, uv.1 * 8.0, 4);
        let crater_noise = voronoi_noise(uv.0 * 6.0, uv.1 * 6.0);
        let mineral_noise = fbm(uv.0 * 20.0, uv.1 * 20.0, 2);
        
        // Capa 3: Selección de color basada en múltiples factores
        let mut base_color = bedrock_color;
        
        // Montañas en elevaciones altas
        if elevation_noise > 0.6 {
            base_color = mix_color(base_color, mountain_color, smoothstep(0.6, 0.8, elevation_noise));
        }
        
        // Suelo en áreas medias
        if surface_noise > 0.3 && elevation_noise < 0.7 {
            base_color = mix_color(base_color, soil_color, smoothstep(0.3, 0.6, surface_noise));
        }
        
        // Cráteres en áreas específicas
        if crater_noise < 0.3 {
            let crater_factor = smoothstep(0.0, 0.3, crater_noise);
            base_color = mix_color(crater_color, base_color, crater_factor);
        }
        
        // Vetas minerales
        if mineral_noise > 0.7 {
            let mineral_factor = smoothstep(0.7, 0.9, mineral_noise) * 0.4;
            base_color = mix_color(base_color, mineral_color, mineral_factor);
        }
        
        // Capa 4: Iluminación avanzada con múltiples componentes
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Iluminación difusa
        let diffuse = normal.dot(&light_dir).max(0.0);
        
        // Iluminación especular para minerales
        let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(16.0) * mineral_noise.max(0.0);
        
        // Oclusión ambiental basada en rugosidad
        let ao = 1.0 - (surface_noise * 0.3).clamp(0.0, 0.4);
        
        // Iluminación de borde (rim lighting)
        let rim = (1.0 - view_dir.dot(&normal)).powf(2.0) * 0.2;
        
        let ambient = 0.15;
        let final_intensity = (ambient + diffuse * 0.7 + specular * 0.3 + rim) * ao;
        
        // Variación de color por altura y temperatura simulada
        let altitude_factor = (elevation_noise * 0.2 + 0.8).clamp(0.6, 1.0);
        let temperature_variation = (position.y * 0.1).sin() * 0.1 + 1.0;
        
        ShaderColor::new(
            (base_color.r * final_intensity * altitude_factor * temperature_variation).clamp(0.0, 1.0),
            (base_color.g * final_intensity * altitude_factor * temperature_variation).clamp(0.0, 1.0),
            (base_color.b * final_intensity * altitude_factor * temperature_variation).clamp(0.0, 1.0),
            1.0,
        )
    }
}

// Shader para gigante gaseoso mejorado con múltiples capas atmosféricas
pub struct GasGiantShader;

impl PlanetShader for GasGiantShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Capa 1: Ondulación atmosférica sutil
        let wave_noise = (position.x * 4.0 + uniforms.time * 0.5).sin() * 
                        (position.z * 3.0 + uniforms.time * 0.3).cos();
        let wave_displacement = wave_noise * 0.02;
        
        // Capa 2: Turbulencia atmosférica
        let turbulence = fbm(position.x * 6.0 + uniforms.time * 0.1, 
                           position.z * 6.0 + uniforms.time * 0.1, 3);
        let turb_displacement = turbulence * 0.015;
        
        let total_displacement = wave_displacement + turb_displacement;
        let new_position = position + normal * total_displacement;
        
        (new_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // Capa 1: Colores base de las diferentes capas atmosféricas
        let deep_atmosphere = ShaderColor::from_rgb(139, 69, 19);    // Marrón profundo
        let mid_atmosphere = ShaderColor::from_rgb(255, 140, 0);     // Naranja
        let high_atmosphere = ShaderColor::from_rgb(255, 215, 0);    // Dorado
        let storm_color = ShaderColor::from_rgb(255, 69, 0);         // Rojo tormenta
        let cloud_color = ShaderColor::from_rgb(255, 248, 220);      // Nubes claras
        let lightning_color = ShaderColor::from_rgb(173, 216, 230);  // Azul eléctrico
        
        // Capa 2: Bandas atmosféricas con múltiples frecuencias
        let band_frequency1 = 6.0;
        let band_frequency2 = 12.0;
        let band_position1 = (uv.1 * band_frequency1 + uniforms.time * 0.08).sin();
        let band_position2 = (uv.1 * band_frequency2 + uniforms.time * 0.05).sin();
        
        // Capa 3: Turbulencia y remolinos complejos
        let turbulence1 = fbm(uv.0 * 8.0 + uniforms.time * 0.03, uv.1 * 6.0, 4) * 0.4;
        let turbulence2 = fbm(uv.0 * 12.0 - uniforms.time * 0.02, uv.1 * 8.0, 3) * 0.3;
        let combined_turbulence = turbulence1 + turbulence2;
        
        // Capa 4: Grandes tormentas circulares (Great Red Spot style)
        let storm_center_x = 0.3 + (uniforms.time * 0.01).sin() * 0.1;
        let storm_center_y = 0.6 + (uniforms.time * 0.015).cos() * 0.05;
        let storm_dist = ((uv.0 - storm_center_x).powi(2) + (uv.1 - storm_center_y).powi(2)).sqrt();
        let storm_intensity = smoothstep(0.3, 0.1, storm_dist);
        
        // Remolino en la tormenta
        let angle = (uv.1 - storm_center_y).atan2(uv.0 - storm_center_x);
        let spiral = (angle * 3.0 + storm_dist * 10.0 + uniforms.time * 2.0).sin();
        let storm_swirl = storm_intensity * spiral * 0.3;
        
        // Capa 5: Rayos y descargas eléctricas
        let lightning_noise = fbm(uv.0 * 25.0 + uniforms.time * 5.0, uv.1 * 25.0, 2);
        let lightning_threshold = 0.85 + (uniforms.time * 10.0).sin() * 0.1;
        let lightning_intensity = if lightning_noise > lightning_threshold { 
            (lightning_noise - lightning_threshold) * 10.0 
        } else { 
            0.0 
        };
        
        // Selección de color base según las bandas distorsionadas
        let distorted_band1 = band_position1 + combined_turbulence + storm_swirl;
        let distorted_band2 = band_position2 + combined_turbulence * 0.5;
        
        let mut base_color = if distorted_band1 > 0.6 {
            high_atmosphere
        } else if distorted_band1 > 0.2 {
            mid_atmosphere
        } else if distorted_band2 > 0.0 {
            mix_color(mid_atmosphere, deep_atmosphere, 0.6)
        } else {
            deep_atmosphere
        };
        
        // Aplicar efectos de tormenta
        if storm_intensity > 0.1 {
            base_color = mix_color(base_color, storm_color, storm_intensity * 0.8);
        }
        
        // Añadir nubes altas
        let cloud_noise = fbm(uv.0 * 15.0 + uniforms.time * 0.02, uv.1 * 10.0, 3);
        if cloud_noise > 0.6 {
            let cloud_factor = smoothstep(0.6, 0.8, cloud_noise) * 0.4;
            base_color = mix_color(base_color, cloud_color, cloud_factor);
        }
        
        // Capa 6: Iluminación atmosférica compleja
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Iluminación difusa con scattering atmosférico
        let diffuse = normal.dot(&light_dir).max(0.0);
        let atmosphere_scattering = (1.0 - diffuse).powf(0.5) * 0.3;
        
        // Iluminación de borde para efecto atmosférico
        let rim = (1.0 - view_dir.dot(&normal)).powf(1.5) * 0.4;
        
        // Iluminación interna de las tormentas
        let internal_glow = storm_intensity * 0.2 + combined_turbulence * 0.1;
        
        let ambient = 0.25;
        let final_intensity = (ambient + diffuse * 0.6 + atmosphere_scattering + rim + internal_glow).min(1.8);
        
        // Aplicar rayos si están presentes
        if lightning_intensity > 0.0 {
            base_color = mix_color(base_color, lightning_color, lightning_intensity.min(0.8));
        }
        
        // Variación de profundidad atmosférica
        let depth_variation = (uv.1 * 3.14159).sin().abs() * 0.2 + 0.8;
        
        ShaderColor::new(
            (base_color.r * final_intensity * depth_variation).clamp(0.0, 1.0),
            (base_color.g * final_intensity * depth_variation).clamp(0.0, 1.0),
            (base_color.b * final_intensity * depth_variation).clamp(0.0, 1.0),
            0.95, // Ligeramente transparente para efecto atmosférico
        )
    }
}

// Shader para planeta de cristal mejorado con múltiples capas cristalinas
pub struct CrystalPlanetShader;

impl PlanetShader for CrystalPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Capa 1: Formaciones cristalinas principales
        let crystal_noise = voronoi_noise(position.x * 6.0, position.z * 6.0);
        let crystal_displacement = (crystal_noise * 0.12).max(0.0);
        
        // Capa 2: Cristales secundarios
        let secondary_crystals = fbm(position.x * 12.0, position.z * 12.0, 3);
        let secondary_displacement = secondary_crystals * 0.04;
        
        // Capa 3: Pulsación cristalina animada
        let pulse = (uniforms.time * 3.0 + position.length() * 2.0).sin() * 0.02;
        
        let total_displacement = crystal_displacement + secondary_displacement + pulse;
        let new_position = position + normal * total_displacement;
        
        (new_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // Capa 1: Colores cristalinos base
        let crystal_core = ShaderColor::from_rgb(240, 248, 255);      // Blanco cristalino
        let crystal_blue = ShaderColor::from_rgb(173, 216, 230);      // Azul claro
        let crystal_purple = ShaderColor::from_rgb(147, 112, 219);    // Púrpura
        let crystal_cyan = ShaderColor::from_rgb(0, 255, 255);        // Cian brillante
        let crystal_pink = ShaderColor::from_rgb(255, 182, 193);      // Rosa cristalino
        let energy_core = ShaderColor::from_rgb(255, 255, 255);       // Energía pura
        
        // Capa 2: Patrones cristalinos complejos
        let main_crystal_pattern = voronoi_noise(uv.0 * 8.0, uv.1 * 8.0);
        let secondary_pattern = fbm(uv.0 * 16.0, uv.1 * 16.0, 4);
        let fractal_pattern = fbm(uv.0 * 32.0, uv.1 * 32.0, 2);
        
        // Capa 3: Efectos de energía y pulsación
        let time_factor = (uniforms.time * 2.0).sin() * 0.5 + 0.5;
        let energy_pulse = (uniforms.time * 4.0 + position.length() * 3.0).sin().abs();
        let energy_flow = fbm(uv.0 * 6.0 + uniforms.time * 0.5, uv.1 * 6.0, 3);
        
        // Selección de color base según patrones cristalinos
        let mut base_color = crystal_core;
        
        // Cristales principales
        if main_crystal_pattern < 0.2 {
            base_color = crystal_cyan;
        } else if main_crystal_pattern < 0.4 {
            base_color = crystal_blue;
        } else if main_crystal_pattern < 0.6 {
            base_color = crystal_purple;
        } else if main_crystal_pattern < 0.8 {
            base_color = crystal_pink;
        }
        
        // Cristales secundarios superpuestos
        if secondary_pattern > 0.7 {
            let blend_factor = smoothstep(0.7, 0.9, secondary_pattern) * 0.6;
            base_color = mix_color(base_color, crystal_cyan, blend_factor);
        }
        
        // Vetas de energía
        if fractal_pattern > 0.8 {
            let energy_factor = smoothstep(0.8, 0.95, fractal_pattern) * energy_pulse;
            base_color = mix_color(base_color, energy_core, energy_factor);
        }
        
        // Capa 4: Iluminación cristalina avanzada
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Iluminación difusa suave
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.4;
        
        // Múltiples reflexiones especulares para efecto cristalino
        let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
        let specular1 = view_dir.dot(&reflect_dir).max(0.0).powf(64.0);
        let specular2 = view_dir.dot(&reflect_dir).max(0.0).powf(16.0);
        let specular3 = view_dir.dot(&reflect_dir).max(0.0).powf(4.0);
        
        // Refracción simulada
        let refraction = (1.0 - view_dir.dot(&normal)).powf(3.0) * 0.3;
        
        // Iluminación interna (subsurface scattering simulado)
        let internal_light = energy_flow * 0.2 + energy_pulse * 0.3;
        
        // Iluminación de borde con múltiples capas
        let rim1 = (1.0 - view_dir.dot(&normal)).powf(2.0) * 0.4;
        let rim2 = (1.0 - view_dir.dot(&normal)).powf(4.0) * 0.6;
        
        let ambient = 0.3;
        let final_intensity = (ambient + diffuse + specular1 * 0.8 + specular2 * 0.4 + 
                              specular3 * 0.2 + refraction + internal_light + rim1 + rim2).min(2.5);
        
        // Capa 5: Efectos de color dinámicos
        let color_shift = (uniforms.time * 1.5 + position.x * 0.5).sin() * 0.1;
        let final_color = mix_color(base_color, 
                                   ShaderColor::new(base_color.b, base_color.r, base_color.g, base_color.a), 
                                   color_shift.abs());
        
        // Variación de transparencia basada en el patrón
        let alpha_variation = (main_crystal_pattern * 0.2 + 0.7).clamp(0.6, 0.95);
        
        ShaderColor::new(
            (final_color.r * final_intensity).clamp(0.0, 1.0),
            (final_color.g * final_intensity).clamp(0.0, 1.0),
            (final_color.b * final_intensity).clamp(0.0, 1.0),
            alpha_variation,
        )
    }
}

// Shader para planeta de lava (cuarto planeta adicional)
pub struct LavaPlanetShader;

impl PlanetShader for LavaPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Capa 1: Deformación volcánica
        let volcanic_noise = fbm(position.x * 4.0, position.z * 4.0, 4);
        let volcanic_displacement = volcanic_noise * 0.06;
        
        // Capa 2: Flujos de lava
        let lava_flow = fbm(position.x * 8.0 + uniforms.time * 0.1, position.z * 6.0, 3);
        let flow_displacement = lava_flow * 0.03;
        
        // Capa 3: Actividad volcánica pulsante
        let volcanic_activity = (uniforms.time * 2.0 + position.length()).sin() * 0.02;
        
        let total_displacement = volcanic_displacement + flow_displacement + volcanic_activity;
        let new_position = position + normal * total_displacement;
        
        (new_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // Capa 1: Colores volcánicos
        let cooled_lava = ShaderColor::from_rgb(64, 64, 64);          // Lava enfriada
        let warm_rock = ShaderColor::from_rgb(139, 69, 19);          // Roca caliente
        let hot_lava = ShaderColor::from_rgb(255, 69, 0);            // Lava caliente
        let molten_core = ShaderColor::from_rgb(255, 140, 0);        // Núcleo fundido
        let white_hot = ShaderColor::from_rgb(255, 255, 200);        // Blanco caliente
        let ember_glow = ShaderColor::from_rgb(255, 165, 0);         // Brasa
        
        // Capa 2: Patrones de flujo de lava
        let lava_flow1 = fbm(uv.0 * 6.0 + uniforms.time * 0.05, uv.1 * 4.0, 4);
        let lava_flow2 = fbm(uv.0 * 12.0 - uniforms.time * 0.03, uv.1 * 8.0, 3);
        let volcanic_cracks = voronoi_noise(uv.0 * 15.0, uv.1 * 15.0);
        
        // Capa 3: Actividad volcánica y temperatura
        let heat_intensity = (uniforms.time * 3.0 + position.length() * 2.0).sin() * 0.5 + 0.5;
        let volcanic_activity = fbm(uv.0 * 8.0 + uniforms.time * 0.2, uv.1 * 8.0, 2);
        let temperature_map = lava_flow1 * 0.6 + volcanic_activity * 0.4;
        
        // Selección de color basada en temperatura
        let mut base_color = cooled_lava;
        
        if temperature_map > 0.8 {
            // Lava muy caliente
            base_color = mix_color(white_hot, molten_core, (temperature_map - 0.8) * 5.0);
        } else if temperature_map > 0.6 {
            // Lava caliente
            base_color = mix_color(hot_lava, white_hot, (temperature_map - 0.6) * 5.0);
        } else if temperature_map > 0.4 {
            // Lava tibia
            base_color = mix_color(molten_core, hot_lava, (temperature_map - 0.4) * 5.0);
        } else if temperature_map > 0.2 {
            // Roca caliente
            base_color = mix_color(warm_rock, molten_core, (temperature_map - 0.2) * 5.0);
        }
        
        // Grietas volcánicas brillantes
        if volcanic_cracks < 0.1 {
            let crack_intensity = smoothstep(0.0, 0.1, volcanic_cracks);
            let crack_glow = mix_color(white_hot, ember_glow, heat_intensity);
            base_color = mix_color(crack_glow, base_color, crack_intensity);
        }
        
        // Capa 4: Iluminación volcánica
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Iluminación difusa
        let diffuse = normal.dot(&light_dir).max(0.0);
        
        // Emisión de calor (self-illumination)
        let heat_emission = temperature_map * 0.8 + heat_intensity * 0.4;
        
        // Iluminación especular para lava fundida
        let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(8.0) * temperature_map;
        
        // Resplandor volcánico
        let volcanic_glow = (1.0 - view_dir.dot(&normal)).powf(1.5) * heat_emission * 0.3;
        
        let ambient = 0.1; // Ambiente bajo para planeta volcánico
        let final_intensity = (ambient + diffuse * 0.5 + heat_emission + specular * 0.4 + volcanic_glow).min(2.0);
        
        // Parpadeo de la actividad volcánica
        let flicker = (uniforms.time * 15.0 + position.x * 10.0).sin() * 0.1 + 1.0;
        let final_flicker = if temperature_map > 0.6 { flicker } else { 1.0 };
        
        ShaderColor::new(
            (base_color.r * final_intensity * final_flicker).clamp(0.0, 1.0),
            (base_color.g * final_intensity * final_flicker).clamp(0.0, 1.0),
            (base_color.b * final_intensity * final_flicker).clamp(0.0, 1.0),
            1.0,
        )
    }
}

// Estructura para anillos procedurales
pub struct RingShader;

impl RingShader {
    pub fn vertex_shader(vertex: &Vertex, uniforms: &ShaderUniforms) -> (Vector3, ShaderColor) {
        let mut pos = vertex.position;
        
        // Crear anillos procedurales usando coordenadas polares
        let radius = (pos.x * pos.x + pos.z * pos.z).sqrt();
        let angle = pos.z.atan2(pos.x);
        
        // Generar múltiples anillos con diferentes radios
        let ring_count = 8.0;
        let ring_spacing = 0.3;
        let base_radius = 1.5;
        
        // Determinar en qué anillo estamos
        let ring_index = (radius / ring_spacing).floor();
        let ring_center = base_radius + ring_index * ring_spacing;
        
        // Crear variaciones en el anillo usando noise
        let noise_scale = 10.0;
        let ring_noise = simple_noise(angle * noise_scale + uniforms.time * 2.0, 0.0);
        let radial_noise = simple_noise(radius * 15.0 + uniforms.time, 0.0);
        
        // Modular la altura del anillo
        let ring_height = 0.02 + ring_noise * 0.01;
        pos.y = ring_height * (1.0 + radial_noise * 0.5);
        
        // Crear gaps en los anillos
        let gap_noise = simple_noise(angle * 20.0 + ring_index * 3.14159, 0.0);
        if gap_noise > 0.7 {
            pos.y *= 0.1; // Hacer el anillo muy delgado en los gaps
        }
        
        // Rotación de los anillos
        let rotation_speed = 0.5 + ring_index * 0.1;
        let rotated_angle = angle + uniforms.time * rotation_speed;
        pos.x = radius * rotated_angle.cos();
        pos.z = radius * rotated_angle.sin();
        
        // Color base del anillo
        let ring_color_variation = simple_noise(ring_index * 2.0, 0.0);
        let base_color = if ring_color_variation > 0.0 {
            ShaderColor { r: 0.8, g: 0.7, b: 0.5, a: 0.8 } // Dorado
        } else {
            ShaderColor { r: 0.6, g: 0.5, b: 0.4, a: 0.7 } // Marrón
        };
        
        (pos, base_color)
    }
    
    pub fn fragment_shader(
        _world_pos: Vector3,
        _normal: Vector3,
        color: ShaderColor,
        uniforms: &ShaderUniforms,
    ) -> ShaderColor {
        let radius = (_world_pos.x * _world_pos.x + _world_pos.z * _world_pos.z).sqrt();
        
        // Crear bandas de color en los anillos
        let band_frequency = 25.0;
        let band_pattern = (radius * band_frequency).sin() * 0.5 + 0.5;
        
        // Variaciones de densidad
        let density_noise = fbm(_world_pos.x * 30.0, _world_pos.z * 30.0, 3);
        let density = 0.3 + density_noise * 0.4;
        
        // Partículas brillantes ocasionales
        let sparkle_noise = simple_noise(_world_pos.x * 100.0 + _world_pos.z * 100.0 + uniforms.time * 5.0, 0.0);
        let sparkle = if sparkle_noise > 0.95 { 0.5 } else { 0.0 };
        
        // Combinar efectos
        let final_color = ShaderColor {
            r: color.r * (0.7 + band_pattern * 0.3) + sparkle,
            g: color.g * (0.7 + band_pattern * 0.3) + sparkle * 0.8,
            b: color.b * (0.7 + band_pattern * 0.3) + sparkle * 0.6,
            a: color.a * density,
        };
        
        final_color
    }
}

// Estructura para luna procedural
pub struct MoonShader;

impl MoonShader {
    pub fn vertex_shader(vertex: &Vertex, uniforms: &ShaderUniforms) -> (Vector3, ShaderColor) {
        let mut pos = vertex.position;
        
        // Escalar la luna para que sea más pequeña
        let moon_scale = 0.3;
        pos = pos * moon_scale;
        
        // Órbita de la luna alrededor del planeta
        let orbit_radius = 3.0;
        let orbit_speed = 0.8;
        let orbit_angle = uniforms.time * orbit_speed;
        
        // Posición orbital
        let orbit_x = orbit_radius * orbit_angle.cos();
        let orbit_z = orbit_radius * orbit_angle.sin();
        
        // Agregar la posición orbital
        pos.x += orbit_x;
        pos.z += orbit_z;
        
        // Rotación propia de la luna
        let moon_rotation = uniforms.time * 0.3;
        let cos_rot = moon_rotation.cos();
        let sin_rot = moon_rotation.sin();
        
        // Aplicar rotación en Y
        let rotated_x = pos.x * cos_rot - pos.z * sin_rot;
        let rotated_z = pos.x * sin_rot + pos.z * cos_rot;
        pos.x = rotated_x;
        pos.z = rotated_z;
        
        // Crear cráteres usando noise
        let crater_noise1 = simple_noise(pos.x * 15.0, pos.y * 15.0 + pos.z * 15.0);
        let crater_noise2 = simple_noise(pos.x * 25.0 + 100.0, pos.y * 25.0 + pos.z * 25.0 + 100.0);
        
        // Deformación por cráteres
        let crater_depth = 0.0;
        if crater_noise1 > 0.6 {
            let crater_intensity = (crater_noise1 - 0.6) * 2.5;
            pos = pos * (1.0 - crater_intensity * 0.1);
        }
        
        if crater_noise2 > 0.7 {
            let crater_intensity = (crater_noise2 - 0.7) * 3.0;
            pos = pos * (1.0 - crater_intensity * 0.05);
        }
        
        // Color base de la luna (gris lunar)
        let surface_variation = simple_noise(pos.x * 10.0, pos.y * 10.0 + pos.z * 10.0);
        let base_gray = 0.4 + surface_variation * 0.2;
        
        let base_color = ShaderColor { 
            r: base_gray, 
            g: base_gray, 
            b: base_gray + 0.05, 
            a: 1.0 
        };
        
        (pos, base_color)
    }
    
    pub fn fragment_shader(
        world_pos: Vector3,
        normal: Vector3,
        color: ShaderColor,
        uniforms: &ShaderUniforms,
    ) -> ShaderColor {
        // Iluminación básica
        let light_dir = uniforms.light_direction.normalize();
        let dot_product = normal.dot(&light_dir).max(0.0);
        
        // Crear variaciones de superficie
        let surface_detail = fbm(world_pos.x * 20.0, world_pos.y * 20.0 + world_pos.z * 20.0, 4);
        
        // Cráteres más definidos en el fragment shader
        let crater_pattern1 = simple_noise(world_pos.x * 12.0, world_pos.y * 12.0 + world_pos.z * 12.0);
        let crater_pattern2 = simple_noise(world_pos.x * 8.0 + 50.0, world_pos.y * 8.0 + world_pos.z * 8.0 + 50.0);
        
        let mut final_color = color;
        
        // Oscurecer cráteres
        if crater_pattern1 > 0.65 {
            let crater_factor = (crater_pattern1 - 0.65) * 2.0;
            final_color.r *= 1.0 - crater_factor * 0.3;
            final_color.g *= 1.0 - crater_factor * 0.3;
            final_color.b *= 1.0 - crater_factor * 0.25;
        }
        
        if crater_pattern2 > 0.7 {
            let crater_factor = (crater_pattern2 - 0.7) * 2.5;
            final_color.r *= 1.0 - crater_factor * 0.4;
            final_color.g *= 1.0 - crater_factor * 0.4;
            final_color.b *= 1.0 - crater_factor * 0.35;
        }
        
        // Agregar detalles de superficie
        final_color.r += surface_detail * 0.1;
        final_color.g += surface_detail * 0.1;
        final_color.b += surface_detail * 0.12;
        
        // Aplicar iluminación
        final_color.r *= 0.3 + dot_product * 0.7;
        final_color.g *= 0.3 + dot_product * 0.7;
        final_color.b *= 0.3 + dot_product * 0.7;
        
        // Rim lighting para dar más volumen
        let view_dir = (uniforms.camera_position - world_pos).normalize();
        let rim = 1.0 - normal.dot(&view_dir).abs();
        let rim_intensity = rim.powf(2.0) * 0.2;
        
        final_color.r += rim_intensity;
        final_color.g += rim_intensity;
        final_color.b += rim_intensity * 1.1;
        
        final_color
    }
}