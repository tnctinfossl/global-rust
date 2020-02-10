use crate::common::BitField;
use crate::common::Rectangle;
use glm::*;

/// #ビット列で近似した空間支配室の計算
/// 使用例
/// ```
/// use rand::*;
/// use std::time::Instamt;
/// use glm;
/// let mut gen = rand::thread_rng();
/// let field = Rectangle::new(vec2(-64.0, 50.0), vec2(128.0, -100.0));
/// let mine: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
/// let yours: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
/// let cell = team::CellDomination::new(field);
/// let begin = Instant::now();
/// let score = cell.evaluate(&mine, &yours);
/// let end = Instant::now();
/// println!("time={:?},score={:?}", end - begin, score);
/// ```

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

    fn encode(&self, position: Vec2) -> (usize, usize) {
        //[0..1]に正規化する。
        let normal = (position - self.field.point) / self.field.size;
        //bitfieldに合わせる
        let resized = clamp(
            normal * vec2(BitField::width() as f32, BitField::height() as f32),
            vec2(0.0, 0.0),
            vec2(BitField::width() as f32, BitField::height() as f32),
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

        let limit = std::cmp::max(BitField::width(), BitField::height()) - 1;
        for i in 0..limit {
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

#[derive(Debug)]
pub struct CellDomination2 {
    field: Rectangle,
}
//BitFieldを用いて空間支配率を計算する
impl CellDomination2 {
    #[allow(dead_code)]
    pub fn new(field: Rectangle) -> CellDomination {
        CellDomination { field: field }
    }

    fn encode(&self, position: Vec2) -> (usize, usize) {
        //[0..1]に正規化する。
        let normal = (position - self.field.point) / self.field.size;
        //bitfieldに合わせる
        let resized = clamp(
            normal * vec2(BitField::width() as f32, BitField::height() as f32),
            vec2(0.0, 0.0),
            vec2(BitField::width() as f32, BitField::height() as f32),
        );
        (resized.x as usize, resized.y as usize)
    }

    #[allow(dead_code)]
    pub fn evaluate(&self, rights: &[Vec2], lefts: &[Vec2]) -> (f32, f32) {
        let right_points: Vec<_> = rights.iter().map(|p: &Vec2| self.encode(*p)).collect();
        let left_points: Vec<_> = lefts.iter().map(|p: &Vec2| self.encode(*p)).collect();
        let merge = |dest:&mut BitField,points: &[(usize, usize)], size: usize| {
            dest.clear();
            for &point in points {
                dest.write_rect(point, size);
            }
        };
        let mut used = BitField::new();
        let mut right = BitField::new_points(&right_points);
        let mut right_new = BitField::new();
        let mut left = BitField::new_points(&left_points);
        let mut left_new = BitField::new();
        //let mut used_last = BitField::new();
        //let mut conflict=BitField::new();
        let limit = std::cmp::max(BitField::width(), BitField::height()) - 1;
        for i in 0..limit {
            //BitField::expand9(&mut right_new,&right);
            //BitField::expand9(&mut left_new,&left);
            merge(&mut right_new,&right_points, i);
            merge(&mut left_new,&left_points, i);
            //conflict = used | (right_new & left_new);
            let mut equal = true;
            for index in 0..BitField::height() {
                let conflict = used[index] | (right_new[index] & left_new[index]);
                right[index] = right_new[index] & !conflict;
                left[index] = left_new[index] & !conflict;
                let used_now = right[index] | left[index] & conflict;
                equal &= used[index] == used_now;
                used[index] = used_now;
            }
            if equal {
                break;
            };
        }
        (right.rate(), left.rate())
    }
}
