use std::io;
use std::collections::HashMap;
use rand::Rng;
use crossterm::{
    event::{self, read, Event, KeyCode, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
};



#[derive(Debug, Hash, Eq, PartialEq)]
enum InputResult {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Exit,
    Help,
    InvalidKey,
}

#[derive(Debug)]
enum GameState {
    Start,
    Playing,
    Ended,
}

fn main() {
    let mut default_press_handler: HashMap<InputResult, Box<dyn Fn()> > = HashMap::new();
    let mut gamestate = GameState::Start;
    setup(&mut default_press_handler );

    // if(gamestate == GameState::Start) {
        // print_welcome_message();
    // }

    
    let mut result: u32 = rand::thread_rng().r#gen::<u32>();
    let mut currentMax: u32 = u32::MAX;
    let mut currentMin: u32 = 0;
    
    println!("Hello, lets play a guessing game!");
    println!("-> Press Enter to start");
    println!("-> Type e to exit");
    println!("-> Type h for help");

    loop{
        println!("Current result: {}", result);
        print!("\tYour input: ");
        match get_input().unwrap() {
            InputResult::Exit => exit_withMessage(),
            InputResult::Up | InputResult::Right => {
                if result == currentMin {
                    print!("LIAR!");
                }
                println!("OK! Guessing up!");
                currentMin = result;
                result += (currentMax - result)/2;
            },
            InputResult::Down | InputResult::Right => {
                if result == currentMin {
                    print!("LIAR!");
                }
                println!("OK! Guessing down!");
                currentMax = result;
                result -= (result - currentMin)/2;
            },
            _ => println!("Try again!"),
        }
        if currentMax == currentMin {
            return;
        }


    }

    return;
}

fn get_input() -> io::Result<InputResult> {
    enable_raw_mode()?;
    loop {
        let event = event::read()?;
        let result = match  event{
                Event::Key(KeyEvent {code: KeyCode::Char('e'), ..}) =>  InputResult::Exit,
                Event::Key(KeyEvent {code: KeyCode::Char('h'), ..}) =>  InputResult::Help,
                Event::Key(KeyEvent {code: KeyCode::Char(c), ..}) =>  InputResult::Char(c),
                Event::Key(KeyEvent {code: KeyCode::Up, ..}) => InputResult::Up,
                Event::Key(KeyEvent {code: KeyCode::Down, ..}) => InputResult::Down,
                Event::Key(KeyEvent {code: KeyCode::Left, ..}) => InputResult::Left,
                Event::Key(KeyEvent {code: KeyCode::Right, ..}) => InputResult::Right,
                Event::Key(KeyEvent {code: KeyCode::Esc, ..}) => InputResult::Exit,
                _ => InputResult::InvalidKey,            
        };
        if let InputResult::InvalidKey = result {
            continue;
        }else {
            disable_raw_mode()?;
            return Ok(result);
        }
    }
}

fn process_input() {

}



fn exit_withMessage() {
    println!("\nExiting the game. Goodbye!");
    std::process::exit(0);
}



fn setup(default_press_handler: &mut HashMap<InputResult, Box<dyn Fn()>>) {
    default_press_handler.insert(InputResult::Exit, Box::new(exit_withMessage));
    default_press_handler.insert(InputResult::Help, Box::new(print_help));
    default_press_handler.insert(InputResult::InvalidKey, Box::new( || {
        println!("Invalid key pressed, please try again.");
    }));
    default_press_handler.insert(InputResult::Up, Box::new(|| {
        increase_guess();
    }));
    default_press_handler.insert(InputResult::Down, Box::new(|| {
        decrease_guess();
    }));
    default_press_handler.insert(InputResult::Left, Box::new( || {
        decrease_guess();
    }));
    default_press_handler.insert(InputResult::Right, Box::new( || {
        increase_guess();
    }));
    default_press_handler.insert(InputResult::Char(' '), Box::new( || {
        println!("You pressed the space bar.");
    }));

}



fn increase_guess(){

}
fn decrease_guess(){

}

fn print_help() {
    println!("\nHelp Menu:");
    println!("- Press 'e' or Esc to exit the game.");
    println!("- Press 'h' to display this help menu.");
    println!("- Use arrow keys - up, right - to increase, down, left - to decrease the guess.");
}