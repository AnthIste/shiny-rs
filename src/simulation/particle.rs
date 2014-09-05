use util::vector::Vec2;

pub trait Particle {
    fn update(&mut self, dt: f32);
}

pub trait Particle2D {
	fn new(acc: Vec2<f32>, vel: Vec2<f32>, pos: Vec2<f32>) -> Self;

	fn acc(&self) -> Vec2<f32>;

	fn vel(&self) -> Vec2<f32>;

	fn pos(&self) -> Vec2<f32>;
}

pub struct TriangleParticle {
	pub pos: Vec2<f32>,
	pub vel: Vec2<f32>,
    pub len: f32,
    pub col: [f32, ..3],
}

impl TriangleParticle {
	fn new(pos: Vec2<f32>, vel: Vec2<f32>, _acc: Vec2<f32>) -> TriangleParticle {
		TriangleParticle {
			pos: pos,
			vel: vel,
            len: 0.2f32,
            col: [1.0f32, 0f32, 0f32],
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
	fn new(pos: Vec2<f32>, vel: Vec2<f32>, acc: Vec2<f32>) -> TriangleParticle {
		TriangleParticle::new(pos, vel, acc)
	}

	fn pos(&self) -> Vec2<f32> {
		self.pos
	}

    fn vel(&self) -> Vec2<f32> {
        self.vel
    }

    fn acc(&self) -> Vec2<f32> {
        Vec2 { x: 0f32, y: 0f32 }
    }
}