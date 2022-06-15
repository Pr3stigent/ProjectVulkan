use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 2],
    pub colour: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 2], colour: [f32; 3]) -> Self {
        Vertex {
            position: position,
            colour: colour,
        }
    }

    fn sub(self, vertex: Vertex) -> Vertex {
        let x = self.position[0] - vertex.position[0];
        let y = self.position[1] -  vertex.position[1];

        let r = self.colour[0] - vertex.colour[0];
        let g = self.colour[1] -  vertex.colour[1];
        let b = self.colour[2] -  vertex.colour[2];

        Vertex {
            position: [x, y],
            colour: [r, g, b],
        }
    }

    fn add(self, vertex: Vertex) -> Vertex {
        let x = self.position[0] + vertex.position[0];
        let y = self.position[1] +  vertex.position[1];

        let r = self.colour[0] + vertex.colour[0];
        let g = self.colour[1] +  vertex.colour[1];
        let b = self.colour[2] +  vertex.colour[2];

        Vertex {
            position: [x, y],
            colour: [r, g, b],
        }
    }

    fn div(self, vertex: Vertex) -> Vertex {
        let x = self.position[0] / vertex.position[0];
        let y = self.position[1] /  vertex.position[1];

        let r = self.colour[0] / vertex.colour[0];
        let g = self.colour[1] /  vertex.colour[1];
        let b = self.colour[2] /  vertex.colour[2];

        Vertex {
            position: [x, y],
            colour: [r, g, b],
        }
    }

    fn mul(self, vertex: Vertex) -> Vertex {
        let x = self.position[0] * vertex.position[0];
        let y = self.position[1] *  vertex.position[1];

        let r = self.colour[0] * vertex.colour[0];
        let g = self.colour[1] *  vertex.colour[1];
        let b = self.colour[2] *  vertex.colour[2];

        Vertex {
            position: [x, y],
            colour: [r, g, b],
        }
    }

    fn magnitude(self) -> f32 {
        let x = self.position[0];
        let y = self.position[1];
    
        x.powf(2.0) + y.powf(2.0)
    }
}

pub fn get_middle_position(vertices: Vec<Vertex>) -> Vertex {
    let mut x = vertices[1].position[0] + vertices[0].position[0];
    if x != 0.0 { x = x / 2.0 }

    let mut y = vertices[1].position[1] + vertices[2].position[1];
    if y != 0.0 { y = y / 2.0 }

    Vertex {
        position: [x, y],
        colour: [0.0 , 0.0, 0.0]
    }
}