use crate::route::*;
use glm::*;

//隣接行列を生成する。
pub struct LinkResolver<R> {
    resolver: R,
}

pub struct LinkMattrix<T> {
    mattrix: Vec<Vec<T>>,
}

/*
impl<R> LinkResolver{
    pub fn new(R)
}*/
