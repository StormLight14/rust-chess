use rust_chess::*;

mod board;
mod piece;

use board::*;
use piece::*;

fn main() {
    let mut gameboard = create_board();
    let playing = true;

    print_board(&gameboard);

    while playing {
        gameboard.putting_in_check();
        let cli_input_result = get_cli_input();
        let positions: (u8, u8, u8, u8);
        match cli_input_result {
            Ok(move_pos) => positions = move_pos,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
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

fn square_is_empty(piece: &Piece, row: u8, col: u8, gameboard: &Board) -> bool {
    let to_square = &gameboard.squares[row as usize][col as usize];
    to_square.piece.piece_type == PieceType::None || to_square.piece.color != piece.color
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
