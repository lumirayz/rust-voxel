#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3]
}

implement_vertex!(Vertex, position, color, normal);

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 4], normal: [f32; 3]) -> Vertex {
        Vertex {position: position, color: color, normal: normal}
    }
}
