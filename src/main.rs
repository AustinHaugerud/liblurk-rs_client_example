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
    let mock_msgs = VecDeque::from(vec![
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

    let mock_player = Entity {
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

    let mock_room = Room {
        name: String::from("Bob's Room"),
        description: String::from("A room that belongs to Bob."),
        number: 8,
    };

    let mock_connections = vec![
        Room {
            name: String::from("Closet"),
            description: String::from("You keep clothes here."),
            number: 7,
        },
        Room {
            name: String::from("Hallway"),
            description: String::from("Very hallwayish."),
            number: 5,
        }
    ];

    let mock_enemies = vec![
        Entity {
            name: String::from("Grimbo"),
            is_alive: true,
            join_battle: false,
            is_monster: true,
            is_started: false,
            is_ready: false,
            attack: 10,
            defense: 5,
            regen: 1,
            health: 100,
            gold: 38,
            location: 8,
            description: String::from("Grimbo is hungry."),
        },
        Entity {
            name: String::from("Ronald"),
            is_alive: true,
            join_battle: false,
            is_monster: true,
            is_started: false,
            is_ready: false,
            attack: 10,
            defense: 10,
            regen: 2,
            health: 60,
            gold: 34,
            location: 8,
            description: String::from("Ronald has been eating too much fast food."),
        },
        Entity {
            name: String::from("Door Watching Troll"),
            is_alive: true,
            join_battle: false,
            is_monster: true,
            is_started: false,
            is_ready: false,
            attack: 15,
            defense: 10,
            regen: 20,
            health: 200,
            gold: 121,
            location: 8,
            description: String::from("Healthy as a horse, or is a horse as healthy as troll?"),
        },
    ];

    let game_info = Arc::new(Mutex::new(GameInformation {
        messages: mock_msgs,
        player : mock_player,
        current_room : mock_room,
        adjacent_rooms : mock_connections,
        current_enemies : mock_enemies,
        game : Game {
            stat_limit: 100,
            init_points: 100,
            description: String::from("Mock game description."),
        }
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
