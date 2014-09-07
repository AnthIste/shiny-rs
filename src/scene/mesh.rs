extern crate gfx;

use gfx::{DeviceHelper};

use simulation::particle::TriangleParticle;
use scene::shader::Vertex;

static GRAPHICS_SCALE_FACTOR: f32 = 1.0;

pub trait ToMesh<D: gfx::Device<C>, C: gfx::CommandBuffer> {
    fn to_mesh(&self, device: &mut D) -> gfx::Mesh;
}

impl<D: gfx::Device<C>, C: gfx::CommandBuffer> ToMesh<D, C> for TriangleParticle {
    fn to_mesh(&self, device: &mut D) -> gfx::Mesh {
        let len = self.len * GRAPHICS_SCALE_FACTOR;
        let x = self.pos.x * GRAPHICS_SCALE_FACTOR; // Bad
        let y = self.pos.y * GRAPHICS_SCALE_FACTOR; // Bad

        let vertex_data = vec![
            Vertex { pos: [ -len + x, -len + y ], color: self.col },
            Vertex { pos: [  len + x, -len + y ], color: self.col },
            Vertex { pos: [  0.0 + x,  len + y ], color: self.col },
        ];

        // Here's our memory leak (but of course...)
        // Generating a mesh with the rendering offsets is not a great idea...
        // The mesh should be created once and then rendered in space...
        device.create_mesh(vertex_data)
    }
}