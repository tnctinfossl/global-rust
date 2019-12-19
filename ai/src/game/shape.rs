use super::*;
use glm::*;
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    position: Vec2, //左上
    size: Vec2,     //大きさ
}

impl Rectangle {
    #[allow(dead_code)]
    pub fn new(position: Vec2, size: Vec2) -> Rectangle {
        Rectangle {
            position: position,
            size: size,
        }
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.size.x
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.size.y
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn center(&self) -> Vec2 {
        self.position + self.size / 2.0
    }
}

impl Overlap<Rectangle> for Rectangle {
    fn overlap(&self, rhs: &Rectangle) -> bool {
        let check_x = abs(self.center().x - rhs.center().x) < self.size.x / 2.0 + rhs.size.x / 2.0;
        let check_y = abs(self.center().y - rhs.center().y) < self.size.y / 2.0 + rhs.size.y / 2.0;
        check_x && check_y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    center: Vec2,
    radius: f32,
}

impl Circle {
    #[allow(dead_code)]
    pub fn new(center: Vec2, radius: f32) -> Circle {
        Circle {
            center: center,
            radius: radius,
        }
    }
}

impl Overlap<Circle> for Circle {
    fn overlap(&self, rhs: &Circle) -> bool {
        distance(self.center, rhs.center) < self.radius + rhs.radius
    }
}
