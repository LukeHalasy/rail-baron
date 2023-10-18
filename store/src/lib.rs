use std::collections::HashMap;

use city::City;
use deed::Deed;
use serde::{Deserialize, Serialize};

type PlayerId = u64;
pub type Cash = u64;

pub mod city;
pub mod deed;
pub mod payout;
pub mod rail_road;
pub mod region;
pub mod state;
pub mod sub_city;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,
    InGame,
    Ended,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Piece {
    Yellow,
    Green,
    Blue,
    White,
    Black,
    Red,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub cash: u64,
    pub name: String,
    pub piece: Piece,
    pub home_city: City,
    pub deeds: Vec<Deed>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub stage: Stage,
    pub active_player_id: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub player_order: Vec<PlayerId>,
    pub history: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    RollDice { player_id: PlayerId },
}

impl State {
    pub fn consume(&self, event: &Event) {
        use Event::*;
        match event {
            RollDice { player_id } => {}
            // TODO: Remove
            _ => {}
        }
    }

    pub fn validate(&self, event: &Event) -> bool {
        use Event::*;
        match event {
            RollDice { player_id } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }
            }
            // TODO: Remove
            _ => {}
        }

        true
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            stage: Stage::PreGame,
            active_player_id: 0,
            players: HashMap::new(),
            player_order: Vec::new(),
            history: Vec::new(),
        }
    }
}
