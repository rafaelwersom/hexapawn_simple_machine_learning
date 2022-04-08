/// This module contains the game itself, it will carry functions to make moves, check who won
/// and be able to choose to play against the computer or against another player.

pub mod game {
    #[derive(PartialEq)]
    pub enum Turn { Player1, Player2, }

    #[derive(PartialEq)]
    pub enum Square {Player1, Player2, Empty, }

    pub struct Hexapawn {
        pub turn: Turn,
        pub board: Vec<Vec<Square>>,
    }
    
    impl Hexapawn {
        // Creates a new board with all pieces in the right position and the first move set to Player1 by default
        pub fn new() -> Self {
            Hexapawn { turn: Turn::Player1, board: vec![vec![Square::Player1, Square::Player1, Square::Player1], 
                                                    vec![Square::Empty, Square::Empty, Square::Empty], 
                                                    vec![Square::Player2, Square::Player2, Square::Player2]] }
        }

        pub fn toggle_turn(&mut self) {
            match self.turn {
                Turn::Player1 => self.turn = Turn::Player2,
                _ => self.turn = Turn::Player1,
            }
        }

        // Checks if the passed player won
        pub fn is_winner(&self, turn: Turn) -> bool {
            if turn == Turn::Player1 {
                if self.board[2][0] == Square::Player1 || self.board[2][1] == Square::Player1 || self.board[2][2] == Square::Player1 || !self.opponent_can_move() {
                    return true;
                }
                return false;
            } else {
                if self.board[0][0] == Square::Player2 || self.board[0][1] == Square::Player2 || self.board[0][2] == Square::Player2 || !self.opponent_can_move() {
                    return true;
                }
                return false;
            }
        }

        // Check if game is finished
        pub fn game_finished(&self) -> bool {
            return self.is_winner(Turn::Player1) || self.is_winner(Turn::Player2);
        }

        // Checks if there are any legal moves to be done by the passed player
        fn opponent_can_move(&self) -> bool {
            if self.turn == Turn::Player1 {

            } else {

            }
            return true;
        }

        // Make a move
        pub fn move_piece(&mut self, pawn_move: String) -> bool {
            if pawn_move.len() != 3 && pawn_move.len() != 5 {
                return false;
            } 
            if self.turn == Turn::Player1 {
                if pawn_move.len() == 3 {
                    let col = pawn_move.chars().nth(0).unwrap() as i32 - 97;
                    let row = pawn_move.chars().nth(1).unwrap() as i32 - '0' as i32 - 1;
                    if col < 0 || col > 2 || row <= 0 || row > 2 {
                        return false;
                    }
                    if self.board[row as usize][col as usize] == Square::Empty && self.board[row as usize - 1][col as usize] == Square::Player1 {
                        self.board[row as usize][col as usize] = Square::Player1;
                        self.board[row as usize - 1][col as usize] = Square::Empty;
                        return true;
                    }
                } else {
                    let start_col = pawn_move.chars().nth(0).unwrap()as i32 - 97;
                    let col = pawn_move.chars().nth(2).unwrap() as i32 - 97;
                    let row = pawn_move.chars().nth(3).unwrap() as i32 - '0' as i32 - 1;
                    if start_col < 0 || start_col > 2 || col < 0 || col > 2 || start_col == col || (col - start_col).abs() > 1 || row <= 0 || row > 2 {
                        return false;
                    }
                    if self.board[row as usize][col as usize] == Square::Player2 && self.board[row as usize - 1][start_col as usize] == Square::Player1 {
                        self.board[row as usize][col as usize] = Square::Player1;
                        self.board[row as usize - 1][start_col as usize] = Square::Empty;
                        return true;
                    }
                }
            } else {
                if pawn_move.len() == 3 {
                    let col = pawn_move.chars().nth(0).unwrap() as i32 - 97;
                    let row = pawn_move.chars().nth(1).unwrap() as i32 - '0' as i32 - 1;
                    if col < 0 || col > 2 || row < 0 || row >= 2 {
                        return false;
                    }
                    if self.board[row as usize][col as usize] == Square::Empty && self.board[row as usize + 1][col as usize] == Square::Player2 {
                        self.board[row as usize][col as usize] = Square::Player2;
                        self.board[row as usize + 1][col as usize] = Square::Empty;
                        return true;
                    }
                    
                } else {
                    let start_col = pawn_move.chars().nth(0).unwrap()as i32 - 97;
                    let col = pawn_move.chars().nth(2).unwrap() as i32 - 97;
                    let row = pawn_move.chars().nth(3).unwrap() as i32 - '0' as i32 - 1;
                    if start_col < 0 || start_col > 2 || col < 0 || col > 2 || start_col == col || (col - start_col).abs() > 1 || row < 0 || row >= 2 {
                        return false;
                    }
                    if self.board[row as usize][col as usize] == Square::Player1 && self.board[row as usize + 1][start_col as usize] == Square::Player2 {
                        self.board[row as usize][col as usize] = Square::Player2;
                        self.board[row as usize + 1][start_col as usize] = Square::Empty;
                        return true;
                    }
                }
            }
            false
        }

        // Prints the board based on self.board
        pub fn print_board(&self) {
            println!("                  _        _    \n        /\\       |_)      /     \n       /--\\      |_)      \\_    ");
            match self.board[2][1] {
                Square::Empty => println!("  _  #########         #########"),
                _ => println!("  _  #########    _    #########"), 
            }
            match self.board[2][0] {
                Square::Empty => print!("  _) #########"),
                Square::Player1 => print!("  _) ###(_)###"),
                Square::Player2 => print!("  _) ###(:)###"),
            }
            match self.board[2][1] {
                Square::Empty => print!("         "),
                Square::Player1 => print!("   (_)   "),
                Square::Player2 => print!("   (:)   "),
            }
            match self.board[2][2] {
                Square::Empty => println!("#########"),
                Square::Player1 => println!("###(_)###"),
                Square::Player2 => println!("###(:)###"),
            }
            match self.board[2][0] {
                Square::Empty => print!("  _) #########"),
                Square::Player1 => print!("  _) ###| |###"),
                Square::Player2 => print!("  _) ###|:|###"),
            }
            match self.board[2][1] {
                Square::Empty => print!("         "),
                Square::Player1 => print!("   | |   "),
                Square::Player2 => print!("   |:|   "),
            }
            match self.board[2][2] {
                Square::Empty => println!("#########"),
                Square::Player1 => println!("###| |###"),
                Square::Player2 => println!("###|:|###"),
            }
            match self.board[2][0] {
                Square::Empty => print!("     #########"),
                Square::Player1 => print!("     ###|_|###"),
                Square::Player2 => print!("     ###|:|###"),
            }
            match self.board[2][1] {
                Square::Empty => print!("         "),
                Square::Player1 => print!("   |_|   "),
                Square::Player2 => print!("   |:|   "),
            }
            match self.board[2][2] {
                Square::Empty => println!("#########"),
                Square::Player1 => println!("###|_|###"),
                Square::Player2 => println!("###|:|###"),
            }
            println!("     #########         #########");
            match self.board[1][0] {
                Square::Empty => print!("  _           "),
                _ => print!("  _      _    "),
            }
            print!("#########");
            match self.board[1][2] {
                Square::Empty => println!("         "),
                _ => println!("    _    "),
            }
            match self.board[1][0] {
                Square::Empty => print!("   )          "),
                Square::Player1 => print!("   )    (_)   "),
                Square::Player2 => print!("   )    (:)   "),
            }
            match self.board[1][1] {
                Square::Empty => print!("#########"),
                Square::Player1 => print!("###(_)###"),
                Square::Player2 => print!("###(:)###"),
            }
            match self.board[1][2] {
                Square::Empty => println!("         "),
                Square::Player1 => println!("   (_)   "),
                Square::Player2 => println!("   (:)   "),
            }
            match self.board[1][0] {
                Square::Empty => print!("  /_          "),
                Square::Player1 => print!("  /_    | |   "),
                Square::Player2 => print!("  /_    |:|   "),
            }
            match self.board[1][1] {
                Square::Empty => print!("#########"),
                Square::Player1 => print!("###| |###"),
                Square::Player2 => print!("###|:|###"),
            }
            match self.board[1][2] {
                Square::Empty => println!("         "),
                Square::Player1 => println!("   | |   "),
                Square::Player2 => println!("   |:|   "),
            }
            match self.board[1][0] {
                Square::Empty => print!("              "),
                Square::Player1 => print!("        |_|   "),
                Square::Player2 => print!("        |:|   "),
            }
            match self.board[1][1] {
                Square::Empty => print!("#########"),
                Square::Player1 => print!("###|_|###"),
                Square::Player2 => print!("###|:|###"),
            }
            match self.board[1][2] {
                Square::Empty => println!("         "),
                Square::Player1 => println!("   |_|   "),
                Square::Player2 => println!("   |:|   "),
            }
            println!("              #########         ");
            match self.board[0][1] {
                Square::Empty => println!("     #########         #########"),
                _ => println!("     #########    _    #########"),
            }
            match self.board[0][0] {
                Square::Empty => print!("  /| #########"),
                Square::Player1 => print!("  /| ###(_)###"),
                Square::Player2 => print!("  /| ###(:)###"),
            }
            match self.board[0][1] {
                Square::Empty => print!("         "),
                Square::Player1 => print!("   (_)   "),
                Square::Player2 => print!("   (:)   "),
            }
            match self.board[0][2] {
                Square::Empty => println!("#########"),
                Square::Player1 => println!("###(_)###"),
                Square::Player2 => println!("###(:)###"),
            }
            match self.board[0][0] {
                Square::Empty => print!("   | #########"),
                Square::Player1 => print!("   | ###| |###"),
                Square::Player2 => print!("   | ###|:|###"),
            }
            match self.board[0][1] {
                Square::Empty => print!("         "),
                Square::Player1 => print!("   | |   "),
                Square::Player2 => print!("   |:|   "),
            }
            match self.board[0][2] {
                Square::Empty => println!("#########"),
                Square::Player1 => println!("###| |###"),
                Square::Player2 => println!("###|:|###"),
            }
            match self.board[0][0] {
                Square::Empty => print!("     #########"),
                Square::Player1 => print!("     ###|_|###"),
                Square::Player2 => print!("     ###|:|###"),
            }
            match self.board[0][1] {
                Square::Empty => print!("         "),
                Square::Player1 => print!("   |_|   "),
                Square::Player2 => print!("   |:|   "),
            }
            match self.board[0][2] {
                Square::Empty => println!("#########"),
                Square::Player1 => println!("###|_|###"),
                Square::Player2 => println!("###|:|###"),
            }
            println!("     #########         #########");
        }
    }
}

/// This will take care of the interface with the user, check if he wants to play against
/// a player or the computer, help, teach the commands, etc.
pub mod interface {
    use crate::game::{Hexapawn, Turn};
    use std::io::stdin;
    use std::io::Write;

    // command to help players use the right commands
    pub fn help_main_menu() {
        println!("Valid commands:\n");
        println!("help - Show valid commands");
        println!("play_ai - Play against the computer");
        println!("play_friend - Play against friend");
        println!("exit - Exit program\n");
    }

    pub fn help_game() {
        println!("Invalid move!");
        println!("Usage: (column + row) for walking move. ex: \"b2\"");
        println!("       (initial position + \"x\" + column + row) for attacking move. ex: \"axb2\"");
        println!("       You can't go backwards, walk to a spot where there is a piece already, or capture your own piece");
    }

    pub fn play_friend() {
        let mut game = Hexapawn::new();
        game.print_board();
        while !game.game_finished() {
            match game.turn {
                Turn::Player1 => {
                    let mut input_string = String::new();
                    print!("Player1 move: ");
                    let _ = std::io::stdout().flush();
                    stdin().read_line(&mut input_string).expect("Failed to read line");
                    if !game.move_piece(input_string) {
                        help_game();
                    } else {
                        game.print_board();
                        game.toggle_turn();
                    }
                },
                _ => {
                    let mut input_string = String::new();
                    print!("Player2 move: ");
                    let _ = std::io::stdout().flush();
                    stdin().read_line(&mut input_string).ok().expect("Failed to read line");
                    if !game.move_piece(input_string) {
                        help_game();
                    } else {
                        game.print_board();
                        game.toggle_turn();
                    }
                },
            }
        }
        if game.turn == Turn::Player1 {
            println!("Player2 Wins!");
        } else {
            println!("Player1 Wins!");
        }
    }
}