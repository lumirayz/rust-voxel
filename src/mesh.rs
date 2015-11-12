extern crate glium;

use glium::backend::Facade;
use glium::vertex::BufferCreationError as VertexCreationError;
use glium::index::BufferCreationError as IndexCreationError;

use vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>
}

impl Mesh {
    pub fn empty() -> Mesh {
        Mesh { vertices: Vec::new(), indices: Vec::new() }
    }

    pub fn create_vbo<F: Facade>(&self, facade: &F) -> Result<glium::VertexBuffer<Vertex>, VertexCreationError> {
        glium::VertexBuffer::new(facade, &self.vertices)
    }

    pub fn create_ibo<F: Facade>(&self, facade: &F) -> Result<glium::IndexBuffer<u32>, IndexCreationError> {
        glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &self.indices)
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> u32 {
        self.vertices.push(vertex);
        (self.vertices.len() - 1) as u32
    }

    pub fn add_triangle(&mut self, index1: u32, index2: u32, index3: u32) {
        self.indices.push(index1);
        self.indices.push(index2);
        self.indices.push(index3);
    }
}
