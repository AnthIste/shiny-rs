extern crate gfx;

use gfx::{DeviceHelper, ToSlice};

use simulation::MySimulation;
use self::mesh::ToMesh;

mod mesh;
mod shader;

pub struct Scene<D: gfx::Device<C>, C: gfx::CommandBuffer>;

impl<D: gfx::Device<C>, C: gfx::CommandBuffer> Scene<D, C> {
    pub fn new() -> Scene<D, C> {
        Scene
    }

    pub fn render(&mut self, graphics: &mut gfx::Graphics<D, C>, frame: &gfx::Frame, simulation: &MySimulation) {
        let vertex_shader = shader::VERTEX_SRC.clone();
        let fragment_shader = shader::FRAGMENT_SRC.clone();
        let program = graphics.device.link_program(vertex_shader, fragment_shader).unwrap();

        for tri in simulation.triangles() {
            let mesh = tri.to_mesh(&mut graphics.device);
            let slice = mesh.to_slice(gfx::TriangleList);
            let batch: gfx::batch::RefBatch<(), ()> = graphics.make_batch(
                &program, &mesh, slice, &gfx::DrawState::new()).unwrap();

            graphics.draw(&batch, &(), frame);
        }
    }
}