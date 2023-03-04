#[derive(Debug, Clone, Copy)]
enum Piece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    fn value(&self) -> u8 {
        match self {
            Piece::Pawn => 1,
            Piece::Knight => 3,
            Piece::Bishop => 3,
            Piece::Rook => 5,
            Piece::Queen => 9,
            Piece::King => 0,
            _ => 0,
        }
    }

    fn can_move(&self, from: (u8, u8), to: (u8, u8)) -> bool {
        // implementation of can_move method for each type of piece
        match self {
            Piece::Pawn => {
                false
            },
            Piece::Knight => {
                false
            },
            Piece::Bishop => {
                false
            },
            Piece::Rook => {
                false
            },
            Piece::Queen => {
                false
            },
            Piece::King => {
                false
            },
            _ => {
                false
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone)]
struct Square {
    piece: Piece,
    color: Color,
}

#[derive(Debug)]
struct Board {
    squares: Vec<Vec<Square>>,
}

fn main() {
    let gameboard = create_board();

    print_board(&gameboard);
}

    fn create_board() -> Board {
        // created nested Vecs that hold Squares
        let mut squares: Vec<Vec<Square>> = Vec::new();
    
        for row in 0..8 {
            let mut row_squares: Vec<Square> = Vec::new();
    
            for col in 0..8 {
                // set positions of each piece on the chessboard
                let piece = match (row, col) {
                    (1, _) | (6, _) => Piece::Pawn,
                    (0, 0) | (0, 7) | (7, 0) | (7, 7) => Piece::Rook,
                    (0, 1) | (0, 6) | (7, 1) | (7, 6) => Piece::Knight,
                    (0, 2) | (0, 5) | (7, 2) | (7, 5) => Piece::Bishop,
                    (0, 3) | (7, 3) => Piece::Queen,
                    (0, 4) | (7, 4) => Piece::King,
                    _ => Piece::None,
                };
    
                let color = if row < 2 { Color::White } else { Color::Black };
    
                let square = Square {
                    piece,
                    color,
                };
                // add each square to the row
                row_squares.push(square);
            }
            // add each row to the board
            squares.push(row_squares);
        }
        
        // return a Board
        Board {
            squares,
        }
        
    }

fn print_board(gameboard: &Board) {
    for i in (0..8).rev() {
        println!("{:?}", gameboard.squares[i]);
    }
}