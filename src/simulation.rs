use vector::Vec2;

pub trait Particle {
	fn update(&mut self, dt: f32);
}

pub trait Particle2D {
	fn new(acc: Vec2<f32>, vel: Vec2<f32>, pos: Vec2<f32>) -> Self;

	fn acc(&self) -> Vec2<f32>;

	fn vel(&self) -> Vec2<f32>;

	fn pos(&self) -> Vec2<f32>;
}

pub trait ParticleSystem {
	fn spawn(&mut self);

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
		self.vel = self.vel + acc * dt;
		self.pos = self.pos + self.vel * dt;
	}
}

impl Particle2D for TriangleParticle {
	fn new(acc: Vec2<f32>, vel: Vec2<f32>, pos: Vec2<f32>) -> TriangleParticle {
		TriangleParticle {
			pos: Vec2 { x: 0f32, y: 0f32 },
			vel: Vec2 { x: 0f32, y: 0f32 },
		}
	}

	fn acc(&self) -> Vec2<f32> {
		Vec2 { x: 0f32, y: 0f32 }
	}

	fn vel(&self) -> Vec2<f32> {
		self.vel
	}

	fn pos(&self) -> Vec2<f32> {
		self.pos
	}
}

pub struct ScatterSystem<T> {
	ps: Vec<T>,
}

impl<T: Particle + Particle2D> ScatterSystem<T> {
	pub fn new() -> ScatterSystem<T> {
		ScatterSystem {
			ps: Vec::new(),
		}
	}

	pub fn particles(&self) -> Iterator<T> {
		self.ps.iter() as Iterator<T>
	}
}

impl<T: Particle + Particle2D> ParticleSystem for ScatterSystem<T> {
	fn spawn(&mut self) {
		let initial_acc = Vec2 { x: 0f32, y: 0f32 };
		let initial_vel = Vec2 { x: 0f32, y: 0f32 };
		let initial_pos = Vec2 { x: 0f32, y: 0f32 };
		
		let new_particle = Particle2D::new(initial_acc, initial_vel, initial_pos);

		self.ps.push(new_particle);
	}

	fn update(&mut self, dt: f32) {
		for p in self.ps.mut_iter() {
			p.update(dt);
		}
	}
}

pub struct MySimulation {
	scattered_triangles: ScatterSystem<TriangleParticle>,
}

impl MySimulation {
	pub fn new() -> MySimulation {
		MySimulation {
			scattered_triangles: ScatterSystem::new(),
		}
	}

	pub fn update(&mut self, dt: f32) {
		self.scattered_triangles.update(dt);
	}
}