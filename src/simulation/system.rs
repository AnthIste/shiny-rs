use std::slice::Items;

use simulation::particle::{Particle, Particle2D};
use util::vector::Vec2;

/// Main particle system trait - a system emits new particles and updates particles over time
pub trait ParticleSystem {
    fn emit(&mut self);

    fn update(&mut self, dt: f32);
}

pub struct ScatterSystem<T: Particle + Particle2D> {
	ps: Vec<T>,
}

impl<T: Particle + Particle2D> ScatterSystem<T> {
	pub fn new() -> ScatterSystem<T> {
		ScatterSystem {
			ps: Vec::new(),
		}
	}

	pub fn particles(&self) -> Items<T> {
		self.ps.iter()
	}
}

impl<T: Particle + Particle2D> ParticleSystem for ScatterSystem<T> {
	fn emit(&mut self) {
		let initial_acc = Vec2 { x: 0.0f32, y: 0.0f32 };
		let initial_vel = Vec2 { x: 1.0f32, y: 0.0f32 }; // Move towards right
		let initial_pos = Vec2 { x: 0.0f32, y: 0.0f32 };
		
		let new_particle = Particle2D::new(initial_acc, initial_vel, initial_pos);

		self.ps.push(new_particle);
	}

	fn update(&mut self, dt: f32) {
		for p in self.ps.mut_iter() {
			p.update(dt);
		}
	}
}