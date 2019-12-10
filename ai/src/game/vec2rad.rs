use glm::*;
use serde_derive::*;
use std::ops::*;
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Vec2Rad {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Vec2Rad {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, theta: f32) -> Vec2Rad {
        Vec2Rad {
            x: x,
            y: y,
            theta: theta,
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn from_vec2_rad(position: Vec2, theta: f32) -> Vec2Rad {
        Vec2Rad {
            x: position.x,
            y: position.y,
            theta: theta,
        }
    }
}
#[inline(always)]
#[allow(dead_code)]
pub fn vec2rad<T: Into<f32>>(x: T, y: T, theta: T) -> Vec2Rad {
    Vec2Rad::new(x.into(), y.into(), theta.into())
}

impl Add for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Vec2Rad::new(self.x + rhs.x, self.y + rhs.y, self.theta + rhs.theta)
    }
}

impl AddAssign for Vec2Rad {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.theta += rhs.theta;
    }
}

impl Sub for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Vec2Rad::new(self.x - rhs.x, self.y - rhs.y, self.theta - rhs.theta)
    }
}

impl SubAssign for Vec2Rad {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.theta -= rhs.theta;
    }
}

impl Neg for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Vec2Rad::new(-self.x, -self.y, -self.theta)
    }
}

impl Mul<f32> for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Vec2Rad::new(self.x * rhs, self.y * rhs, self.theta * rhs)
    }
}

impl Mul<Vec2Rad> for f32 {
    type Output = Vec2Rad;
    #[inline(always)]
    fn mul(self, rhs: Vec2Rad) -> Vec2Rad {
        Vec2Rad::new(self * rhs.x, self * rhs.y, self * rhs.theta)
    }
}

impl MulAssign<f32> for Vec2Rad {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.theta *= rhs;
    }
}

impl Div<f32> for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        Vec2Rad::new(self.x / rhs, self.y / rhs, self.theta / rhs)
    }
}

impl DivAssign<f32> for Vec2Rad {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.theta /= rhs;
    }
}

impl Into<Vec2> for Vec2Rad {
    fn into(self) -> Vec2 {
        vec2(self.x, self.y)
    }
}

impl Index<usize> for Vec2Rad {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.theta,
            _ => &0.0,
        }
    }
}
