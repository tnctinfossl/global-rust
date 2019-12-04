use glm::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug,Clone,Copy)]
pub struct Robot{
    id:u32,
    position:Vec2,
    angle:f32//rad
}

impl Default for Robot{
    #[allow(dead_code)]
    fn default() ->Robot{
        Robot{
            id:0,
            position:Vec2::new(0.0,0.0),
            angle:0.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ball{
    position: Vec2,
}

impl Default for Ball{
    #[allow(dead_code)]
    fn default()->{
        position:Vec2::new(0.0,0.0)
    }
}


#[derive(Debug,Clone)]
pub struct Scene{
    mines:Vec<Robot>,
    enemies:Vec<Robot>,
    ball:Vec<Ball>
}

impl Default for Scene{
    #[allow(dead_code)]
    fn default() ->Scene{
        Scene{
            mines:vec![],
            enemies:vec![],
            ball:vec![]
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct History{
    scenes:VecDeque<Rc<RefCell<Scene>>>,
    score:(f32,f32),
    futures:Vec<History>
}