extern crate cgmath;

use std::slice::Items;
use std::rand::{task_rng, Rng};
use self::cgmath::Vector2; // Why is self needed???

use simulation::particle::{Particle, Particle2D};

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
		for _ in range(0i, 5i) {
			let mut rng = task_rng();

			// TODO: generate vectors of random magnitute and random direction
			// but ensure that the magnitude has a reasonable lower bound
			let rand_x: int = rng.gen_range(0, 100);
			let rand_y: int = rng.gen_range(0, 100);
			let rand_x_dir: int = if rng.gen::<bool>() { 1 } else { -1 };
			let rand_y_dir: int = if rng.gen::<bool>() { 1 } else { -1 };

			let initial_acc = Vector2 { x: 0.0f32, y: 0.0f32 };
			let initial_vel = Vector2 { x: (rand_x * rand_x_dir) as f32, y: (rand_y * rand_y_dir) as f32 };
			let initial_pos = Vector2 { x: 0.0f32, y: 0.0f32 };
			
			let new_particle = Particle2D::new(initial_acc, initial_vel, initial_pos);

			self.ps.push(new_particle);
		}
	}

	fn update(&mut self, dt: f32) {
		for p in self.ps.iter_mut() {
			p.update(dt);
		}
	}
}