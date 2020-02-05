use crate::webgl_rendering_context::WebGLRenderingContext;

pub trait Model {
    fn positions(&self) -> Vec<f32>;
    fn colors(&self) -> Vec<f32>;
    fn indices(&self) -> Vec<u8>;
    fn render(&self, gl: &WebGLRenderingContext);
}

pub struct Cube {
    positions: [f32; 72],
    colors: [f32; 24],
    indices: [u8; 36],
}

impl Cube {
    pub fn new() -> Self {
        Self {
            positions: [
                // Front face
                -1.0, -1.0,  1.0,
                1.0, -1.0,  1.0,
                1.0,  1.0,  1.0,
                -1.0,  1.0,  1.0,

                // Back face
                -1.0, -1.0, -1.0,
                -1.0,  1.0, -1.0,
                1.0,  1.0, -1.0,
                1.0, -1.0, -1.0,

                // Top face
                -1.0,  1.0, -1.0,
                -1.0,  1.0,  1.0,
                1.0,  1.0,  1.0,
                1.0,  1.0, -1.0,

                // Bottom face
                -1.0, -1.0, -1.0,
                1.0, -1.0, -1.0,
                1.0, -1.0,  1.0,
                -1.0, -1.0,  1.0,

                // Right face
                1.0, -1.0, -1.0,
                1.0,  1.0, -1.0,
                1.0,  1.0,  1.0,
                1.0, -1.0,  1.0,

                // Left face
                -1.0, -1.0, -1.0,
                -1.0, -1.0,  1.0,
                -1.0,  1.0,  1.0,
                -1.0,  1.0, -1.0
            ],
            colors: [
                1.0,  1.0,  1.0,  1.0,    // Front face: white
                1.0,  0.0,  0.0,  1.0,    // Back face: red
                0.0,  1.0,  0.0,  1.0,    // Top face: green
                0.0,  0.0,  1.0,  1.0,    // Bottom face: blue
                1.0,  1.0,  0.0,  1.0,    // Right face: yellow
                1.0,  0.0,  1.0,  1.0,    // Left face: purple
            ],
            indices: [
                0,  1,  2,      0,  2,  3,    // front
                4,  5,  6,      4,  6,  7,    // back
                8,  9,  10,     8,  10, 11,   // top
                12, 13, 14,     12, 14, 15,   // bottom
                16, 17, 18,     16, 18, 19,   // right
                20, 21, 22,     20, 22, 23,   // left
            ]
        }
    }
}

impl Model for Cube {
    fn positions(&self) -> Vec<f32> {
        self.positions.to_vec()
    }

    fn colors(&self) -> Vec<f32> {
        self.colors.to_vec()
    }

    fn indices(&self) -> Vec<u8> {
        self.indices.to_vec()
    }

    fn render(&self, _gl: &WebGLRenderingContext) {
        let _num_components = 3;  // todo(heyzoos): Should this be a struct property?
    }
}

