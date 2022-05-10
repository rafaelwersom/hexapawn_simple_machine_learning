// This library is where the primary components of the game are coded

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

        // Toggles turn between Turn::Player1 and Turn::Player2
        pub fn toggle_turn(&mut self) {
            match self.turn {
                Turn::Player1 => self.turn = Turn::Player2,
                _ => self.turn = Turn::Player1,
            }
        }

        // Check if game is finished
        pub fn game_finished(&self) -> bool {
            self.board[2][0] == Square::Player1 || self.board[2][1] == Square::Player1 || self.board[2][2] == Square::Player1 || !self.opponent_can_move() 
            || self.board[0][0] == Square::Player2 || self.board[0][1] == Square::Player2 || self.board[0][2] == Square::Player2
        }

        // Checks if there are any legal moves to be done by the passed player
        fn opponent_can_move(&self) -> bool {
            if self.turn == Turn::Player1 {
                for row in 1..3 {
                    for col in 1..3 {
                        if self.board[row][col] == Square::Player2 {
                            if self.board[row - 1][col] == Square::Empty {
                                return true;
                            }
                        }
                        if col == 0 {
                            if self.board[row][col] == Square::Player2 && self.board[row - 1][col + 1] == Square::Player1 {
                                return true;
                            }
                        } else if col == 1 {
                            if self.board[row][col] == Square::Player2 && (self.board[row - 1][col + 1] == Square::Player1 || self.board[row - 1][col - 1] == Square::Player1) {
                                return true;
                            }
                        } else {
                            if self.board[row][col] == Square::Player2 && self.board[row - 1][col - 1] == Square::Player1 {
                                return true;
                            }
                        }
                    }
                }
            } else {
                for row in 0..2 {
                    for col in 0..2 {
                        if self.board[row][col] == Square::Player1 {
                            if self.board[row + 1][col] == Square::Empty {
                                return true;
                            }
                        }
                        if col == 0 {
                            if self.board[row][col] == Square::Player1 && self.board[row + 1][col + 1] == Square::Player2 {
                                return true;
                            }
                        } else if col == 1 {
                            if self.board[row][col] == Square::Player1 && (self.board[row + 1][col + 1] == Square::Player2 || self.board[row + 1][col - 1] == Square::Player2) {
                                return true;
                            }
                        } else {
                            if self.board[row][col] == Square::Player1 && self.board[row + 1][col - 1] == Square::Player2 {
                                return true;
                            }
                        }
                    }
                }
            }
            return false;
        }

        pub fn give_ai_possible_moves(&self) -> Vec<String> {
            let mut retval = Vec::new();
            if self.board[2][0] == Square::Player2 && self.board[1][0] == Square::Empty { retval.push("a2".to_owned()); }
            if self.board[2][1] == Square::Player2 && self.board[1][1] == Square::Empty { retval.push("b2".to_owned()); }
            if self.board[2][2] == Square::Player2 && self.board[1][2] == Square::Empty { retval.push("c2".to_owned()); }
            if self.board[1][0] == Square::Player2 && self.board[0][0] == Square::Empty { retval.push("a1".to_owned()); }
            if self.board[1][1] == Square::Player2 && self.board[0][1] == Square::Empty { retval.push("b1".to_owned()); }
            if self.board[1][2] == Square::Player2 && self.board[0][2] == Square::Empty { retval.push("c1".to_owned()); }
            if self.board[2][0] == Square::Player2 && self.board[1][1] == Square::Player1 { retval.push("axb2".to_owned()); }
            if self.board[2][2] == Square::Player2 && self.board[1][1] == Square::Player1 { retval.push("cxb2".to_owned()); }
            if self.board[2][1] == Square::Player2 && self.board[1][0] == Square::Player1 { retval.push("bxa2".to_owned()); }
            if self.board[2][1] == Square::Player2 && self.board[1][2] == Square::Player1 { retval.push("bxc2".to_owned()); }
            if self.board[1][0] == Square::Player2 && self.board[0][1] == Square::Player1 { retval.push("axb1".to_owned()); }
            if self.board[1][2] == Square::Player2 && self.board[0][1] == Square::Player1 { retval.push("cxb1".to_owned()); }
            if self.board[1][1] == Square::Player2 && self.board[0][0] == Square::Player1 { retval.push("bxa1".to_owned()); }
            if self.board[1][1] == Square::Player2 && self.board[0][2] == Square::Player1 { retval.push("bxc1".to_owned()); }
            retval
        }

        // Make a move based on a string
        pub fn move_piece(&mut self, pawn_move: &String) -> bool {
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
    use crate::ai::AI;
    use std::io::stdin;
    use std::io::Write;

    // command to help players use the right commands
    pub fn help_main_menu() {
        println!("\nValid commands:");
        println!("help - Show valid commands");
        println!("play_friend - Play against friend");
        println!("play_ai - Play against the computer");
        println!("clear - erases data collected by AI opponent");
        println!("info - get the data collected by AI opponent");
        println!("exit - Exit program\n");
    }

    pub fn help_game() {
        println!("\nInvalid move!");
        println!("Usage:");
        println!("-(column + row) for walking move. ex: \"b2\"");
        println!("-(initial column + \"x\" + target piece column + row) for attacking move. ex: \"axb2\"");
        println!("-You can't go backwards or capture your own piece.");
        println!("-You can't walk to a spot where there is a piece already.\n")
    }

    pub fn info() {
        println!("\nInfo:");
        let ai = AI::from("data/moves_ai.csv");
        println!("You played against the computer {} times and won {} times!\n", ai.num_times_played, ai.num_times_lost);
    }

    pub fn clear() -> Result<(), std::io::Error> {
        let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open("data/moves_ai.csv")?;
        f.write_all(b"0\n0\n")?;
        Ok(())
    }

    pub fn play_ai() {
        let mut game = Hexapawn::new();
        game.print_board();
        let mut ai = AI::from("data/moves_ai.csv");
        let mut game_sequence = String::new();
        while !game.game_finished() {
            match game.turn {
                Turn::Player1 => { 
                    let mut input_string = String::new();
                    print!("Player1 move: ");
                    let _ = std::io::stdout().flush();
                    stdin().read_line(&mut input_string).expect("Failed to read line");
                    if !game.move_piece(&input_string) {
                        help_game();
                    } else {
                        game.print_board();
                        game.toggle_turn();
                        if !game.game_finished() {
                            game_sequence.push_str(&input_string[..input_string.len() - 1]);
                        } else {
                            ai.losing_sequences.push((&game_sequence[..]).to_string());
                        }
                        println!("Current game sequence: {}", game_sequence);
                    }
                },
                _ => {
                    let mut input_string = ai.get_move(game.give_ai_possible_moves(), &game_sequence);
                    game_sequence.push_str(&input_string[..]);
                    input_string.push('\n');
                    game.move_piece(&input_string);
                    game.print_board();
                    game.toggle_turn();
                    print!("The computer made the move: {}", input_string);
                    println!("Current game sequence: {}", game_sequence);
                },
            }
        }
        if game.turn == Turn::Player1 {
            println!("The Machine Wins!\n");
        } else {
            println!("Player1 Wins! But the Machine gets smarter!\n");
            ai.num_times_lost += 1;
        }
        ai.num_times_played += 1;
        println!("You played against the computer {} times and won {} times!\n", ai.num_times_played, ai.num_times_lost);
        ai.write_to_file().unwrap();
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
                    if !game.move_piece(&input_string) {
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
                    if !game.move_piece(&input_string) {
                        help_game();
                    } else {
                        game.print_board();
                        game.toggle_turn();
                    }
                },
            }
        }
        if game.turn == Turn::Player1 {
            println!("Player2 Wins!\n");
        } else {
            println!("Player1 Wins!\n");
        }
    }
}

/// This will be where the computer will read the text document of moves and
/// translate that into possible moves
pub mod ai {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use rand::seq::SliceRandom;
    use std::io::Write;

    pub struct AI {
        pub num_times_played: u8,
        pub num_times_lost: u8,
        pub losing_sequences: Vec<String>
    }

    impl AI {
        pub fn new() -> Self {
            AI { num_times_played: 0, num_times_lost: 0, losing_sequences: Vec::new() }
        }

        pub fn from(filename: &str) -> Self {
            let mut retval = AI::new();
            let file = File::open(filename);
            let reader = BufReader::new(file.unwrap());
            let mut line_num = 0;
            for read_line in reader.lines() {
                let line = read_line.unwrap();
                if line_num == 0 {
                    retval.num_times_played = line.parse().unwrap();
                } else if line_num == 1 {
                    retval.num_times_lost = line.parse().unwrap();
                } else {
                    for pmove in line.split(',') {
                        retval.losing_sequences.push(pmove.to_owned());
                    }
                }
                line_num += 1;
            }
            retval
        }

        pub fn get_move(&self, possible_moves: Vec<String>, game_sequence: &String) -> String {
            let mut rng = rand::thread_rng();
            let mut input_string = possible_moves.choose(&mut rng).unwrap();
            let mut new_sequence = String::from(game_sequence);
            new_sequence.push_str(&input_string);
            while self.losing_sequences.iter().any(|i| i == new_sequence.as_str()) {
                rng = rand::thread_rng();
                input_string = possible_moves.choose(&mut rng).unwrap();
                new_sequence = String::from(game_sequence);
                new_sequence.push_str(&input_string);
            }
            return input_string.to_owned();
        }

        pub fn write_to_file(&self) -> Result<(), std::io::Error> {
            let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open("data/moves_ai.csv")?;
            f.write_all(self.num_times_played.to_string().as_bytes())?;
            f.write_all(b"\n")?;
            f.write_all(self.num_times_lost.to_string().as_bytes())?;
            f.write_all(b"\n")?;
            for sequence in &self.losing_sequences[..] {
                f.write_all(sequence.as_bytes())?;
                f.write_all(b",")?;
            }
            Ok(())
        }
    }
}