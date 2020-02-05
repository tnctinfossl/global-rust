use std::cmp::{max, min};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Fn, Not,Index,IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct BitField {
    pub field: Vec<u64>, //メモ 縦*横
}

impl Default for BitField {
    fn default() -> BitField {
        BitField {
            field: vec![0; Self::height()],
        }
    }
}

#[allow(dead_code)]
impl BitField {
    pub fn width() -> usize {
        std::mem::size_of::<u64>() * 8
    }

    pub const fn height() -> usize {
        50
    }

    pub fn area() -> usize {
        Self::width() * Self::height()
    }

    pub fn new() -> BitField {
        Self::default()
    }

    pub fn new_value(value: bool) -> BitField {
        if value {
            BitField {
                field: vec![!0; Self::height()],
            }
        } else {
            BitField {
                field: vec![0; Self::height()],
            }
        }
    }
    pub fn new_points(points: &[(usize, usize)]) -> BitField {
        let mut field = Self::default();
        for (i, j) in points {
            field.field[*j] |= 1u64 << i;
        }
        field
    }

    pub fn new_rect((i, j): (usize, usize), k: usize) -> BitField {
        let mut field = vec![0; Self::height()];
        let i = i as isize;
        let j = j as isize;
        let k = k as isize;

        let masker = |n: isize| -> u64 {
            match n {
                n if n < 0 => 0,
                n if n >= (Self::width() - 1) as isize => !0,
                n => (1 << n) - 1,
            }
        };

        let (high, low) = (masker(i + k + 1), masker(i - k));
        let line: u64 = high ^ low;
        for l in max(j - k, 0)..min(j + k + 1, (Self::height() - 1) as isize) {
            let l = l as usize;
            field[l] |= line;
        }
        BitField { field: field }
    }

    pub fn clear(&mut self){
        for i in 0..Self::height(){
            self.field[i]=0;
        }
    }


    pub fn dump(&self) -> String {
        self.field
            .iter()
            .rev()
            .map(|x| {
                (0..Self::width())
                    .rev()
                    .map(|index| {
                        if (x & 1 << index) == 0 {
                            "_".to_owned()
                        } else {
                            "*".to_owned()
                        }
                    })
                    .fold("".to_owned(), |a, b| format!("{}{}", a, b))
            })
            .fold("".to_owned(), |x, y| format!("{}\n{}", x, y))
    }

    pub fn read(&self, (x, y): (usize, usize)) -> bool {
        if x < Self::width() && y < Self::height() {
            (self.field[y] & (1 << x)) != 0
        } else {
            false
        }
    }

    pub fn write(&mut self, (x, y): (usize, usize), value: bool) -> &mut Self {
        if x < Self::width() && y < Self::height() {
            let mask = 1 << x;
            if value {
                self.field[y] |= mask;
            } else {
                self.field[y] &= !mask;
            }
        }
        self
    }

    fn op_double<F>(&self, rhs: &BitField, f: F) -> BitField
    where
        F: Fn(u64, u64) -> u64,
    {
        let mut result = BitField::new();
        for i in 0..Self::height() {
            result.field[i] = f(self.field[i], rhs.field[i]);
        }
        result
    }

    fn op_single<F>(&self, f: F) -> BitField
    where
        F: Fn(u64) -> u64,
    {
        let mut result = BitField::new();
        for i in 0..Self::height() {
            result.field[i] = f(self.field[i]);
        }
        result
    }

    fn op_assign<F>(&mut self, rhs: &BitField, f: F)
    where
        F: Fn(&mut u64, u64),
    {
        self.field
            .iter_mut()
            .zip(rhs.field.iter())
            .for_each(|(x, y)| f(x, *y))
    }

    pub fn count_ones(&self) -> u32 {
        self.field.iter().map(|x| x.count_ones()).sum()
    }

    pub fn count_zeros(&self) -> u32 {
        self.field.iter().map(|x| x.count_zeros()).sum()
    }

    pub fn move_right(&mut self, i: u32) -> &mut Self {
        self.field.iter_mut().for_each(|x| *x >>= i);
        self
    }

    pub fn move_left(&mut self, i: u32) -> &mut Self {
        self.field.iter_mut().for_each(|x| *x <<= i);
        self
    }

    pub fn move_up(&mut self, i: usize) -> &mut Self {
        for j in (0..Self::height()).rev() {
            self.field[j] = if (j as isize - i as isize) >= 0 {
                self.field[j - i]
            } else {
                0
            };
        } //TODO 若干遅いので修正すること
        self
    }

    pub fn move_down(&mut self, i: usize) -> &mut Self {
        for j in 0..Self::height() - i {
            //let j = j.into();
            self.field[j] = self.field[i + j];
        }
        for j in Self::height() - i..Self::height() {
            self.field[j] = 0;
        }
        self
    }

    pub fn write_rect(&mut self, (i, j): (usize, usize), k: usize) {
        let i = i as isize;
        let j = j as isize;
        let k = k as isize;

        let masker = |n: isize| -> u64 {
            match n {
                n if n < 0 => 0,
                n if n >= (Self::width() - 1) as isize => !0,
                n => (1 << n) - 1,
            }
        };

        let (high, low) = (masker(i + k + 1), masker(i - k));
        let line: u64 = high ^ low;
        for l in max(j - k, 0)..min(j + k + 1, (Self::height() - 1) as isize) {
            let l = l as usize;
            self.field[l] |= line;
        }
    }

    pub fn rate(&self) -> f32 {
        self.count_ones() as f32 / Self::area() as f32
    }

    //膨張処理 
    #[allow(dead_code)]
    pub fn expand9<'a>(dest:&'a mut Self,src:&Self) ->&'a mut Self {
        
        let field = &src.field;

        dest.field[0] =
            field[0] << 1 | field[0] | field[0] >> 1 | field[1] << 1 | field[1] | field[1] >> 1;
        for index in 1..Self::height() - 1 {
            dest.field[index] = field[index] << 1
                | field[index]
                | field[index] >> 1
                | field[index - 1] << 1
                | field[index - 1]
                | field[index - 1] >> 1
                | field[index + 1] << 1
                | field[index + 1]
                | field[index + 1] >> 1;
        }
        let end = Self::height() - 1;
        dest.field[end] = field[end] << 1
            | field[end]
            | field[end] >> 1
            | field[end - 1] << 1
            | field[end - 1]
            | field[end - 1] >> 4;
  
        dest
    }
   
}

impl BitAnd<&BitField> for &BitField {
    type Output = BitField;
    fn bitand(self, rhs: &BitField) -> BitField {
        self.op_double(rhs, |x, y| x & y)
    }
}

impl BitOr<&BitField> for &BitField {
    type Output = BitField;
    fn bitor(self, rhs: &BitField) -> BitField {
        self.op_double(rhs, |x, y| x | y)
    }
}

impl BitXor<&BitField> for &BitField {
    type Output = BitField;
    fn bitxor(self, rhs: &BitField) -> BitField {
        self.op_double(rhs, |x, y| x ^ y)
    }
}

impl Not for &BitField {
    type Output = BitField;
    fn not(self) -> BitField {
        self.op_single(|x| !x)
    }
}

impl BitAndAssign<&BitField> for BitField {
    fn bitand_assign(&mut self, rhs: &BitField) {
        self.op_assign(rhs, |x, y| *x &= y);
    }
}

impl BitOrAssign<&BitField> for BitField {
    fn bitor_assign(&mut self, rhs: &BitField) {
        self.op_assign(rhs, |x, y| *x |= y);
    }
}

impl Index<usize> for BitField{
    type Output = u64;
    fn index(&self,index:usize)->&u64{
        &self.field[index]
    }
}


impl IndexMut<usize> for BitField{
    fn index_mut(&mut self,index:usize)->&mut u64{
        &mut self.field[index]
    }
}

#[test]
fn bitfield_and() {
    let mut a = BitField::new();
    a.write((0, 0), true).write((1, 0), true);
    let mut b = BitField::new();
    b.write((0, 0), true).write((0, 1), true);
    let mut and = BitField::new();
    and.write((0, 0), true);
    assert_eq!(&a & &b, and);
}

