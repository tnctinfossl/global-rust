pub trait Plotable<Rhs> {
    fn plot<'a>(&self, canvas: &'a mut Rhs);
}

pub trait Overlap<T> {
    //重なっている
    fn overlap(&self, rhs: &T) -> bool;
}
