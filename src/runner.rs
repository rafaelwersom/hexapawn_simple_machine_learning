use std::io::Write;

use crate::hexapawn::{ interface::*};

pub fn run() {
    let mut input_string = String::new();
    println!("Welcome to Hexapawn");
    print!("Enter a command: ");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input_string).expect("Failed to read line");

    if input_string.eq("help\n") {
        help_main_menu();
        run();
    } else if input_string.eq("exit\n") {
        println!("Thanks for Playing!");
        return;
    } else if input_string.eq("play_friend\n") {
        play_friend();
        run()
    } else if input_string.eq("play_ai\n") {
        play_ai();
        run();
    } else if input_string.eq("clear\n") {
        match clear() {
            Err(_i) => {
                panic!();
            },
            _ => {
                println!("The data collected by the AI opponent has been cleared\n");
            },
        }
        run();
    } else { 
        println!("\nEnter a valid command, use \"help\" for complete list of commands\n");
        run(); 
    }
}