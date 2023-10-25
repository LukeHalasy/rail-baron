use std::collections::{HashMap, HashSet};

use city::City;
use deed::Deed;
use dice::DiceRoll;
use rail_road::C;
use region::Region;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    dice::Dice,
    payout::{payout, travel_payout},
    rail_road::RAILROAD_GRAPH,
};
type PlayerId = u64;
pub type Cash = u64;

pub mod city;
pub mod deed;
pub mod dice;
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub enum InGameStage {
//     DiceRoll(DiceRollStage),
//     Purchase(PurchaseStage),
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiceRollStage {
    HomeCityRoll,
    DestinationRoll,
    MovementRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PurchaseStage {
    DeedPurchase,
    ExpressPurchase,
    SuperChiefPurchase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Piece {
    Yellow,
    Green,
    Blue,
    Orange,
    Purple,
    Red,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum Train {
    Freight,
    Express,
    SuperChief,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub cash: i64,
    pub name: String,
    pub piece: Piece,
    pub home_city: Option<City>,
    pub route_history: Vec<(crate::rail_road::C, deed::Deed)>,
    pub start: Option<City>, // Default is home-city
    pub destination: Option<City>,
    pub spaces_left_to_move: Option<u8>, // Default is 0
    pub deeds: Vec<Deed>,
    pub train: Train,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub stage: Stage,
    pub active_player_id: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub player_order: Vec<PlayerId>,
    pub history: Vec<Event>,
    pub deed_ledger: HashMap<Deed, Option<PlayerId>>,
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
        route: (C, Deed),
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

                // Check if the user is at their destination
                match city {
                    C::D(main_city) => {
                        if *main_city == self.players.get(player_id).unwrap().destination.unwrap() {
                            self.players.entry(*player_id).and_modify(|player| {
                                // Pay the player for reaching their destination
                                player.cash += travel_payout(
                                    player.start.unwrap(),
                                    player.destination.unwrap(),
                                ) as i64;

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
                                })
                            });
                        }
                    }
                    _ => {}
                }

                // handle payouts
                if self.players.get(player_id).unwrap().spaces_left_to_move == Some(0) {
                    self.players.entry(*player_id).and_modify(|player| {
                        player.spaces_left_to_move = None;
                    });

                    // determine which rail-roads the player used along their route
                    let mut unique_rail_roads_on_route: HashSet<Deed> = HashSet::new();
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
                        if let Some(rail_road_owner_id) = self.deed_ledger.get(&rail_road).unwrap()
                        {
                            let mut payout = 5000;
                            if self.all_roads_bought {
                                payout *= 2;
                            }

                            // Pay owner
                            self.players
                                .entry(*rail_road_owner_id)
                                .and_modify(|player| player.cash = player.cash + payout);

                            // Subtract from player
                            self.players
                                .entry(*player_id)
                                .and_modify(|player| player.cash = player.cash - payout);
                        } else {
                            let payout = 1000;
                            if self.all_roads_bought {
                                payout *= 2;
                            }

                            // Subtract from player
                            self.players
                                .entry(*player_id)
                                .and_modify(|player| player.cash = player.cash - payout);
                        }
                    }
                }
                // NOTE: Should I also check for a win here

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
                let roll = DiceRoll::movement_roll(&player.train);

                player.spaces_left_to_move = Some(roll.sum());

                self.history.push(Event::MovementRoll {
                    player_id: *player_id,
                    roll,
                })
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
            deed_ledger: Deed::iter().map(|deed| (deed, None)).collect(),
            all_roads_bought: false,
        }
    }
}
