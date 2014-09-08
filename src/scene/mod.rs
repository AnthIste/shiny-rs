extern crate cgmath;
extern crate gfx;
extern crate device; // This feels really dodgy

use std::collections::HashMap;
use self::cgmath::{FixedArray, Matrix, Point3, Vector3}; // Why do these need 'self' but not gfx???
use self::cgmath::{Transform, AffineMatrix3};
use gfx::{DeviceHelper, ToSlice};

use simulation::MySimulation;
use simulation::particle::{Particle, TriangleParticle};
use self::mesh::ToMesh;
use self::shader::{CubeBatch, Params};

mod mesh;
mod shader;

type ShaderProgram = device::Handle<u32, gfx::ProgramInfo>;

pub struct Scene<D: gfx::Device<C>, C: gfx::CommandBuffer> {
    shader_program: ShaderProgram,
    triangle_meshes: HashMap<u64, gfx::Mesh>,
}

impl<D: gfx::Device<C>, C: gfx::CommandBuffer> Scene<D, C> {
    pub fn new(graphics: &mut gfx::Graphics<D, C>) -> Scene<D, C> {
        let vertex_shader = shader::VERTEX_SRC.clone();
        let fragment_shader = shader::FRAGMENT_SRC.clone();
        let shader_program = graphics.device.link_program(vertex_shader, fragment_shader).unwrap();

        Scene {
            shader_program: shader_program,
            triangle_meshes: HashMap::new(),
        }
    }

    pub fn render(&mut self, graphics: &mut gfx::Graphics<D, C>, frame: &gfx::Frame, simulation: &MySimulation) {
        for tri in simulation.triangles() {
            let mesh = self.get_triangle_mesh(tri, graphics);
            let slice = mesh.to_slice(gfx::TriangleList);
            let batch: CubeBatch = graphics.make_batch(
                &self.shader_program, &mesh, slice, &gfx::DrawState::new()).unwrap();

            let view: AffineMatrix3<f32> = Transform::look_at(
                &Point3::new(1.5f32, -5.0, 3.0),
                &Point3::new(0f32, 0.0, 0.0),
                &Vector3::unit_z(),
            );
            let aspect = 16f32 / 9f32; // Fixed 16:9
            let proj = cgmath::perspective(cgmath::deg(45.0f32), aspect, 1.0, 10.0);

            let shader_args = Params {
                transform: proj.mul_m(&view.mat).into_fixed(),
            };

            graphics.draw(&batch, &shader_args, frame);
        }
    }

    fn get_triangle_mesh(&mut self, tri: &TriangleParticle, graphics: &mut gfx::Graphics<D, C>) -> gfx::Mesh {
        let key = tri.id();

        match self.triangle_meshes.find_copy(&key) {
            Some(mesh) => mesh,
            None => {
                let mesh = tri.to_mesh(&mut graphics.device);
                self.triangle_meshes.insert(key, mesh);

                self.triangle_meshes.get_copy(&key) // FIXME: There must be a better way...
            }
        }
    }
}