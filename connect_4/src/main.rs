use std::io;
fn main() {
    println!("Welcome to Connect 4");
    let width = 7;
    let height = 6;

    let mut player_1 = true;
    let mut game_board = vec![0; width * height];

    while !is_winner(&game_board) {
        let player = if player_1 { "Player 1" } else { "Player 2" };
        println!("{}'s turn", player);
        print_board(&game_board);

        let player_move = get_user_input();
        let result = make_move(&mut game_board, player_move, player_1);

        match result {
            Ok(_) => {
                player_1 = !player_1;
            }
            Err(error) => println!("Error: {}", error),
        }
    }

    let winner = if player_1 { "Player 2" } else { "Player 1" };
    println!("Game over, {} wins!", winner);
    print_board(&game_board);
}

//[y * w + x]
// for every y, check if every x is full
// unused currently

fn valid_moves(board: &[i32]) -> Vec<i32> {
    let height: i32 = 6;
    let width: i32 = 7;
    let mut valid_moves: Vec<i32> = Vec::new();

    for x in 0..width {
        let mut is_full = true;
        for y in 0..height {
            let index: usize = (y * width + x) as usize;
            if board[index] == 0 {
                is_full = false;
                break;
            }
        }
        if !is_full {
            valid_moves.push(x);
        }
    }

    valid_moves
}

fn is_winner(arr: &[i32]) -> bool {
    let height = 6;
    let width = 7;

    // Check for horizontal wins
    for y in 0..height {
        for x in 0..width - 3 {
            let idx = y * width + x;
            if arr[idx] != 0
                && arr[idx] == arr[idx + 1]
                && arr[idx] == arr[idx + 2]
                && arr[idx] == arr[idx + 3]
            {
                return true;
            }
        }
    }

    // Check for vertical wins
    for y in 0..height - 3 {
        for x in 0..width {
            let idx = y * width + x;
            if arr[idx] != 0
                && arr[idx] == arr[idx + width]
                && arr[idx] == arr[idx + 2 * width]
                && arr[idx] == arr[idx + 3 * width]
            {
                return true;
            }
        }
    }

    // Check for diagonal wins (bottom-left to top-right)
    for y in 3..height {
        for x in 0..width - 3 {
            let idx = y * width + x;
            if arr[idx] != 0
                && arr[idx] == arr[idx - width + 1]
                && arr[idx] == arr[idx - 2 * (width - 1)]
                && arr[idx] == arr[idx - 3 * (width - 1)]
            {
                return true;
            }
        }
    }

    // Check for diagonal wins (top-left to bottom-right)
    for y in 0..height - 3 {
        for x in 0..width - 3 {
            let idx = y * width + x;
            if arr[idx] != 0
                && arr[idx] == arr[idx + width + 1]
                && arr[idx] == arr[idx + 2 * (width + 1)]
                && arr[idx] == arr[idx + 3 * (width + 1)]
            {
                return true;
            }
        }
    }

    false
}

fn get_user_input() -> i32 {
    let mut input_line = String::new();
    if io::stdin().read_line(&mut input_line).is_err() {
        eprintln!("Failed to read line");
        return -1;
    }
    match input_line.trim().parse::<i32>() {
        Ok(x) => x - 1,
        Err(_) => -1,
    }
}

fn make_move(game_board: &mut [i32], x: i32, is_player_one: bool) -> Result<(), String> {
    if !(0..7).contains(&x) {
        return Err("Invalid column index".to_string());
    }

    for y in (0..6).rev() {
        let index = y * 7 + x;
        if game_board[index as usize] == 0 {
            let value = if is_player_one { 1 } else { 2 };
            game_board[index as usize] = value;
            return Ok(());
        }
    }

    Err("Column is full".to_string())
}

fn print_board(game_board: &[i32]) {
    println!("1 2 3 4 5 6 7");
    for (i, value) in game_board.iter().enumerate() {
        match value {
            0 => print!("⚪"),
            1 => print!("🔴"),
            2 => print!("🟡"),
            _ => print!("🚩"), //bad case shouldnt happen
        }
        if (i + 1) % 7 == 0 {
            println!();
        }
    }
}
