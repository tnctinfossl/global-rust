use std::ops::*;
/*
* 32x16[bit]によって構成される。
* 32bitは横である
*/
const BITFIELD_HEIGHT: usize = 32;
const BITFIELD_WIDTH: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct BitField {
    field: [u32; BITFIELD_WIDTH],
}

impl Default for BitField {
    fn default() -> BitField {
        BitField {
            field: [0; BITFIELD_WIDTH],
        }
    }
}
#[allow(dead_code)]
impl BitField {
    fn new() -> Self {
        Self::default()
    }

    fn get(&self, (x, y): (usize, usize)) -> bool {
        self.field[y] & (1 << x) != 0
    }
    fn set(&mut self, (x, y): (usize, usize), value: bool) -> &mut Self {
        let mask = 1 << x;
        if value {
            self.field[y] |= mask;
        } else {
            self.field[y] |= mask;
        }
        self
    }
    fn count_ones(&self) -> u32 {
        self.field.iter().map(|x| x.count_ones()).sum()
    }

    fn count_zeros(&self) -> u32 {
        self.field.iter().map(|x| x.count_zeros()).sum()
    }
    /*
    fn find_first_one(&self)->(u32,u32){

    }*/
}

impl BitAnd for BitField {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = BitField::default();
        for index in 0..BITFIELD_WIDTH {
            result.field[index] = self.field[index] & rhs.field[index];
        }
        result
    }
}

impl BitOr for BitField {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = BitField::default();
        for index in 0..BITFIELD_WIDTH {
            result.field[index] = self.field[index] | rhs.field[index];
        }
        result
    }
}

impl BitXor for BitField {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = BitField::default();
        for index in 0..BITFIELD_WIDTH {
            result.field[index] = self.field[index] ^ rhs.field[index];
        }
        result
    }
}

impl Neg for BitField {
    type Output = BitField;
    fn neg(self) -> BitField {
        let mut result = BitField::default();
        for index in 0..BITFIELD_WIDTH {
            result.field[index] = !self.field[index];
        }
        result
    }
}
