use std::ops::*;
use std::u128;
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq)]
pub struct BitField {
    pub field: u128,
    //メモ 縦*横
}

#[allow(dead_code)]
impl BitField {
    pub fn new() -> BitField {
        Self::default()
    }

    pub fn from_u128(x: u128) -> BitField {
        Self { field: x }
    }

    pub fn height() -> usize {
        8
    }

    pub fn height_log2() -> usize {
        3
    }

    pub fn width() -> usize {
        16
    }

    pub fn width_log2() -> usize {
        4
    }

    fn index(x: usize, y: usize) -> usize {
        x << Self::height_log2() + y
    }

    pub fn new_position(x: usize, y: usize) -> BitField {
        Self::from_u128(1 << BitField::index(x, y))
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        (self.field & (1 << BitField::index(x, y))) != 0
    }

    pub fn is_zero(&self) -> bool {
        self.field == 0
    }

    pub fn new_right()->BitField{
        let right = 0xffffffff_ffffffff;
        Self::from_u128(right)
    }

    pub fn new_left()->BitField{
        !(BitField::new_right())
    }

    

}

impl BitAnd for BitField {
    type Output = BitField;
    fn bitand(self, rhs: BitField) -> BitField {
        Self::from_u128(self.field & rhs.field)
    }
}

impl BitOr for BitField {
    type Output = BitField;
    fn bitor(self, rhs: BitField) -> BitField {
        Self::from_u128(self.field | rhs.field)
    }
}

impl BitXor for BitField {
    type Output = BitField;
    fn bitxor(self, rhs: BitField) -> BitField {
        Self::from_u128(self.field ^ rhs.field)
    }
}

impl Not for BitField {
    type Output = BitField;
    fn not(self) -> BitField {
        Self::from_u128(!self.field)
    }
}

impl BitAndAssign for BitField {
    fn bitand_assign(&mut self, rhs: BitField) {
        self.field &= rhs.field;
    }
}

impl BitOrAssign for BitField {
    fn bitor_assign(&mut self, rhs: BitField) {
        self.field |= rhs.field;
    }
}

impl BitXorAssign for BitField {
    fn bitxor_assign(&mut self, rhs: BitField) {
        self.field ^= rhs.field;
    }
}

impl Deref for BitField {
    type Target = u128;
    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

