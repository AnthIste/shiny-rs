use std::slice::Items;

use self::particle::{Particle, TriangleParticle};
use self::system::{ParticleSystem, ScatterSystem};

pub mod particle;
pub mod system;

static TIME_SCALE_FACTOR: f32 = 0.01f32;

pub struct MySimulation {
    scattered_triangles: ScatterSystem<TriangleParticle>,
}

impl MySimulation {
    pub fn new() -> MySimulation {
        MySimulation {
            scattered_triangles: ScatterSystem::new(),
        }
    }

    pub fn spawn(&mut self) {
        self.scattered_triangles.spawn();
    }

    pub fn update(&mut self, dt: f32) {
        self.scattered_triangles.update(dt * TIME_SCALE_FACTOR);
    }

    pub fn triangles(&self) -> Items<TriangleParticle> {
        self.scattered_triangles.particles()
    }
}