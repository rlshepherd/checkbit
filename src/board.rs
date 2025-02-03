use crate::bitboard::Bitboard;
use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub struct Board {
    white_pawns: Bitboard,
    white_knights: Bitboard,
    white_bishops: Bitboard,
    white_rooks: Bitboard,
    white_queens: Bitboard,
    white_king: Bitboard,
    black_pawns: Bitboard,
    black_knights: Bitboard,
    black_bishops: Bitboard,
    black_rooks: Bitboard,
    black_queens: Bitboard,
    black_king: Bitboard,
    en_passant_square: Option<u8>, // Square behind a pawn that just moved two squares
}

impl Board {
    /// Creates a new empty board
    pub fn empty() -> Self {
        Board {
            white_pawns: Bitboard::empty(),
            white_knights: Bitboard::empty(),
            white_bishops: Bitboard::empty(),
            white_rooks: Bitboard::empty(),
            white_queens: Bitboard::empty(),
            white_king: Bitboard::empty(),
            black_pawns: Bitboard::empty(),
            black_knights: Bitboard::empty(),
            black_bishops: Bitboard::empty(),
            black_rooks: Bitboard::empty(),
            black_queens: Bitboard::empty(),
            black_king: Bitboard::empty(),
            en_passant_square: None,
        }
    }

    /// Creates a new board with pieces in their initial positions
    pub fn initial() -> Self {
        Board {
            white_pawns: Bitboard::from_u64(WHITE_PAWNS_INIT),
            white_knights: Bitboard::from_u64(WHITE_KNIGHTS_INIT),
            white_bishops: Bitboard::from_u64(WHITE_BISHOPS_INIT),
            white_rooks: Bitboard::from_u64(WHITE_ROOKS_INIT),
            white_queens: Bitboard::from_u64(WHITE_QUEENS_INIT),
            white_king: Bitboard::from_u64(WHITE_KING_INIT),
            black_pawns: Bitboard::from_u64(BLACK_PAWNS_INIT),
            black_knights: Bitboard::from_u64(BLACK_KNIGHTS_INIT),
            black_bishops: Bitboard::from_u64(BLACK_BISHOPS_INIT),
            black_rooks: Bitboard::from_u64(BLACK_ROOKS_INIT),
            black_queens: Bitboard::from_u64(BLACK_QUEENS_INIT),
            black_king: Bitboard::from_u64(BLACK_KING_INIT),
            en_passant_square: None,
        }
    }

    /// Gets all pieces of a given color
    pub fn get_color_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::White => {
                self.white_pawns
                    | self.white_knights
                    | self.white_bishops
                    | self.white_rooks
                    | self.white_queens
                    | self.white_king
            }
            Color::Black => {
                self.black_pawns
                    | self.black_knights
                    | self.black_bishops
                    | self.black_rooks
                    | self.black_queens
                    | self.black_king
            }
        }
    }

    /// Gets all pieces of a given type and color
    pub fn get_pieces(&self, piece_type: PieceType, color: Color) -> Bitboard {
        match (color, piece_type) {
            (Color::White, PieceType::Pawn) => self.white_pawns,
            (Color::White, PieceType::Knight) => self.white_knights,
            (Color::White, PieceType::Bishop) => self.white_bishops,
            (Color::White, PieceType::Rook) => self.white_rooks,
            (Color::White, PieceType::Queen) => self.white_queens,
            (Color::White, PieceType::King) => self.white_king,
            (Color::Black, PieceType::Pawn) => self.black_pawns,
            (Color::Black, PieceType::Knight) => self.black_knights,
            (Color::Black, PieceType::Bishop) => self.black_bishops,
            (Color::Black, PieceType::Rook) => self.black_rooks,
            (Color::Black, PieceType::Queen) => self.black_queens,
            (Color::Black, PieceType::King) => self.black_king,
        }
    }

    /// Gets all pieces (both colors)
    pub fn get_all_pieces(&self) -> Bitboard {
        self.get_color_pieces(Color::White) | self.get_color_pieces(Color::Black)
    }

    /// Gets the piece type at a given square, if any
    pub fn get_piece_at(&self, square: u8) -> Option<(PieceType, Color)> {
        let white_pieces = self.get_color_pieces(Color::White);
        let black_pieces = self.get_color_pieces(Color::Black);

        if !white_pieces.test_bit(square) && !black_pieces.test_bit(square) {
            return None;
        }

        let color = if white_pieces.test_bit(square) {
            Color::White
        } else {
            Color::Black
        };

        let piece_type = if self.get_pieces(PieceType::Pawn, color).test_bit(square) {
            PieceType::Pawn
        } else if self.get_pieces(PieceType::Knight, color).test_bit(square) {
            PieceType::Knight
        } else if self.get_pieces(PieceType::Bishop, color).test_bit(square) {
            PieceType::Bishop
        } else if self.get_pieces(PieceType::Rook, color).test_bit(square) {
            PieceType::Rook
        } else if self.get_pieces(PieceType::Queen, color).test_bit(square) {
            PieceType::Queen
        } else {
            PieceType::King
        };

        Some((piece_type, color))
    }

    /// Places a piece on the board and updates en passant square if it's a pawn moving two squares
    pub fn place_piece(&mut self, piece_type: PieceType, color: Color, square: u8) {
        let bitboard = match (color, piece_type) {
            (Color::White, PieceType::Pawn) => &mut self.white_pawns,
            (Color::White, PieceType::Knight) => &mut self.white_knights,
            (Color::White, PieceType::Bishop) => &mut self.white_bishops,
            (Color::White, PieceType::Rook) => &mut self.white_rooks,
            (Color::White, PieceType::Queen) => &mut self.white_queens,
            (Color::White, PieceType::King) => &mut self.white_king,
            (Color::Black, PieceType::Pawn) => &mut self.black_pawns,
            (Color::Black, PieceType::Knight) => &mut self.black_knights,
            (Color::Black, PieceType::Bishop) => &mut self.black_bishops,
            (Color::Black, PieceType::Rook) => &mut self.black_rooks,
            (Color::Black, PieceType::Queen) => &mut self.black_queens,
            (Color::Black, PieceType::King) => &mut self.black_king,
        };

        // Update en passant square if this is a pawn moving two squares
        if piece_type == PieceType::Pawn {
            match color {
                Color::White if square >= 24 && square < 32 => {
                    self.en_passant_square = Some(square - 8);
                }
                Color::Black if square >= 32 && square < 40 => {
                    self.en_passant_square = Some(square + 8);
                }
                _ => self.en_passant_square = None,
            }
        } else {
            self.en_passant_square = None;
        }

        bitboard.set_bit(square);
    }

    /// Gets all legal moves for a piece at the given square
    pub fn get_moves(&self, square: u8) -> Bitboard {
        match self.get_piece_at(square) {
            Some((piece_type, color)) => {
                let moves = match piece_type {
                    PieceType::Knight => self.get_knight_moves(square, color),
                    PieceType::King => self.get_king_moves(square, color),
                    PieceType::Pawn => self.get_pawn_moves(square, color),
                    PieceType::Bishop => self.get_bishop_moves(square, color),
                    PieceType::Rook => self.get_rook_moves(square, color),
                    PieceType::Queen => self.get_queen_moves(square, color),
                };
                // Remove moves that would capture own pieces
                moves & !self.get_color_pieces(color)
            }
            None => Bitboard::empty(),
        }
    }

    fn get_knight_moves(&self, square: u8, _color: Color) -> Bitboard {
        Bitboard::from_u64(KNIGHT_MOVES[square as usize])
    }

    fn get_king_moves(&self, square: u8, _color: Color) -> Bitboard {
        Bitboard::from_u64(KING_MOVES[square as usize])
    }

    fn get_pawn_moves(&self, square: u8, color: Color) -> Bitboard {
        let mut moves = Bitboard::empty();
        let all_pieces = self.get_all_pieces();
        let enemy_pieces = self.get_color_pieces(color.opposite());

        match color {
            Color::White => {
                // Check if pawn is not on the last rank
                if square < 56 {
                    // Single push
                    if !all_pieces.test_bit(square + 8) {
                        moves.set_bit(square + 8);
                        // Double push from starting position
                        if square < 16 && !all_pieces.test_bit(square + 16) {
                            moves.set_bit(square + 16);
                        }
                    }
                    // Regular captures
                    if square % 8 != 0 && square < 57 && enemy_pieces.test_bit(square + 7) {
                        moves.set_bit(square + 7);
                    }
                    if square % 8 != 7 && square < 55 && enemy_pieces.test_bit(square + 9) {
                        moves.set_bit(square + 9);
                    }
                    // En passant captures
                    if let Some(ep_square) = self.en_passant_square {
                        if square >= 32 && square < 40 {
                            // White pawns on rank 5
                            if square % 8 != 0 && ep_square == square + 7 {
                                moves.set_bit(ep_square);
                            }
                            if square % 8 != 7 && ep_square == square + 9 {
                                moves.set_bit(ep_square);
                            }
                        }
                    }
                }
            }
            Color::Black => {
                // Check if pawn is not on the first rank
                if square >= 8 {
                    // Single push
                    if !all_pieces.test_bit(square - 8) {
                        moves.set_bit(square - 8);
                        // Double push from starting position
                        if square >= 48 && !all_pieces.test_bit(square - 16) {
                            moves.set_bit(square - 16);
                        }
                    }
                    // Regular captures
                    if square % 8 != 0 && square >= 9 && enemy_pieces.test_bit(square - 9) {
                        moves.set_bit(square - 9);
                    }
                    if square % 8 != 7 && square >= 7 && enemy_pieces.test_bit(square - 7) {
                        moves.set_bit(square - 7);
                    }
                    // En passant captures
                    if let Some(ep_square) = self.en_passant_square {
                        if square >= 24 && square < 32 {
                            // Black pawns on rank 4
                            if square % 8 != 0 && ep_square == square - 9 {
                                moves.set_bit(ep_square);
                            }
                            if square % 8 != 7 && ep_square == square - 7 {
                                moves.set_bit(ep_square);
                            }
                        }
                    }
                }
            }
        }
        moves
    }

    fn get_bishop_moves(&self, square: u8, color: Color) -> Bitboard {
        let mut moves = Bitboard::empty();
        let all_pieces = self.get_all_pieces();
        let enemy_pieces = self.get_color_pieces(color.opposite());

        // Northeast
        self.add_diagonal_moves(&mut moves, square, true, true, all_pieces, enemy_pieces);
        // Southeast
        self.add_diagonal_moves(&mut moves, square, false, true, all_pieces, enemy_pieces);
        // Southwest
        self.add_diagonal_moves(&mut moves, square, false, false, all_pieces, enemy_pieces);
        // Northwest
        self.add_diagonal_moves(&mut moves, square, true, false, all_pieces, enemy_pieces);

        moves
    }

    fn get_rook_moves(&self, square: u8, color: Color) -> Bitboard {
        let mut moves = Bitboard::empty();
        let all_pieces = self.get_all_pieces();
        let enemy_pieces = self.get_color_pieces(color.opposite());

        // Get all possible moves from precomputed rays
        let north = Bitboard::from_u64(NORTH_RAY[square as usize]);
        let south = Bitboard::from_u64(SOUTH_RAY[square as usize]);
        let east = Bitboard::from_u64(EAST_RAY[square as usize]);
        let west = Bitboard::from_u64(WEST_RAY[square as usize]);

        // For each direction, find the first piece (if any) and adjust moves accordingly
        moves = moves | self.get_ray_moves(north, all_pieces, enemy_pieces);
        moves = moves | self.get_ray_moves(south, all_pieces, enemy_pieces);
        moves = moves | self.get_ray_moves(east, all_pieces, enemy_pieces);
        moves = moves | self.get_ray_moves(west, all_pieces, enemy_pieces);

        moves
    }

    fn get_queen_moves(&self, square: u8, color: Color) -> Bitboard {
        self.get_bishop_moves(square, color) | self.get_rook_moves(square, color)
    }

    fn get_ray_moves(
        &self,
        ray: Bitboard,
        all_pieces: Bitboard,
        enemy_pieces: Bitboard,
    ) -> Bitboard {
        let blockers = ray & all_pieces;
        if blockers.as_u64() == 0 {
            // No blocking pieces, can move anywhere along the ray
            return ray;
        }

        // Find the first blocker
        let first_blocker = if blockers.lsb().is_some() {
            blockers.lsb().unwrap()
        } else {
            blockers.msb().unwrap()
        };

        // Get moves up to and including the first blocker
        let mut moves = Bitboard::empty();
        let mut current_ray = ray;

        while let Some(sq) = current_ray.lsb() {
            moves.set_bit(sq);
            if sq == first_blocker {
                // If the blocker is an enemy piece, include it as a valid move
                // If it's a friendly piece, remove it
                if !enemy_pieces.test_bit(sq) {
                    moves.clear_bit(sq);
                }
                break;
            }
            current_ray.clear_bit(sq);
        }

        moves
    }

    fn add_diagonal_moves(
        &self,
        moves: &mut Bitboard,
        square: u8,
        up: bool,
        right: bool,
        all_pieces: Bitboard,
        enemy_pieces: Bitboard,
    ) {
        let mut current_square = square;
        let delta_rank: i8 = if up { 1 } else { -1 };
        let delta_file: i8 = if right { 1 } else { -1 };

        loop {
            let rank = (current_square / 8) as i8;
            let file = (current_square % 8) as i8;

            let new_rank = rank + delta_rank;
            let new_file = file + delta_file;

            // Check board boundaries
            if new_rank < 0 || new_rank > 7 || new_file < 0 || new_file > 7 {
                break;
            }

            let new_square = (new_rank * 8 + new_file) as u8;
            moves.set_bit(new_square);

            // Stop if we hit any piece
            if all_pieces.test_bit(new_square) {
                // If it's not an enemy piece, remove this square from valid moves
                if !enemy_pieces.test_bit(new_square) {
                    moves.clear_bit(new_square);
                }
                break;
            }

            current_square = new_square;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        board.place_piece(PieceType::Pawn, Color::White, 36); // e5
        let moves = board.get_moves(27);
        assert_eq!(moves.pop_count(), 10); // Should have 3 fewer moves due to blocking
    }
}
