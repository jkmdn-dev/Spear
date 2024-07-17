use std::{fmt::{Display, Formatter, Result}, ops::{Add, BitXor}};

use super::bitboard::Bitboard;

#[derive(Copy, Clone, PartialEq)]
pub struct Square(usize);
impl Square {
    pub const A1: Self = Self { 0: 0 };
    pub const B1: Self = Self { 0: 1 };
    pub const C1: Self = Self { 0: 2 };
    pub const D1: Self = Self { 0: 3 };
    pub const E1: Self = Self { 0: 4 };
    pub const F1: Self = Self { 0: 5 };
    pub const G1: Self = Self { 0: 6 };
    pub const H1: Self = Self { 0: 7 };
    pub const A2: Self = Self { 0: 8 };
    pub const B2: Self = Self { 0: 9 };
    pub const C2: Self = Self { 0: 10 };
    pub const D2: Self = Self { 0: 11 };
    pub const E2: Self = Self { 0: 12 };
    pub const F2: Self = Self { 0: 13 };
    pub const G2: Self = Self { 0: 14 };
    pub const H2: Self = Self { 0: 15 };
    pub const A3: Self = Self { 0: 16 };
    pub const B3: Self = Self { 0: 17 };
    pub const C3: Self = Self { 0: 18 };
    pub const D3: Self = Self { 0: 19 };
    pub const E3: Self = Self { 0: 20 };
    pub const F3: Self = Self { 0: 21 };
    pub const G3: Self = Self { 0: 22 };
    pub const H3: Self = Self { 0: 23 };
    pub const A4: Self = Self { 0: 24 };
    pub const B4: Self = Self { 0: 25 };
    pub const C4: Self = Self { 0: 26 };
    pub const D4: Self = Self { 0: 27 };
    pub const E4: Self = Self { 0: 28 };
    pub const F4: Self = Self { 0: 29 };
    pub const G4: Self = Self { 0: 30 };
    pub const H4: Self = Self { 0: 31 };
    pub const A5: Self = Self { 0: 32 };
    pub const B5: Self = Self { 0: 33 };
    pub const C5: Self = Self { 0: 34 };
    pub const D5: Self = Self { 0: 35 };
    pub const E5: Self = Self { 0: 36 };
    pub const F5: Self = Self { 0: 37 };
    pub const G5: Self = Self { 0: 38 };
    pub const H5: Self = Self { 0: 39 };
    pub const A6: Self = Self { 0: 40 };
    pub const B6: Self = Self { 0: 41 };
    pub const C6: Self = Self { 0: 42 };
    pub const D6: Self = Self { 0: 43 };
    pub const E6: Self = Self { 0: 44 };
    pub const F6: Self = Self { 0: 45 };
    pub const G6: Self = Self { 0: 46 };
    pub const H6: Self = Self { 0: 47 };
    pub const A7: Self = Self { 0: 48 };
    pub const B7: Self = Self { 0: 49 };
    pub const C7: Self = Self { 0: 50 };
    pub const D7: Self = Self { 0: 51 };
    pub const E7: Self = Self { 0: 52 };
    pub const F7: Self = Self { 0: 53 };
    pub const G7: Self = Self { 0: 54 };
    pub const H7: Self = Self { 0: 55 };
    pub const A8: Self = Self { 0: 56 };
    pub const B8: Self = Self { 0: 57 };
    pub const C8: Self = Self { 0: 58 };
    pub const D8: Self = Self { 0: 59 };
    pub const E8: Self = Self { 0: 60 };
    pub const F8: Self = Self { 0: 61 };
    pub const G8: Self = Self { 0: 62 };
    pub const H8: Self = Self { 0: 63 };
    pub const NULL: Self = Self { 0: 64 };

    #[inline]
    pub const fn from_raw(value: usize) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn from_coords(rank: usize, file: usize) -> Self {
        Self { 0: rank * 8 + file }
    }

    #[inline]
    pub const fn get_value(&self) -> usize {
        self.0
    }

    #[inline]
    pub const fn get_rank(&self) -> usize {
        self.get_value() / 8
    }

    #[inline]
    pub const fn get_file(&self) -> usize {
        self.get_value() % 8
    }

    #[inline]
    pub const fn get_bit(&self) -> Bitboard {
        Bitboard::from_raw(1u64 << self.get_value())
    }

    #[inline]
    pub const fn equals(&self, rhs: Square) -> bool {
        self.get_value() == rhs.get_value()
    }

    #[inline]
    pub const fn flip(&self) -> Self {
        Self::from_raw(self.get_value() ^ 56)
    }

    pub fn to_string(&self) -> String {
        if *self == Square::NULL {
            return "NULL".to_string();
        }

        let file: usize = self.get_value() % 8;
        let rank: usize = ((self.get_value() as f32) / 8_f32).floor() as usize + 1;
        return format!("{}{}", ('a' as usize + file) as u8 as char, rank);
    }

    pub fn from_string(square: &str) -> Square {
        let signatures: Vec<char> = square.chars().collect();
        let file = (signatures[0] as u8 - 'a' as u8).into();
        let rank = (signatures[1].to_string().parse::<u8>().unwrap() - 1).into();
        Square::from_coords(rank, file)
    }
}
impl Add<usize> for Square {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self { 0: self.get_value() + rhs }
    }
}
impl Add<Square> for usize {
    type Output = Square;

    #[inline]
    fn add(self, rhs: Square) -> Self::Output {
        Square { 0: self + rhs.get_value() }
    }
}
impl BitXor<usize> for Square {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: usize) -> Self::Output {
        Self::from_raw(self.get_value() ^ rhs)
    }
}

impl Display for Square {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!( formatter, "{}", self.to_string() )
    }
}