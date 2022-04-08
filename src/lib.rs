/// This module contains the game itself, it will carry functions to make moves, check who won
/// and be able to choose to play against the computer or against another player.

pub mod game {
    #[derive(PartialEq)]
    pub enum Turn { Player1, Player2, }

    #[derive(PartialEq)]
    pub enum Square {Player1, Player2, Empty, }

    pub struct Hexapawn {
        turn: Turn,
        board: Vec<Vec<Square>>,
    }
    
    impl Hexapawn {
        // Creates a new board with all pieces in the right position and the first move set to Player1 by default
        pub fn new() -> Self {
            Hexapawn { turn: Turn::Player1, board: vec![vec![Square::Player1, Square::Player1, Square::Player1], 
                                                    vec![Square::Empty, Square::Empty, Square::Empty], 
                                                    vec![Square::Player2, Square::Player2, Square::Player2]] }
        }

        pub fn is_winner(self, turn: Turn) -> bool {
            if turn == Turn::Player1 {
                true
            } else {
                false
            }
        }

        // Prints the board based on self.board
        pub fn print_board(self) -> () {
            print!("                  _        _    \n        /\\       |_)      /     \n       /--\\      |_)      \\_    \n");
            print!("     #########");
            match self.board[0][1] {
                Square::Empty => print!("         "),
                _ => print!("    _    "),
            }
            print!("#########\n");
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
                Square::Empty => print!("#########\n"),
                Square::Player1 => print!("###(_)###\n"),
                Square::Player2 => print!("###(:)###\n"),
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
                Square::Empty => print!("#########\n"),
                Square::Player1 => print!("###| |###\n"),
                Square::Player2 => print!("###|:|###\n"),
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
                Square::Empty => print!("#########\n"),
                Square::Player1 => print!("###|_|###\n"),
                Square::Player2 => print!("###|:|###\n"),
            }
            print!("     #########         #########\n");
            match self.board[1][0] {
                Square::Empty => print!("  _           "),
                _ => print!("  _      _    "),
            }
            print!("#########");
            match self.board[1][2] {
                Square::Empty => print!("         \n"),
                _ => print!("    _    \n"),
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
                Square::Empty => print!("         \n"),
                Square::Player1 => print!("   (_)   \n"),
                Square::Player2 => print!("   (:)   \n"),
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
                Square::Empty => print!("         \n"),
                Square::Player1 => print!("   | |   \n"),
                Square::Player2 => print!("   |:|   \n"),
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
                Square::Empty => print!("         \n"),
                Square::Player1 => print!("   |_|   \n"),
                Square::Player2 => print!("   |:|   \n"),
            }
            print!("              #########         \n");
            print!("  _  #########");
            match self.board[2][1] {
                Square::Empty => print!("         "),
                _ => print!("    _    "), 
            }
            print!("#########\n");
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
                Square::Empty => print!("#########\n"),
                Square::Player1 => print!("###(_)###\n"),
                Square::Player2 => print!("###(:)###\n"),
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
                Square::Empty => print!("#########\n"),
                Square::Player1 => print!("###| |###\n"),
                Square::Player2 => print!("###|:|###\n"),
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
                Square::Empty => print!("#########\n"),
                Square::Player1 => print!("###|_|###\n"),
                Square::Player2 => print!("###|:|###\n"),
            }
            print!("     #########         #########\n");
        }
    }
}

/// This will take care of the interface with the user, check if he wants to play against
/// a player or the computer, help, teach the commands, etc.
pub mod interface {

}


