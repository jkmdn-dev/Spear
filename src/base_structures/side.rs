#[derive(Copy, Clone, PartialEq)]
pub struct Side(usize);
impl Side {
    pub const WHITE: Side = Side::from_raw(0);
    pub const BLACK: Side = Side::from_raw(1);

    #[inline]
    pub const fn from_raw(value: usize) -> Self {
        Self { 0: value }
    }

    #[inline]
    pub const fn current(&self) -> usize {
        self.0
    }

    #[inline]
    pub const fn opposite(&self) -> usize {
        1 - self.0
    }

    #[inline]
    pub const fn flipped(&self) -> Self {
        Self { 0: 1 - self.0 }
    }

    #[inline]
    pub fn mut_flip(&mut self) {
        self.0 = 1 - self.0;
    }
}