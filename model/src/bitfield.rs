use std::ops::*;
/*
* 32x16[bit]によって構成される。
*/
#[derive(Debug, Clone, Copy)]
pub struct BitFiled {
    bits: [u16; 32],
}
#[allow(dead_code)]
impl BitFiled {
    pub const fn new() -> BitFiled {
        BitFiled { bits: [0; 32] }
    }
    pub const fn height() -> usize {
        16
    }
    pub const fn width() -> usize {
        32
    }
    pub const fn position((x, y): (usize, usize)) -> BitFiled {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                bits[x] |= 1 << y;
                bits
            },
        }
    }
    pub fn mask_right() -> BitFiled {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                for index in 0..16 {
                    bits[index] = u16::max_value()
                }
                bits
            },
        }
    }
    pub fn mask_left() -> BitFiled {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                for index in 16..32 {
                    bits[index] = u16::max_value()
                }
                bits
            },
        }
    }
    pub fn mask_inside() -> BitFiled {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                for index in 1..31 {
                    bits[index] = 0x7ffe;
                }
                bits
            },
        }
    }
    pub fn mask_outside() -> BitFiled {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                bits[0] = 0xffff;
                for index in 1..31 {
                    bits[index] = 0x8001;
                }
                bits[31] = 0xffff;
                bits
            },
        }
    }
    pub fn exist(&self) -> bool {
        self.bits.iter().map(|x| *x != 0).fold(false, |x, y| x | y)
    }

    pub fn count_ones(self) -> u32 {
        self.bits.iter().map(|x| x.count_ones()).sum()
    }

    pub fn count_zeros(self) -> u32 {
        self.bits.iter().map(|x| x.count_zeros()).sum()
    }

    pub fn set(&mut self, (x, y): (usize, usize)) -> &mut Self {
        self.bits[y] |= 1 << x;
        self
    }

    pub fn clear(&mut self, (x, y): (usize, usize)) -> &mut Self {
        self.bits[y] &= !(1 << x);
        self
    }

    pub fn set_value(&mut self, (x, y): (usize, usize), value: bool) -> &mut Self {
        if value {
            self.set((x, y))
        } else {
            self.clear((x, y))
        }
    }
    
    pub fn get(&self, (x, y): (usize, usize)) -> bool {
        (self.bits[y]&(1<<x))!=0
    }

}

impl BitOr for BitFiled {
    type Output = BitFiled;
    fn bitor(self, rhs: BitFiled) -> Self::Output {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                for index in 0..32 {
                    bits[index] = self.bits[index] | rhs.bits[index];
                }
                bits
            },
        }
    }
}

impl BitAnd for BitFiled {
    type Output = BitFiled;
    fn bitand(self, rhs: BitFiled) -> Self::Output {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                for index in 0..32 {
                    bits[index] = self.bits[index] & rhs.bits[index];
                }
                bits
            },
        }
    }
}

impl BitXor for BitFiled {
    type Output = BitFiled;
    fn bitxor(self, rhs: BitFiled) -> Self::Output {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                for index in 0..32 {
                    bits[index] = self.bits[index] ^ rhs.bits[index];
                }
                bits
            },
        }
    }
}

impl BitOrAssign for BitFiled {
    fn bitor_assign(&mut self, rhs: BitFiled) {
        for index in 0..32 {
            self.bits[index] |= rhs.bits[index];
        }
    }
}

impl BitAndAssign for BitFiled {
    fn bitand_assign(&mut self, rhs: BitFiled) {
        for index in 0..32 {
            self.bits[index] &= rhs.bits[index];
        }
    }
}

impl BitXorAssign for BitFiled {
    fn bitxor_assign(&mut self, rhs: BitFiled) {
        for index in 0..32 {
            self.bits[index] ^= rhs.bits[index];
        }
    }
}

impl Neg for BitFiled {
    type Output = BitFiled;
    fn neg(self) -> BitFiled {
        BitFiled {
            bits: {
                let mut bits = [0; 32];
                for index in 0..32 {
                    bits[index] = !self.bits[index];
                }
                bits
            },
        }
    }
}
