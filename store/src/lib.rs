#![feature(const_fn_floating_point_arithmetic)]

use std::collections::{HashMap, HashSet};

use dice::DiceRoll;
use main_city::City;
use rail::Rail;
use rail::C;
use region::Region;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::{rail::RAILROAD_GRAPH, travel_payout::travel_payout};
pub type PlayerId = u64;
pub type Cash = u64;

pub mod dice;
pub mod main_city;
pub mod rail;
pub mod region;
pub mod state;
pub mod sub_city;
pub mod travel_payout;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,
    InGame(InGameStage),
    Ended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InGameStage {
    // DiceRoll(DiceRollStage),
    BankruptcyAuction,
    Movement,
    Purchase,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub enum DiceRollStage {
//     HomeCityRoll,
//     DestinationRoll,
//     MovementRoll,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter)]
pub enum Piece {
    Yellow,
    Green,
    Blue,
    Orange,
    Purple,
    Red,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Piece::Red => write!(f, "Red"),
            Piece::Blue => write!(f, "Blue"),
            Piece::Green => write!(f, "Green"),
            Piece::Yellow => write!(f, "Yellow"),
            Piece::Orange => write!(f, "Orange"),
            Piece::Purple => write!(f, "Purple"),
        }
    }
}

// create a from string method for Piece
impl Piece {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Red" => Some(Piece::Red),
            "Blue" => Some(Piece::Blue),
            "Green" => Some(Piece::Green),
            "Yellow" => Some(Piece::Yellow),
            "Orange" => Some(Piece::Orange),
            "Purple" => Some(Piece::Purple),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum Engine {
    Freight,
    Express,
    SuperChief,
}

impl Engine {
    pub const fn cost(&self) -> u64 {
        match self {
            Engine::Freight => 0,
            Engine::Express => 4000,
            Engine::SuperChief => 40000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub cash: i64,
    pub name: String,
    pub piece: Piece,
    pub home_city: Option<City>,
    pub route_history: Vec<(crate::rail::C, Rail)>,
    pub start: Option<City>, // Default is home-city
    pub destination: Option<City>,
    pub spaces_left_to_move: Option<u8>, // Default is 0
    pub rails: Vec<Rail>,
    pub engine: Engine,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub stage: Stage,
    pub active_player_id: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub player_order: Vec<PlayerId>,
    pub history: Vec<Event>,
    pub rail_ledger: HashMap<Rail, Option<PlayerId>>,
    pub all_roads_bought: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    HomeCityRollRequest {
        player_id: PlayerId,
    },
    DestinationCityRollRequest {
        player_id: PlayerId,
    },
    MovementRollRequest {
        player_id: PlayerId,
    },
    HomeCityRoll {
        player_id: PlayerId,
        region_roll: DiceRoll,
        city_roll: DiceRoll,
        region: Region,
        city: City,
    },
    DestinationCityRoll {
        player_id: PlayerId,
        region_roll: DiceRoll,
        city_roll: DiceRoll,
        region: Region,
        city: City,
    },
    MovementRoll {
        player_id: PlayerId,
        roll: DiceRoll,
    },
    Move {
        player_id: PlayerId,
        route: (C, Rail),
    },
    PurchaseRail {
        player_id: PlayerId,
        rail: Rail,
    },
    PurchaseEngine {
        player_id: PlayerId,
        engine: Engine,
    },
    EndPurchaseStage {
        player_id: PlayerId,
    },
    PlayerJoined {
        player_id: PlayerId,
        name: String,
        color: Piece
    },
}

impl State {
    pub fn consume(&mut self, valid_event: &Event) {
        use Event::*;
        match valid_event {
            Move { player_id, route } => {
                let (city, _) = route;

                self.players.entry(*player_id).and_modify(|player| {
                    // Add the route to the players route history
                    player.route_history.push(*route);

                    // decrease the number of spaces the user has left to move
                    player.spaces_left_to_move = Some(player.spaces_left_to_move.unwrap() - 1);
                });

                let is_last_move =
                    self.players.get(player_id).unwrap().spaces_left_to_move == Some(0);

                // handle payouts
                if is_last_move {
                    self.players.entry(*player_id).and_modify(|player| {
                        player.spaces_left_to_move = None;
                    });

                    // determine which rail-roads the player used along their route
                    let mut unique_rail_roads_on_route: HashSet<Rail> = HashSet::new();
                    for route in &self.players.get(player_id).unwrap().route_history {
                        let (_, rail) = route;
                        unique_rail_roads_on_route.insert(*rail);
                    }

                    for rail_road in unique_rail_roads_on_route.into_iter() {
                        // TODO:
                        // Need to handle grand-fathering
                        // so that if a user was on a rail-road
                        // before a player buys that road they should only pay $1000 to the bank
                        // for that rail-road
                        if let Some(rail_road_owner_id) = self.rail_ledger.get(&rail_road).unwrap()
                        {
                            let mut payout = 5000;
                            if self.all_roads_bought {
                                payout *= 2;
                            }

                            // Pay owner
                            self.players
                                .entry(*rail_road_owner_id)
                                .and_modify(|player| player.cash += payout);

                            // Subtract from player
                            self.players
                                .entry(*player_id)
                                .and_modify(|player| player.cash -= payout);
                        } else {
                            let mut payout = 1000;
                            if self.all_roads_bought {
                                payout *= 2;
                            }

                            // Subtract from player
                            self.players
                                .entry(*player_id)
                                .and_modify(|player| player.cash -= payout);
                        }
                    }
                }

                // Check if the user is at their destination
                if let C::D(main_city) = city {
                    if *main_city == self.players.get(player_id).unwrap().destination.unwrap() {
                        self.players.entry(*player_id).and_modify(|player| {
                            // Pay the player for reaching their destination
                            player.cash +=
                                travel_payout(player.start.unwrap(), player.destination.unwrap())
                                    as i64;

                            // Reset the user's route history
                            player.route_history = vec![];

                            // Set the start of the user's next route
                            player.start = Some(*main_city);

                            // Act as if the user initiated a destination selection dice roll
                            // Will need to think through whether I actually want to auto-roll
                            let (region_roll, region) = DiceRoll::region();
                            let (city_roll, city) = DiceRoll::city_in_region(region);

                            player.destination = Some(city);
                            self.history.push(Event::DestinationCityRoll {
                                player_id: *player_id,
                                region_roll,
                                city_roll,
                                region,
                                city,
                            });

                            // Set the stage to purchasing
                            self.stage = Stage::InGame(InGameStage::Purchase)
                        });
                    }
                }

                // NOTE: Should I also check for a win here
                if is_last_move && self.players.get(player_id).unwrap().cash <= 0 {
                    self.stage = Stage::InGame(InGameStage::BankruptcyAuction);

                    self.advance_turn();
                }

                // Check for Rover
                // Win Check
            }
            HomeCityRollRequest { player_id } => {
                self.history.push(valid_event.clone());

                let (region_roll, region) = DiceRoll::region();
                let (city_roll, city) = DiceRoll::city_in_region(region);

                self.players.get_mut(player_id).unwrap().home_city = Some(city);
                self.history.push(Event::HomeCityRoll {
                    player_id: *player_id,
                    region_roll,
                    city_roll,
                    region,
                    city,
                })
            }
            // DestinationCityRollRequest { player_id } => {
            //     self.history.push(valid_event.clone());

            //     let (region_roll, region) = DiceRoll::region();
            //     let (city_roll, city) = DiceRoll::city_in_region(region);

            //     self.players.get_mut(player_id).unwrap().destination = Some(city);
            //     self.history.push(Event::DestinationCityRoll {
            //         player_id: *player_id,
            //         region_roll,
            //         city_roll,
            //         region,
            //         city,
            //     })
            // }
            MovementRollRequest { player_id } => {
                self.history.push(valid_event.clone());

                let player: &mut Player = self.players.get_mut(player_id).unwrap();
                let roll = DiceRoll::movement_roll(&player.engine);

                player.spaces_left_to_move = Some(roll.sum());

                self.history.push(Event::MovementRoll {
                    player_id: *player_id,
                    roll,
                })
            }
            EndPurchaseStage { .. } => {
                self.advance_turn();
            }
            PlayerJoined { player_id, name, color } => {
                self.players.insert(
                    *player_id,
                    Player {
                        cash: 0,
                        name: name.clone(),
                        piece: color.clone(),
                        home_city: None,
                        route_history: vec![],
                        start: None,
                        destination: None,
                        spaces_left_to_move: None,
                        rails: vec![],
                        engine: Engine::Freight,
                    },
                );

                self.player_order.push(*player_id);
            }
            // TODO: Remove
            _ => {}
        }

        match valid_event {
            HomeCityRollRequest { player_id: _ } => {}
            DestinationCityRollRequest { player_id: _ } => {}
            MovementRollRequest { player_id: _ } => {}
            _ => self.history.push(valid_event.clone()),
        }
    }

    fn advance_turn(&mut self) {
        // Advance stage
        self.stage = Stage::InGame(InGameStage::Movement);

        // Change active player
        for (index, player_id) in self.player_order.iter().enumerate() {
            // find the index of the current player
            if player_id == &self.active_player_id {
                // Check if we need to wrap around to first player
                if index == self.player_order.len() - 1 {
                    self.active_player_id = self.player_order[0];
                } else {
                    self.active_player_id = self.player_order[index + 1];
                }

                break;
            }
        }
    }

    pub fn validate(&self, event: &Event) -> bool {
        use Event::*;
        match event {
            Move { player_id, route } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // Verify that the user has a destination
                let player = self.players.get(player_id).unwrap();
                if player.destination.is_none() {
                    return false;
                }

                // Verify that the user has more moves left
                if player.spaces_left_to_move == Some(0) {
                    return false;
                }

                let (city, _) = route;

                // Verify that the city that is being traveled to can be reached in 1 move from the player's location
                let (current_city, _) = player.route_history.last().unwrap();
                if !RAILROAD_GRAPH
                    .get(current_city)
                    .unwrap()
                    .iter()
                    .any(|(r, _)| r == city)
                {
                    return false;
                }
            }
            // These should only be sent from server to client
            HomeCityRoll {
                player_id: _,
                region_roll: _,
                city_roll: _,
                region: _,
                city: _,
            } => return false,
            DestinationCityRoll {
                player_id: _,
                region_roll: _,
                city_roll: _,
                region: _,
                city: _,
            } => return false,
            MovementRoll {
                player_id: _,
                roll: _,
            } => return false,
            HomeCityRollRequest { player_id } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // Verify that the user doesn't already have a home-city
                if self.players.get(player_id).unwrap().home_city.is_some() {
                    return false;
                }
            }
            DestinationCityRollRequest { player_id } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // Verify that the user doesn't already have a destination
                if self.players.get(player_id).unwrap().destination.is_some() {
                    return false;
                }
            }
            MovementRollRequest { player_id } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // Check that the player isn't in the middle-of-moving
                if self
                    .players
                    .get(player_id)
                    .unwrap()
                    .spaces_left_to_move
                    .is_some()
                {
                    return false;
                }
            }
            PurchaseRail { player_id, rail } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // ensure that it's the purchase stage
                if self.stage != Stage::InGame(InGameStage::Purchase) {
                    return false;
                }

                // ensure that the rail is not owned
                if self.rail_ledger.get(rail).unwrap().is_some() {
                    return false;
                }

                // ensure the player has enough money to purchase it
                if self.players.get(player_id).unwrap().cash < (rail.cost() as i64) {
                    return false;
                }
            }
            PurchaseEngine { player_id, engine } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // ensure that it's the purchase stage
                if self.stage != Stage::InGame(InGameStage::Purchase) {
                    return false;
                }

                // ensure the player has enough money to purchase it
                if self.players.get(player_id).unwrap().cash < (engine.cost() as i64) {
                    return false;
                }

                // a player shouldn't buy an engine they already have
                if self.players.get(player_id).unwrap().engine == *engine {
                    return false;
                }

                // a player shouldn't buy a less-expensive engine then the one they already have
                if self.players.get(player_id).unwrap().engine.cost() >= engine.cost() {
                    return false;
                }
            }
            EndPurchaseStage { player_id } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // ensure that it's the purchase stage
                if self.stage != Stage::InGame(InGameStage::Purchase) {
                    return false;
                }
            }
            PlayerJoined { player_id, name, color } => {
                // Check player doesn't already exist
                if self.players.contains_key(player_id) {
                    return false;
                }

                // Check that no other player has the same color
                if self.players.values().any(|player| player.piece == *color) {
                    return false;
                }
                
                // Check that the player name is unique
                if self.players.values().any(|player| player.name == *name) {
                    return false;
                }
            }
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
            rail_ledger: Rail::iter().map(|rail| (rail, None)).collect(),
            all_roads_bought: false,
        }
    }
}
