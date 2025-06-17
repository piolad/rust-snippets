use std::{io, thread::sleep};
use std::collections::HashMap;
use crossterm::event::{KeyEventKind, KeyModifiers};
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

fn main() {

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
                println!("up!");
                currentMin = result;
                result += (currentMax - result)/2;
            },
            InputResult::Down | InputResult::Right => {
                if result == currentMin {
                    print!("LIAR!");
                }
                println!(" down!");
                currentMax = result;
                result -= (result - currentMin)/2;
            },
            _ => println!("Try again!"),
        }
        if currentMax == currentMin {
            exit_withMessage();
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
                Event::Key(KeyEvent {code: KeyCode::Char('e'), ..}) =>  InputResult::Exit,
                Event::Key(KeyEvent {code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, ..}) =>  InputResult::Exit,
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


fn exit_withMessage() {
    println!("\nExiting the game. Goodbye!");
    std::process::exit(0);
}
