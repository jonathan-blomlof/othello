use crate::{do_move, Colour, Game, Square, BOARD_SIZE};
const DEPTH: usize = 6;

pub fn get_best_move(game: &Game) -> Option<Square> {
    // temp solution, get best move for black.

    let (mut min, mut best_x, mut best_y) = (isize::MAX, BOARD_SIZE, BOARD_SIZE);
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if game.possible_moves[x][y].len() == 0 {
                continue;
            }
            let mut clone = game.clone();
            do_move(x, y, &mut clone);
            if clone.game_over {
                match clone.winner {
                    Colour::BLACK => return Some(Square { x: x, y: y }),
                    Colour::EMPTY => {
                        if min > 0 {
                            (min, best_x, best_y) = (0, x, y)
                        }
                    }
                    Colour::WHITE => continue,
                }
            }
            let temp = max_search(min, clone, DEPTH.min(BOARD_SIZE*BOARD_SIZE - game.amount_of_stone));
            if temp < min {
                (min, best_x, best_y) = (temp, x, y)
            }
        }
    }
    if best_x == BOARD_SIZE {
        return None;
    }
    Some(Square {
        x: best_x,
        y: best_y,
    })
}

// invariant: if we are maximizing then any value higher than alpha is valid return.
fn max_search(alpha: isize, game: Game, depth: usize) -> isize {
    if depth == 0 {
        return evaluate_game(&game);
    }

    let mut max = isize::MIN;
    let mut move_found = false;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if game.possible_moves[x][y].len() == 0 {
                continue;
            }
            move_found = true;
            let mut clone = game.clone();
            do_move(x, y, &mut clone);
            if clone.game_over {
                match clone.winner {
                    Colour::WHITE => return isize::MAX,
                    Colour::EMPTY => max = max.max(0),
                    Colour::BLACK => continue,
                }
            }
            max = max.max(min_search(max, clone, depth - 1));
            if max > alpha {
                return max;
            }
        }
    }

    /*TODO SAVE POS IN HASHTABLE */
    if !move_found {
        return min_search(isize::MIN, game, depth)
    }

    return max;
}

fn min_search(alpha: isize, game: Game, depth: usize) -> isize {
    if depth == 0 {
        return evaluate_game(&game);
    }

    let mut min = isize::MAX;
    let mut move_found = false;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if game.possible_moves[x][y].len() == 0 {
                continue;
            }
            move_found = true;
            let mut clone = game.clone();
            do_move(x, y, &mut clone);
            if clone.game_over {
                match clone.winner {
                    Colour::BLACK => return isize::MIN,
                    Colour::EMPTY => min = min.min(0),
                    Colour::WHITE => continue,
                }
            }
            min = min.min(max_search(min, clone, depth - 1));
            if min < alpha {
                return min;
            }
        }
    }
    if !move_found {
        return max_search(isize::MAX, game, depth)
    }

    return min;
}

fn evaluate_game(game: &Game) -> isize {
    let mut res = 0;
    for col in game.board.iter() {
        for colour in col.iter() {
            match colour {
                Colour::BLACK => res -= 1,
                Colour::WHITE => res += 1,
                _ => (),
            }
        }
    }

    for i in 0..BOARD_SIZE {
        match game.board[0][i] {
            Colour::BLACK => res -= 2,
            Colour::WHITE => res += 2,
            _ => (),
        }
        match game.board[BOARD_SIZE - 1][i] {
            Colour::BLACK => res -= 2,
            Colour::WHITE => res += 2,
            _ => (),
        }
        match game.board[i][0] {
            Colour::BLACK => res -= 2,
            Colour::WHITE => res += 2,
            _ => (),
        }
        match game.board[i][BOARD_SIZE - 1] {
            Colour::BLACK => res -= 2,
            Colour::WHITE => res += 2,
            _ => (),
        }
    }

    return res;
}
