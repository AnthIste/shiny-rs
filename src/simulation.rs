use vector::Vec2;

pub trait Particle {
	fn update(&mut self, dt: f32);
}

pub trait ParticleSystem {
	fn update(&mut self, dt: f32);
}

pub struct TriangleParticle {
	pos: Vec2<f32>,
	vel: Vec2<f32>,
}

impl TriangleParticle {
	fn new(pos: Vec2<f32>) -> TriangleParticle {
		TriangleParticle {
			pos: pos,
			vel: Vec2 { x: 0f32, y: 0f32 },
		}
	}
}

impl Particle for TriangleParticle {
	fn update(&mut self, dt: f32) {
		let acc = Vec2 { x: 0f32, y: 0f32 };

		// Acceleration
		self.vel = self.vel + acc * dt;

		// Velocity
		self.pos = self.pos + self.vel * dt;
	}
}

pub struct ScatterSystem<T: Particle> {
	ps: Vec<T>,
}

impl<T: Particle> ParticleSystem for ScatterSystem<T> {
	fn update(&mut self, dt: f32) {
		// NOP
	}
}