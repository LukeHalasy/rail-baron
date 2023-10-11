use std::collections::HashMap;

use city::City;
use serde::{Deserialize, Serialize};

type PlayerId = u64;
pub type Cash = u64;

pub mod city;
pub mod payout;
pub mod region;

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
    // pub deeds: Vec<Deed>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub stage: Stage,
    pub active_player_id: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub player_order: Vec<PlayerId>,
    // pub history: Vec<GameEvent>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            stage: Stage::PreGame,
            active_player_id: 0,
            players: HashMap::new(),
            player_order: Vec::new(),
            // history: Vec::new(),
        }
    }
}
