mod vector;
mod matrix;
mod camera;
mod sphere;
mod shaders;

use raylib::prelude::*;
use vector::Vector3;
use camera::Camera;
use sphere::{Mesh, Vertex};
use shaders::{PlanetShader, RockyPlanetShader, GasGiantShader, CrystalPlanetShader, ShaderUniforms, ShaderColor};
use std::f32::consts::PI;

enum PlanetType {
    Rocky,
    GasGiant,
    Crystal,
}

struct Planet {
    mesh: Mesh,
    shader: Box<dyn PlanetShader>,
    rotation: f32,
    rotation_speed: f32,
}

impl Planet {
    fn new(planet_type: PlanetType) -> Self {
        let mesh = Mesh::create_sphere(1.0, 32, 32);
        
        let (shader, rotation_speed): (Box<dyn PlanetShader>, f32) = match planet_type {
            PlanetType::Rocky => (Box::new(RockyPlanetShader), 0.5),
            PlanetType::GasGiant => (Box::new(GasGiantShader), 1.2),
            PlanetType::Crystal => (Box::new(CrystalPlanetShader), 0.8),
        };
        
        Planet {
            mesh,
            shader,
            rotation: 0.0,
            rotation_speed,
        }
    }
    
    fn update(&mut self, dt: f32) {
        self.rotation += self.rotation_speed * dt;
    }
}

fn render_planet_software(
    planet: &Planet,
    camera: &Camera,
    uniforms: &ShaderUniforms,
    rl: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
) {
    let view_matrix = camera.get_view_matrix();
    let proj_matrix = matrix::create_projection_matrix(
        45.0_f32.to_radians(),
        width as f32 / height as f32,
        0.1,
        100.0,
    );
    let viewport_matrix = matrix::create_viewport_matrix(0.0, 0.0, width as f32, height as f32);
    
    // Matriz de rotación del planeta
    let rotation_matrix = matrix::create_rotation_y(planet.rotation);
    
    // Renderizar triángulos
    for i in (0..planet.mesh.indices.len()).step_by(3) {
        let i1 = planet.mesh.indices[i] as usize;
        let i2 = planet.mesh.indices[i + 1] as usize;
        let i3 = planet.mesh.indices[i + 2] as usize;
        
        if i1 >= planet.mesh.vertices.len() || i2 >= planet.mesh.vertices.len() || i3 >= planet.mesh.vertices.len() {
            continue;
        }
        
        let v1 = &planet.mesh.vertices[i1];
        let v2 = &planet.mesh.vertices[i2];
        let v3 = &planet.mesh.vertices[i3];
        
        // Aplicar transformaciones de vértices
        let (pos1, norm1) = planet.shader.vertex_shader(
            rotation_matrix.transform_vector(&v1.position),
            rotation_matrix.transform_vector(&v1.normal),
            v1.uv,
            uniforms,
        );
        let (pos2, norm2) = planet.shader.vertex_shader(
            rotation_matrix.transform_vector(&v2.position),
            rotation_matrix.transform_vector(&v2.normal),
            v2.uv,
            uniforms,
        );
        let (pos3, norm3) = planet.shader.vertex_shader(
            rotation_matrix.transform_vector(&v3.position),
            rotation_matrix.transform_vector(&v3.normal),
            v3.uv,
            uniforms,
        );
        
        // Transformar a espacio de pantalla
        let screen1 = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&pos1)));
        let screen2 = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&pos2)));
        let screen3 = viewport_matrix.transform_vector(&proj_matrix.transform_vector(&view_matrix.transform_vector(&pos3)));
        
        // Calcular colores usando fragment shader
        let color1 = planet.shader.fragment_shader(pos1, norm1, v1.uv, uniforms);
        let color2 = planet.shader.fragment_shader(pos2, norm2, v2.uv, uniforms);
        let color3 = planet.shader.fragment_shader(pos3, norm3, v3.uv, uniforms);
        
        // Dibujar triángulo (simplificado - usar color promedio)
        let avg_color = ShaderColor::new(
            (color1.r + color2.r + color3.r) / 3.0,
            (color1.g + color2.g + color3.g) / 3.0,
            (color1.b + color2.b + color3.b) / 3.0,
            (color1.a + color2.a + color3.a) / 3.0,
        );
        
        // Verificar que los puntos estén en pantalla
        if screen1.x >= 0.0 && screen1.x < width as f32 && screen1.y >= 0.0 && screen1.y < height as f32 &&
           screen2.x >= 0.0 && screen2.x < width as f32 && screen2.y >= 0.0 && screen2.y < height as f32 &&
           screen3.x >= 0.0 && screen3.x < width as f32 && screen3.y >= 0.0 && screen3.y < height as f32 {
            
            rl.draw_triangle(
                Vector2::new(screen1.x, screen1.y),
                Vector2::new(screen2.x, screen2.y),
                Vector2::new(screen3.x, screen3.y),
                avg_color.to_raylib_color(),
            );
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("Laboratorio de Planetas - Shaders")
        .build();

    let mut camera = Camera::new();
    let mut planets = vec![
        Planet::new(PlanetType::Rocky),
        Planet::new(PlanetType::GasGiant),
        Planet::new(PlanetType::Crystal),
    ];
    
    let mut current_planet = 0;
    let mut time = 0.0f32;
    
    // Configurar cámara 3D de raylib para comparación
    let mut raylib_camera = Camera3D::perspective(
        raylib::prelude::Vector3::new(0.0, 0.0, 5.0),
        raylib::prelude::Vector3::new(0.0, 0.0, 0.0),
        raylib::prelude::Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        time += dt;
        
        // Actualizar cámara
        camera.update(&rl);
        raylib_camera = camera.get_raylib_camera();
        
        // Cambiar planeta con teclas
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_planet = 0;
        } else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_planet = 1;
        } else if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_planet = 2;
        }
        
        // Actualizar planeta actual
        planets[current_planet].update(dt);
        
        // Configurar uniforms para shaders
        let uniforms = ShaderUniforms {
            time,
            light_direction: Vector3::new(1.0, 1.0, 1.0).normalize(),
            camera_position: camera.eye,
        };
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(raylib::prelude::Color::BLACK);
        
        // Renderizar usando nuestro software renderer
        render_planet_software(
            &planets[current_planet],
            &camera,
            &uniforms,
            &mut d,
            1024,
            768,
        );
        
        // UI
        d.draw_text("Laboratorio de Planetas con Shaders", 10, 10, 20, raylib::prelude::Color::WHITE);
        d.draw_text("Controles:", 10, 40, 16, raylib::prelude::Color::WHITE);
        d.draw_text("1 - Planeta Rocoso", 10, 60, 14, raylib::prelude::Color::WHITE);
        d.draw_text("2 - Gigante Gaseoso", 10, 80, 14, raylib::prelude::Color::WHITE);
        d.draw_text("3 - Planeta de Cristal", 10, 100, 14, raylib::prelude::Color::WHITE);
        d.draw_text("Click izquierdo + arrastrar: Rotar cámara", 10, 120, 14, raylib::prelude::Color::WHITE);
        d.draw_text("Rueda del ratón: Zoom", 10, 140, 14, raylib::prelude::Color::WHITE);
        d.draw_text("Click derecho + arrastrar: Mover cámara", 10, 160, 14, raylib::prelude::Color::WHITE);
        
        let planet_names = ["Planeta Rocoso", "Gigante Gaseoso", "Planeta de Cristal"];
        d.draw_text(
            &format!("Planeta actual: {}", planet_names[current_planet]),
            10,
            200,
            16,
            raylib::prelude::Color::YELLOW,
        );
    }
}
