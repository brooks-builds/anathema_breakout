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

    pub fn coords_as_i32(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
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
