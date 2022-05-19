use crate::vertex::Vertex;

// Fixed model data
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.949397],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.84732914],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.2652641],
    }, // E
];

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub struct Model {
    z_rot: f32,
}

impl Model {
    pub fn new() -> Model {
        Model { z_rot: 0.0 }
    }
    pub fn build_transformation_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from_angle_z(cgmath::Deg(self.z_rot))
    }
    pub fn rotate_z_delta(&mut self, z_rot_delta: f32) {
        self.z_rot += z_rot_delta;
    }
}

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    transformation_matrix: [[f32; 4]; 4],
}

impl ModelUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            transformation_matrix: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_transformation_matrix(&mut self, model: &Model) {
        self.transformation_matrix = model.build_transformation_matrix().into();
    }
}
