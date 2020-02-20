use super::*;
use glm::*;
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub position: Vec2, //左上
    pub size: Vec2,     //大きさ
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

    pub fn points(&self) -> [Vec2; 4] {
        [
            self.position,
            self.position + vec2(self.size.x, 0.0),
            self.position + self.size,
            self.position + vec2(0.0, self.size.y),
        ]
    }
}
//重複判定
impl Overlap<Rectangle> for Rectangle {
    fn overlap(&self, rhs: Rectangle) -> bool {
        let check_x = abs(self.center().x - rhs.center().x) < self.size.x / 2.0 + rhs.size.x / 2.0;
        let check_y = abs(self.center().y - rhs.center().y) < self.size.y / 2.0 + rhs.size.y / 2.0;
        check_x && check_y
    }
}

impl Overlap<Vec2> for Rectangle {
    fn overlap(&self, rhs: Vec2) -> bool {
        let begin = self.position;
        let end = self.position + self.size;
        let check_x = begin.x < rhs.x && rhs.x < end.x;
        let check_y = begin.y < rhs.y && rhs.y < end.y;
        check_x && check_y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
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
    fn overlap(&self, rhs: Circle) -> bool {
        distance(self.center, rhs.center) < self.radius + rhs.radius
    }
}

impl Overlap<Circle> for Rectangle {
    fn overlap(&self, rhs: Circle) -> bool {
        //矩形として確認する
        let wrect = Rectangle::new(
            self.position - vec2(rhs.radius, 0.0),
            self.position + vec2(rhs.radius * 2.0, 0.0),
        );
        if wrect.overlap(rhs.center) {
            return true;
        }

        let hrect = Rectangle::new(
            self.position - vec2(0.0, rhs.radius),
            self.position + vec2(0.0, rhs.radius * 2.0),
        );
        if hrect.overlap(rhs.center) {
            return true;
        }
        //円として確認する
        return self
            .points()
            .iter()
            .any(|&p| distance(p, rhs.center) < rhs.radius);
    }
}
