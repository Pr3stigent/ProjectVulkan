use bytemuck::{Pod, Zeroable};

trait Position {
    fn sub(self, vertex: Vertex) -> Vertex;

    fn add(self, vertex: Vertex) -> Vertex;

    fn div(self, vertex: Vertex) -> Vertex;

    fn mul(self, vertex: Vertex) -> Vertex;

    fn magnitude(self) -> f32;
}

#[allow(dead_code)]
pub fn get_middle_position(vertices: Vec<[f32; 2]>) -> [f32; 2] {
    let mut x = vertices[1][0] + vertices[0][0];
    if x != 0.0 { x = x / 2.0 }

    let mut y = vertices[1][1] + vertices[2][1];
    if y != 0.0 { y = y / 2.0 }

    [x, y]
}

#[allow(dead_code)]
fn sub_position(position: [f32; 2], position2: [f32; 2]) -> [f32; 2] {
    let x = position[0] - position2[0];
    let y = position[1] -  position2[1];
    [x, y]
}

#[allow(dead_code)]
fn sub_colour(colour: [f32; 3], colour2: [f32; 3]) -> [f32; 3] {
    let r = colour[0] - colour2[0];
    let g = colour[1] - colour2[1];
    let b = colour[2] - colour2[2];
    [r, g, b]
}

#[allow(dead_code)]
fn add_position(position: [f32; 2], position2: [f32; 2]) -> [f32; 2] {
    let x = position[0] + position2[0];
    let y = position[1] +  position2[1];
    [x, y]
}

#[allow(dead_code)]
fn add_colour(colour: [f32; 3], colour2: [f32; 3]) -> [f32; 3] {
    let r = colour[0] + colour2[0];
    let g = colour[1] + colour2[1];
    let b = colour[2] + colour2[2];
    [r, g, b]
}

#[allow(dead_code)]
fn mul_position(position: [f32; 2], position2: [f32; 2]) -> [f32; 2] {
    let x = position[0] * position2[0];
    let y = position[1] *  position2[1];
    [x, y]
}

#[allow(dead_code)]
fn mul_colour(colour: [f32; 3], colour2: [f32; 3]) -> [f32; 3] {
    let r = colour[0] * colour2[0];
    let g = colour[1] * colour2[1];
    let b = colour[2] * colour2[2];
    [r, g, b]
}

#[allow(dead_code)]
fn div_position(position: [f32; 2], position2: [f32; 2]) -> [f32; 2] {
    let x = position[0] / position2[0];
    let y = position[1] / position2[1];
    [x, y]
}

#[allow(dead_code)]
fn div_colour(colour: [f32; 3], colour2: [f32; 3]) -> [f32; 3] {
    let r = colour[0] / colour2[0];
    let g = colour[1] / colour2[1];
    let b = colour[2] / colour2[2];
    [r, g, b]
}

#[allow(dead_code)]
fn magnitude(position: [f32; 2])-> f32 {
    let x = position[0];
    let y = position[1];

    x.powf(2.0) + y.powf(2.0)
}

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 2],
    pub colour: [f32; 3],
}

impl Vertex {
    #[allow(dead_code)]
    pub fn sub(self, vertex: Vertex) -> Vertex {
        Vertex { position:  sub_position(self.position, vertex.position), colour:  sub_colour(self.colour, vertex.colour) }
    }

    #[allow(dead_code)]
    pub fn add(self, vertex: Vertex) -> Vertex {
        Vertex { position:  add_position(self.position, vertex.position), colour:  add_colour(self.colour, vertex.colour) }
    }

    #[allow(dead_code)]
    pub fn div(self, vertex: Vertex) -> Vertex {
        Vertex { position:  div_position(self.position, vertex.position), colour:  div_colour(self.colour, vertex.colour) }
    }

    #[allow(dead_code)]
    pub fn mul(self, vertex: Vertex) -> Vertex {
        Vertex { position:  mul_position(self.position, vertex.position), colour:  mul_colour(self.colour, vertex.colour) }
    }

    #[allow(dead_code)]
    pub fn magnitude(self) -> f32 { magnitude(self.position) }
}

#[allow(dead_code)]
pub struct Rect {
    pub position: [f32; 2],
}

impl Rect {
    #[allow(dead_code)]
    pub fn sub(self, vertex: Rect) -> Rect {
        Rect { position: sub_position(self.position, vertex.position) }
    }

    #[allow(dead_code)]
    pub fn add(self, vertex: Rect) -> Rect {
        Rect { position: add_position(self.position, vertex.position) }
    }

    #[allow(dead_code)]
    pub fn div(self, vertex: Rect) -> Rect {
        Rect { position: div_position(self.position, vertex.position) }
    }

    #[allow(dead_code)]
    pub fn mul(self, vertex: Rect) -> Rect {
        Rect { position: mul_position(self.position, vertex.position) }
    }

    #[allow(dead_code)]
    pub fn magnitude(self) -> f32 { magnitude(self.position) }
}