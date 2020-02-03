use crate::common::BitField;
use crate::common::Rectangle;
use glm::*;
#[derive(Debug)]
pub struct CellDomination {
    field: Rectangle,
}
//BitFieldを用いて空間支配率を計算する
impl CellDomination {
    #[allow(dead_code)]
    pub fn new(field: Rectangle) -> CellDomination {
        CellDomination { field: field }
    }

    pub fn encode(&self, position: Vec2) -> (usize, usize) {
        //[0..1]に正規化する。
        let normal = (position - self.field.point) / self.field.size;
        //bitfieldに合わせる
        let resized = clamp(
            normal * vec2(128.0, 100.0),
            vec2(0.0, 0.0),
            vec2(128.0, 100.0),
        );
        (resized.x as usize, resized.y as usize)
    }

    #[allow(dead_code)]
    pub fn evaluate(&self, rights: &[Vec2], lefts: &[Vec2]) -> (f32, f32) {
        let rights: Vec<_> = rights.iter().map(|p: &Vec2| self.encode(*p)).collect();
        let lefts: Vec<_> = lefts.iter().map(|p: &Vec2| self.encode(*p)).collect();
        let merge = |points: &[(usize, usize)], size: usize| -> BitField {
            let mut field = BitField::new();
            for &point in points {
                field.write_rect(point, size);
            }
            field
        };

        let mut used = BitField::new();
        let mut right = BitField::new();
        let mut left = BitField::new();
        let mut used_last = BitField::new();

        for i in 0..127 {
            let right_new = merge(&rights, i);
            let left_new = merge(&lefts, i);
            let conflict = &used | &(&right_new & &left_new);
            right |= &(&right_new & &!(&conflict));
            left |= &(&left_new & &!(&conflict));
            used |= &(&right | &(&left | &conflict));
            if used == used_last {
                break;
            };
            used_last = used.clone();
        }
        (right.rate(), left.rate())
    }
}
