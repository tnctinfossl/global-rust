use glm::*;
use serde_derive::*;
#[derive(Debug, Serialize, Deserialize)]
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
}
