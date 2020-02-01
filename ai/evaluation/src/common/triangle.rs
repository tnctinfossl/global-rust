use super::cross2d;
use glm::*;
use serde_derive::*;
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Triangle {
    #[allow(dead_code)]
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle {
        Triangle { a: a, b: b, c: c }
    }

    #[allow(dead_code)]
    pub fn rocation(&self) -> bool {
        cross2d(self.b - self.a, self.c - self.a) > 0.0
    }

    #[allow(dead_code)]
    pub fn has(&self, point: Vec2) -> bool {
        let x = Triangle::new(point, self.b, self.c).rocation();
        let y = Triangle::new(self.a, point, self.c).rocation();
        let z = Triangle::new(self.a, self.b, point).rocation();
        (x == y) && (x == z) //回転方向がすべて一致するか?
    }

    #[allow(dead_code)]
    pub fn center(&self) -> Vec2 {
        (self.a + self.b + self.c) / 3.0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn triangle_test() {
        let triangle = Triangle::new(vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0));
        assert_eq!(triangle.has(vec2(0.1, 0.1)), true);
        assert_eq!(triangle.has(vec2(-0.1, -0.1)), false);
    }
}
