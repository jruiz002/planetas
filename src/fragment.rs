use crate::vector::Vector3;
use crate::shaders::ShaderColor;

/// Estructura que representa un fragmento (pixel candidato)
#[derive(Debug, Clone)]
pub struct Fragment {
    pub position: Vector3,      // Posición en pantalla (x, y, depth)
    pub color: ShaderColor,     // Color del fragmento
    pub world_position: Vector3, // Posición en espacio del mundo
    pub normal: Vector3,        // Normal interpolada
    pub depth: f32,             // Profundidad Z
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: ShaderColor, depth: f32) -> Self {
        Fragment {
            position: Vector3::new(x, y, depth),
            color,
            world_position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            depth,
        }
    }

    pub fn new_with_data(
        x: f32,
        y: f32,
        color: ShaderColor,
        depth: f32,
        world_pos: Vector3,
        normal: Vector3,
    ) -> Self {
        Fragment {
            position: Vector3::new(x, y, depth),
            color,
            world_position: world_pos,
            normal,
            depth,
        }
    }
}

/// Estructura que representa un vértice transformado
#[derive(Debug, Clone)]
pub struct TransformedVertex {
    pub screen_position: Vector3,  // Posición en espacio de pantalla
    pub world_position: Vector3,   // Posición en espacio del mundo
    pub normal: Vector3,           // Normal del vértice
    pub color: ShaderColor,        // Color del vértice
    pub uv: (f32, f32),           // Coordenadas UV
}

/// Calcula las coordenadas baricéntricas de un punto P respecto a un triángulo ABC
/// Retorna (w, v, u) donde w, v, u son los pesos baricéntricos
pub fn barycentric_coordinates(
    p_x: f32,
    p_y: f32,
    a: &TransformedVertex,
    b: &TransformedVertex,
    c: &TransformedVertex,
) -> (f32, f32, f32) {
    let a_x = a.screen_position.x;
    let b_x = b.screen_position.x;
    let c_x = c.screen_position.x;
    let a_y = a.screen_position.y;
    let b_y = b.screen_position.y;
    let c_y = c.screen_position.y;

    // Calcular el área del triángulo
    let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

    // Si el área es muy pequeña, el triángulo es degenerado
    if area.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    // Calcular los pesos baricéntricos
    let w = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
    let v = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
    let u = 1.0 - w - v;

    (w, v, u)
}

/// Rasteriza un triángulo y genera fragmentos
/// Usa el algoritmo de escaneo con coordenadas baricéntricas
pub fn triangle(
    v1: &TransformedVertex,
    v2: &TransformedVertex,
    v3: &TransformedVertex,
) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let a_x = v1.screen_position.x;
    let b_x = v2.screen_position.x;
    let c_x = v3.screen_position.x;
    let a_y = v1.screen_position.y;
    let b_y = v2.screen_position.y;
    let c_y = v3.screen_position.y;

    // Calcular el bounding box del triángulo
    let min_x = a_x.min(b_x).min(c_x).floor() as i32;
    let min_y = a_y.min(b_y).min(c_y).floor() as i32;
    let max_x = a_x.max(b_x).max(c_x).ceil() as i32;
    let max_y = a_y.max(b_y).max(c_y).ceil() as i32;

    // Iterar sobre cada pixel en el bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (w, v, u) = barycentric_coordinates(x as f32 + 0.5, y as f32 + 0.5, v1, v2, v3);

            // Si el punto está dentro del triángulo (todas las coordenadas baricéntricas son positivas)
            if w >= 0.0 && v >= 0.0 && u >= 0.0 {
                // Interpolar la profundidad usando coordenadas baricéntricas
                let depth = w * v1.screen_position.z + v * v2.screen_position.z + u * v3.screen_position.z;

                // Interpolar el color
                let color = ShaderColor::new(
                    w * v1.color.r + v * v2.color.r + u * v3.color.r,
                    w * v1.color.g + v * v2.color.g + u * v3.color.g,
                    w * v1.color.b + v * v2.color.b + u * v3.color.b,
                    w * v1.color.a + v * v2.color.a + u * v3.color.a,
                );

                // Interpolar la posición del mundo
                let world_pos = Vector3::new(
                    w * v1.world_position.x + v * v2.world_position.x + u * v3.world_position.x,
                    w * v1.world_position.y + v * v2.world_position.y + u * v3.world_position.y,
                    w * v1.world_position.z + v * v2.world_position.z + u * v3.world_position.z,
                );

                // Interpolar la normal
                let normal = Vector3::new(
                    w * v1.normal.x + v * v2.normal.x + u * v3.normal.x,
                    w * v1.normal.y + v * v2.normal.y + u * v3.normal.y,
                    w * v1.normal.z + v * v2.normal.z + u * v3.normal.z,
                ).normalize();

                fragments.push(Fragment::new_with_data(
                    x as f32,
                    y as f32,
                    color,
                    depth,
                    world_pos,
                    normal,
                ));
            }
        }
    }

    fragments
}
