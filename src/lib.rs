use std::io;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Color {
    None,
    White,
    Black,
}

pub fn get_cli_input() -> Result<(u8, u8, u8, u8), String> {
    let mut user_input = "".to_string();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read line.");
    let trimmed_input = user_input.trim();

    let input_square: Vec<&str> = trimmed_input.split(' ').collect();

    if input_square.len() != 2 {
        return Err(String::from("Needs 2 arguments. (Example: e2 e4)"));
    }

    let from_square = input_square[0];
    let to_square = input_square[1];

    if from_square.len() != 2 || to_square.len() != 2 {
        return Err(String::from("Please enter a valid move. (Example: e2 e4)"));
    }

    let from_file_result = letter_to_number(&from_square[0..1]);
    let to_file_result = letter_to_number(&to_square[0..1]);

    let from_file = match from_file_result {
        Some(x) => x,
        None => {
            return Err(String::from("Invalid file."));
        }
    };
    let to_file = match to_file_result {
        Some(x) => x,
        None => {
            return Err(String::from("Invalid file."));
        }
    };

    // dont forget to do -1
    let from_rank_result = &from_square[1..2].parse::<u8>();
    let from_rank: u8;

    match from_rank_result {
        Ok(rank) => {
            from_rank = rank - 1;
        }
        Err(_err) => {
            return Err(String::from("Invalid rank."));
        }
    };

    let to_rank_result = &to_square[1..2].parse::<u8>();
    let to_rank: u8;

    match to_rank_result {
        Ok(rank) => {
            to_rank = rank - 1;
        }
        Err(_err) => {
            return Err(String::from("Invalid rank."));
        }
    };

    if from_rank > 7 || to_rank > 7 {
        return Err(String::from(
            "You cannot move from or to outside the board.",
        ));
    }

    Ok((from_rank, from_file, to_rank, to_file))
}

fn letter_to_number(letter: &str) -> Option<u8> {
    let alphabet = "abcdefgh";
    let index = alphabet.find(letter)?;
    Some((index) as u8)
}
