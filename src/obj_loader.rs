#![allow(dead_code)]

use crate::vector::Vector3;
use crate::sphere::{Mesh, Vertex};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_obj(file_path: &str) -> Result<Mesh, String> {
    let file = File::open(file_path)
        .map_err(|e| format!("Error opening file {}: {}", file_path, e))?;
    let reader = BufReader::new(file);
    
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading line: {}", e))?;
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "v" => {
                // Vertex position
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse().map_err(|_| "Invalid vertex x coordinate")?;
                    let y: f32 = parts[2].parse().map_err(|_| "Invalid vertex y coordinate")?;
                    let z: f32 = parts[3].parse().map_err(|_| "Invalid vertex z coordinate")?;
                    positions.push(Vector3::new(x, y, z));
                }
            }
            "vn" => {
                // Vertex normal
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse().map_err(|_| "Invalid normal x coordinate")?;
                    let y: f32 = parts[2].parse().map_err(|_| "Invalid normal y coordinate")?;
                    let z: f32 = parts[3].parse().map_err(|_| "Invalid normal z coordinate")?;
                    normals.push(Vector3::new(x, y, z));
                }
            }
            "vt" => {
                // Texture coordinate
                if parts.len() >= 3 {
                    let u: f32 = parts[1].parse().map_err(|_| "Invalid texture u coordinate")?;
                    let v: f32 = parts[2].parse().map_err(|_| "Invalid texture v coordinate")?;
                    uvs.push((u, v));
                }
            }
            "f" => {
                // Face (triangle)
                if parts.len() >= 4 {
                    // Parse face indices (assuming triangulated mesh)
                    for i in 1..4 {
                        let face_data: Vec<&str> = parts[i].split('/').collect();
                        
                        // Position index (1-based in OBJ, convert to 0-based)
                        let pos_idx: usize = face_data[0].parse::<usize>()
                            .map_err(|_| "Invalid face position index")? - 1;
                        
                        // UV index (optional)
                        let uv_idx = if face_data.len() > 1 && !face_data[1].is_empty() {
                            face_data[1].parse::<usize>().ok().map(|idx| idx - 1)
                        } else {
                            None
                        };
                        
                        // Normal index (optional)
                        let normal_idx = if face_data.len() > 2 && !face_data[2].is_empty() {
                            face_data[2].parse::<usize>().ok().map(|idx| idx - 1)
                        } else {
                            None
                        };
                        
                        if pos_idx < positions.len() {
                            let position = positions[pos_idx];
                            
                            // Use provided normal or calculate from position (for sphere)
                            let normal = if let Some(idx) = normal_idx {
                                if idx < normals.len() {
                                    normals[idx]
                                } else {
                                    position.normalize() // Fallback for sphere
                                }
                            } else {
                                position.normalize() // Calculate normal for sphere
                            };
                            
                            // Use provided UV or calculate spherical UV
                            let uv = if let Some(idx) = uv_idx {
                                if idx < uvs.len() {
                                    uvs[idx]
                                } else {
                                    calculate_spherical_uv(position)
                                }
                            } else {
                                calculate_spherical_uv(position)
                            };
                            
                            vertices.push(Vertex {
                                position,
                                normal,
                                uv,
                            });
                            
                            indices.push((vertices.len() - 1) as u32);
                        }
                    }
                }
            }
            _ => {
                // Ignore other OBJ commands
            }
        }
    }
    
    if vertices.is_empty() {
        return Err("No vertices found in OBJ file".to_string());
    }
    
    println!("Loaded OBJ: {} vertices, {} indices", vertices.len(), indices.len());
    
    Ok(Mesh {
        vertices,
        indices,
    })
}

fn calculate_spherical_uv(position: Vector3) -> (f32, f32) {
    let normalized = position.normalize();
    let u = 0.5 + (normalized.z.atan2(normalized.x)) / (2.0 * std::f32::consts::PI);
    let v = 0.5 - (normalized.y.asin()) / std::f32::consts::PI;
    (u, v)
}