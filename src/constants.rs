// File masks (vertical strips)
pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_C: u64 = FILE_A << 2;
pub const FILE_D: u64 = FILE_A << 3;
pub const FILE_E: u64 = FILE_A << 4;
pub const FILE_F: u64 = FILE_A << 5;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;

// Rank masks (horizontal strips)
pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_2: u64 = RANK_1 << (8 * 1);
pub const RANK_3: u64 = RANK_1 << (8 * 2);
pub const RANK_4: u64 = RANK_1 << (8 * 3);
pub const RANK_5: u64 = RANK_1 << (8 * 4);
pub const RANK_6: u64 = RANK_1 << (8 * 5);
pub const RANK_7: u64 = RANK_1 << (8 * 6);
pub const RANK_8: u64 = RANK_1 << (8 * 7);

// Special square combinations
pub const CENTER_SQUARES: u64 = 0x0000001818000000; // e4, d4, e5, d5
pub const EXTENDED_CENTER: u64 = 0x00003C3C3C3C0000; // 16 central squares

// Initial piece positions
pub const WHITE_PAWNS_INIT: u64 = RANK_2;
pub const BLACK_PAWNS_INIT: u64 = RANK_7;
pub const WHITE_KNIGHTS_INIT: u64 = 0x0000000000000042;
pub const BLACK_KNIGHTS_INIT: u64 = 0x4200000000000000;
pub const WHITE_BISHOPS_INIT: u64 = 0x0000000000000024;
pub const BLACK_BISHOPS_INIT: u64 = 0x2400000000000000;
pub const WHITE_ROOKS_INIT: u64 = 0x0000000000000081;
pub const BLACK_ROOKS_INIT: u64 = 0x8100000000000000;
pub const WHITE_QUEENS_INIT: u64 = 0x0000000000000008;
pub const BLACK_QUEENS_INIT: u64 = 0x0800000000000000;
pub const WHITE_KING_INIT: u64 = 0x0000000000000010;
pub const BLACK_KING_INIT: u64 = 0x1000000000000000;

// Castling relevant squares
pub const KINGSIDE_SQUARES: u64 = 0x0000000000000060; // f1, g1 for white
pub const QUEENSIDE_SQUARES: u64 = 0x000000000000000E; // b1, c1, d1 for white

// Move pattern masks
pub const KNIGHT_MOVES: [u64; 64] = generate_knight_moves();
pub const KING_MOVES: [u64; 64] = generate_king_moves();
pub const NORTH_RAY: [u64; 64] = generate_ray_moves(8);
pub const SOUTH_RAY: [u64; 64] = generate_ray_moves(-8);
pub const EAST_RAY: [u64; 64] = generate_ray_moves(1);
pub const WEST_RAY: [u64; 64] = generate_ray_moves(-1);

const fn generate_knight_moves() -> [u64; 64] {
    let mut moves = [0u64; 64];
    let mut square = 0;

    while square < 64 {
        let mut bb = 0u64;
        let rank = square / 8;
        let file = square % 8;

        // All eight possible knight moves
        if rank < 6 && file < 7 {
            bb |= 1u64 << (square + 17);
        } // up 2 right 1
        if rank < 7 && file < 6 {
            bb |= 1u64 << (square + 10);
        } // up 1 right 2
        if rank > 0 && file < 6 {
            bb |= 1u64 << (square - 6);
        } // down 1 right 2
        if rank > 1 && file < 7 {
            bb |= 1u64 << (square - 15);
        } // down 2 right 1
        if rank > 1 && file > 0 {
            bb |= 1u64 << (square - 17);
        } // down 2 left 1
        if rank > 0 && file > 1 {
            bb |= 1u64 << (square - 10);
        } // down 1 left 2
        if rank < 7 && file > 1 {
            bb |= 1u64 << (square + 6);
        } // up 1 left 2
        if rank < 6 && file > 0 {
            bb |= 1u64 << (square + 15);
        } // up 2 left 1

        moves[square as usize] = bb;
        square += 1;
    }
    moves
}

const fn generate_ray_moves(delta: i8) -> [u64; 64] {
    let mut moves = [0u64; 64];
    let mut square = 0;

    while square < 64 {
        let mut bb = 0u64;
        let mut current = square;
        let start_file = current % 8;

        loop {
            let new_square = (current as i8 + delta) as u8;
            if new_square >= 64 {
                break;
            }

            let new_file = new_square % 8;
            // Check for wrapping around board edges
            if (delta == 1 && new_file == 0) || (delta == -1 && new_file == 7) {
                break;
            }
            // Check for vertical bounds
            if (delta == 8 && current >= 56) || (delta == -8 && current < 8) {
                break;
            }

            bb |= 1u64 << new_square;
            current = new_square;
        }

        moves[square as usize] = bb;
        square += 1;
    }
    moves
}

const fn generate_king_moves() -> [u64; 64] {
    let mut moves = [0u64; 64];
    let mut square = 0;

    while square < 64 {
        let mut bb = 0u64;
        let rank = square / 8;
        let file = square % 8;

        // All eight possible king moves
        if file < 7 {
            bb |= 1u64 << (square + 1);
        } // right
        if file > 0 {
            bb |= 1u64 << (square - 1);
        } // left
        if rank < 7 {
            bb |= 1u64 << (square + 8);
        } // up
        if rank > 0 {
            bb |= 1u64 << (square - 8);
        } // down
        if rank < 7 && file < 7 {
            bb |= 1u64 << (square + 9);
        } // up right
        if rank < 7 && file > 0 {
            bb |= 1u64 << (square + 7);
        } // up left
        if rank > 0 && file < 7 {
            bb |= 1u64 << (square - 7);
        } // down right
        if rank > 0 && file > 0 {
            bb |= 1u64 << (square - 9);
        } // down left

        moves[square as usize] = bb;
        square += 1;
    }
    moves
}
