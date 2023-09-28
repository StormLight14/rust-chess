use crate::board::{Board, Square};
use crate::Color;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn value(&self) -> u8 {
        match self.piece_type {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 0,
            _ => 0,
        }
    }

    pub fn can_move(&self, from: (u8, u8), to: (u8, u8), gameboard: &mut Board) -> bool {
        // implementation of can_move method for each type of piece
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        let from_square = &gameboard.squares[from_row as usize][from_col as usize];
        let to_square = &gameboard.squares[to_row as usize][to_col as usize];

        /* isn't taking own piece */
        if to_square.piece.color != self.color || to_square.piece.piece_type == PieceType::None {
            if gameboard.in_check != from_square.piece.color {
                // is turn
                if gameboard.current_turn == self.color {
                    if to_col <= 7 && to_row <= 7 {
                        // possible col
                        match self.piece_type {
                            PieceType::Pawn => {
                                if self.color == Color::White {
                                    return (from_col == to_col // pawn is on same column 
                          && (to_row == from_row + 1 || // AND pawn is moving one square
                          (from_row == 1 && to_row == 3))) // OR pawn is on starting square, moving 2
                          ||
                          (from_col == to_col-1 || from_col == to_col+1) &&  // pawn moving to dif column AND
                          (to_row == from_row + 1 && to_square.piece.piece_type != PieceType::None && to_square.piece.color != self.color);

                                // enemy piece is takeable
                                } else {
                                    return (from_col == to_col // pawn is on same column 
                          && (to_row == from_row - 1 || // AND pawn is moving one square
                          (from_row == 6 && to_row == 4))) // OR pawn is on starting square, moving 2
                          ||
                          (from_col == to_col-1 || from_col == to_col+1) &&  // pawn moving to different column AND
                          (to_row == from_row - 1 && to_square.piece.piece_type != PieceType::None && to_square.piece.color != self.color);
                                    // piece one row ahead, and one column to side
                                }
                            }
                            PieceType::Knight => {
                                let row_diff = to_row.abs_diff(from_row);
                                let col_diff = to_col.abs_diff(from_col);

                                // row change must be 1 and col change 2, or vice versa
                                (row_diff == 1 && col_diff == 2) || (row_diff == 2 && col_diff == 1)
                            }
                            PieceType::Rook => match (from_row.cmp(&to_row), from_col.cmp(&to_col))
                            {
                                // if to_row < and to_col ==
                                (Ordering::Less, Ordering::Equal) => {
                                    // iterate through each row between
                                    for row in from_row + 1..to_row {
                                        // iterate through each square in between the rook and destination
                                        let square =
                                            &gameboard.squares[row as usize][from_col as usize];
                                        if square.piece.piece_type != PieceType::None {
                                            return false;
                                        }
                                    }
                                    true
                                }
                                // if to_row == and to_col <
                                (Ordering::Equal, Ordering::Less) => {
                                    // iterate through each col between
                                    for col in from_col + 1..to_col {
                                        // iterate through each square in between the rook and destination
                                        let square =
                                            &gameboard.squares[from_row as usize][col as usize];
                                        if square.piece.piece_type != PieceType::None {
                                            return false;
                                        }
                                    }
                                    true
                                }
                                // if to_row == and to_col >
                                (Ordering::Equal, Ordering::Greater) => {
                                    // iterate through each col between, but reversed order.
                                    for col in (to_col + 1..from_col).rev() {
                                        // iterate through each square in between the rook and destination
                                        let square =
                                            &gameboard.squares[from_row as usize][col as usize];
                                        if square.piece.piece_type != PieceType::None {
                                            return false;
                                        }
                                    }
                                    true
                                }
                                // if to_row > and to_col ==
                                (Ordering::Greater, Ordering::Equal) => {
                                    // iterate through each row between, but reversed order.
                                    for row in (to_row + 1..from_row).rev() {
                                        // iterate through each square in between the rook and destination
                                        let square =
                                            &gameboard.squares[row as usize][from_col as usize];
                                        if square.piece.piece_type != PieceType::None {
                                            return false;
                                        }
                                    }
                                    true
                                }
                                _ => false,
                            },
                            PieceType::Bishop => {
                                if from_row != to_row
                                    && from_col != to_col
                                    && to_row.abs_diff(from_row) == to_col.abs_diff(from_col)
                                {
                                    let row_step = (to_row as i8 - from_row as i8).signum(); // +1 for upward diagonal, -1 for downward diagonal
                                    let col_step = (to_col as i8 - from_col as i8).signum(); // +1 for rightward diagonal, -1 for leftward diagonal

                                    let mut row = from_row as i8 + row_step;
                                    let mut col = from_col as i8 + col_step;

                                    while row != to_row as i8 && col != to_col as i8 {
                                        let square = &gameboard.squares[row as usize][col as usize];
                                        if square.piece.piece_type != PieceType::None {
                                            // square in between from and to squares are empty
                                            return false;
                                        }
                                        row += row_step;
                                        col += col_step;
                                    }

                                    true
                                } else {
                                    false
                                }
                            }
                            PieceType::Queen => {
                                if from_row == to_row
                                    || from_col == to_col
                                    || (from_row as i8 - to_row as i8).abs()
                                        == (from_col as i8 - to_col as i8).abs()
                                {
                                    let row_step = if from_row == to_row {
                                        0
                                    } else {
                                        (to_row as i8 - from_row as i8).signum()
                                    }; // +1 for upward diagonal, -1 for downward diagonal
                                    let col_step = if from_col == to_col {
                                        0
                                    } else {
                                        (to_col as i8 - from_col as i8).signum()
                                    }; // +1 for rightward diagonal, -1 for leftward diagonal

                                    let mut row = from_row as i8 + row_step;
                                    let mut col = from_col as i8 + col_step;

                                    while row != to_row as i8 || col != to_col as i8 {
                                        let square = &gameboard.squares[row as usize][col as usize];
                                        if square.piece.piece_type != PieceType::None {
                                            return false;
                                        }
                                        row += row_step;
                                        col += col_step;
                                    }

                                    true
                                } else {
                                    false
                                }
                            }
                            PieceType::King => {
                                return (to_row <= from_row + 1 && to_row >= from_row - 1) && (to_col <= from_col + 1 && to_col >= from_col - 1) && // can only move 1 square in any direction
                      to_square.piece.piece_type == PieceType::None
                                    || to_square.piece.color != self.color; // can only move to square if it is empty or has enemy piece on it
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                // not this color's turn
                } else {
                    println!("It is not currently {:?}'s turn.", &self.color);
                    false
                }
            // IN CHECK, ADD STUFF LATER
            } else {
                println!("In check.");
                false
            }
        // trying to take own piece
        } else {
            println!("Can't take own piece.");
            false
        }
    }
    pub fn move_piece(&self, from: (u8, u8), to: (u8, u8), gameboard: &mut Board) -> Board {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        if self.can_move(from, to, gameboard) {
            gameboard.squares[from_row as usize][from_col as usize] = Square {
                piece: Piece {
                    piece_type: PieceType::None,
                    color: Color::Black,
                },
            };
            gameboard.squares[to_row as usize][to_col as usize] = Square {
                piece: Piece {
                    piece_type: self.piece_type.clone(),
                    color: self.color,
                },
            };
            if gameboard.current_turn == Color::White {
                gameboard.current_turn = Color::Black;
            } else {
                gameboard.current_turn = Color::White;
            }

            gameboard.clone()
        } else {
            println!("Piece can not move there.");
            gameboard.clone()
        }
    }
}
