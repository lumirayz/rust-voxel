#[macro_use] extern crate glium;
extern crate nalgebra as na;
extern crate rand;

mod mesh;
mod vertex;
mod palette;
mod voxelblock;

use glium::glutin;
use glium::Surface;

use na::{Mat4, PerspMat3};

use rand::distributions::{IndependentSample, Range};

const VERTEX_SHADER_SOURCE: &'static str = r#"
    #version 140

    in vec3 position;
    in vec4 color;
    in vec3 normal;

    out vec4 v_color;
    out vec3 v_normal;

    uniform mat4 mat;
    uniform mat4 persp;

    void main() {
        v_color = color;
        v_normal = transpose(inverse(mat3(mat))) * normal;
        gl_Position = persp * mat * vec4(position, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"
    #version 140
    
    in vec4 v_color;
    in vec3 v_normal;

    out vec4 color;

    uniform vec3 u_light;

    void main() {
        float brightness = dot(normalize(v_normal), normalize(u_light));
        color = vec4(mix(v_color.xyz * 0.1, v_color.xyz, brightness), v_color.z);
    }
"#;

fn main() {
    use glium::DisplayBuild;

    let display = glutin::WindowBuilder::new()
        .with_dimensions(640, 480)
        .with_title("hai".to_string())
        .build_glium()
        .unwrap();

    let mut block = voxelblock::VoxelBlock::new();

    let rand_range = Range::new(0u8, 4u8);
    let mut rng = rand::thread_rng();
    for x in 0..voxelblock::BLOCK_SIZE {
        for y in 0..voxelblock::BLOCK_SIZE {
            for z in 0..voxelblock::BLOCK_SIZE {
                block.data[x][y][z] = rand_range.ind_sample(&mut rng);
            }
        }
    }

    let pal = palette::Palette::new(vec![
        [1.0f32, 0.0, 0.0, 1.0],
        [0.0f32, 1.0, 0.0, 1.0],
        [0.0f32, 0.0, 1.0, 1.0]
    ]);

    let mesh = block.to_mesh(&pal);

    let vertex_buffer = mesh.create_vbo(&display).unwrap();
    let indices = mesh.create_ibo(&display).unwrap();

    let program = glium::Program::from_source(&display,
        VERTEX_SHADER_SOURCE,
        FRAGMENT_SHADER_SOURCE, None).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        ..Default::default()
    };

    let trans = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 2.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let mut t = 0f32;

    loop {
        t += 0.01;

        let mut mat = Mat4::new(
            t.cos(), 0., t.sin(), 0.,
            0., 1., 0., 0.,
            -t.sin(), 0., t.cos(), 0.,
            0., 0., 0., 1.
        );
        
        mat = trans * mat;
        
        let mut frame = display.draw();
        let (w, h) = frame.get_dimensions();
        let persp = PerspMat3::new(w as f32 / h as f32, 3.141592 / 3.0, 0.1, 1024.0);
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniform! {mat: mat, persp: persp, u_light: [0.0f32, 0.5, -1.0]},
            &params).unwrap();
        frame.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
