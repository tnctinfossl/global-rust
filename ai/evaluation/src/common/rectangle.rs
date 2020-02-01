use glm::*;
use rand::Rng;
use serde_derive::*;
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rectangle {
    pub point: Vec2, //右上の座標
    pub size: Vec2,  //フィールドの大きさ
}

impl Rectangle {
    #[allow(dead_code)]
    pub fn new(point: Vec2, size: Vec2) -> Rectangle {
        Rectangle {
            point: point,
            size: size,
        }
    }
    #[allow(dead_code)]
    pub fn points(&self) -> Vec<Vec2> {
        let point = self.point;
        let size = self.size;
        vec![
            point,
            point + vec2(size.x, 0.0),
            point + size,
            point + vec2(0.0, size.y),
        ]
    }

    #[allow(dead_code)]
    pub fn has(&self, point: Vec2) -> bool {
        let begin = self.point;
        let end = self.point + self.size;
        //領域確認
        let check_x = begin.x < point.x && point.x < end.x;
        let check_y = begin.y < point.y && point.y < end.y;
        check_x && check_y
    }

    #[allow(dead_code)]
    pub fn center(&self) -> Vec2 {
        self.point + self.size / 2.0
    }

    #[allow(dead_code)]
    pub fn sample<R: Rng + ?Sized>(&self, rand: &mut R) -> Vec2 {
        let max_min = |x: f32, y: f32| -> (f32, f32) {
            if x > y {
                (x, y)
            } else {
                (y, x)
            }
        };

        let (x_max, x_min) = max_min(self.point.x, self.point.x + self.size.x);
        let x = rand.gen_range(x_min, x_max);
        let (y_max, y_min) = max_min(self.point.y, self.point.y + self.size.y);
        let y = rand.gen_range(y_min, y_max);
        vec2(x, y)
    }
}
