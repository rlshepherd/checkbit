use checkbit::board::{Board, Color, PieceType};
use checkbit::constants::*;

#[test]
fn test_initial_board() {
    let board = Board::initial();

    // Check total piece counts
    assert_eq!(board.get_color_pieces(Color::White).pop_count(), 16);
    assert_eq!(board.get_color_pieces(Color::Black).pop_count(), 16);

    // Check specific pieces
    assert_eq!(
        board.get_pieces(PieceType::Pawn, Color::White).pop_count(),
        8
    );
    assert_eq!(
        board
            .get_pieces(PieceType::Knight, Color::White)
            .pop_count(),
        2
    );
    assert_eq!(
        board
            .get_pieces(PieceType::Bishop, Color::White)
            .pop_count(),
        2
    );
    assert_eq!(
        board.get_pieces(PieceType::Rook, Color::White).pop_count(),
        2
    );
    assert_eq!(
        board.get_pieces(PieceType::Queen, Color::White).pop_count(),
        1
    );
    assert_eq!(
        board.get_pieces(PieceType::King, Color::White).pop_count(),
        1
    );
}

#[test]
fn test_get_piece_at() {
    let board = Board::initial();

    // Test white pieces
    assert_eq!(board.get_piece_at(8), Some((PieceType::Pawn, Color::White))); // a2
    assert_eq!(
        board.get_piece_at(1),
        Some((PieceType::Knight, Color::White))
    ); // b1
    assert_eq!(
        board.get_piece_at(2),
        Some((PieceType::Bishop, Color::White))
    ); // c1
    assert_eq!(board.get_piece_at(0), Some((PieceType::Rook, Color::White))); // a1
    assert_eq!(
        board.get_piece_at(3),
        Some((PieceType::Queen, Color::White))
    ); // d1
    assert_eq!(board.get_piece_at(4), Some((PieceType::King, Color::White))); // e1

    // Test empty square
    assert_eq!(board.get_piece_at(16), None); // a3
}

#[test]
fn test_knight_moves() {
    let mut board = Board::empty();
    board.place_piece(PieceType::Knight, Color::White, 27); // d4

    let moves = board.get_moves(27);
    assert_eq!(moves.pop_count(), 8); // Knight should have 8 moves from d4
}

#[test]
fn test_pawn_moves() {
    let mut board = Board::empty();

    // Test white pawn initial moves
    board.place_piece(PieceType::Pawn, Color::White, 8); // a2
    let moves = board.get_moves(8);
    assert_eq!(moves.pop_count(), 2); // Should have 2 moves (single and double push)

    // Test white pawn capture
    board.place_piece(PieceType::Pawn, Color::Black, 17); // b3
    let moves = board.get_moves(8);
    assert_eq!(moves.pop_count(), 3); // Should have 3 moves (pushes + capture)
}

#[test]
fn test_bishop_moves() {
    let mut board = Board::empty();
    board.place_piece(PieceType::Bishop, Color::White, 27); // d4

    let moves = board.get_moves(27);
    assert_eq!(moves.pop_count(), 13); // Bishop should have 13 moves from d4

    // Test blocking
    board.place_piece(PieceType::Pawn, Color::White, 45); // f6
    let moves = board.get_moves(27);
    assert_eq!(moves.pop_count(), 10); // Should have 3 fewer moves due to blocking
}

// Additional comprehensive tests for board operations

#[test]
fn test_rook_moves() {
    let mut board = Board::empty();
    board.place_piece(PieceType::Rook, Color::White, 27); // d4

    let moves = board.get_moves(27);
    assert_eq!(moves.pop_count(), 14); // Rook should have 14 moves from d4

    // Test blocking
    board.place_piece(PieceType::Pawn, Color::White, 27 + 8); // d5
    let moves = board.get_moves(27);
    assert_eq!(moves.pop_count(), 10); // Should have 4 fewer moves due to blocking (can't move through or capture own pawn)
}

#[test]
fn test_queen_moves() {
    let mut board = Board::empty();
    board.place_piece(PieceType::Queen, Color::White, 27); // d4

    let moves = board.get_moves(27);
    assert_eq!(moves.pop_count(), 27); // Queen should have 27 moves from d4 (13 bishop + 14 rook)

    // Test blocking
    board.place_piece(PieceType::Pawn, Color::White, 27 + 8); // d5
    board.place_piece(PieceType::Pawn, Color::White, 27 + 9); // e5
    let moves = board.get_moves(27);
    assert_eq!(moves.pop_count(), 19); // Should have 8 fewer moves due to blocking (can't move through or capture own pawns)
}

#[test]
fn test_king_moves() {
    let mut board = Board::empty();
    board.place_piece(PieceType::King, Color::White, 27); // d4

    let moves = board.get_moves(27);
    assert_eq!(moves.pop_count(), 8); // King should have 8 moves from d4

    // Test corner position
    board = Board::empty();
    board.place_piece(PieceType::King, Color::White, 0); // a1
    let moves = board.get_moves(0);
    assert_eq!(moves.pop_count(), 3); // King should have 3 moves from a1
}

#[test]
fn test_pawn_special_moves() {
    let mut board = Board::empty();

    // Test en passant
    board.place_piece(PieceType::Pawn, Color::White, 35); // d5
    board.place_piece(PieceType::Pawn, Color::Black, 34); // c5
                                                          // Simulate black pawn moving two squares
    board = Board::empty();
    board.place_piece(PieceType::Pawn, Color::White, 35); // d5
    board.place_piece(PieceType::Pawn, Color::Black, 49); // c7
    board.place_piece(PieceType::Pawn, Color::Black, 33); // c4 (moved two squares)
    let moves = board.get_moves(35);
    assert!(moves.pop_count() > 0); // Should have at least one move (en passant)

    // Test promotion squares
    board = Board::empty();
    board.place_piece(PieceType::Pawn, Color::White, 48); // a7
    let moves = board.get_moves(48);
    assert!(moves.pop_count() > 0); // Should have promotion moves
}

#[test]
fn test_piece_capture() {
    let mut board = Board::empty();

    // Setup pieces
    board.place_piece(PieceType::Rook, Color::White, 27); // d4
    board.place_piece(PieceType::Pawn, Color::Black, 35); // d5
    board.place_piece(PieceType::Knight, Color::Black, 19); // d3

    let moves = board.get_moves(27);
    assert!(moves.test_bit(35)); // Should be able to capture pawn
    assert!(moves.test_bit(19)); // Should be able to capture knight

    // Verify can't capture own pieces
    board.place_piece(PieceType::Pawn, Color::White, 43); // d6
    let moves = board.get_moves(27);
    assert!(!moves.test_bit(43)); // Should not be able to capture own pawn
}

#[test]
fn test_board_state() {
    let mut board = Board::empty();

    // Test piece placement
    board.place_piece(PieceType::Queen, Color::White, 27);
    assert_eq!(
        board.get_piece_at(27),
        Some((PieceType::Queen, Color::White))
    );

    // Test piece tracking
    let white_pieces = board.get_color_pieces(Color::White);
    assert_eq!(white_pieces.pop_count(), 1);
    assert!(white_pieces.test_bit(27));

    // Test all pieces tracking
    board.place_piece(PieceType::Knight, Color::Black, 36);
    let all_pieces = board.get_all_pieces();
    assert_eq!(all_pieces.pop_count(), 2);
    assert!(all_pieces.test_bit(27));
    assert!(all_pieces.test_bit(36));
}
