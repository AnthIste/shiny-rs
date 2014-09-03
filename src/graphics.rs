extern crate gfx;

use gfx::{DeviceHelper, ToSlice};

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

pub struct MyGraphics<'a, G: 'a> {
    graphics: &'a mut G
}

impl<'a, D: gfx::Device<C>, C: gfx::CommandBuffer> MyGraphics<'a, gfx::Graphics<D, C>> {
    pub fn new(graphics: &'a mut gfx::Graphics<D, C>) -> MyGraphics<'a, gfx::Graphics<D, C>> {
        MyGraphics {
            graphics: graphics
        }
    }

    pub fn render(&mut self, frame: &gfx::Frame) {
        let vertex_data = vec![
            Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
            Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] },
        ];
        let mesh = self.graphics.device.create_mesh(vertex_data);
        let slice = mesh.to_slice(gfx::TriangleList);

        let program = self.graphics.device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone())
                                 .unwrap();

        let batch: gfx::batch::RefBatch<(), ()> = self.graphics.make_batch(
            &program, &mesh, slice, &gfx::DrawState::new()).unwrap();

        let clear_data = gfx::ClearData {
            color: Some([0.3, 0.3, 0.3, 1.0]),
            depth: None,
            stencil: None,
        };

        self.graphics.clear(clear_data, frame);
        self.graphics.draw(&batch, &(), frame);
        self.graphics.end_frame();
    }
}