# othello-ai

Simple min-max algorithm that plays as black in the game othello.

### Benchmark

Beat the ai at https://mindsports.nl/index.php/dagaz/867-othello-ai 2/2 times.

### Bugs
Some bugs were discovered during benchmarks. Theese should be fixed but have not been tested yet.
* Stack-overflow. The min-max did overflow because neither had any moves. (The `move_found` for both max and min was false).
  * Solved by setting `depth`-arg as minimium of (`DEPTH`, `BOARD_SIZE*BOARD_SIZE - amount_of_stones`)
* When white places last move black tried to do a move, which it couldn't. That caused a run-time error. Fixed by adding checks for `game.game_over`.
