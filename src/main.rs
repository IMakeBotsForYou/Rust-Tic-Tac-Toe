use std::io;
use std::fmt::{self, Formatter, Display};
use std::result::Result;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
struct InputError {
    message: String,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InputError {}


impl InputError {
    fn new(message: &str) -> InputError {
        InputError {
            message: message.to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
enum Winner {
    Player(Player),
    Draw
}

#[derive(PartialEq, Clone, Copy, Eq)]
struct Board {
    cells: [Option<Player>; 9],
    turn: Player,
    winner: Option<Winner>,
    highlight: Option<Coordinate>,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let _ = match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        };
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        
        // for i in [Index::Zero, Index::One, Index::Two] {
        //       for j in [Index::Zero, Index::One, Index::Two] 
        //         let tile = match self.cells[i * 3 + j] {
        //             Some(true) =>  if i == row && j == col {"[Ｘ]"} else {" Ｘ "},
        //             Some(false) => if i == row && j == col {"[Ｏ]"} else {" Ｏ "},
        //             None =>        if i == row && j == col {"[　]"} else {" 　 "},
        //         };
        //         let wall: &str = if j != 2 { "|" } else { "" };
        //         write!(f, "{}{}", tile, wall)?;
        //     }
        //     writeln!(f, "\n ー | ー | ー ")?;
        // }
        // let board: [Option<Player>; 9] = self.cells;
        let formatted_board: String = format!(
            "         1   2   3
       ╔═══╤═══╤═══╗
     1 ║{}│{}│{}║
       ╟───┼───┼───╢
     2 ║{}│{}│{}║
       ╟───┼───┼───╢
     3 ║{}│{}│{}║
       ╚═══╧═══╧═══╝",
            cell_to_char(&self, 0), cell_to_char(&self, 1), cell_to_char(&self, 2),
            cell_to_char(&self, 3), cell_to_char(&self, 4), cell_to_char(&self, 5),
            cell_to_char(&self, 6), cell_to_char(&self, 7), cell_to_char(&self, 8),
        );
        writeln!(f, "{}", formatted_board)?;
        Ok(())
    }
}

fn cell_to_char(board: &Board, index: usize) -> &'static str {
    let highlighted_index: usize;

    if let Some(coord) = board.highlight {
        highlighted_index = (coord.row as usize) * 3 + coord.col as usize;
    } else {
        highlighted_index = 10; // Out of bounds of array, never matches.
    }

    match board.cells[index] {
        Some(Player::X) => {" X "},
        Some(Player::O) => {" O "},
        None => if highlighted_index == index {"[ ]"} else {"   "}
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
enum Player {
    X,
    O
}

impl Board {
    fn new() -> Self {
        Self { cells: [None; 9], turn: Player::O, winner: None, highlight: None }
    }

    fn highlight(&mut self, cell: Coordinate) {
        self.highlight = Some(cell);
    }

    fn play_move(&mut self, pl_move: Coordinate) {
        self.cells[(pl_move.row as usize) * 3 + pl_move.col as usize] = Some(self.turn);
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell.is_some())
    }
    
    fn undo_move(&mut self, coord: Coordinate) {
        self.cells[(coord.row as usize * 3) + coord.col as usize] = None;
        self.winner = None; // Reset winner state
    }

    fn check_if_game_over(&mut self) { 
        /*
        0 | 1 | 2
        3 | 4 | 5
        6 | 7 | 8
                 */

        const WINNING_PATTERNS: [[usize; 3]; 8] = [
            // Rows
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // Columns
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // Diagonals
            [0, 4, 8],
            [2, 4, 6],
        ];
        
        for pattern in WINNING_PATTERNS.iter() {
            if let Some(player) = self.cells[pattern[0]] {
                if self.cells[pattern[0]] == self.cells[pattern[1]] && self.cells[pattern[1]] == self.cells[pattern[2]] {
                    self.winner = Some(Winner::Player(player));
                    return;
                }
            }
        }

        if self.is_full(){
            self.winner = Some(Winner::Draw)
        } else {
            self.winner = None;
        }
    }

    fn get_cell(&self, coord: Coordinate) -> Option<Player> {
        self.cells[(coord.row as usize * 3) + coord.col as usize]
    }

    fn generate_all_equivalent_states(&self) -> Vec<[Option<Player>; 9]> {
        let mut states = Vec::new();
        let mut current = self.cells;

        // Add the original state and all its rotations and reflections
        for _ in 0..2 {
            for _ in 0..4 {
                current = rotate_90(current);
                states.push(current);
            }
            current = reflect(current);
        }
        
        states
    }

}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
    }
}

fn rotate_90(board: [Option<Player>; 9]) -> [Option<Player>; 9] {
    [
        board[6], board[3], board[0],
        board[7], board[4], board[1],
        board[8], board[5], board[2],
    ]
}

fn reflect(board: [Option<Player>; 9]) -> [Option<Player>; 9] {
    [
        board[2], board[1], board[0],
        board[5], board[4], board[3],
        board[8], board[7], board[6],
    ]
}


#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Hash, Eq)]
enum Index {
    Zero,
    One,
    Two,
}

impl Index {
    fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(Index::Zero),
            1 => Some(Index::One),
            2 => Some(Index::Two),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
struct Coordinate {
    row: Index,
    col: Index
}


impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.row, self.col)?;
        Ok(())
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let _ = match self {
            Index::Zero => write!(f, "1"), 
            Index::One  => write!(f, "2"),
            Index::Two  => write!(f, "3"),
        };
        Ok(())
    }
}




fn get_input_from_console() -> Result<String, InputError> {
    let mut my_input = String::new();
    io::stdin()
    .read_line(&mut my_input)
    .map_err(|_| InputError::new("Failed to read line"))?;
    Ok(my_input)
}

fn parse_coordinates(input: String) -> Result<Coordinate, InputError> {
    let parts: Vec<&str> = input.trim().split('-').collect();

    if parts.len() != 2 {
        return Err(InputError::new("Input must be in the format 'row-col'"));
    }

    let row: usize = parts[0].parse().map_err(|_| InputError::new("Invalid row index"))?;
    let col: usize = parts[1].parse().map_err(|_| InputError::new("Invalid column index"))?;

    let row_index = Index::from_usize(row - 1).ok_or(InputError::new("Row index out of range"))?;
    let col_index = Index::from_usize(col - 1).ok_or(InputError::new("Column index out of range"))?;

    Ok(Coordinate { row: row_index, col: col_index })
} 

fn get_and_play_user_move(board: &mut Board) -> Result<bool, InputError> {
    // Prompt the user for their turn if no cell is highlighted
    if board.highlight.is_none() {
        println!("Your turn, {}.", board.turn);
    }

    // Get user input from the console
    let my_input = get_input_from_console()?.trim().to_string();

    // Handle the 'ok' command to confirm a move
    if my_input == "ok" {
        if let Some(highlight) = board.highlight {
            board.play_move(highlight);
            board.highlight = None;
            // Move played, turn is over.
            return Ok(true);
        } else {
            return Err(InputError::new("You must select a cell to play something in it."));
        }
    }

    // Attempt to parse the user input as coordinates
    match parse_coordinates(my_input) {
        Ok(coordinates) => {
            // Check if the selected cell is already occupied
            if board.get_cell(coordinates).is_none() {
                // Highlight the selected cell if it's empty
                board.highlight(coordinates);
                println!("{}", board);
            } else {
                return Err(InputError::new("That cell is already taken."));
            }
        }
        Err(_) => {
            // Handle invalid coordinate format
            return Err(InputError::new("Invalid input. Please enter coordinates in the format 'row-col' (e.g., '1-2')."));
        }
    }

    // Indicate that the move is not yet confirmed
    Ok(false)
}


fn ai_best_move(board: &mut Board, ai_character: Player) -> (Coordinate, usize) {
    let mut best_move: Coordinate = Coordinate { row: Index::Zero, col: Index::Zero };
    let mut best_score = i32::MIN;
    let mut seen_states = HashSet::new();
    let mut counter: usize = 0;


    for row in [Index::Zero, Index::One, Index::Two] {
        for col in [Index::Zero, Index::One, Index::Two] {
            let coord: Coordinate = Coordinate { row, col };
            if board.get_cell(coord).is_some() {
                continue; // Skip non-empty cells
            }

            board.play_move(coord);
            let original_turn = board.turn;
            board.turn = match board.turn {
                Player::X => Player::O,
                Player::O => Player::X,
            };

            board.check_if_game_over();
            let state = board.generate_all_equivalent_states();

            if seen_states.insert(state) {
                let score = min_max(board, false, ai_character, 0, i32::MIN, i32::MAX, &mut counter);

                if score > best_score {
                    best_score = score;
                    best_move = Coordinate { row, col };
                }
            }

            board.undo_move(coord); // Properly undo the move
            board.turn = original_turn;
        }
    }

    (best_move, counter)
}

fn min_max(board: &mut Board, maximizing: bool, ai_player: Player, depth: i32, mut alpha: i32, mut beta: i32, counter: &mut usize) -> i32 {
    *counter += 1; // Increment the position counter

    if let Some(winner) = &board.winner {
        return match winner {
            Winner::Player(p) => {
                if *p == ai_player { 10 - depth } else { depth - 10 }
            }
            Winner::Draw => 0,
        };
    }

    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

    for row in [Index::Zero, Index::One, Index::Two] {
        for col in [Index::Zero, Index::One, Index::Two] {
            let coord: Coordinate = Coordinate { row, col };
            if board.get_cell(coord).is_some() {
                continue;
            }

            board.play_move(coord);
            let original_turn = board.turn;
            board.turn = match board.turn {
                Player::X => Player::O,
                Player::O => Player::X,
            };

            board.check_if_game_over();
            let score = min_max(board, !maximizing, ai_player, depth + 1, alpha, beta, counter);

            board.undo_move(coord); // Properly undo the move
            board.turn = original_turn;

            if maximizing {
                best_score = best_score.max(score);
                alpha = alpha.max(score);
            } else {
                best_score = best_score.min(score);
                beta = beta.min(score);
            }

            if beta <= alpha {
                break; // Alpha-beta pruning
            }
        }
    }

    best_score
}

fn play_bot_move(board: &mut Board, player: Player) -> Coordinate {
    let (best_move, counter): (Coordinate, usize) = ai_best_move(board, player);
    board.play_move(best_move);
    println!("I looked at {counter} parallel universes,\nand {best_move} was the only one in which I win.");
    best_move
}

fn pick_side() -> Result<Player, InputError> {
    println!("Pick a side. x/o. O always plays first.");
    let user_input: String = get_input_from_console()?.trim().to_lowercase();
    
    match user_input.as_str() {
        "x" => Ok(Player::X),
        "o" => Ok(Player::O),
        _ => Err(InputError::new("Invalid input. Please pick between 'x' and 'o'.")),
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let mut game = Board::new();

    // Determine player's side
    let picked_side = match pick_side() {
        Ok(side) => side,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    // Main game loop
    while game.winner.is_none() && !game.is_full() {
        clear_screen();
        println!("{}", game);

        if game.turn == picked_side {
            // Player's turn
            loop {
                match get_and_play_user_move(&mut game) {
                    Ok(true) => break,
                    Ok(false) => continue,
                    Err(e) => println!("Error: {}", e),
                }
                
            }
        } else {
            // Bot's turn
            play_bot_move(&mut game, if picked_side == Player::X { Player::O } else { Player::X });
        }

        // Switch turns
        game.turn = match game.turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        // Check if the game is over
        game.check_if_game_over();
    }

    // Print final board state
    println!("{}", game);

    // Print game result
    match game.winner {
        Some(Winner::Player(player)) => println!("{} wins.", player),
        Some(Winner::Draw) => println!("The game is a draw."),
        None => println!("Game ended unexpectedly."),
    }
}
