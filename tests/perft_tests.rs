use checkbit::board::{Board, Color, PieceType};

// Helper function to count all possible moves at a given depth
fn perft(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    // For each square
    for square in 0..64 {
        if let Some((piece_type, color)) = board.get_piece_at(square) {
            // Get all possible moves for this piece
            let moves = board.get_moves(square);

            // For each possible move
            for target_square in 0..64 {
                if moves.test_bit(target_square) {
                    // Make move
                    let mut new_board = board.clone();
                    new_board.place_piece(piece_type, color, target_square);

                    // Count nodes in subtree
                    nodes += perft(&new_board, depth - 1);
                }
            }
        }
    }

    nodes
}

#[test]
fn test_perft_initial_position_depth_1() {
    let board = Board::initial();
    assert_eq!(perft(&board, 1), 20); // Initial position should have 20 possible moves
}

#[test]
fn test_perft_initial_position_depth_2() {
    let board = Board::initial();
    assert_eq!(perft(&board, 2), 400); // After white's first move and black's response
}

#[test]
fn test_perft_empty_board_with_kings() {
    let mut board = Board::empty();

    // Place just the kings
    board.place_piece(PieceType::King, Color::White, 4); // e1
    board.place_piece(PieceType::King, Color::Black, 60); // e8

    assert_eq!(perft(&board, 1), 5); // White king should have 5 moves
}

#[test]
fn test_perft_single_knight() {
    let mut board = Board::empty();

    // Place a single knight in the center
    board.place_piece(PieceType::Knight, Color::White, 27); // d4
    board.place_piece(PieceType::King, Color::White, 4); // e1
    board.place_piece(PieceType::King, Color::Black, 60); // e8

    assert_eq!(perft(&board, 1), 13); // 8 knight moves + 5 king moves
}

#[test]
fn test_perft_bishops() {
    let mut board = Board::empty();

    // Place bishops on their starting squares
    board.place_piece(PieceType::Bishop, Color::White, 2); // c1
    board.place_piece(PieceType::Bishop, Color::White, 5); // f1
    board.place_piece(PieceType::King, Color::White, 4); // e1
    board.place_piece(PieceType::King, Color::Black, 60); // e8

    // Each bishop should have 7 moves initially, plus 5 king moves
    assert_eq!(perft(&board, 1), 19);
}

#[test]
fn test_perft_rooks() {
    let mut board = Board::empty();

    // Place rooks on their starting squares
    board.place_piece(PieceType::Rook, Color::White, 0); // a1
    board.place_piece(PieceType::Rook, Color::White, 7); // h1
    board.place_piece(PieceType::King, Color::White, 4); // e1
    board.place_piece(PieceType::King, Color::Black, 60); // e8

    // Each rook should have 14 moves initially, plus 5 king moves
    assert_eq!(perft(&board, 1), 33);
}

#[test]
fn test_perft_queen() {
    let mut board = Board::empty();

    // Place queen on starting square
    board.place_piece(PieceType::Queen, Color::White, 3); // d1
    board.place_piece(PieceType::King, Color::White, 4); // e1
    board.place_piece(PieceType::King, Color::Black, 60); // e8

    // Queen should have 21 moves initially, plus 5 king moves
    assert_eq!(perft(&board, 1), 26);
}

#[test]
fn test_perft_complex_position() {
    let mut board = Board::empty();

    // Setup a complex middlegame position
    // White pieces
    board.place_piece(PieceType::King, Color::White, 4); // e1
    board.place_piece(PieceType::Queen, Color::White, 19); // d3
    board.place_piece(PieceType::Rook, Color::White, 0); // a1
    board.place_piece(PieceType::Bishop, Color::White, 28); // e4
    board.place_piece(PieceType::Knight, Color::White, 42); // c6

    // Black pieces
    board.place_piece(PieceType::King, Color::Black, 60); // e8
    board.place_piece(PieceType::Queen, Color::Black, 59); // d8
    board.place_piece(PieceType::Rook, Color::Black, 56); // a8
    board.place_piece(PieceType::Bishop, Color::Black, 61); // f8
    board.place_piece(PieceType::Knight, Color::Black, 57); // b8

    // Test depth 1 for this position
    let nodes = perft(&board, 1);
    assert!(nodes > 0); // Should have multiple possible moves
    println!("Complex position perft(1) nodes: {}", nodes);
}

// Note: In a real chess engine, you would want to add more perft tests
// with known positions and their node counts at various depths.
// These would serve as regression tests to ensure move generation
// remains accurate as you optimize the code.

// You would also want to add benchmarks to measure the performance
// of move generation and track improvements. Consider using
// Rust's built-in benchmark framework or criterion.rs for this.
