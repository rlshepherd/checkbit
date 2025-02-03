use crate::constants::{FILE_A, FILE_H, RANK_1, RANK_8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    /// Creates a new empty bitboard
    pub fn empty() -> Self {
        Bitboard(0)
    }

    /// Creates a new bitboard with all bits set
    pub fn full() -> Self {
        Bitboard(!0)
    }

    /// Creates a new bitboard from a u64
    pub fn from_u64(bits: u64) -> Self {
        Bitboard(bits)
    }

    /// Gets the underlying u64 representation
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Sets the bit at the given square (0-63)
    pub fn set_bit(&mut self, square: u8) {
        debug_assert!(square < 64, "square must be in range 0-63");
        self.0 |= 1u64 << square;
    }

    /// Clears the bit at the given square (0-63)
    pub fn clear_bit(&mut self, square: u8) {
        debug_assert!(square < 64, "square must be in range 0-63");
        self.0 &= !(1u64 << square);
    }

    /// Tests if the bit at the given square (0-63) is set
    pub fn test_bit(&self, square: u8) -> bool {
        debug_assert!(square < 64, "square must be in range 0-63");
        (self.0 & (1u64 << square)) != 0
    }

    /// Returns the number of set bits (population count)
    pub fn pop_count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the index of the least significant set bit, or None if the bitboard is empty
    pub fn lsb(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    /// Returns the index of the most significant set bit, or None if the bitboard is empty
    pub fn msb(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(63 - self.0.leading_zeros() as u8)
        }
    }

    /// Returns a new bitboard with all bits shifted north (up) by one rank, or None if any bits would shift off the board
    pub fn shift_north(&self) -> Option<Bitboard> {
        if self.0 & RANK_8 != 0 {
            None
        } else {
            Some(Bitboard(self.0 << 8))
        }
    }

    /// Returns a new bitboard with all bits shifted south (down) by one rank, or None if any bits would shift off the board
    pub fn shift_south(&self) -> Option<Bitboard> {
        if self.0 & RANK_1 != 0 {
            None
        } else {
            Some(Bitboard(self.0 >> 8))
        }
    }

    /// Returns a new bitboard with all bits shifted east (right) by one file, or None if any bits would shift off the board
    pub fn shift_east(&self) -> Option<Bitboard> {
        if self.0 & FILE_H != 0 {
            None
        } else {
            Some(Bitboard(self.0 << 1))
        }
    }

    /// Returns a new bitboard with all bits shifted west (left) by one file, or None if any bits would shift off the board
    pub fn shift_west(&self) -> Option<Bitboard> {
        if self.0 & FILE_A != 0 {
            None
        } else {
            Some(Bitboard(self.0 >> 1))
        }
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self {
        Bitboard(!self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{FILE_A, FILE_H, RANK_1, RANK_8};

    #[test]
    fn test_empty_and_full() {
        assert_eq!(Bitboard::empty().as_u64(), 0);
        assert_eq!(Bitboard::full().as_u64(), !0);
    }

    #[test]
    fn test_set_clear_test_bit() {
        let mut bb = Bitboard::empty();
        bb.set_bit(0);
        assert!(bb.test_bit(0));
        assert!(!bb.test_bit(1));
        bb.clear_bit(0);
        assert!(!bb.test_bit(0));
    }

    #[test]
    fn test_pop_count() {
        let mut bb = Bitboard::empty();
        assert_eq!(bb.pop_count(), 0);
        bb.set_bit(0);
        bb.set_bit(1);
        bb.set_bit(63);
        assert_eq!(bb.pop_count(), 3);
    }

    #[test]
    fn test_lsb_msb() {
        let mut bb = Bitboard::empty();
        assert_eq!(bb.lsb(), None);
        assert_eq!(bb.msb(), None);
        bb.set_bit(0);
        bb.set_bit(63);
        assert_eq!(bb.lsb(), Some(0));
        assert_eq!(bb.msb(), Some(63));
    }

    #[test]
    fn test_shifts() {
        let mut bb = Bitboard::empty();
        bb.set_bit(8); // e2

        // Test north shift
        let north = bb.shift_north().expect("Should be able to shift north");
        assert!(north.test_bit(16)); // e3

        // Test south shift
        let south = bb.shift_south().expect("Should be able to shift south");
        assert!(south.test_bit(0)); // e1

        // Test edge cases
        bb = Bitboard::empty();
        bb.set_bit(0); // a1
        assert!(
            bb.shift_west().is_none(),
            "Should not be able to shift west from a1"
        );

        let east = bb.shift_east().expect("Should be able to shift east");
        assert!(east.test_bit(1)); // b1
    }
}
