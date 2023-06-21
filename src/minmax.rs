use crate::{Colour, Square, BOARD_SIZE, DEPTH, BoardEssentials, do_move_essentials};

pub fn get_for_whoever_best_move(board_essential: &BoardEssentials) -> Option<Square> {
    if board_essential.game_over {
        return None;
    } else if board_essential.white_turn {
        return Some(get_best_move_for_white(board_essential));
    } else {
        return Some(get_best_move_for_black(board_essential));
    }
}

fn get_best_move_for_black(board_essential: &BoardEssentials) -> Square {
    // We need some code dupe, here. max_search and min_search doesnt return a move, and would be probably be a lot slower with keeping track of that
    // And since those functions are doing the heavy work we want them to be fast.
    let (mut minimum_for_best_move, mut best_x, mut best_y) = (isize::MAX, BOARD_SIZE, BOARD_SIZE);
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if board_essential.possible_moves[x][y].len() == 0 {
                continue;
            }
            let mut clone = board_essential.clone();
            do_move_essentials(x, y, &mut clone);
            if clone.game_over {
                match clone.winner {
                    Colour::BLACK => return Square { x: x, y: y },
                    Colour::EMPTY => {
                        if minimum_for_best_move > 0 {
                            (minimum_for_best_move, best_x, best_y) = (0, x, y)
                        }
                    }
                    Colour::WHITE => {
                        if best_x == BOARD_SIZE {
                            best_x = x;
                        }
                        continue;
                    }
                }
            } else {
                let value_of_move = max_search(
                    minimum_for_best_move,
                    clone,
                    DEPTH.min(BOARD_SIZE * BOARD_SIZE - board_essential.amount_of_stone),
                );
                if value_of_move < minimum_for_best_move {
                    (minimum_for_best_move, best_x, best_y) = (value_of_move, x, y)
                }
            }
        }
    }
    return Square {
        x: best_x,
        y: best_y,
    };
}

fn get_best_move_for_white(board_essential: &BoardEssentials) -> Square {
    // I chose to do code-dupe, since otherwise i believe it would be confusing.
    // It's the same as the above but maximising instead
    let (mut max_for_best_move, mut best_x, mut best_y) = (isize::MIN, BOARD_SIZE, BOARD_SIZE);
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if board_essential.possible_moves[x][y].len() == 0 {
                continue;
            }
            let mut clone = board_essential.clone();
            do_move_essentials(x, y, &mut clone);
            if clone.game_over {
                match clone.winner {
                    Colour::WHITE => return Square { x: x, y: y },
                    Colour::EMPTY => {
                        if max_for_best_move < 0 {
                            (max_for_best_move, best_x, best_y) = (0, x, y)
                        }
                    }
                    Colour::BLACK => {
                        if best_x == BOARD_SIZE {
                            best_x = x;
                        }
                        continue;
                    }
                }
            } else {
                let value_of_move = min_search(
                    max_for_best_move,
                    clone,
                    DEPTH.min(BOARD_SIZE * BOARD_SIZE - board_essential.amount_of_stone),
                );
                if value_of_move > max_for_best_move {
                    (max_for_best_move, best_x, best_y) = (value_of_move, x, y)
                }
            }
        }
    }
    return Square {
        x: best_x,
        y: best_y,
    }
}

// invariant: if we are maximizing then any value higher than alpha is valid return.
// looking for best move for white
fn max_search(alpha: isize, board_essential: BoardEssentials, depth: usize) -> isize {
    if depth == 0 {
        return evaluate_game(&board_essential);
    }

    let mut max = isize::MIN;
    let mut move_found = false;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if board_essential.possible_moves[x][y].len() == 0 {
                continue;
            }
            move_found = true;
            let mut clone = board_essential.clone();
            do_move_essentials(x, y, &mut clone);
            if clone.game_over {
                match clone.winner {
                    Colour::WHITE => return isize::MAX,
                    Colour::EMPTY => max = max.max(0),
                    Colour::BLACK => continue,
                }
            } else {
                max = max.max(min_search(max, clone, depth - 1));
            }
            if max > alpha {
                return max;
            }
        }
    }

    if !move_found {
        return min_search(isize::MIN, board_essential, depth);
    }

    return max;
}

fn min_search(alpha: isize, board_essential: BoardEssentials, depth: usize) -> isize {
    if depth == 0 {
        return evaluate_game(&board_essential);
    }

    let mut min = isize::MAX;
    let mut move_found = false;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if board_essential.possible_moves[x][y].len() == 0 {
                continue;
            }
            move_found = true;
            let mut clone = board_essential.clone();
            do_move_essentials(x, y, &mut clone);
            if clone.game_over {
                match clone.winner {
                    Colour::BLACK => return isize::MIN,
                    Colour::EMPTY => min = min.min(0),
                    Colour::WHITE => continue,
                }
            } else {
                min = min.min(max_search(min, clone, depth - 1));
            }
            if min < alpha {
                return min;
            }
        }
    }
    if !move_found {
        return max_search(isize::MAX, board_essential, depth);
    }

    return min;
}

fn evaluate_game(board_essential: &BoardEssentials) -> isize {
    let mut res = 0;
    for col in board_essential.board.iter() {
        for colour in col.iter() {
            match colour {
                Colour::BLACK => res -= 1,
                Colour::WHITE => res += 1,
                _ => (),
            }
        }
    }

    for i in 0..BOARD_SIZE {
        match board_essential.board[0][i] {
            Colour::BLACK => res -= 2,
            Colour::WHITE => res += 2,
            _ => (),
        }
        match board_essential.board[BOARD_SIZE - 1][i] {
            Colour::BLACK => res -= 2,
            Colour::WHITE => res += 2,
            _ => (),
        }
        match board_essential.board[i][0] {
            Colour::BLACK => res -= 2,
            Colour::WHITE => res += 2,
            _ => (),
        }
        match board_essential.board[i][BOARD_SIZE - 1] {
            Colour::BLACK => res -= 2,
            Colour::WHITE => res += 2,
            _ => (),
        }
    }

    return res;
}
