extern crate gfx;

use gfx::{DeviceHelper, ToSlice};
use simulation::{MySimulation, TriangleParticle};

#[vertex_format]
struct Vertex {
    #[name = "a_Pos"]
    pos: [f32, ..2],

    #[name = "a_Color"]
    color: [f32, ..3],
}

static VERTEX_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120

    attribute vec2 a_Pos;
    attribute vec3 a_Color;
    varying vec4 v_Color;

    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = vec4(a_Pos, 0.0, 1.0);
    }
"
GLSL_150: b"
    #version 150 core

    in vec2 a_Pos;
    in vec3 a_Color;
    out vec4 v_Color;

    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = vec4(a_Pos, 0.0, 1.0);
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120

    varying vec4 v_Color;

    void main() {
        gl_FragColor = v_Color;
    }
"
GLSL_150: b"
    #version 150 core

    in vec4 v_Color;
    out vec4 o_Color;

    void main() {
        o_Color = v_Color;
    }
"
};

pub struct MyGraphics<G>;

impl<D: gfx::Device<C>, C: gfx::CommandBuffer> MyGraphics<gfx::Graphics<D, C>> {
    pub fn new() -> MyGraphics<gfx::Graphics<D, C>> {
        MyGraphics
    }

    pub fn render(&mut self, graphics: &mut gfx::Graphics<D, C>, frame: &gfx::Frame, simulation: &MySimulation) {
        let program = graphics.device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone()).unwrap();

        for tri in simulation.triangles() {
            let mesh = tri.to_mesh(&mut graphics.device);
            let slice = mesh.to_slice(gfx::TriangleList);
            let batch: gfx::batch::RefBatch<(), ()> = graphics.make_batch(
                &program, &mesh, slice, &gfx::DrawState::new()).unwrap();

            graphics.draw(&batch, &(), frame);
        }
    }
}

trait ToMesh<D: gfx::Device<C>, C: gfx::CommandBuffer> {
    fn to_mesh(&self, device: &mut D) -> gfx::Mesh;
}

impl<D: gfx::Device<C>, C: gfx::CommandBuffer> ToMesh<D, C> for TriangleParticle {
    fn to_mesh(&self, device: &mut D) -> gfx::Mesh {
        let vertex_data = vec![
            Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
            Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] },
        ];

        device.create_mesh(vertex_data)
    }
}