use checkbit::board::{Board, Color, PieceType};

// Helper function to setup a position from FEN-like description
fn setup_position(pieces: &[(&str, PieceType, Color)]) -> Board {
    let mut board = Board::empty();
    for (square_name, piece_type, color) in pieces {
        let file = square_name.chars().nth(0).unwrap() as u8 - b'a';
        let rank = square_name.chars().nth(1).unwrap() as u8 - b'1';
        let square = rank * 8 + file;
        board.place_piece(*piece_type, *color, square);
    }
    board
}

#[test]
fn test_scholars_mate_position() {
    // Test the Scholar's Mate position (1.e4 e5 2.Bc4 Nc6 3.Qh5 Nf6?? 4.Qxf7#)
    let position = [
        ("e4", PieceType::Pawn, Color::White),
        ("e5", PieceType::Pawn, Color::Black),
        ("c4", PieceType::Bishop, Color::White),
        ("c6", PieceType::Knight, Color::Black),
        ("h5", PieceType::Queen, Color::White),
        ("f6", PieceType::Knight, Color::Black),
        ("e1", PieceType::King, Color::White),
        ("e8", PieceType::King, Color::Black),
    ];

    let board = setup_position(&position);

    // Verify queen can checkmate on f7
    let queen_moves = board.get_moves(board.get_piece_at(39).unwrap().0 as u8); // h5
    assert!(queen_moves.test_bit(45)); // f7
}

#[test]
fn test_pin_position() {
    // Test a position where a piece is pinned to the king
    let position = [
        ("e1", PieceType::King, Color::White),
        ("e2", PieceType::Knight, Color::White), // Pinned knight
        ("e8", PieceType::Rook, Color::Black),   // Pinning rook
    ];

    let board = setup_position(&position);

    // Verify pinned knight has no legal moves
    let knight_moves = board.get_moves(12); // e2
    assert_eq!(knight_moves.pop_count(), 0);
}

#[test]
fn test_discovered_attack_position() {
    // Test a position with a potential discovered attack
    let position = [
        ("e4", PieceType::Bishop, Color::White),
        ("f4", PieceType::Rook, Color::White),
        ("h4", PieceType::King, Color::Black),
    ];

    let board = setup_position(&position);

    // Verify bishop can move while maintaining rook's attack
    let bishop_moves = board.get_moves(28); // e4
    assert!(bishop_moves.pop_count() > 0);
}

#[test]
fn test_pawn_promotion_position() {
    // Test a position where a pawn is about to promote
    let position = [
        ("e7", PieceType::Pawn, Color::White),
        ("e1", PieceType::King, Color::White),
        ("e8", PieceType::King, Color::Black),
    ];

    let board = setup_position(&position);

    // Verify pawn can move to promotion square
    let pawn_moves = board.get_moves(52); // e7
    assert!(pawn_moves.test_bit(60)); // e8
}

#[test]
fn test_trapped_piece_position() {
    // Test a position where a piece is trapped
    let position = [
        ("a8", PieceType::Rook, Color::White),
        ("a7", PieceType::Pawn, Color::Black),
        ("b7", PieceType::Pawn, Color::Black),
        ("b8", PieceType::King, Color::Black),
    ];

    let board = setup_position(&position);

    // Verify trapped rook has no legal moves
    let rook_moves = board.get_moves(56); // a8
    assert_eq!(rook_moves.pop_count(), 0);
}

#[test]
fn test_maximum_mobility_position() {
    // Test a position where a queen has maximum mobility
    let position = [
        ("d4", PieceType::Queen, Color::White),
        ("e1", PieceType::King, Color::White),
        ("e8", PieceType::King, Color::Black),
    ];

    let board = setup_position(&position);

    // Verify queen has maximum possible moves from center
    let queen_moves = board.get_moves(27); // d4
    assert_eq!(queen_moves.pop_count(), 27); // Queen should have 27 moves from d4
}

#[test]
fn test_king_safety_position() {
    // Test a position evaluating king safety
    let position = [
        ("g1", PieceType::King, Color::White),
        ("f1", PieceType::Rook, Color::White),
        ("h1", PieceType::Rook, Color::White),
        ("g2", PieceType::Pawn, Color::White),
        ("f2", PieceType::Pawn, Color::White),
        ("h2", PieceType::Pawn, Color::White),
    ];

    let board = setup_position(&position);

    // Verify king has limited but safe moves
    let king_moves = board.get_moves(6); // g1
    assert!(king_moves.pop_count() <= 3); // King should have very limited mobility
}

#[test]
fn test_fork_position() {
    // Test a position with a knight fork
    let position = [
        ("d4", PieceType::Knight, Color::White),
        ("c6", PieceType::King, Color::Black),
        ("e6", PieceType::Queen, Color::Black),
    ];

    let board = setup_position(&position);

    // Verify knight can attack both pieces
    let knight_moves = board.get_moves(27); // d4
    assert!(knight_moves.test_bit(42)); // c6
    assert!(knight_moves.test_bit(44)); // e6
}

#[test]
fn test_double_attack_position() {
    // Test a position with multiple attackers
    let position = [
        ("d4", PieceType::Queen, Color::White),
        ("h4", PieceType::Rook, Color::White),
        ("d8", PieceType::King, Color::Black),
    ];

    let board = setup_position(&position);

    // Verify both pieces can attack the king's position
    let queen_moves = board.get_moves(27); // d4
    let rook_moves = board.get_moves(31); // h4
    assert!(queen_moves.test_bit(59)); // d8
    assert!(rook_moves.test_bit(59)); // d8
}

#[test]
fn test_blocked_pawns_position() {
    // Test a position with blocked pawns
    let position = [
        ("d4", PieceType::Pawn, Color::White),
        ("d5", PieceType::Pawn, Color::Black),
        ("e4", PieceType::Pawn, Color::White),
        ("e5", PieceType::Pawn, Color::Black),
    ];

    let board = setup_position(&position);

    // Verify pawns are properly blocked
    let white_d_pawn_moves = board.get_moves(27); // d4
    let black_d_pawn_moves = board.get_moves(35); // d5
    assert_eq!(white_d_pawn_moves.pop_count(), 0);
    assert_eq!(black_d_pawn_moves.pop_count(), 0);
}
