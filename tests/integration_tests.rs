use checkbit::board::{Board, Color, PieceType};

#[test]
fn test_basic_game_sequence() {
    let mut board = Board::initial();

    // Test e4 opening
    let e2 = 12; // e2 square
    let e4 = 28; // e4 square
    assert_eq!(
        board.get_piece_at(e2),
        Some((PieceType::Pawn, Color::White))
    );
    let moves = board.get_moves(e2);
    assert!(moves.test_bit(e4)); // e4 should be a valid move

    // Make the move
    let piece = board.get_piece_at(e2).unwrap();
    board.place_piece(piece.0, piece.1, e4);
    assert_eq!(
        board.get_piece_at(e4),
        Some((PieceType::Pawn, Color::White))
    );
    assert_eq!(board.get_piece_at(e2), None);
}

#[test]
fn test_capture_sequence() {
    let mut board = Board::empty();

    // Setup a simple capture position
    board.place_piece(PieceType::Pawn, Color::White, 35); // d5
    board.place_piece(PieceType::Pawn, Color::Black, 44); // e6

    // Test capture is possible
    let moves = board.get_moves(35);
    assert!(moves.test_bit(44)); // Should be able to capture on e6

    // Make the capture
    let piece = board.get_piece_at(35).unwrap();
    board.place_piece(piece.0, piece.1, 44);
    assert_eq!(
        board.get_piece_at(44),
        Some((PieceType::Pawn, Color::White))
    );
    assert_eq!(board.get_piece_at(35), None);
}

#[test]
fn test_en_passant_sequence() {
    let mut board = Board::empty();

    // Setup en passant position
    board.place_piece(PieceType::Pawn, Color::White, 35); // d5
    board.place_piece(PieceType::Pawn, Color::Black, 49); // c7

    // Move black pawn two squares
    board.place_piece(PieceType::Pawn, Color::Black, 33); // c5

    // Test en passant capture is possible
    let moves = board.get_moves(35);
    assert!(moves.test_bit(32)); // Should be able to capture en passant on c6
}

#[test]
fn test_piece_development() {
    let mut board = Board::initial();

    // Test knight development
    let b1 = 1; // b1 square
    let c3 = 18; // c3 square
    assert_eq!(
        board.get_piece_at(b1),
        Some((PieceType::Knight, Color::White))
    );
    let moves = board.get_moves(b1);
    assert!(moves.test_bit(c3)); // Nc3 should be a valid move

    // Make the move
    let piece = board.get_piece_at(b1).unwrap();
    board.place_piece(piece.0, piece.1, c3);
    assert_eq!(
        board.get_piece_at(c3),
        Some((PieceType::Knight, Color::White))
    );
    assert_eq!(board.get_piece_at(b1), None);
}

#[test]
fn test_complex_position() {
    let mut board = Board::empty();

    // Setup a complex middlegame position
    // White pieces
    board.place_piece(PieceType::King, Color::White, 4); // e1
    board.place_piece(PieceType::Queen, Color::White, 19); // d3
    board.place_piece(PieceType::Rook, Color::White, 0); // a1
    board.place_piece(PieceType::Bishop, Color::White, 28); // e4
    board.place_piece(PieceType::Knight, Color::White, 42); // c6
    board.place_piece(PieceType::Pawn, Color::White, 8); // a2
    board.place_piece(PieceType::Pawn, Color::White, 9); // b2
    board.place_piece(PieceType::Pawn, Color::White, 10); // c2

    // Black pieces
    board.place_piece(PieceType::King, Color::Black, 60); // e8
    board.place_piece(PieceType::Queen, Color::Black, 59); // d8
    board.place_piece(PieceType::Rook, Color::Black, 56); // a8
    board.place_piece(PieceType::Bishop, Color::Black, 61); // f8
    board.place_piece(PieceType::Knight, Color::Black, 57); // b8
    board.place_piece(PieceType::Pawn, Color::Black, 48); // a7
    board.place_piece(PieceType::Pawn, Color::Black, 49); // b7
    board.place_piece(PieceType::Pawn, Color::Black, 50); // c7

    // Test piece mobility
    let white_queen_moves = board.get_moves(19); // d3
    let black_queen_moves = board.get_moves(59); // d8
    assert!(white_queen_moves.pop_count() > 0);
    assert!(black_queen_moves.pop_count() > 0);

    // Test piece interactions
    let white_bishop_moves = board.get_moves(28); // e4
    let black_knight_moves = board.get_moves(57); // b8
    assert!(white_bishop_moves.pop_count() > 0);
    assert!(black_knight_moves.pop_count() > 0);
}

#[test]
fn test_pawn_structure() {
    let mut board = Board::empty();

    // Setup a pawn chain
    board.place_piece(PieceType::Pawn, Color::White, 27); // d4
    board.place_piece(PieceType::Pawn, Color::White, 36); // e5
    board.place_piece(PieceType::Pawn, Color::White, 45); // f6

    board.place_piece(PieceType::Pawn, Color::Black, 34); // c5
    board.place_piece(PieceType::Pawn, Color::Black, 43); // d6
    board.place_piece(PieceType::Pawn, Color::Black, 52); // e7

    // Test pawn moves in chain
    let white_pawn_moves = board.get_moves(36); // e5
    let black_pawn_moves = board.get_moves(43); // d6

    // Pawns should be restricted by the chain
    assert!(white_pawn_moves.pop_count() < 2);
    assert!(black_pawn_moves.pop_count() < 2);
}
