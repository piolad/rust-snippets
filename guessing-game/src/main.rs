use crossterm::{
    event::{self, read, Event, KeyCode, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
};

use std::io;


#[derive(Debug)]
enum InputResult {
    Char(char),
    Exit,
    Help,
    InvalidKey,
}

fn get_input() -> io::Result<InputResult> {
    enable_raw_mode()?;
    let result = InputResult::InvalidKey;
    loop {
        let event = event::read()?;
        let result = match  event{
                Event::Key(KeyEvent {code: KeyCode::Char('e'), ..}) =>  InputResult::Exit,
                Event::Key(KeyEvent {code: KeyCode::Char('h'), ..}) =>  InputResult::Help,
                Event::Key(KeyEvent {code: KeyCode::Char(c), ..}) =>  InputResult::Char(c),
                _ => InputResult::InvalidKey,            
        };
        println!("You pressed: {:?}", event);
        println!("\tClassified as: {:?}", result);
        if let InputResult::InvalidKey = result {
            // If the key is invalid, continue to read the next event
            continue;
        }else {
            break;
        }
    }

    disable_raw_mode()?;
    
    Ok(result)
}


fn main() {
    println!("{:?}", get_input().unwrap_or(InputResult::InvalidKey));


    return;



    println!("Hello, lets play a guessing game!");
    println!("-> Press Enter to start");
    println!("-> Type e to exit");
    println!("-> Type h for help");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim() == "e" {
        println!("Exiting the game. Goodbye!");
        return;
    } else if input.trim() == "h" {
        println!("Help: Choose a number between 1 and 10 and try to guess it.");
        return;
    }

    if input.trim().is_empty() {
        println!("Great! Let's begin.");
    } else {
        println!("Invalid input. Please press Enter to start.");
        return;
    }
    

    println!("Choose a number between 1 and 10 but don't tell me!");
    println!("I will try to guess it. You can type 'e' to exit at any time.");
    println!("Press Enter to continue...");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim() == "e" {
        println!("Exiting the game. Goodbye!");
        return;
    }
    println!("Okay, I will start guessing now.");
    // println!



    

    // ask for user input
}
