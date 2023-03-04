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

    fn can_move(&self, from: (u8, u8), to: (u8, u8), gameboard: &Board) -> bool {
        // implementation of can_move method for each type of piece
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        let to_square = &gameboard.squares[to_row as usize][to_col as usize];
        
        if to_col <= 7 && to_row <= 7 { // possible col
            match self.piece_type {
                PieceType::Pawn => {
                    if self.color == Color::White {
                        return (from_col == to_col // pawn is on same column 
                        && (to_row == from_row + 1 || // AND pawn is moving one square
                        (from_row == 1 && to_row == 3))) // OR pawn is on starting square, moving 2
                        ||
                        (from_col == to_col-1 || from_col == to_col+1) &&  // pawn moving to dif column AND
                        (to_row == from_row + 1 && &to_square.piece.piece_type != &PieceType::None && &to_square.piece.color != &self.color) // enemy piece is takeable
                    } else {
                        return (from_col == to_col // pawn is on same column 
                        && (to_row == from_row + 1 || // AND pawn is moving one square
                        (from_row == 6 && to_row == 4))) // OR pawn is on starting square, moving 2
                        ||
                        (from_col == to_col-1 || from_col == to_col+1) &&  // pawn moving to dif column AND
                        (to_row == from_row - 1 && &to_square.piece.piece_type != &PieceType::None && &to_square.piece.color != &self.color) // piece one row ahead, and one column to side
                    }
                },
                PieceType::Knight => {
                    false
                },
                PieceType::Bishop => {
                    false
                },
                PieceType::Rook => {
                    false
                },
                PieceType::Queen => {
                    false
                },
                PieceType::King => {
                    false
                },
                _ => {
                    false
                }
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone)]
struct Square {
    piece: Piece,
}

#[derive(Debug)]
struct Board {
    squares: Vec<Vec<Square>>,
}

//MAIN FUNCTION
fn main() {
    let gameboard = create_board();

    print_board(&gameboard);
    println!("{:?}", gameboard.squares[6][0].piece.can_move((6,0),(5,1),&gameboard));

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
            
            let piece = Piece {piece_type: piece_type, color: color};
            let square = Square {
                piece: piece
            };
            // add each square to the row
            row_squares.push(square);
        }
        // add each row to the board
        squares.push(row_squares);
        
    }
    squares[2][1] = Square{piece: Piece{piece_type: PieceType::Pawn, color: Color::Black}};
    squares[5][1] = Square{piece: Piece{piece_type: PieceType::Pawn, color: Color::White}};
    
    // return a Board
    Board {
        squares,
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
                Color::White => {
                    
                    match square.piece.piece_type {
                        PieceType::Pawn => "P ",
                        PieceType::Knight => "N ",
                        PieceType::Bishop => "B ",
                        PieceType::Rook => "R ",
                        PieceType::Queen => "Q ",
                        PieceType::King => "K ",
                        _ => "# ",
                    }
                },
                Color::Black => {
                    match square.piece.piece_type {
                        PieceType::Pawn => "p ",
                        PieceType::Knight => "n ",
                        PieceType::Bishop => "b ",
                        PieceType::Rook => "r ",
                        PieceType::Queen => "q ",
                        PieceType::King => "k ",
                        _ => "# ",
                    }
                }
            };
            row_str.push_str(square_str);
            
        }
        println!("{:?}", row_str);
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}