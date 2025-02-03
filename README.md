# Chess Bitboard Library

A Rust library implementing a bitboard-based chess board representation, providing efficient operations for chess move generation and position analysis.

## Features

- Efficient bitboard operations for chess piece movements
- Complete move generation for all piece types
- Position analysis and board state management
- Comprehensive test coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chess_bitboard = { git = "https://github.com/yourusername/chess_bitboard" }
```

## Usage

### Basic Board Operations

```rust
use chess_bitboard::{Board, Color, PieceType};

// Create a new board with pieces in starting positions
let board = Board::initial();

// Or create an empty board and place pieces manually
let mut board = Board::empty();
board.place_piece(PieceType::Knight, Color::White, 27); // Place white knight on d4
```

### Move Generation

```rust
// Get all legal moves for a piece
let moves = board.get_moves(27); // Get moves for piece on d4
let move_count = moves.pop_count(); // Count number of legal moves

// Example: Calculate all possible moves for white
let all_white_moves = (0..64)
    .filter(|&square| {
        matches!(board.get_piece_at(square), 
                Some((_, Color::White)))
    })
    .fold(Bitboard::empty(), |acc, square| {
        acc | board.get_moves(square)
    });
```

### Complex Position Analysis

```rust
// Example: Setting up a specific position
let mut board = Board::empty();

// Place pieces
board.place_piece(PieceType::Bishop, Color::White, 27); // d4
board.place_piece(PieceType::Pawn, Color::White, 36);   // e5 - blocking own pawn
board.place_piece(PieceType::Pawn, Color::Black, 18);   // c3 - enemy pawn can be captured

// Get legal moves for the bishop
let bishop_moves = board.get_moves(27);
// bishop_moves will contain: c3 (capture), c5, b6, a7, e3, f2, g1

// Get all pieces of a specific color
let white_pieces = board.get_color_pieces(Color::White);
let black_pieces = board.get_color_pieces(Color::Black);

// Check if a square is occupied
if let Some((piece_type, color)) = board.get_piece_at(27) {
    println!("Square d4 contains a {:?} {:?}", color, piece_type);
}
```

### Working with Bitboards Directly

```rust
use chess_bitboard::Bitboard;

// Create bitboards
let mut bb = Bitboard::empty();
bb.set_bit(0);  // Set a1 square

// Bitboard operations
let bb2 = Bitboard::from_u64(0x00FF); // Set first rank
let combined = bb | bb2;  // Union of bitboards
let intersection = bb & bb2;  // Intersection of bitboards

// Bit manipulation
let count = combined.pop_count();  // Count set bits
if let Some(lsb) = combined.lsb() {
    println!("Least significant set bit: {}", lsb);
}
```

## Implementation Details

### Bitboard Representation

The library uses a 64-bit integer to represent the chess board, where each bit corresponds to a square:

```text
8 | 56 57 58 59 60 61 62 63
7 | 48 49 50 51 52 53 54 55
6 | 40 41 42 43 44 45 46 47
5 | 32 33 34 35 36 37 38 39
4 | 24 25 26 27 28 29 30 31
3 | 16 17 18 19 20 21 22 23
2 | 8  9  10 11 12 13 14 15
1 | 0  1  2  3  4  5  6  7
  +-----------------------
    a  b  c  d  e  f  g  h
```

### Move Generation

The library implements efficient move generation for all piece types:

- **Pawns**: Handles single moves, double moves from starting position, and captures
- **Knights**: Uses pre-calculated move patterns
- **Bishops**: Uses ray-tracing along diagonals
- **Rooks**: Uses ray-tracing along ranks and files
- **Queens**: Combines bishop and rook movements
- **Kings**: Uses pre-calculated move patterns

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests for:
- Board initialization
- Piece placement and retrieval
- Move generation for all piece types
- Edge cases and special situations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
