use glm::Vec2;
#[derive(Debug)]
pub struct Robot {
    pub id: u32,
    pub position: Vec2,
    pub angle: f32,
    //追加する
}

#[derive(Debug)]
pub struct Ball {
    pub position: Vec2,
    //追加する
}

#[derive(Debug)]
pub struct Items {
    pub balls: Vec<Ball>,
    pub blues: Vec<Robot>,
    pub yellows: Vec<Robot>,
}

impl Default for Items {
    fn default() -> Items {
        Items {
            balls: vec![],
            blues: vec![],
            yellows: vec![],
        }
    }
}