use mesh::Mesh;
use palette::Palette;
use vertex::Vertex;

pub const BLOCK_SIZE: usize = 16;

#[derive(Copy, Clone)]
enum Direction {
    XPlus,
    XMinus,
    YPlus,
    YMinus,
    ZPlus,
    ZMinus
}

const DIRECTION_ORDERING: &'static [Direction; 6] = &[
    Direction::XPlus,
    Direction::XMinus,
    Direction::YPlus,
    Direction::YMinus,
    Direction::ZPlus,
    Direction::ZMinus
];

impl Direction {
    fn shift_pos(&self, x: usize, y: usize, z: usize) -> Option<(usize, usize, usize)> {
        match *self {
            Direction::XPlus if x < BLOCK_SIZE - 1 => Some((x + 1, y, z)),
            Direction::XMinus if x > 0             => Some((x - 1, y, z)),
            Direction::YPlus if y < BLOCK_SIZE - 1 => Some((x, y + 1, z)),
            Direction::YMinus if y > 0             => Some((x, y - 1, z)),
            Direction::ZPlus if z < BLOCK_SIZE - 1 => Some((x, y, z + 1)),
            Direction::ZMinus if z > 0             => Some((x, y, z - 1)),
            _ => None
        }
    }

    fn iter_values() -> ::std::slice::Iter<'static, Direction> {
        DIRECTION_ORDERING.iter()
    }
}

struct Cube {
    x: usize,
    y: usize,
    z: usize
}

impl Cube {
    pub fn write_face(&self, mesh: &mut Mesh, dir: Direction, palette: &Palette, cell: u8) {
        let color = palette.get_color(cell);
        let blocksize = BLOCK_SIZE as f32;
        let x: f32 = self.x as f32 / blocksize - 0.5;
        let y: f32 = self.y as f32 / blocksize - 0.5;
        let z: f32 = self.z as f32 / blocksize - 0.5;
        let size = 1f32 / blocksize / 2f32;
        match dir {
            Direction::XPlus => {
                let normal = [1.0f32, 0.0, 0.0];
                let v1 = mesh.add_vertex(Vertex::new([x + size, y - size, z - size], color, normal));
                let v2 = mesh.add_vertex(Vertex::new([x + size, y + size, z - size], color, normal));
                let v3 = mesh.add_vertex(Vertex::new([x + size, y + size, z + size], color, normal));
                let v4 = mesh.add_vertex(Vertex::new([x + size, y - size, z + size], color, normal));
                mesh.add_triangle(v1, v2, v3);
                mesh.add_triangle(v3, v4, v1);
            },
            Direction::XMinus => {
                let normal = [-1.0f32, 0.0, 0.0];
                let v1 = mesh.add_vertex(Vertex::new([x - size, y - size, z - size], color, normal));
                let v2 = mesh.add_vertex(Vertex::new([x - size, y + size, z - size], color, normal));
                let v3 = mesh.add_vertex(Vertex::new([x - size, y + size, z + size], color, normal));
                let v4 = mesh.add_vertex(Vertex::new([x - size, y - size, z + size], color, normal));
                mesh.add_triangle(v1, v2, v3);
                mesh.add_triangle(v3, v4, v1);
            },
            Direction::YPlus => {
                let normal = [0.0, 1.0f32, 0.0];
                let v1 = mesh.add_vertex(Vertex::new([x - size, y + size, z - size], color, normal));
                let v2 = mesh.add_vertex(Vertex::new([x + size, y + size, z - size], color, normal));
                let v3 = mesh.add_vertex(Vertex::new([x + size, y + size, z + size], color, normal));
                let v4 = mesh.add_vertex(Vertex::new([x - size, y + size, z + size], color, normal));
                mesh.add_triangle(v1, v2, v3);
                mesh.add_triangle(v3, v4, v1);
            },
            Direction::YMinus => {
                let normal = [0.0, -1.0f32, 0.0];
                let v1 = mesh.add_vertex(Vertex::new([x - size, y - size, z - size], color, normal));
                let v2 = mesh.add_vertex(Vertex::new([x + size, y - size, z - size], color, normal));
                let v3 = mesh.add_vertex(Vertex::new([x + size, y - size, z + size], color, normal));
                let v4 = mesh.add_vertex(Vertex::new([x - size, y - size, z + size], color, normal));
                mesh.add_triangle(v1, v2, v3);
                mesh.add_triangle(v3, v4, v1);
            },
            Direction::ZPlus => {
                let normal = [0.0, 0.0, 1.0f32];
                let v1 = mesh.add_vertex(Vertex::new([x - size, y - size, z + size], color, normal));
                let v2 = mesh.add_vertex(Vertex::new([x - size, y + size, z + size], color, normal));
                let v3 = mesh.add_vertex(Vertex::new([x + size, y + size, z + size], color, normal));
                let v4 = mesh.add_vertex(Vertex::new([x + size, y - size, z + size], color, normal));
                mesh.add_triangle(v1, v2, v3);
                mesh.add_triangle(v3, v4, v1);
            },
            Direction::ZMinus => {
                let normal = [0.0, 0.0, -1.0f32];
                let v1 = mesh.add_vertex(Vertex::new([x - size, y - size, z - size], color, normal));
                let v2 = mesh.add_vertex(Vertex::new([x - size, y + size, z - size], color, normal));
                let v3 = mesh.add_vertex(Vertex::new([x + size, y + size, z - size], color, normal));
                let v4 = mesh.add_vertex(Vertex::new([x + size, y - size, z - size], color, normal));
                mesh.add_triangle(v1, v2, v3);
                mesh.add_triangle(v3, v4, v1);
            }
        }
    }
}

pub struct VoxelBlock {
    pub data: [[[u8; BLOCK_SIZE]; BLOCK_SIZE]; BLOCK_SIZE]
}

impl VoxelBlock {
    pub fn new() -> VoxelBlock {
        VoxelBlock {
            data: [[[0u8; BLOCK_SIZE]; BLOCK_SIZE]; BLOCK_SIZE]
        }
    }

    fn is_occupied(&self, x: usize, y: usize, z: usize) -> bool {
        self.data[x][y][z] > 0
    }

    pub fn to_mesh(&self, palette: &Palette) -> Mesh {
        let mut mesh = Mesh::empty();

        for x in 0..BLOCK_SIZE {
            for y in 0..BLOCK_SIZE {
                for z in 0..BLOCK_SIZE {
                    let current = self.data[x][y][z];
                    
                    if current > 0 {
                        let cube = Cube { x: x, y: y, z: z };

                        for dir in Direction::iter_values() {
                            match dir.shift_pos(x, y, z) {
                                Some((nx, ny, nz)) if !self.is_occupied(nx, ny, nz) => {
                                    cube.write_face(&mut mesh, *dir, palette, current);
                                },
                                None => {
                                    cube.write_face(&mut mesh, *dir, palette, current);
                                },
                                _ => ()
                            }
                        }
                    }
                }
            }
        }

        mesh
    }
}
