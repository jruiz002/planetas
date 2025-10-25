use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    pub data: [[f32; 4]; 4],
}

impl Matrix {
    pub fn identity() -> Self {
        Matrix {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Self {
        Matrix {
            data: [
                [m00, m01, m02, m03],
                [m10, m11, m12, m13],
                [m20, m21, m22, m23],
                [m30, m31, m32, m33],
            ],
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::identity();
        
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        
        result
    }

    pub fn transform_vector(&self, v: &Vector3) -> Vector3 {
        let x = self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z + self.data[0][3];
        let y = self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z + self.data[1][3];
        let z = self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z + self.data[2][3];
        let w = self.data[3][0] * v.x + self.data[3][1] * v.y + self.data[3][2] * v.z + self.data[3][3];
        
        if w != 0.0 {
            Vector3::new(x / w, y / w, z / w)
        } else {
            Vector3::new(x, y, z)
        }
    }
}

pub fn new_matrix4(
    m00: f32, m01: f32, m02: f32, m03: f32,
    m10: f32, m11: f32, m12: f32, m13: f32,
    m20: f32, m21: f32, m22: f32, m23: f32,
    m30: f32, m31: f32, m32: f32, m33: f32,
) -> Matrix {
    Matrix::new(
        m00, m01, m02, m03,
        m10, m11, m12, m13,
        m20, m21, m22, m23,
        m30, m31, m32, m33,
    )
}

/// Creates a view matrix using camera position, target, and up vector
/// This implements a lookAt matrix for camera transformations
pub fn create_view_matrix(eye: Vector3, target: Vector3, up: Vector3) -> Matrix {
    // Calculate forward vector (from eye to target, normalized)
    let mut forward = Vector3::new(
        target.x - eye.x,
        target.y - eye.y,
        target.z - eye.z,
    );
    // Normalize forward
    let forward_length = (forward.x * forward.x + forward.y * forward.y + forward.z * forward.z).sqrt();
    forward.x /= forward_length;
    forward.y /= forward_length;
    forward.z /= forward_length;

    // Calculate right vector (cross product of forward and up, normalized)
    let mut right = Vector3::new(
        forward.y * up.z - forward.z * up.y,
        forward.z * up.x - forward.x * up.z,
        forward.x * up.y - forward.y * up.x,
    );
    // Normalize right
    let right_length = (right.x * right.x + right.y * right.y + right.z * right.z).sqrt();
    right.x /= right_length;
    right.y /= right_length;
    right.z /= right_length;

    // Calculate actual up vector (cross product of right and forward)
    let actual_up = Vector3::new(
        right.y * forward.z - right.z * forward.y,
        right.z * forward.x - right.x * forward.z,
        right.x * forward.y - right.y * forward.x,
    );

    // Create the view matrix (inverse of camera transformation)
    // This is the lookAt matrix formula
    new_matrix4(
        right.x, right.y, right.z, -(right.x * eye.x + right.y * eye.y + right.z * eye.z),
        actual_up.x, actual_up.y, actual_up.z, -(actual_up.x * eye.x + actual_up.y * eye.y + actual_up.z * eye.z),
        -forward.x, -forward.y, -forward.z, forward.x * eye.x + forward.y * eye.y + forward.z * eye.z,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Creates a perspective projection matrix
/// fov_y: Field of view in radians (vertical)
/// aspect: Aspect ratio (width / height)
/// near: Near clipping plane distance
/// far: Far clipping plane distance
pub fn create_projection_matrix(fov_y: f32, aspect: f32, near: f32, far: f32) -> Matrix {
    let tan_half_fov = (fov_y / 2.0).tan();

    new_matrix4(
        1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0,
        0.0, 1.0 / tan_half_fov, 0.0, 0.0,
        0.0, 0.0, -(far + near) / (far - near), -(2.0 * far * near) / (far - near),
        0.0, 0.0, -1.0, 0.0,
    )
}

/// Creates a viewport matrix to transform NDC coordinates to screen space
/// x, y: Viewport position (typically 0, 0)
/// width, height: Viewport dimensions in pixels
pub fn create_viewport_matrix(x: f32, y: f32, width: f32, height: f32) -> Matrix {
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    new_matrix4(
        half_width, 0.0, 0.0, x + half_width,
        0.0, -half_height, 0.0, y + half_height,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Creates a rotation matrix around the Y axis
pub fn create_rotation_y(angle: f32) -> Matrix {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    new_matrix4(
        cos_a, 0.0, sin_a, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_a, 0.0, cos_a, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Creates a translation matrix
pub fn create_translation(x: f32, y: f32, z: f32) -> Matrix {
    new_matrix4(
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0,
    )
}