use checkbit::constants::*;

#[test]
fn test_file_masks() {
    assert_eq!(FILE_A & FILE_B, 0);
    assert_eq!(
        FILE_A | FILE_B | FILE_C | FILE_D | FILE_E | FILE_F | FILE_G | FILE_H,
        !0
    );
}

#[test]
fn test_rank_masks() {
    assert_eq!(RANK_1 & RANK_2, 0);
    assert_eq!(
        RANK_1 | RANK_2 | RANK_3 | RANK_4 | RANK_5 | RANK_6 | RANK_7 | RANK_8,
        !0
    );
}

#[test]
fn test_initial_positions() {
    // Test that initial positions don't overlap
    let white_pieces = WHITE_PAWNS_INIT
        | WHITE_KNIGHTS_INIT
        | WHITE_BISHOPS_INIT
        | WHITE_ROOKS_INIT
        | WHITE_QUEENS_INIT
        | WHITE_KING_INIT;
    let black_pieces = BLACK_PAWNS_INIT
        | BLACK_KNIGHTS_INIT
        | BLACK_BISHOPS_INIT
        | BLACK_ROOKS_INIT
        | BLACK_QUEENS_INIT
        | BLACK_KING_INIT;

    assert_eq!(white_pieces & black_pieces, 0);
    assert_eq!(white_pieces.count_ones(), 16);
    assert_eq!(black_pieces.count_ones(), 16);
}

#[test]
fn test_knight_moves() {
    // Test knight moves from center square (d4)
    let center_moves = KNIGHT_MOVES[27]; // d4 is index 27
    assert_eq!(center_moves.count_ones(), 8); // Should have 8 possible moves

    // Test knight moves from corner (a1)
    let corner_moves = KNIGHT_MOVES[0];
    assert_eq!(corner_moves.count_ones(), 2); // Should only have 2 possible moves
}

#[test]
fn test_king_moves() {
    // Test king moves from center square (d4)
    let center_moves = KING_MOVES[27]; // d4 is index 27
    assert_eq!(center_moves.count_ones(), 8); // Should have 8 possible moves

    // Test king moves from corner (a1)
    let corner_moves = KING_MOVES[0];
    assert_eq!(corner_moves.count_ones(), 3); // Should only have 3 possible moves
}

// Additional comprehensive tests for constants

#[test]
fn test_file_relationships() {
    // Test file adjacency
    assert_eq!(FILE_A << 1, FILE_B);
    assert_eq!(FILE_B << 1, FILE_C);
    assert_eq!(FILE_C << 1, FILE_D);
    assert_eq!(FILE_D << 1, FILE_E);
    assert_eq!(FILE_E << 1, FILE_F);
    assert_eq!(FILE_F << 1, FILE_G);
    assert_eq!(FILE_G << 1, FILE_H);

    // Test file isolation
    let all_files = FILE_A | FILE_B | FILE_C | FILE_D | FILE_E | FILE_F | FILE_G | FILE_H;
    for file in [
        FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
    ] {
        // Each file should be unique
        assert_eq!((file & (all_files ^ file)), 0);
        // Each file should have exactly 8 bits set
        assert_eq!(file.count_ones(), 8);
    }
}

#[test]
fn test_rank_relationships() {
    // Test rank adjacency
    assert_eq!(RANK_1 << 8, RANK_2);
    assert_eq!(RANK_2 << 8, RANK_3);
    assert_eq!(RANK_3 << 8, RANK_4);
    assert_eq!(RANK_4 << 8, RANK_5);
    assert_eq!(RANK_5 << 8, RANK_6);
    assert_eq!(RANK_6 << 8, RANK_7);
    assert_eq!(RANK_7 << 8, RANK_8);

    // Test rank isolation
    let all_ranks = RANK_1 | RANK_2 | RANK_3 | RANK_4 | RANK_5 | RANK_6 | RANK_7 | RANK_8;
    for rank in [
        RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
    ] {
        // Each rank should be unique
        assert_eq!((rank & (all_ranks ^ rank)), 0);
        // Each rank should have exactly 8 bits set
        assert_eq!(rank.count_ones(), 8);
    }
}

#[test]
fn test_center_squares() {
    // Test center squares definition
    assert_eq!(CENTER_SQUARES.count_ones(), 4); // Should be exactly 4 center squares
    assert!(CENTER_SQUARES & (1u64 << 27) != 0); // d4
    assert!(CENTER_SQUARES & (1u64 << 28) != 0); // e4
    assert!(CENTER_SQUARES & (1u64 << 35) != 0); // d5
    assert!(CENTER_SQUARES & (1u64 << 36) != 0); // e5

    // Test extended center
    assert_eq!(EXTENDED_CENTER.count_ones(), 16); // Should be 16 squares
    assert!(EXTENDED_CENTER & CENTER_SQUARES == CENTER_SQUARES); // Should include center squares
}

#[test]
fn test_castling_squares() {
    // Test kingside castling squares
    assert_eq!(KINGSIDE_SQUARES.count_ones(), 2); // f1 and g1 for white
    assert!(KINGSIDE_SQUARES & (1u64 << 5) != 0); // f1
    assert!(KINGSIDE_SQUARES & (1u64 << 6) != 0); // g1

    // Test queenside castling squares
    assert_eq!(QUEENSIDE_SQUARES.count_ones(), 3); // b1, c1, and d1 for white
    assert!(QUEENSIDE_SQUARES & (1u64 << 1) != 0); // b1
    assert!(QUEENSIDE_SQUARES & (1u64 << 2) != 0); // c1
    assert!(QUEENSIDE_SQUARES & (1u64 << 3) != 0); // d1
}

#[test]
fn test_ray_moves() {
    // Test north ray from d4
    let north = NORTH_RAY[27]; // d4
    assert_eq!(north.count_ones(), 4); // Should have 4 squares (d5-d8)

    // Test south ray from d5
    let south = SOUTH_RAY[35]; // d5
    assert_eq!(south.count_ones(), 4); // Should have 4 squares (d4-d1)

    // Test east ray from d4
    let east = EAST_RAY[27]; // d4
    assert_eq!(east.count_ones(), 4); // Should have 4 squares (e4-h4)

    // Test west ray from d4
    let west = WEST_RAY[27]; // d4
    assert_eq!(west.count_ones(), 3); // Should have 3 squares (c4-a4)
}

#[test]
fn test_initial_piece_positions() {
    // Test white piece initial positions
    assert_eq!(WHITE_PAWNS_INIT, RANK_2);
    assert_eq!(WHITE_KNIGHTS_INIT & RANK_1, WHITE_KNIGHTS_INIT);
    assert_eq!(WHITE_BISHOPS_INIT & RANK_1, WHITE_BISHOPS_INIT);
    assert_eq!(WHITE_ROOKS_INIT & RANK_1, WHITE_ROOKS_INIT);
    assert_eq!(WHITE_QUEENS_INIT & RANK_1, WHITE_QUEENS_INIT);
    assert_eq!(WHITE_KING_INIT & RANK_1, WHITE_KING_INIT);

    // Test black piece initial positions
    assert_eq!(BLACK_PAWNS_INIT, RANK_7);
    assert_eq!(BLACK_KNIGHTS_INIT & RANK_8, BLACK_KNIGHTS_INIT);
    assert_eq!(BLACK_BISHOPS_INIT & RANK_8, BLACK_BISHOPS_INIT);
    assert_eq!(BLACK_ROOKS_INIT & RANK_8, BLACK_ROOKS_INIT);
    assert_eq!(BLACK_QUEENS_INIT & RANK_8, BLACK_QUEENS_INIT);
    assert_eq!(BLACK_KING_INIT & RANK_8, BLACK_KING_INIT);
}
