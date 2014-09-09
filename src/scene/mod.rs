extern crate cgmath;
extern crate gfx;
extern crate device; // This feels really dodgy

use std::collections::HashMap;
use self::cgmath::{Transform, FixedArray, Matrix, Matrix2, Matrix3, Matrix4, Point3, Vector, Vector2, Vector3, Vector4, AffineMatrix3}; // Why do these need 'self' but not gfx???
use gfx::{DeviceHelper, ToSlice};

use simulation::MySimulation;
use simulation::particle::{Particle, TriangleParticle};
use self::mesh::{ToMesh, ToMatrix};
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

            // let aspect = 640f32 / 480f32;
            // let v: Vector3<f32> = Vector2::zero().extend(0.0f32);
            // let m: Matrix4<f32> = Matrix4::from_translation(&v);

            // let model = Matrix4::identity().mul_v(&v);
            // let model = m;
            let model = tri.to_matrix();
            let view: AffineMatrix3<f32> = Transform::look_at(
                &Point3::new(0.0f32, 0.0f32, 1.0f32), // Position on z axis
                &Point3::new(0.0f32, 0.0f32, 0.0f32), // Look down to origin
                &Vector3::unit_y(), // We are rotated 90 degrees along the x axis, so 'up' is on the y axis
            );
            let projection = cgmath::ortho(-1.0f32, 1.0f32, -1.0f32, 1.0f32, -1.0f32, 1.0f32);

            let mvp = projection * view.mat * model;

            let shader_args = Params {
                transform: mvp.into_fixed(),
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