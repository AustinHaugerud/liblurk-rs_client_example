extern crate liblurk;
extern crate termion;
extern crate tui;

mod ui;
mod game;
mod app;

use std::io;
use termion::event;
use termion::input::TermRead;
use std::sync::*;

use game::*;

use std::collections::VecDeque;

fn main() {
    let msgs = VecDeque::from(vec![
        Message {
            sender: String::from("A"),
            receiver: String::new(),
            content: String::from("Hello, world"),
        },
        Message {
            sender: String::from("B"),
            receiver: String::new(),
            content: String::from("World, hello"),
        },
    ]);

    let player = Entity {
        name: String::from("Bob"),
        is_alive: false,
        join_battle: false,
        is_monster: false,
        is_started: false,
        is_ready: false,
        attack: 100,
        defense: 50,
        regen: 75,
        health: 100,
        gold: 50,
        location: 0,
        description: String::from("Description here"),
    };

    let game_info = Arc::new(Mutex::new(GameInformation {
        messages: msgs,
        player,
    }));

    let mut user_interface = ui::TerminalInterface::new(game_info.clone()).unwrap();

    let stdin = io::stdin();

    user_interface.render().expect("Failed to render.");

    for c in stdin.keys() {
        user_interface.render().expect("Failed to render");
        match c {
            Ok(evt) => {
                if evt == event::Key::Char('q') {
                    break;
                }
            }
            Err(_) => {
                eprintln!("Failed to handle key input.");
            }
        }
    }
}
