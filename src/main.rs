use std::cmp::Ordering;
use std::io;

fn main() {
    let mut gameboard = create_board();
    let mut playing = true;

    print_board(&gameboard);

    while playing {
        let positions = get_cli_input();
        let from_row = positions.0;
        let from_col = positions.1;
        let to_row = positions.2;
        let to_col = positions.3;
        println!("{:?}", positions);

        let piece = &gameboard.squares[from_row as usize][from_col as usize]
            .piece
            .clone();
        let new_gameboard =
            piece.move_piece((from_row, from_col), (to_row, to_col), &mut gameboard);

        let updated_gameboard = new_gameboard;
        gameboard = updated_gameboard;

        print_board(&gameboard);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq)]
struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    fn value(&self) -> u8 {
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

    fn can_move(&self, from: (u8, u8), to: (u8, u8), gameboard: &mut Board) -> bool {
        println!("{:?}", &self.color);
        // implementation of can_move method for each type of piece
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        let from_square = &gameboard.squares[from_row as usize][to_col as usize];
        let to_square = &gameboard.squares[to_row as usize][to_col as usize];
        
        if &to_square.piece.color != &self.color || &to_square.piece.piece_type == &PieceType::None { /* isn't taking own piece */
        if &gameboard.current_turn == &self.color {
            // is turn
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
                          (to_row == from_row + 1 && &to_square.piece.piece_type != &PieceType::None && &to_square.piece.color != &self.color);

                        // enemy piece is takeable
                        } else {
                            return (from_col == to_col // pawn is on same column 
                          && (to_row == from_row - 1 || // AND pawn is moving one square
                          (from_row == 6 && to_row == 4))) // OR pawn is on starting square, moving 2
                          ||
                          (from_col == to_col-1 || from_col == to_col+1) &&  // pawn moving to different column AND
                          (to_row == from_row - 1 && &to_square.piece.piece_type != &PieceType::None && &to_square.piece.color != &self.color);
                            // piece one row ahead, and one column to side
                        }
                    }
                    PieceType::Knight => {
                        let row_diff = to_row.abs_diff(from_row);
                        let col_diff = to_col.abs_diff(from_col);

                        // row change must be 1 and col change 2, or vice versa
                        (row_diff == 1 && col_diff == 2) || (row_diff == 2 && col_diff == 1)
                    }
                    PieceType::Rook => match (from_row.cmp(&to_row), from_col.cmp(&to_col)) {
                        // if to_row < and to_col ==
                        (Ordering::Less, Ordering::Equal) => {
                            // iterate through each row between
                            for row in from_row + 1..to_row {
                                // iterate through each square in between the rook and destination
                                let square = &gameboard.squares[row as usize][from_col as usize];
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
                                let square = &gameboard.squares[from_row as usize][col as usize];
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
                                let square = &gameboard.squares[from_row as usize][col as usize];
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
                                let square = &gameboard.squares[row as usize][from_col as usize];
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
                      &to_square.piece.piece_type == &PieceType::None
                            || &to_square.piece.color != &self.color; // can only move to square if it is empty or has enemy piece on it
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
      // trying to take own piece
      } else {
        println!("Can't take own piece.");
        false
      }
    }
    fn move_piece(&self, from: (u8, u8), to: (u8, u8), gameboard: &mut Board) -> Board {
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
                    color: self.color.clone(),
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

#[derive(Debug, Clone, PartialEq)]
enum Color {
    None,
    White,
    Black,
}

#[derive(Debug, Clone)]
struct Square {
    piece: Piece,
}

#[derive(Debug, Clone)]
struct Board {
    squares: Vec<Vec<Square>>,
    current_turn: Color,
    in_check: Color,
}

fn create_board() -> Board {
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

            let piece = Piece {
                piece_type: piece_type,
                color: color,
            };
            let square = Square { piece: piece };
            // add each square to the row
            row_squares.push(square);
        }
        // add each row to the board
        squares.push(row_squares);
    }
    squares[5][4].piece = Piece {piece_type: PieceType::Pawn, color: Color::White};

    // return a Board
    Board {
        squares: squares,
        current_turn: Color::White,
        in_check: Color::None,
    }
}

fn print_board(gameboard: &Board) {
    // iterate through each board row, reversed to display it as white side
    for row in (0..8).rev() {
        let mut row_str = "".to_string();
        // iterate through each square in each row, also reversed
        for col in (0..8) {
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

fn get_cli_input() -> (u8, u8, u8, u8) {
    let mut user_input = "".to_string();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read line.");
    let trimmed_input = user_input.trim();

    let input_square: Vec<&str> = trimmed_input.split(" ").collect();
    let from_square = input_square[0];
    let to_square = input_square[1];

    let from_file_n = letter_to_number(&from_square[0..1]);
    let from_rank = &from_square[1..2].parse::<u8>().unwrap() - 1;

    let to_file_n = letter_to_number(&to_square[0..1]);
    let to_rank = &to_square[1..2].parse::<u8>().unwrap() - 1;

    let from_file = match from_file_n {
        Some(x) => x,
        None => {
            panic!("invalid file.");
        }
    };
    let to_file = match to_file_n {
        Some(x) => x,
        None => {
            panic!("invalid file.");
        }
    };

    (from_rank, from_file, to_rank, to_file)
}

fn letter_to_number(letter: &str) -> Option<u8> {
    let alphabet = "abcdefgh";
    let index = alphabet.find(letter)?;
    Some((index) as u8)
}

fn square_is_empty(piece: &Piece, row: u8, col: u8, gameboard: &Board) -> bool {
    let to_square = &gameboard.squares[row as usize][col as usize];
    &to_square.piece.piece_type == &PieceType::None || &to_square.piece.color != &piece.color
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
