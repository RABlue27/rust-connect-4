fn get_cpu_move(game_board: &[i32]) -> i32 {
    let move_list = valid_moves(game_board);
    println!("{:?}", move_list);
    let mut best_move = 0;
    let mut best_score = std::i32::MIN;

    for &mv in &move_list {
        let mut new_board = game_board.to_vec();
        let _ = make_move(&mut new_board, mv, false);
        let score = minimax(&new_board, 3, false);

        if score > best_score {
            best_score = score;
            best_move = mv;
        }
    }
    best_move
}

// player 1 is maximzing player
fn minimax(board: &[i32], depth: i32, is_maximizing_player: bool) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut best_score = if is_maximizing_player {
        std::i32::MIN
    } else {
        std::i32::MAX
    };

    let move_list = valid_moves(board);

    for &mv in &move_list {
        let mut new_board = board.to_vec();
        let _ = make_move(&mut new_board, mv, is_maximizing_player);

        let score = minimax(&new_board, depth - 1, !is_maximizing_player);

        if is_maximizing_player {
            best_score = best_score.max(score);
        } else {
            best_score = best_score.min(score);
        }
    }
    best_score
}

fn evaluate(arr: &[i32]) -> i32 {
    let height = 6;
    let width = 7;
    let mut score = 0;

    if is_winner(arr) {
        score -= 1000
    }

    // Check for horizontal wins
    for y in 0..height {
        for x in 0..width - 3 {
            let idx = y * width + x;
            if arr[idx] != 0
                && arr[idx] == arr[idx + 1]
                && arr[idx] == arr[idx + 2]
                && arr[idx] == arr[idx + 3]
            {
                if arr[idx] == 1 {
                    score += 1;
                }
                score -= 1
            }
        }
    }
    for y in 0..height - 3 {
        for x in 0..width {
            let idx = y * width + x;
            if arr[idx] != 0
                && arr[idx] == arr[idx + width]
                && arr[idx] == arr[idx + 2 * width]
                && arr[idx] == arr[idx + 3 * width]
            {
                if arr[idx] == 1 {
                    score += 1;
                }
                score -= 1
            }
        }
    }

    for y in 3..height {
        for x in 0..width - 3 {
            let idx = y * width + x;
            if arr[idx] != 0
                && arr[idx] == arr[idx - width + 1]
                && arr[idx] == arr[idx - 2 * (width - 1)]
                && arr[idx] == arr[idx - 3 * (width - 1)]
            {
                if arr[idx] == 1 {
                    score += 1;
                }
                score -= 1
            }
        }
    }
    for y in 0..height - 3 {
        for x in 0..width - 3 {
            let idx = y * width + x;
            if arr[idx] != 0
                && arr[idx] == arr[idx + width + 1]
                && arr[idx] == arr[idx + 2 * (width + 1)]
                && arr[idx] == arr[idx + 3 * (width + 1)]
            {
                if arr[idx] == 1 {
                    score += 1;
                }
                score -= 1
            }
        }
    }
    score
}
