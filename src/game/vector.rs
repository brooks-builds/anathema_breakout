use std::ops::AddAssign;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalize(&mut self) {
        let length = self.x.powf(2.0) + self.y.powf(2.0);
        self.x /= length;
        self.y /= length;
    }

    pub fn mult(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<(i64, i64)> for Vector {
    fn from((x, y): (i64, i64)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}
