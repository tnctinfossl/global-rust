use std::ops::*;
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct BitField {
    pub field: Vec<u128>, //メモ 縦*横
}

impl Default for BitField {
    fn default() -> BitField {
        BitField {
            field: vec![0; 100],
        }
    }
}




#[allow(dead_code)]
impl BitField {
    pub fn new() -> BitField {
        Self::default()
    }

    pub fn width(&self) -> usize {
        std::mem::size_of::<u128>() * 8
    }

    pub fn height(&self) -> usize {
        self.field.len()
    }

    pub fn dump(&self) -> String {
        self.field
            .iter()
            .map(|x| {
                (0..self.width())
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
        if x < self.width() && y < self.height() {
            (self.field[y] & (1 << x)) != 0
        } else {
            false
        }
    }

    pub fn write(&mut self, (x, y): (usize, usize), value: bool) -> &mut Self {
        if x < self.width() && y < self.height() {
            let mask = 1 << x;
            if value {
                self.field[y] |= mask;
            } else {
                self.field[y] &= !mask;
            }
        }
        self
    }

    fn op_double<F>(&self, rhs: BitField, f: F) -> BitField
    where
        F: Fn(u128, u128) -> u128,
    {
        BitField {
            field: self
                .field
                .iter()
                .zip(rhs.field.iter())
                .map(|(x, y)| f(*x, *y))
                .collect(),
        }
    }
    fn op_single<F>(&self, f: F) -> BitField
    where
        F: Fn(u128) -> u128,
    {
        BitField {
            field: self.field.iter().map(|x| f(*x)).collect(),
        }
    }

    fn op_assign<F>(&mut self, rhs: BitField, f: F)
    where
        F: Fn(&mut u128, u128),
    {
        self.field
            .iter_mut()
            .zip(rhs.field.iter())
            .for_each(|(x, y)| f(x, *y))
    }
}

impl BitAnd for BitField {
    type Output = BitField;
    fn bitand(self, rhs: BitField) -> BitField {
        self.op_double(rhs, |x, y| x & y)
    }
}

impl BitOr for BitField {
    type Output = BitField;
    fn bitor(self, rhs: BitField) -> BitField {
        self.op_double(rhs, |x, y| x | y)
    }
}

impl BitXor for BitField {
    type Output = BitField;
    fn bitxor(self, rhs: BitField) -> BitField {
        self.op_double(rhs, |x, y| x ^ y)
    }
}

impl Not for BitField {
    type Output = BitField;
    fn not(self) -> BitField {
        self.op_single(|x| !x)
    }
}

impl BitAndAssign for BitField {
    fn bitand_assign(&mut self, rhs: BitField) {
        self.op_assign(rhs, |x, y| *x &= y);
    }
}

impl BitOrAssign  for BitField {
    fn bitor_assign(&mut self, rhs: BitField) {
        self.op_assign(rhs, |x, y| *x |= y);
        }
}
#[test]
fn bitfield_test() {
    let mut a = BitField::new();
    a.write((0, 0), true).write((1, 0), true);
    let mut b = BitField::new();
    b.write((0, 0), true).write((0, 1), true);
    let mut and = BitField::new();
    and.write((0, 0), true);
    assert_eq!((&a)&(&b),and);
    let mut or = BitField::new();
    or.write((0, 0), true).write((1, 0), true).write((0, 1), true);
    assert_eq!(&a|&b,or);
    let mut xor= BitField::new();
    xor.write((1, 0), true).write((0, 1), true);
    assert_eq!(&a^&b,xor);
}
