use std::collections::VecDeque;

#[derive(Clone)]
pub struct Entity {
    pub name: String,

    pub is_alive: bool,
    pub join_battle: bool,
    pub is_monster: bool,
    pub is_started: bool,
    pub is_ready: bool,

    pub attack: u16,
    pub defense: u16,
    pub regen: u16,

    pub health: i16,

    pub gold: u16,

    pub location: u16,

    pub description: String,
}

#[derive(Clone)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub content: String,
}

pub struct GameInformation {
    pub messages: VecDeque<Message>,
    pub player: Entity,
}
