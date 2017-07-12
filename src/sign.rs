use std::ops::{Neg, Mul};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}

impl Sign {
    pub fn to_i32(self) -> i32 {
        match self {
            Sign::Plus => 1,
            Sign::Minus => -1,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Sign::Plus => '+',
            Sign::Minus => '-',
        }
    }
}

impl Neg for Sign {
    type Output = Sign;
    fn neg(self) -> Sign {
        match self {
            Sign::Plus => Sign::Minus,
            Sign::Minus => Sign::Plus,
        }
    }
}

impl Mul<Sign> for Sign {
    type Output = Sign;
    fn mul(self, other: Sign) -> Sign {
        match self {
            Sign::Plus => other,
            Sign::Minus => -other,
        }
    }
}
