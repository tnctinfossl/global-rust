use glm::BaseFloat;
//行列を表現する
#[derive(Debug, Clone)]
pub struct Mattrix<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Mattrix<T>
where
    T: Copy + Clone,
{
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize, value: T) -> Mattrix<T> {
        Mattrix {
            width: width,
            height: height,
            data: vec![value; width * height],
        }
    }

    //プロパティ
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, i: usize, j: usize) -> T {
        assert!(i < self.width);
        assert!(j < self.height);
        self.data[i + self.width * j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        assert!(i < self.width);
        assert!(j < self.height);
        self.data[i + self.width * j] = value;
    }
}
