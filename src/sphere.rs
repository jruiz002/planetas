use crate::vector::Vector3;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub uv: (f32, f32),
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Generates a sphere mesh with the given radius and subdivisions
    pub fn create_sphere(radius: f32, rings: u32, sectors: u32) -> Self {
        let mut mesh = Mesh::new();
        
        let ring_step = std::f32::consts::PI / rings as f32;
        let sector_step = 2.0 * std::f32::consts::PI / sectors as f32;

        // Generate vertices
        for i in 0..=rings {
            let ring_angle = std::f32::consts::PI / 2.0 - i as f32 * ring_step;
            let xy = radius * ring_angle.cos();
            let z = radius * ring_angle.sin();

            for j in 0..=sectors {
                let sector_angle = j as f32 * sector_step;
                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();

                let position = Vector3::new(x, y, z);
                let normal = position.normalize();
                let u = j as f32 / sectors as f32;
                let v = i as f32 / rings as f32;

                mesh.vertices.push(Vertex {
                    position,
                    normal,
                    uv: (u, v),
                });
            }
        }

        // Generate indices
        for i in 0..rings {
            let k1 = i * (sectors + 1);
            let k2 = k1 + sectors + 1;

            for j in 0..sectors {
                if i != 0 {
                    mesh.indices.push(k1 + j);
                    mesh.indices.push(k2 + j);
                    mesh.indices.push(k1 + j + 1);
                }

                if i != rings - 1 {
                    mesh.indices.push(k1 + j + 1);
                    mesh.indices.push(k2 + j);
                    mesh.indices.push(k2 + j + 1);
                }
            }
        }

        mesh
    }
}