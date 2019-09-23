use serde_derive::{Deserialize, Serialize};
use glm::Vec2;
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Field{
    pub back_color:[f64;3],
    pub line_color:[f64;3],
    pub full_size:[f64;2],
    pub field_size:[f64;2],
    pub goal_size:[f64;2],
    pub goal_length:f64,
    pub center_diameter:f64,
    pub line_width:f64,
}

impl Default for Field{
    fn default() ->Field{
        //division Aを参考にした。
        Field{
            back_color:[0.1,0.9,0.1],
            line_color:[0.9,0.9,0.9],
            full_size:[13400.0,10400.0],
            field_size:[12000.0,9000.0],
            goal_size:[1200.0,2400.0],
            goal_length:1200.0,
            center_diameter:1000.0,
            line_width:10.0
        }
    }
}
