extern crate rustbox;
mod dice;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

pub use crate::dice::Die;

pub fn new_game() {
    
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let mut d = Die::new();
    d.throw();
    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Y-Dice Game");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.print(1, 5, rustbox::RB_BOLD, Color::Red, Color::Black, &d.get_val().to_string());
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    Key::Char('n') => { new_game(); }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}