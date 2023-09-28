use crate::piece::{Piece, PieceType};
use crate::Color;

#[derive(Debug, Clone)]
pub struct Square {
    pub piece: Piece,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: Vec<Vec<Square>>,
    pub white_king_square: (u8, u8),
    pub black_king_square: (u8, u8),
    pub current_turn: Color,
    pub in_check: Color,
}

impl Board {
    pub fn get_king_pos(&mut self) {
        for row in 0..self.squares.len() {
            for col in 0..self.squares[row].len() {
                let piece = &self.squares[row][col].piece;
                let piece_type = &piece.piece_type;
                if piece_type == &PieceType::King {
                    if piece.color == Color::White {
                        self.white_king_square = (row as u8, col as u8);
                        //println!("White king square: {:?}", self.white_king_square);
                    } else {
                        self.black_king_square = (row as u8, col as u8);
                    }
                }
            }
        }
    }
    pub fn putting_in_check(&mut self) -> bool {
        self.get_king_pos();
        for row in 0..self.squares.len() {
            for col in 0..self.squares[row].len() {
                let piece = &self.squares[row][col].piece;
                let piece_type = &piece.piece_type;

                match &piece_type {
                    PieceType::Pawn => {
                        let mut attack_row = 0;
                        let attacked_color;
                        let mut pieces: Vec<&Piece> = Vec::new();

                        if piece.color == Color::White {
                            attacked_color = Color::Black;

                            // not in top row
                            if row <= 6 {
                                attack_row = row + 1;
                            }
                            // Color::Black
                        } else {
                            attacked_color = Color::White;
                            // not on bottom row
                            if row >= 1 {
                                attack_row = row - 1;
                            }
                        }

                        // not on very left
                        if col >= 1 {
                            let left_piece = &self.squares[attack_row][col - 1].piece;
                            pieces.push(left_piece);
                        }
                        // not on very right
                        if col <= 6 {
                            let right_piece = &self.squares[attack_row][col + 1].piece;
                            pieces.push(right_piece);
                        }
                        for piece in pieces {
                            if piece.piece_type == PieceType::King {
                                self.in_check = attacked_color;
                                println!("In check: {:?}", self.in_check);
                            }
                        }
                    }
                    PieceType::Knight => continue,
                    PieceType::Bishop => continue,
                    PieceType::Rook => continue,
                    PieceType::Queen => continue,
                    _ => continue,
                };
            }
        }
        // REMOVE LATER
        false
    }
}

pub fn create_board() -> Board {
    // created nested Vecs that hold Squares
    let mut squares: Vec<Vec<Square>> = Vec::new();

    for row in 0..8 {
        let mut row_squares: Vec<Square> = Vec::new();

        for col in 0..8 {
            // set positions of each piece on the chessboard
            let piece_type = match (row, col) {
                (1, _) | (6, _) => PieceType::Pawn,
                (0, 0) | (0, 7) | (7, 0) | (7, 7) => PieceType::Rook,
                (0, 1) | (0, 6) | (7, 1) | (7, 6) => PieceType::Knight,
                (0, 2) | (0, 5) | (7, 2) | (7, 5) => PieceType::Bishop,
                (0, 3) | (7, 3) => PieceType::Queen,
                (0, 4) | (7, 4) => PieceType::King,
                _ => PieceType::None,
            };

            let color = if row < 2 { Color::White } else { Color::Black };

            let piece = Piece { piece_type, color };
            let square = Square { piece };
            // add each square to the row
            row_squares.push(square);
        }
        // add each row to the board
        squares.push(row_squares);
    }

    /* pawns for testing checks
    squares[5][4].piece = Piece {
        piece_type: PieceType::Pawn,
        color: Color::White,
    };
    squares[2][4].piece = Piece {
        piece_type: PieceType::Pawn,
        color: Color::Black,
    };
    */

    // return a Board
    Board {
        squares,
        white_king_square: (0, 4),
        black_king_square: (7, 4),
        current_turn: Color::White,
        in_check: Color::None,
    }
}

pub fn print_board(gameboard: &Board) {
    // iterate through each board row, reversed to display it as white side
    for row in (0..8).rev() {
        let mut row_str = "".to_string();
        // iterate through each square in each row, also reversed
        for col in 0..8 {
            let square = &gameboard.squares[row][col];
            // set what the square gets displayed as, depending on color and piece
            let square_str = match square.piece.color {
                Color::White => match square.piece.piece_type {
                    PieceType::Pawn => "P ",
                    PieceType::Knight => "N ",
                    PieceType::Bishop => "B ",
                    PieceType::Rook => "R ",
                    PieceType::Queen => "Q ",
                    PieceType::King => "K ",
                    _ => "# ",
                },
                Color::Black => match square.piece.piece_type {
                    PieceType::Pawn => "p ",
                    PieceType::Knight => "n ",
                    PieceType::Bishop => "b ",
                    PieceType::Rook => "r ",
                    PieceType::Queen => "q ",
                    PieceType::King => "k ",
                    _ => "# ",
                },
                Color::None => "#",
            };
            row_str.push_str(square_str);
        }
        println!("{:?}", row_str);
    }
    println!("");
}
