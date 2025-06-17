use std::{io, thread::sleep};
use crossterm::event::{KeyEventKind, KeyModifiers};
use rand::Rng;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
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

fn main() {
    let mut current_max: u32 = 1000;
    let mut current_min: u32 = 0;
    let mut result: u32 = rand::thread_rng().gen_range(current_min..=current_max);
    
    println!("Hello, lets play a guessing game!");
    println!("-> Press Enter to start");
    println!("-> Type e to exit");
    println!("-> Type h for help");

    loop{
        println!("Current result: {}", result);
        print!("\tYour input: ");
        match get_input().unwrap() {
            InputResult::Exit => exit_with_message(),
            InputResult::Up | InputResult::Right => {
                if result == current_max {
                    exit_with_message();
                }
                println!("up!");
                current_min = result;
                result += (current_max - result)/2;
            },
            InputResult::Down | InputResult::Left => {
                if result == current_min {
                    exit_with_message();
                }
                println!(" down!");
                current_max = result;
                result -= (result - current_min)/2;
            },
            _ => println!("Try again!"),
        }
        if current_max == current_min {
            exit_with_message();
        }
        
        sleep(std::time::Duration::from_millis(250));  // debounce
        
    }
}

fn get_input() -> io::Result<InputResult> {
    enable_raw_mode()?;
    loop {
        let event = event::read()?;
        if let Event::Key(KeyEvent { kind: KeyEventKind::Release, .. }) = event {
            continue;
        }
        // print!("{:?}", event);
        let result = match  event{
                Event::Key(KeyEvent {code: KeyCode::Char('e') | KeyCode::Char('E'), ..}) =>  InputResult::Exit,
                Event::Key(KeyEvent {code: KeyCode::Char('c') | KeyCode::Char('C'), modifiers: KeyModifiers::CONTROL, ..}) =>  InputResult::Exit,
                Event::Key(KeyEvent {code: KeyCode::Char('h') | KeyCode::Char('H'), ..}) =>  InputResult::Help,
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


fn exit_with_message() {
    println!("\nExiting the game. Goodbye!");
    std::process::exit(0);
}
