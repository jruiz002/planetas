mod vector;
mod matrix;
mod camera;
mod shaders;
mod sphere;
mod obj_loader;
mod framebuffer;
mod fragment;

use raylib::prelude::*;
use vector::Vector3;
use camera::Camera;
use sphere::{Mesh, Vertex};
use obj_loader::load_obj;
use shaders::{PlanetShader, RockyPlanetShader, GasGiantShader, CrystalPlanetShader, LavaPlanetShader, RingShader, MoonShader, ShaderUniforms};
use std::f32::consts::PI;
use framebuffer::Framebuffer;
use fragment::{TransformedVertex, triangle};

enum PlanetType {
    Rocky,
    GasGiant,
    Crystal,
    Lava,
}

struct Planet {
    mesh: Mesh,
    shader: Box<dyn PlanetShader>,
    rotation: f32,
    rotation_speed: f32,
    has_rings: bool,
    has_moon: bool,
}

impl Planet {
    fn new(planet_type: PlanetType) -> Self {
        // Load the sphere mesh from OBJ file
        let mesh = load_obj("images/sphere.obj")
            .unwrap_or_else(|_| {
                // Fallback to generated sphere if OBJ loading fails
                println!("Warning: Could not load sphere.obj, generating sphere instead");
                Mesh::create_sphere(1.0, 32, 32)
            });
        
        let (shader, rotation_speed, has_rings, has_moon): (Box<dyn PlanetShader>, f32, bool, bool) = match planet_type {
            PlanetType::Rocky => (Box::new(RockyPlanetShader), 0.5, false, true),
            PlanetType::GasGiant => (Box::new(GasGiantShader), 1.2, true, false),
            PlanetType::Crystal => (Box::new(CrystalPlanetShader), 0.8, true, false),
            PlanetType::Lava => (Box::new(LavaPlanetShader), 1.5, false, false),
        };
        
        Planet {
            mesh,
            shader,
            rotation: 0.0,
            rotation_speed,
            has_rings,
            has_moon,
        }
    }
    
    fn update(&mut self, dt: f32) {
        self.rotation += self.rotation_speed * dt;
    }
}

/// Función de renderizado usando framebuffer personalizado (implementación académica)
/// Esta función demuestra el pipeline completo de renderizado 3D:
/// 1. Vertex Shader - Transformación de vértices
/// 2. Primitive Assembly - Ensamblaje de triángulos
/// 3. Rasterization - Conversión a fragmentos usando coordenadas baricéntricas
/// 4. Fragment Shader - Procesamiento de color por pixel
/// 5. Framebuffer - Escritura final con depth testing
fn render_planet_software(
    framebuffer: &mut Framebuffer,
    planet: &mut Planet,
    camera: &Camera,
    time: f32,
    width: i32,
    height: i32,
) {
    use matrix;
    
    // PASO 1: Construir matrices de transformación (multiplicación de matrices)
    let view_matrix = matrix::create_view_matrix(camera.eye, camera.target, camera.up);
    let proj_matrix = matrix::create_projection_matrix(45.0, width as f32 / height as f32, 0.1, 100.0);
    let viewport_matrix = matrix::create_viewport_matrix(0.0, 0.0, width as f32, height as f32);
    
    // Configurar uniformes del shader
    let uniforms = ShaderUniforms {
        time,
        camera_position: camera.eye,
        light_direction: Vector3::new(1.0, 1.0, 1.0).normalize(),
    };
    
    // PASO 2: Primitive Assembly - Procesar cada triángulo
    for i in (0..planet.mesh.indices.len()).step_by(3) {
        let idx1 = planet.mesh.indices[i] as usize;
        let idx2 = planet.mesh.indices[i + 1] as usize;
        let idx3 = planet.mesh.indices[i + 2] as usize;
        
        let v1 = &planet.mesh.vertices[idx1];
        let v2 = &planet.mesh.vertices[idx2];
        let v3 = &planet.mesh.vertices[idx3];
        
        // PASO 3: Vertex Shader - Aplicar transformaciones a cada vértice
        let (pos1, norm1) = planet.shader.vertex_shader(v1.position, v1.normal, v1.uv, &uniforms);
        let (pos2, norm2) = planet.shader.vertex_shader(v2.position, v2.normal, v2.uv, &uniforms);
        let (pos3, norm3) = planet.shader.vertex_shader(v3.position, v3.normal, v3.uv, &uniforms);
        
        // Aplicar rotación del planeta (modelo matrix)
        let rot_matrix = matrix::create_rotation_y(planet.rotation);
        let world_pos1 = rot_matrix.transform_vector(&pos1);
        let world_pos2 = rot_matrix.transform_vector(&pos2);
        let world_pos3 = rot_matrix.transform_vector(&pos3);
        let world_norm1 = rot_matrix.transform_vector(&norm1).normalize();
        let world_norm2 = rot_matrix.transform_vector(&norm2).normalize();
        let world_norm3 = rot_matrix.transform_vector(&norm3).normalize();
        
        // Multiplicación de matrices: Model * View * Projection
        let screen1 = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&world_pos1)));
        let screen2 = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&world_pos2)));
        let screen3 = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&world_pos3)));
        
        // PASO 4: Fragment Shader - Calcular color por vértice
        let color1 = planet.shader.fragment_shader(world_pos1, world_norm1, v1.uv, &uniforms);
        let color2 = planet.shader.fragment_shader(world_pos2, world_norm2, v2.uv, &uniforms);
        let color3 = planet.shader.fragment_shader(world_pos3, world_norm3, v3.uv, &uniforms);
        
        // Crear vértices transformados para rasterización
        let tv1 = TransformedVertex {
            screen_position: screen1,
            world_position: world_pos1,
            normal: world_norm1,
            color: color1,
            uv: v1.uv,
        };
        
        let tv2 = TransformedVertex {
            screen_position: screen2,
            world_position: world_pos2,
            normal: world_norm2,
            color: color2,
            uv: v2.uv,
        };
        
        let tv3 = TransformedVertex {
            screen_position: screen3,
            world_position: world_pos3,
            normal: world_norm3,
            color: color3,
            uv: v3.uv,
        };
        
        // PASO 5: Rasterization - Generar fragmentos usando coordenadas baricéntricas
        let fragments = triangle(&tv1, &tv2, &tv3);
        
        // PASO 6: Framebuffer - Escribir fragmentos con depth testing
        for fragment in fragments {
            if fragment.position.x >= 0.0 && fragment.position.x < width as f32 &&
               fragment.position.y >= 0.0 && fragment.position.y < height as f32 {
                framebuffer.set_pixel_with_depth(
                    fragment.position.x as u32,
                    fragment.position.y as u32,
                    fragment.color.to_raylib_color(),
                    fragment.depth,
                );
            }
        }
    }
    
    // Renderizar anillos si el planeta los tiene
    if planet.has_rings {
        render_rings(framebuffer, &view_matrix, &proj_matrix, &viewport_matrix, &uniforms, width, height);
    }
    
    // Renderizar luna si el planeta la tiene
    if planet.has_moon {
        render_moon(framebuffer, &view_matrix, &proj_matrix, &viewport_matrix, &uniforms, width, height);
    }
}

fn render_rings(
    framebuffer: &mut Framebuffer,
    view_matrix: &matrix::Matrix,
    proj_matrix: &matrix::Matrix,
    viewport_matrix: &matrix::Matrix,
    uniforms: &ShaderUniforms,
    width: i32,
    height: i32,
) {
    // Generar anillos procedurales usando rasterización manual
    let ring_segments = 64;
    let rings = 8;
    
    for ring in 0..rings {
        let radius = 1.5 + ring as f32 * 0.3;
        
        for segment in 0..ring_segments {
            let angle1 = (segment as f32 / ring_segments as f32) * 2.0 * PI;
            let angle2 = ((segment + 1) as f32 / ring_segments as f32) * 2.0 * PI;
            
            // Crear vértices de anillo
            let vertex1 = Vertex {
                position: Vector3::new(radius * angle1.cos(), 0.0, radius * angle1.sin()),
                normal: Vector3::new(0.0, 1.0, 0.0),
                uv: (0.5, 0.5),
            };
            
            let vertex2 = Vertex {
                position: Vector3::new(radius * angle2.cos(), 0.0, radius * angle2.sin()),
                normal: Vector3::new(0.0, 1.0, 0.0),
                uv: (0.5, 0.5),
            };
            
            // Aplicar shader de anillos
            let (pos1, base_color1) = RingShader::vertex_shader(&vertex1, uniforms);
            let (pos2, _) = RingShader::vertex_shader(&vertex2, uniforms);
            
            // Transformar a pantalla
            let screen1 = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&pos1)));
            let screen2 = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&pos2)));
            
            // Calcular color usando fragment shader
            let color1 = RingShader::fragment_shader(pos1, vertex1.normal, base_color1, uniforms);
            
            // Dibujar línea de anillo en el framebuffer
            draw_line_framebuffer(framebuffer, screen1, screen2, color1.to_raylib_color(), width, height);
        }
    }
}

fn render_moon(
    framebuffer: &mut Framebuffer,
    view_matrix: &matrix::Matrix,
    proj_matrix: &matrix::Matrix,
    viewport_matrix: &matrix::Matrix,
    uniforms: &ShaderUniforms,
    width: i32,
    height: i32,
) {
    // Posición orbital de la luna
    let orbit_radius = 3.0;
    let orbit_speed = uniforms.time * 0.8;
    let moon_x = orbit_radius * orbit_speed.cos();
    let moon_z = orbit_radius * orbit_speed.sin();
    
    // Crear una esfera pequeña para la luna
    let moon_scale = 0.3;
    let segments = 16;
    
    for i in 0..segments {
        for j in 0..segments {
            let phi = (i as f32 / segments as f32) * PI;
            let theta = (j as f32 / segments as f32) * 2.0 * PI;
            
            let x = moon_scale * phi.sin() * theta.cos() + moon_x;
            let y = moon_scale * phi.cos();
            let z = moon_scale * phi.sin() * theta.sin() + moon_z;
            
            let vertex = Vertex {
                position: Vector3::new(x, y, z),
                normal: Vector3::new(x - moon_x, y, z - moon_z).normalize(),
                uv: (j as f32 / segments as f32, i as f32 / segments as f32),
            };
            
            // Aplicar shader de luna
            let (pos, base_color) = MoonShader::vertex_shader(&vertex, uniforms);
            
            // Transformar a pantalla
            let screen = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&pos)));
            
            // Calcular color usando fragment shader
            let color = MoonShader::fragment_shader(pos, vertex.normal, base_color, uniforms);
            
            // Dibujar punto de luna en el framebuffer
            if screen.x >= 0.0 && screen.x < width as f32 && screen.y >= 0.0 && screen.y < height as f32 {
                framebuffer.point_with_depth(
                    screen.x as i32,
                    screen.y as i32,
                    color.to_raylib_color(),
                    screen.z,
                );
            }
        }
    }
}

// Función auxiliar para dibujar líneas en el framebuffer (algoritmo de Bresenham)
fn draw_line_framebuffer(
    framebuffer: &mut Framebuffer,
    start: Vector3,
    end: Vector3,
    color: Color,
    width: i32,
    height: i32,
) {
    let x0 = start.x as i32;
    let y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;
    
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    
    let mut x = x0;
    let mut y = y0;
    
    loop {
        if x >= 0 && x < width && y >= 0 && y < height {
            framebuffer.set_pixel_color(x as u32, y as u32, color);
        }
        
        if x == x1 && y == y1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("Laboratorio de Planetas - Software Renderer")
        .build();

    let width = 1024;
    let height = 768;
    
    // Crear framebuffer personalizado (implementación académica)
    let mut framebuffer = Framebuffer::new(width as u32, height as u32);
    
    let mut camera = Camera::new();
    let mut planets = vec![
        Planet::new(PlanetType::Rocky),
        Planet::new(PlanetType::GasGiant),
        Planet::new(PlanetType::Crystal),
        Planet::new(PlanetType::Lava),
    ];
    
    let mut current_planet = 0;
    let mut time = 0.0f32;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        time += dt;
        
        // Actualizar cámara
        camera.update(&rl);
        
        // Cambiar planeta con teclas
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_planet = 0;
        } else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_planet = 1;
        } else if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_planet = 2;
        } else if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            current_planet = 3;
        }
        
        // Actualizar planeta actual
        planets[current_planet].update(dt);
        
        // RENDERIZADO: Limpiar framebuffer antes de cada frame
        framebuffer.clear(Color::BLACK);
        
        // Renderizar usando nuestro software renderer con framebuffer personalizado
        render_planet_software(
            &mut framebuffer,
            &mut planets[current_planet],
            &camera,
            time,
            width as i32,
            height as i32,
        );
        
        // Actualizar textura de Raylib con los datos del framebuffer
        framebuffer.swap_buffers(&mut rl, &thread);
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        // Dibujar el framebuffer en pantalla
        framebuffer.draw_to_screen(&mut d);
        
        // UI
        d.draw_text("Laboratorio de Planetas - Software Renderer", 10, 10, 20, Color::WHITE);
        d.draw_text("Controles:", 10, 40, 16, Color::WHITE);
        d.draw_text("1 - Planeta Rocoso (con Luna)", 10, 60, 14, Color::WHITE);
        d.draw_text("2 - Gigante Gaseoso (con Anillos)", 10, 80, 14, Color::WHITE);
        d.draw_text("3 - Planeta de Cristal (con Anillos)", 10, 100, 14, Color::WHITE);
        d.draw_text("4 - Planeta de Lava", 10, 120, 14, Color::WHITE);
        d.draw_text("WASD: Rotar cámara", 10, 140, 14, Color::WHITE);
        d.draw_text("Flechas: Zoom y paneo", 10, 160, 14, Color::WHITE);
        d.draw_text("Q/E: Paneo horizontal, R/F: Paneo vertical", 10, 180, 14, Color::WHITE);
        
        let planet_names = ["Planeta Rocoso (Luna)", "Gigante Gaseoso (Anillos)", "Planeta de Cristal (Anillos)", "Planeta de Lava"];
        let planet_features = [
            "4 capas: Montañas, cráteres, rugosidad, minerales",
            "4 capas: Bandas, turbulencia, vórtices, brillos",
            "4 capas: Cristales, refracción, especular, energía",
            "4 capas: Volcanes, lava, emisión, resplandor"
        ];
        
        d.draw_text(
            &format!("Planeta actual: {}", planet_names[current_planet]),
            10,
            200,
            16,
            raylib::prelude::Color::YELLOW,
        );
        
        d.draw_text(
            &format!("Efectos: {}", planet_features[current_planet]),
            10,
            220,
            12,
            raylib::prelude::Color::LIGHTGRAY,
        );
    }
}
