extern crate cgmath;

use self::cgmath::{Vector, Vector2}; // Why is self needed???

/// Main particle trait - a particle updates over time
pub trait Particle {
    /// Particles have identity
    fn id(&self) -> u64;

    fn update(&mut self, dt: f32);
}

/// A particle that moves in 2D space
pub trait Particle2D {
	fn new(acc: Vector2<f32>, vel: Vector2<f32>, pos: Vector2<f32>) -> Self;

	fn acc(&self) -> Vector2<f32>;

	fn vel(&self) -> Vector2<f32>;

	fn pos(&self) -> Vector2<f32>;
}

/// A particle with a size measured in 1 dimension
pub trait ParticleSizeUniform {
    fn len(&self) -> f32;
}

/// A particle with a color measured in 1 dimension
pub trait ParticleColorUniform {
    fn col(&self) -> [f32, ..3];
}

/// A particle with very basic update behavior and uniform properties
pub struct TriangleParticle {
    id: u64,
	pub pos: Vector2<f32>,
	pub vel: Vector2<f32>,
    pub len: f32,
    pub col: [f32, ..3],
}

impl TriangleParticle {
	fn new(pos: Vector2<f32>, vel: Vector2<f32>, _acc: Vector2<f32>) -> TriangleParticle {
		TriangleParticle {
            id: 0, // FIXME: seed this
			pos: pos,
			vel: vel,
            len: 0.1f32,
            col: [1.0f32, 0f32, 0f32],
		}
	}
}

impl Particle for TriangleParticle {
    fn id(&self) -> u64 {
        self.id
    }

	fn update(&mut self, dt: f32) {
		let acc = Vector2::new(0f32, 0f32);

		self.vel = self.vel + acc.mul_s(dt);
		self.pos = self.pos + self.vel.mul_s(dt);
	}
}

impl Particle2D for TriangleParticle {
	fn new(pos: Vector2<f32>, vel: Vector2<f32>, acc: Vector2<f32>) -> TriangleParticle {
		TriangleParticle::new(pos, vel, acc)
	}

	fn pos(&self) -> Vector2<f32> {
		self.pos
	}

    fn vel(&self) -> Vector2<f32> {
        self.vel
    }

    fn acc(&self) -> Vector2<f32> {
        Vector2 { x: 0f32, y: 0f32 }
    }
}

impl ParticleSizeUniform for TriangleParticle {
    fn len(&self) -> f32 {
        self.len
    }
}

impl ParticleColorUniform for TriangleParticle {
    fn col(&self) -> [f32, ..3] {
        self.col
    }
}