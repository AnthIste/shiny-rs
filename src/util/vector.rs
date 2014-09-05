/// Re-inventing the wheel, incorrectly, so replace me with a library implementation.
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

impl<T: Add<T, T>> Add<Vec2<T>, Vec2<T>> for Vec2<T> {
	fn add(&self, _rhs: &Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: self.x + _rhs.x,
			y: self.y + _rhs.y,
		}
	}
}

impl<T: Mul<T, T>> Mul<T, Vec2<T>> for Vec2<T> {
    fn mul(&self, _rhs: &T) -> Vec2<T> {
    	Vec2 {
    		x: self.x * (*_rhs),
    		y: self.y * (*_rhs),
    	}
    }
}