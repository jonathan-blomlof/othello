use piston_window::*;
mod minmax;
use minmax::get_for_whoever_best_move;

const BOARD_SIZE: usize = 8;
const WHITE_IS_STARTING: bool = true;
const AI_COLOUR: Colour = Colour::WHITE;
const DEPTH: usize = 5;
const STARING_STONE: usize = 4;
const WINDOW_SIZE: u32 = 500;

#[derive(Clone, Copy)]
enum Colour {
    WHITE,
    BLACK,
    EMPTY,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Square {
    x: usize,
    y: usize,
}

#[derive(Clone)]
pub struct Game {
    board: [[Colour; BOARD_SIZE]; BOARD_SIZE],
    white_turn: bool,
    possible_moves: Vec<Vec<Vec<Square>>>,

    prev_boards: Vec<([[Colour; BOARD_SIZE]; BOARD_SIZE], bool)>,
    last_placed: Square,
    flipped_tiles_from_move: Vec<Square>,

    amount_of_stone: usize,
    game_over: bool,
    winner: Colour,
}

fn init_game() -> Game {
    let board = set_up_board();
    let mut game = Game {
        board: board,
        white_turn: WHITE_IS_STARTING,
        possible_moves: vec![],
        prev_boards: Vec::with_capacity(BOARD_SIZE * BOARD_SIZE),
        last_placed: Square { x: 0, y: 0 },
        flipped_tiles_from_move: Vec::new(),
        amount_of_stone: STARING_STONE,
        game_over: false,
        winner: Colour::EMPTY, //empty indicates draw. Value only use-able if game_over == true
    };
    game.possible_moves = get_all_possible_moves(&game).0;
    game
}

fn main() {
    println!("Welcome to OTHELLO, the game");
    let mut game = init_game();
    let mut mouse_x = 0;
    let mut mouse_y = 0;
    let mut dist_per_block = 0.0;

    let mut do_ai_move = false;
    let mut window: PistonWindow = WindowSettings::new("Othello", [WINDOW_SIZE, WINDOW_SIZE])
        .build()
        .unwrap();

    /* GAME LOOP */
    while let Some(event) = window.next() {
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            // we pressed a button.

            if mouse_x < BOARD_SIZE
                && mouse_y < BOARD_SIZE
                && game.possible_moves[mouse_x][mouse_y].len() > 0
                && !game.game_over
            {
                if game.white_turn {
                    do_move(mouse_x, mouse_y, &mut game);
                    do_ai_move = false;
                }
            }
        } else if let Some(m) = event.mouse_cursor_args() {
            dist_per_block = window.size().height.min(window.size().width) / BOARD_SIZE as f64;
            mouse_x = (m[0] / dist_per_block) as usize;
            mouse_y = (m[1] / dist_per_block) as usize;
        } else if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::U {
                undo(&mut game);
                wait_before_ai_move = true;
            }
        }

        if do_ai_move && !game.white_turn && !game.game_over {
            let best = get_best_move(&game).unwrap();
            do_move(best.x, best.y, &mut game);
        }
        if let Some(_) = event.render_args() {
            do_ai_move = true;
        }

        /* DRAWING */

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.5, 0.5, 0.5, 1.0], graphics);

            // shading newly flipped and placed stones
            // only if we have stored a prev board
            if game.prev_boards.len() > 0 {
                let new_placed_colour = [0.0, 0.0, 0.8, 1.0];
                let new_flipped_colour = [0.5, 0.5, 1.0, 1.0];
                let (x, y) = (game.last_placed.x, game.last_placed.y);
                rectangle(
                    new_placed_colour,
                    [
                        dist_per_block * x as f64,
                        dist_per_block * y as f64,
                        dist_per_block,
                        dist_per_block,
                    ],
                    context.transform,
                    graphics,
                );
                for sq in game.flipped_tiles_from_move.iter() {
                    rectangle(
                        new_flipped_colour,
                        [
                            dist_per_block * sq.x as f64,
                            dist_per_block * sq.y as f64,
                            dist_per_block,
                            dist_per_block,
                        ],
                        context.transform,
                        graphics,
                    );
                }
            }

            let black = [0.0, 0.0, 0.0, 1.0];

            for i in 1..(BOARD_SIZE + 1) {
                let temp = dist_per_block * i as f64;
                line(
                    black,
                    2.0,
                    [0.0, temp, dist_per_block * BOARD_SIZE as f64, temp],
                    context.transform,
                    graphics,
                );
                line(
                    black,
                    2.0,
                    [temp, 0.0, temp, dist_per_block * BOARD_SIZE as f64],
                    context.transform,
                    graphics,
                );
            }
            let white = [1.0; 4];
            for (x, column) in game.board.iter().enumerate() {
                for (y, colour) in column.iter().enumerate() {
                    match colour {
                        Colour::BLACK => ellipse(
                            black,
                            [
                                (x as f64 + 0.2) * dist_per_block,
                                (y as f64 + 0.2) * dist_per_block,
                                dist_per_block * 0.6,
                                dist_per_block * 0.6,
                            ],
                            context.transform,
                            graphics,
                        ),
                        Colour::WHITE => ellipse(
                            white,
                            [
                                (x as f64 + 0.2) * dist_per_block,
                                (y as f64 + 0.2) * dist_per_block,
                                dist_per_block * 0.6,
                                dist_per_block * 0.6,
                            ],
                            context.transform,
                            graphics,
                        ),
                        _ => (),
                    }
                }
            }
            if player_turn(game.white_turn) {
                let green = [0.0, 0.5, 0.0, 1.0];
                for (x, outer_vec) in game.possible_moves.iter().enumerate() {
                    for (y, inner_vec) in outer_vec.iter().enumerate() {
                        if inner_vec.len() > 0 {
                            //this is a valid move
                            ellipse(
                                green,
                                [
                                    (x as f64 + 0.4) * dist_per_block,
                                    (y as f64 + 0.4) * dist_per_block,
                                    dist_per_block * 0.2,
                                    dist_per_block * 0.2,
                                ],
                                context.transform,
                                graphics,
                            )
                        }
                    }
                }
            }
        });

    }
}

fn undo(game: &mut Game) {
    match game.prev_boards.pop() {
        Some(old_board) => {
            game.board = old_board.0;
            game.white_turn = old_board.1;
            game.possible_moves = get_all_possible_moves(game).0;
            game.amount_of_stone -= 1;
            game.game_over = false;
        }
        None => (),
    }
}

fn get_winner(board: &[[Colour; BOARD_SIZE]; BOARD_SIZE]) -> Colour {
    let mut black_count = 0;
    let mut white_count = 0;
    for outer in board {
        for colour in outer {
            match colour {
                Colour::BLACK => black_count += 1,
                Colour::WHITE => white_count += 1,
                _ => (),
            }
        }
    }

    if black_count == white_count {
        Colour::EMPTY
    } else if black_count > white_count {
        Colour::BLACK
    } else {
        Colour::WHITE
    }
}

fn player_turn(white_turn: bool) -> bool {
    match AI_COLOUR {
        Colour::EMPTY => return true,       // No one is AI
        Colour::WHITE => return !white_turn,// AI is white
        Colour::BLACK => return white_turn, // AI is black
    }
}

fn set_up_board() -> [[Colour; BOARD_SIZE]; BOARD_SIZE] {
    let mut board = [[Colour::EMPTY; BOARD_SIZE]; BOARD_SIZE];
    board[BOARD_SIZE / 2 - 1][BOARD_SIZE / 2 - 1] = Colour::BLACK;
    board[BOARD_SIZE / 2][BOARD_SIZE / 2 - 1] = Colour::WHITE;
    board[BOARD_SIZE / 2 - 1][BOARD_SIZE / 2] = Colour::WHITE;
    board[BOARD_SIZE / 2][BOARD_SIZE / 2] = Colour::BLACK;
    return board;
}

fn do_move_and_print_info(x: usize, y: usize, game: &mut Game) {
    do_move(x, y, game);
    print_game_information(game);
}

fn do_move(x: usize, y: usize, game: &mut Game) {
    game.prev_boards.push((game.board.clone(), game.white_turn));
    let colour = if game.white_turn {
        Colour::WHITE
    } else {
        Colour::BLACK
    };

    game.last_placed = Square { x: x, y: y };
    game.flipped_tiles_from_move = game.possible_moves[x][y].clone();
    for sq in game.possible_moves[x][y].iter() {
        game.board[sq.x][sq.y] = colour
    }
    game.board[x][y] = colour;

    game.white_turn = !game.white_turn;
    let temp = get_all_possible_moves(&game);
    game.possible_moves = temp.0;
    let a_move_exists = temp.1;
    game.amount_of_stone += 1;

    if game.amount_of_stone == BOARD_SIZE * BOARD_SIZE {
        game.game_over = true;
        game.winner = get_winner(&game.board);
    } else if !a_move_exists {
        game.white_turn = !game.white_turn;
        let temp = get_all_possible_moves(&game);
        game.possible_moves = temp.0;
        let a_move_exists = temp.1;
        if !a_move_exists {
            game.game_over = true;
            game.winner = get_winner(&game.board);
        }
    }
}

fn print_game_information(game: &Game) {
    if game.game_over {
        match game.winner {
            Colour::BLACK => {
                println!("Game is over. The winner is black.");
            }
            Colour::WHITE => {
                println!("Game is over. White won.");
            }
            Colour::EMPTY => {
                println!("Game is over. It's a draw, a rare occurance.");
            }
        }
    } else {
        println!(
            "Current game state:\n\tCurrent player to do a move: {}\n\tAmount of stones on table: {}",
            if game.white_turn { "White" } else { "Black" },
            game.amount_of_stone
        );
    }
}

/*fn print_board(board: &[[Colour; BOARD_SIZE]; BOARD_SIZE], possible_moves: &Vec<Vec<Vec<Square>>>) {
    let mut _str = String::new();
    _str += "----------------\n";
    for y in 0..BOARD_SIZE {
        _str += "|";
        for x in 0..BOARD_SIZE {
            _str += match board[x][y] {
                Colour::WHITE => "w,",
                Colour::BLACK => "b,",
                Colour::EMPTY => {
                    if possible_moves[x][y].len() > 0 {
                        "x,"
                    } else {
                        "e,"
                    }
                }
            }
        }
        _str.pop();
        _str += "|\n"
    }
    _str += "----------------\n";
    print!("{}", _str);
}*/

fn get_all_possible_moves(game: &Game) -> (Vec<Vec<Vec<Square>>>, bool) {
    // The reuslt will be hashed based on square
    // e.g sqaure (x,y) has index [y*BOARD_SIZE + x]
    let mut result: Vec<Vec<Vec<Square>>> = Vec::with_capacity(BOARD_SIZE);
    let mut a_move_exists = false;
    let mut _temp = Vec::with_capacity(BOARD_SIZE);
    _temp.resize(BOARD_SIZE, vec![]);
    result.resize(BOARD_SIZE, _temp);

    for (x, row) in game.board.iter().enumerate() {
        for (y, square) in row.iter().enumerate() {
            match (square, game.white_turn) {
                (Colour::BLACK, false) => {
                    if insert_square_possible_moves(&game, x, y, &mut result) {
                        a_move_exists = true;
                    }
                }
                (Colour::WHITE, true) => {
                    if insert_square_possible_moves(&game, x, y, &mut result) {
                        a_move_exists = true;
                    }
                }
                _ => (),
            }
        }
    }
    (result, a_move_exists)
}

fn insert_square_possible_moves(
    game: &Game,
    x: usize,
    y: usize,
    dump: &mut Vec<Vec<Vec<Square>>>,
) -> bool {
    let mut squares_passed: Vec<Square> = vec![];
    let mut a_move_exists = false;

    // towards (0,0)
    let mut min = x.min(y);
    let mut passed = false;

    for i in 1..(min + 1) {
        match (game.board[x - i][y - i], game.white_turn) {
            (Colour::EMPTY, _) => {
                if passed {
                    dump[x - i][y - i].append(&mut squares_passed);
                    a_move_exists = true;
                }
                break;
            }
            (Colour::WHITE, false) => {
                passed = true;
                squares_passed.push(Square {
                    x: (x - i),
                    y: (y - i),
                })
            }
            (Colour::BLACK, true) => {
                passed = true;
                squares_passed.push(Square {
                    x: (x - i),
                    y: (y - i),
                })
            }
            _ => break,
        }
    }

    // towards (0, _)
    squares_passed = vec![];
    passed = false;
    for i in 1..(x + 1) {
        match (game.board[x - i][y], game.white_turn) {
            (Colour::EMPTY, _) => {
                if passed {
                    dump[x - i][y].append(&mut squares_passed);
                    a_move_exists = true;
                }
                break;
            }
            (Colour::WHITE, false) => {
                passed = true;
                squares_passed.push(Square { x: (x - i), y: (y) })
            }
            (Colour::BLACK, true) => {
                passed = true;
                squares_passed.push(Square { x: (x - i), y: (y) })
            }
            _ => break,
        }
    }

    // towards (0,7)
    min = x.min(7 - y);
    squares_passed = vec![];
    passed = false;
    for i in 1..(min + 1) {
        match (game.board[x - i][y + i], game.white_turn) {
            (Colour::EMPTY, _) => {
                if passed {
                    dump[x - i][y + i].append(&mut squares_passed);
                    a_move_exists = true;
                }
                break;
            }
            (Colour::WHITE, false) => {
                passed = true;
                squares_passed.push(Square {
                    x: (x - i),
                    y: (y + i),
                })
            }
            (Colour::BLACK, true) => {
                passed = true;
                squares_passed.push(Square {
                    x: (x - i),
                    y: (y + i),
                })
            }
            _ => break,
        }
    }

    // towards (_, 7)
    squares_passed = vec![];
    passed = false;
    for i in 1..(7 - y + 1) {
        match (game.board[x][y + i], game.white_turn) {
            (Colour::EMPTY, _) => {
                if passed {
                    dump[x][y + i].append(&mut squares_passed);
                    a_move_exists = true;
                }
                break;
            }
            (Colour::WHITE, false) => {
                passed = true;
                squares_passed.push(Square { x: (x), y: (y + i) })
            }
            (Colour::BLACK, true) => {
                passed = true;
                squares_passed.push(Square { x: (x), y: (y + i) })
            }
            _ => break,
        }
    }

    // towards (7,7)
    squares_passed = vec![];
    passed = false;
    min = (7 - x).min(7 - y);
    for i in 1..(min + 1) {
        match (game.board[x + i][y + i], game.white_turn) {
            (Colour::EMPTY, _) => {
                if passed {
                    dump[x + i][y + i].append(&mut squares_passed);
                    a_move_exists = true;
                }
                break;
            }
            (Colour::WHITE, false) => {
                passed = true;
                squares_passed.push(Square {
                    x: (x + i),
                    y: (y + i),
                })
            }
            (Colour::BLACK, true) => {
                passed = true;
                squares_passed.push(Square {
                    x: (x + i),
                    y: (y + i),
                })
            }
            _ => break,
        }
    }

    // towards (7, _)
    squares_passed = vec![];
    passed = false;
    for i in 1..(7 - x + 1) {
        match (game.board[x + i][y], game.white_turn) {
            (Colour::EMPTY, _) => {
                if passed {
                    dump[x + i][y].append(&mut squares_passed);
                    a_move_exists = true;
                }
                break;
            }
            (Colour::WHITE, false) => {
                passed = true;
                squares_passed.push(Square { x: (x + i), y: (y) })
            }
            (Colour::BLACK, true) => {
                passed = true;
                squares_passed.push(Square { x: (x + i), y: (y) })
            }
            _ => break,
        }
    }

    // towards (7,0)
    min = (7 - x).min(y);
    squares_passed = vec![];
    passed = false;
    for i in 1..(min + 1) {
        match (game.board[x + i][y - i], game.white_turn) {
            (Colour::EMPTY, _) => {
                if passed {
                    dump[x + i][y - i].append(&mut squares_passed);
                    a_move_exists = true;
                }
                break;
            }
            (Colour::WHITE, false) => {
                passed = true;
                squares_passed.push(Square {
                    x: (x + i),
                    y: (y - i),
                })
            }
            (Colour::BLACK, true) => {
                passed = true;
                squares_passed.push(Square {
                    x: (x + i),
                    y: (y - i),
                })
            }
            _ => break,
        }
    }

    // towards (_, 0)
    squares_passed = vec![];
    passed = false;
    for i in 1..(y + 1) {
        match (game.board[x][y - i], game.white_turn) {
            (Colour::EMPTY, _) => {
                if passed {
                    dump[x][y - i].append(&mut squares_passed);
                    a_move_exists = true;
                }
                break;
            }
            (Colour::WHITE, false) => {
                passed = true;
                squares_passed.push(Square { x: (x), y: (y - i) })
            }
            (Colour::BLACK, true) => {
                passed = true;
                squares_passed.push(Square { x: (x), y: (y - i) })
            }
            _ => break,
        }
    }

    a_move_exists
}
