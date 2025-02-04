use checkbit::bitboard::Bitboard;
use checkbit::constants::{FILE_A, FILE_H, RANK_1, RANK_8};

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

// Additional comprehensive tests for bitboard operations
#[test]
fn test_bitwise_operations() {
    let bb1 = Bitboard::from_u64(0x00FF);
    let bb2 = Bitboard::from_u64(0xFF00);

    // Test AND
    assert_eq!((bb1 & bb2).as_u64(), 0);

    // Test OR
    assert_eq!((bb1 | bb2).as_u64(), 0xFFFF);

    // Test XOR
    assert_eq!((bb1 ^ bb2).as_u64(), 0xFFFF);

    // Test NOT
    assert_eq!(!bb1 & Bitboard::from_u64(0xFFFF), bb2);
}

#[test]
fn test_edge_cases() {
    let mut bb = Bitboard::empty();

    // Test corners
    bb.set_bit(0); // a1
    bb.set_bit(7); // h1
    bb.set_bit(56); // a8
    bb.set_bit(63); // h8

    assert_eq!(bb.pop_count(), 4);

    // Test shifts from edges
    assert!(bb.shift_north().is_none()); // Should fail due to pieces on rank 8
    assert!(bb.shift_south().is_none()); // Should fail due to pieces on rank 1
    assert!(bb.shift_east().is_none()); // Should fail due to pieces on file h
    assert!(bb.shift_west().is_none()); // Should fail due to pieces on file a
}

#[test]
fn test_bit_patterns() {
    // Test diagonal pattern
    let mut bb = Bitboard::empty();
    for i in 0..8 {
        bb.set_bit(i * 9); // Set bits on the main diagonal
    }
    assert_eq!(bb.pop_count(), 8);

    // Test L-shape pattern (knight moves from center)
    let mut bb = Bitboard::empty();
    bb.set_bit(27); // d4
    let moves = vec![
        10, 17, // Up-right
        15, 6, // Up-left
        -6, -15, // Down-left
        -10, -17, // Down-right
    ];
    for offset in moves {
        if offset > 0 {
            bb.set_bit((27 + offset) as u8);
        } else {
            bb.set_bit((27 as i8 + offset) as u8);
        }
    }
    assert_eq!(bb.pop_count(), 9); // Center + 8 knight moves
}
