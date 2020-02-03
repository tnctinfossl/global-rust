use crate::common::*;
use glm::*;
use serde_derive::*;
//ここではパスが通りうるか簡易的に検査する

pub struct PassResolver {
    pub radius: f32, //接触したとみなす半径
}

impl PassResolver {
    #[allow(dead_code)]
    pub fn new(radius: f32) -> PassResolver {
        PassResolver { radius: radius }
    }

    #[allow(dead_code)]
    pub fn pass(&self, (begin, end): (Vec2, Vec2), others: &[Vec2]) -> bool {
        //線分[begin,end]とothersの距離を求め、距離がしきい値と比較を行う
        for &other in others {
            if touch_segment_circle((begin, end), (other, self.radius)) {
                return true;
            }
        }
        false
    }
}
