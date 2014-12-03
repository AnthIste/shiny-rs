extern crate gfx;
extern crate cgmath;

use gfx::{DeviceHelper};
use self::cgmath::{BaseNum, Matrix, Matrix4, Vector, Vector2, Vector3};

use simulation::particle::TriangleParticle;
use scene::shader::Vertex;

static GRAPHICS_SCALE_FACTOR: f32 = 1.0;

pub trait ToMesh<D: gfx::Device<C>, C: gfx::CommandBuffer> {
    fn to_mesh(&self, device: &mut D) -> gfx::Mesh;
}

pub trait ToMatrix<T: BaseNum> {
    fn to_matrix(&self) -> Matrix4<T>;
}

impl<D: gfx::Device<C>, C: gfx::CommandBuffer> ToMesh<D, C> for TriangleParticle {
    fn to_mesh(&self, device: &mut D) -> gfx::Mesh {
        let len = self.len * GRAPHICS_SCALE_FACTOR;

        let vertex_data = [
            Vertex { pos: [ -len, -len ], color: self.col },
            Vertex { pos: [  len, -len ], color: self.col },
            Vertex { pos: [  0.0,  len ], color: self.col },
        ];

        device.create_mesh(&vertex_data)
    }
}

impl ToMatrix<f32> for TriangleParticle {
    fn to_matrix(&self) -> Matrix4<f32> {
        let v: Vector3<f32> = self.pos.extend(0.0f32);
        let m: Matrix4<f32> = Matrix4::from_translation(&v); // Translate

        m
    }
}