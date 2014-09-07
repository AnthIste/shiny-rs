extern crate gfx;
extern crate device; // This feels really dodgy

use self::gfx::{DeviceHelper, ToSlice};

use simulation::MySimulation;
use self::mesh::ToMesh;

mod mesh;
mod shader;

type ShaderProgram = device::Handle<u32, gfx::ProgramInfo>;

pub struct Scene<D: gfx::Device<C>, C: gfx::CommandBuffer> {
    shader_program: ShaderProgram,
}

impl<D: gfx::Device<C>, C: gfx::CommandBuffer> Scene<D, C> {
    pub fn new(graphics: &mut gfx::Graphics<D, C>) -> Scene<D, C> {
        let vertex_shader = shader::VERTEX_SRC.clone();
        let fragment_shader = shader::FRAGMENT_SRC.clone();
        let shader_program = graphics.device.link_program(vertex_shader, fragment_shader).unwrap();

        Scene {
            shader_program: shader_program,
        }
    }

    pub fn render(&mut self, graphics: &mut gfx::Graphics<D, C>, frame: &gfx::Frame, simulation: &MySimulation) {
        for tri in simulation.triangles() {
            let mesh = tri.to_mesh(&mut graphics.device);
            let slice = mesh.to_slice(gfx::TriangleList);
            let batch: gfx::batch::RefBatch<(), ()> = graphics.make_batch(
                &self.shader_program, &mesh, slice, &gfx::DrawState::new()).unwrap();

            graphics.draw(&batch, &(), frame);
        }
    }
}