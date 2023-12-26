#![feature(const_fn_floating_point_arithmetic)]

use std::collections::{HashMap, HashSet};
use std::vec;

use dice::DiceRoll;
use main_city::City;
use rail::Rail;
use rail::C;
use region::Region;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::{rail::RAILROAD_GRAPH, travel_payout::travel_payout};
use std::str::FromStr;
pub type PlayerId = u64;

pub type Cash = u64;

pub mod computer;
pub mod dice;
pub mod main_city;
pub mod rail;
pub mod region;
pub mod state;
pub mod sub_city;
pub mod travel_payout;

pub type GameId = u64;

#[derive(Deserialize, Serialize, Debug)]
pub enum ServerMessage {
    Event(Event),
    Error(String),
    Connection(PlayerId),
    GameCreated(GameId),
    GameJoined(GameId),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ClientMessage {
    Event(Event),
    JoinGame(GameId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,
    InGame(InGameStage),
    Ended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InGameStage {
    // DiceRoll(DiceRollStage),
    OrderRoll,
    HomeRoll,
    DestinationRoll,
    MovementRoll,
    BankruptcyHandling, // When the user is deciding what to sell, and whether to auction or sell to the bank
    BankruptcyAuction,
    DeclareOption,
    Movement,
    Purchase,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub enum DiceRollStage {
//     HomeCityRoll,
//     DestinationRoll,
//     MovementRoll,
// }

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter)]
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
impl FromStr for Piece {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Red" => Ok(Piece::Red),
            "Blue" => Ok(Piece::Blue),
            "Green" => Ok(Piece::Green),
            "Yellow" => Ok(Piece::Yellow),
            "Orange" => Ok(Piece::Orange),
            "Purple" => Ok(Piece::Purple),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, EnumIter)]
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
    pub name: Option<String>, // Will be None before the user selects a name
    pub piece: Option<Piece>, // Will be None before the user selects a piece
    pub home_city: Option<City>,
    pub route: Vec<(crate::rail::C, Rail)>,
    pub route_history: Vec<Vec<(crate::rail::C, Rail)>>,
    pub grand_fathered_rail: Option<Rail>,
    pub start: Option<City>, // Default is home-city
    pub destination: Option<City>,
    pub spaces_left_to_move: Option<u8>, // Default is 0
    pub going_home: bool,
    pub engine: Engine,
    pub bankrupt: bool,
    pub computer: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            cash: 20000,
            name: None,
            piece: None,
            home_city: None,
            route: vec![],
            route_history: vec![],
            grand_fathered_rail: None,
            start: None,
            destination: None,
            spaces_left_to_move: None,
            going_home: false,
            engine: Engine::Freight,
            bankrupt: false,
            computer: false,
        }
    }
}

impl Player {
    pub fn current_city(&self) -> Option<C> {
        let mut current_city = None;
        if let Some(city) = self.route.last() {
            current_city = Some(city.0);
        } else if let Some(last_route) = self.route_history.last() {
            current_city = Some(last_route.last().unwrap().0);
        } else if let Some(home_city) = self.home_city {
            current_city = Some(C::D(home_city));
        }

        current_city
    }

    pub fn move_will_repeat_dot_pattern(
        &self,
        start_city: C,
        current_city: C,
        proposed_city: C,
        route: Vec<C>,
    ) -> bool {
        // invariant to test: if route not empty then current_city = last city in route else start city
        assert_ne!(current_city, proposed_city);

        if route.is_empty() {
            return false;
        }

        if proposed_city
            == C::D(
                self.destination
                    .expect("Expected the player to have a destination"),
            )
        {
            return false;
        }

        if route.len() == 1 {
            return start_city == proposed_city;
        }

        for (index, route) in route.iter().enumerate() {
            let city = route;

            if *city == proposed_city {
                if index == 0 && start_city == current_city {
                    return true;
                }

                // check previous city
                if index != 0 && self.route[index - 1].0 == current_city {
                    return true;
                }

                // check next city
                if (index != self.route.len() - 1) && self.route[index + 1].0 == current_city {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub stage: Stage,
    pub active_player_id: Option<PlayerId>,
    pub game_host: Option<PlayerId>,
    pub players: HashMap<PlayerId, Player>,
    pub player_order: Vec<PlayerId>,
    pub history: Vec<Event>,
    pub rail_ledger: HashMap<Rail, Option<PlayerId>>,
    pub winner: Option<PlayerId>,
    pub all_roads_bought: bool,
    pub declare_amount: Cash,
    pub rover_reward: Cash,
    pub auction_bid_increment: Cash, // Todo: Validate that each bid is increasing by this amount exactly
    pub auction_state: Option<AuctionState>,
}

// TODO: Need to ensure we won't get trapped in the auction state
// If a user doesn't have any more rails to sell then we should move on to the next player.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuctionState {
    pub player_id: PlayerId, // The player that is currently auctioning
    pub rail: Rail,
    pub current_bid: Cash, // default bidder is the bank for the cost of the rail / 2
    pub current_bidder: Option<PlayerId>,
    pub auction_history: Vec<(PlayerId, Cash)>,
    pub dropped_out_bidders: HashSet<PlayerId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    // TODO: Add events for bankruptcy auction events
    // Lobby based events
    Create {
        player_id: PlayerId,
    },
    Start {
        player_id: PlayerId,
    },
    SetPlayerAttributes {
        player_id: PlayerId,
        name: String,
        piece: Piece,
    },
    // In-game events
    OrderRollRequest {
        player_id: PlayerId,
    },
    HomeRollRequest {
        player_id: PlayerId,
    },
    DestinationRollRequest {
        player_id: PlayerId,
    },
    MovementRollRequest {
        player_id: PlayerId,
    },
    OrderRoll {
        player_id: PlayerId,
        roll: DiceRoll,
    },
    HomeRoll {
        player_id: PlayerId,
        region_roll: DiceRoll,
        city_roll: DiceRoll,
        region: Region,
        city: City,
    },
    DestinationRoll {
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
    DeclareChoice {
        player_id: PlayerId,
        declare: bool,
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
    },
    // Bankrupcty Events
    SellRailToBank {
        player_id: PlayerId,
        rail: Rail,
    },
    AuctionRail {
        player_id: PlayerId,
        rail: Rail,
    },
    Bid {
        player_id: PlayerId,
        bid: Cash,
    },
    // TODO: have the server send out this message automatically when a bid stays the same for too long
    StopBid {
        player_id: PlayerId,
    },
    EndBankruptcyHandling {
        player_id: PlayerId,
    },
    ChangeStage {
        stage: Stage,
        line: u32,
    }
}

impl State {
    pub fn consume(&mut self, valid_event: &Event) {
        use Event::*;

        match valid_event {
            Create { player_id } | PlayerJoined { player_id } => {
                if let Create { player_id } = valid_event {
                    self.game_host = Some(*player_id);
                }

                self.players.insert(
                    *player_id,
                    Player {
                        ..Player::default()
                    },
                );

                self.player_order.push(*player_id);
            }
            Start { player_id: _ } => {
                self.consume(&Event::ChangeStage {
                    stage: Stage::InGame(InGameStage::OrderRoll),
                    line: line!(),
                });
            }
            SetPlayerAttributes {
                player_id,
                name,
                piece,
            } => {
                self.players.entry(*player_id).and_modify(|player| {
                    player.name = Some(name.clone());
                    player.piece = Some(*piece);
                });
            }
            Move { player_id, route } => {
                /*
                TODO: Handle bouncing out of a city.
                If the player makes it to their destination, and they haven't consumed ANY of their red dice roll,
                then they are entitled to use their bonus roll to proceed towards their destination.
                The player loses any extra movement from their white dice
                */

                let (city, rail) = route;

                self.players.entry(*player_id).and_modify(|player| {
                    if let Some(grand_fathered_rail) = player.grand_fathered_rail {
                        if grand_fathered_rail != *rail {
                            player.grand_fathered_rail = None;
                        }
                    }

                    // Add the route to the players route history
                    player.route.push(*route);

                    // decrease the number of spaces the user has left to move
                    player.spaces_left_to_move = Some(
                        player
                            .spaces_left_to_move
                            .expect("Expected the moving player to have spaces_left_to_move")
                            - 1,
                    );
                });

                // check if anyone else is getting rovered by this move
                let rovered_players: Vec<PlayerId> = self
                    .players
                    .iter()
                    .filter(|(id, player)| {
                        **id != *player_id
                            && player.current_city() == Some(*city)
                            && player.going_home
                    })
                    .map(|(player_id, _)| *player_id)
                    .collect();

                for rovered_player_id in rovered_players {
                    // grant the roverer the rover reward for each player that is rovered
                    self.players.entry(*player_id).and_modify(|player| {
                        player.cash += self.rover_reward as i64;
                    });

                    self.players.entry(rovered_player_id).and_modify(|player| {
                        // subtract the rover reward from the rovered player as a penalty
                        player.cash -= self.rover_reward as i64;

                        // the rovered player is no longer going home
                        player.going_home = false;
                    });
                }

                let is_last_move =
                    self.players.get(player_id).unwrap().spaces_left_to_move == Some(0);

                let at_destination = matches!(city, C::D(c) if *c == self.players.get(player_id).unwrap().destination.unwrap());
                let at_home = matches!(city, C::D(c) if *c == self.players.get(player_id).unwrap().home_city.unwrap());

                // handle payouts
                if is_last_move || at_destination {
                    self.players.entry(*player_id).and_modify(|player| {
                        player.spaces_left_to_move = None;
                    });

                    // determine which rail-roads the player used along their route
                    let mut unique_rail_roads_on_route: HashSet<Rail> = HashSet::new();
                    for route in &self.players.get(player_id).unwrap().route {
                        let (_, rail) = route;
                        unique_rail_roads_on_route.insert(*rail);
                    }

                    for rail_road in unique_rail_roads_on_route.into_iter() {
                        /*
                        TODO: Need to handle grand-fathering
                        so that if a user was on a rail-road
                        before a player buys that road they should only pay $1000 to the bank
                        for that rail-road
                        */
                        let rail_road_owner = self.rail_ledger.get(&rail_road).unwrap();
                        let grand_fathered_rail =
                            self.players.get(player_id).unwrap().grand_fathered_rail;

                        if rail_road_owner.is_none()
                            || rail_road_owner.unwrap() == *player_id
                            || (grand_fathered_rail.is_some()
                                && rail_road == grand_fathered_rail.unwrap())
                        {
                            let mut payout = 1000;
                            if self.all_roads_bought {
                                /*
                                TODO: Ensure that when all rails are bought, but someone sells a road back to the bank,
                                that self.all_roads_bought is set to false

                                */
                                // TODO: Add test to ensure that this block only happens if all roads are bought and the player is using their own rail-road
                                payout *= 2;
                            }

                            // Subtract from player
                            self.players
                                .entry(*player_id)
                                .and_modify(|player| player.cash -= payout);
                        } else {
                            let mut payout = 5000;
                            if self.all_roads_bought {
                                payout *= 2;
                            }

                            // Pay owner
                            self.players
                                .entry(rail_road_owner.unwrap())
                                .and_modify(|player| player.cash += payout);

                            // Subtract from player
                            self.players
                                .entry(*player_id)
                                .and_modify(|player| player.cash -= payout);
                        }
                    }
                }

                // Check if the user is at their destination
                if at_destination {
                    self.players.entry(*player_id).and_modify(|player| {
                        // Pay the player for reaching their destination
                        player.cash +=
                            travel_payout(player.start.unwrap(), player.destination.unwrap())
                                as i64;

                        player.route_history.push(player.route.clone());
                        player.route = vec![];
                        player.start = player.destination;
                        player.spaces_left_to_move = None;
                        player.destination = None;
                    });

                    self.consume(&Event::ChangeStage {
                        line: line!(),
                        stage: Stage::InGame(InGameStage::DestinationRoll),
                    });

                    if self.players.get(player_id).unwrap().cash >= self.declare_amount as i64 {
                        self.consume(&Event::ChangeStage {
                            line: line!(),
                            stage: Stage::InGame(InGameStage::DeclareOption),
                        });
                    } else {
                        self.consume(&Event::ChangeStage {
                            line: line!(),
                            stage: Stage::InGame(InGameStage::DestinationRoll),
                        });
                    }
                }

                // if the player is declared and they drop below the declare amount then they are no longer declared
                let player = self.players.get(player_id).unwrap();
                if player.going_home && player.cash < self.declare_amount as i64 {
                    self.players.get_mut(player_id).unwrap().going_home = false;
                }

                if at_home {
                    if let Some(player) = self.players.get(player_id) {
                        if player.cash >= self.declare_amount as i64 && player.going_home {
                            self.consume(&Event::ChangeStage {
                                line: line!(),
                                stage: Stage::Ended,
                            });
                            self.winner = Some(*player_id);
                        }
                    }
                }

                if is_last_move {
                    if self.players.get(player_id).unwrap().cash <= 0 {
                        if self
                            .rail_ledger
                            .iter()
                            .all(|(_, owner)| *owner != Some(*player_id))
                        {
                            self.players.get_mut(player_id).unwrap().bankrupt = true;

                            // remove the player from the player order
                            self.advance_turn();
                            self.player_order.retain(|id| id != player_id);

                            self.consume(&Event::ChangeStage {
                                line: line!(),
                                stage: Stage::InGame(InGameStage::MovementRoll),
                            });
                        } else {
                            self.consume(&Event::ChangeStage {
                                line: line!(),
                                stage: Stage::InGame(InGameStage::BankruptcyHandling),
                            });
                        }
                    } else if self.stage != Stage::InGame(InGameStage::DestinationRoll) {
                        self.consume(&Event::ChangeStage {
                            line: line!(),
                            stage: Stage::InGame(InGameStage::MovementRoll),
                        });
                        self.advance_turn()
                    }

                    // check if the game is over
                    if self.player_order.len() == 1 {
                        self.stage = Stage::Ended;
                        self.winner = Some(self.active_player_id.unwrap());
                    }
                }
            }
            HomeRollRequest { player_id } => {
                self.history.push(valid_event.clone());

                let (region_roll, region) = DiceRoll::region();
                let (city_roll, city) = DiceRoll::city_in_region(region);

                self.players.get_mut(player_id).unwrap().home_city = Some(city);
                self.players.get_mut(player_id).unwrap().start = Some(city);

                self.history.push(Event::HomeRoll {
                    player_id: *player_id,
                    region_roll,
                    city_roll,
                    region,
                    city,
                });

                // if all player's have rolled for their home city then change the stage
                if self
                    .history
                    .iter()
                    .filter(|event| matches!(event, Event::HomeRoll { .. }))
                    .count()
                    == self.players.len()
                {
                    self.consume(&Event::ChangeStage {
                        line: line!(),
                        stage: Stage::InGame(InGameStage::DestinationRoll),
                    });
                }

                // move to the next player
                self.advance_turn();
            }
            OrderRollRequest { player_id } => {
                self.history.push(valid_event.clone());

                let roll = DiceRoll::red_and_white();

                self.history.push(Event::OrderRoll {
                    player_id: *player_id,
                    roll,
                });

                // TODO: Handle Ties
                // if all rolls are in
                if self
                    .history
                    .iter()
                    .filter(|event| matches!(event, Event::OrderRoll { .. }))
                    .count()
                    == self.players.len()
                {
                    // sort the players by their roll
                    let mut players_sorted_by_roll: Vec<_> = self.players.iter().collect();
                    players_sorted_by_roll.sort_by(|(a_id, _a), (b_id, _b)| {
                        let a_roll = self
                            .history
                            .iter()
                            .find_map(|event| match event {
                                Event::OrderRoll {
                                    player_id: event_player_id,
                                    roll,
                                } if event_player_id == *a_id => Some(roll),
                                _ => None,
                            })
                            .unwrap();
                        let b_roll = self
                            .history
                            .iter()
                            .find_map(|event| match event {
                                Event::OrderRoll {
                                    player_id: event_player_id,
                                    roll,
                                } if event_player_id == *b_id => Some(roll),
                                _ => None,
                            })
                            .unwrap();

                        b_roll.sum().cmp(&a_roll.sum())
                    });

                    // set the player order
                    self.player_order = players_sorted_by_roll.iter().map(|(id, _)| **id).collect();

                    // set the active player
                    self.active_player_id = Some(self.player_order[0]);

                    // set the stage to home roll
                    self.consume(&Event::ChangeStage {
                        line: line!(),
                        stage: Stage::InGame(InGameStage::HomeRoll),
                    });
                }
            }
            DestinationRollRequest { player_id } => {
                self.history.push(valid_event.clone());

                let (region_roll, region) = DiceRoll::region();
                let mut city_roll;
                let mut city;
                while {
                    let roll = DiceRoll::city_in_region(region);
                    city_roll = roll.0;
                    city = roll.1;
                    city == self.players.get(player_id).unwrap().start.unwrap()
                } {}

                self.history.push(Event::DestinationRoll {
                    player_id: *player_id,
                    region_roll,
                    city_roll,
                    region,
                    city,
                });

                if !self.players.get(player_id).unwrap().route_history.is_empty() {
                    self.consume(&Event::ChangeStage {
                        line: line!(),
                        stage: Stage::InGame(InGameStage::Purchase),
                    });
                } else {
                    // otherwise,
                    // must be rolling for first destinations
                    // check if all players have rolled for their destination
                    if self
                        .history
                        .iter()
                        .filter(|event| matches!(event, Event::DestinationRoll { .. }))
                        .count()
                        == self.players.len()
                    {
                        self.consume(&Event::ChangeStage {
                            line: line!(),
                            stage: Stage::InGame(InGameStage::MovementRoll),
                        });
                    }
                    self.advance_turn();
                }

                self.players.get_mut(player_id).unwrap().destination = Some(city);
            }
            MovementRollRequest { player_id } => {
                self.history.push(valid_event.clone());

                let player: &mut Player = self.players.get_mut(player_id).unwrap();
                let roll = DiceRoll::movement_roll(&player.engine);

                player.spaces_left_to_move = Some(roll.sum());

                self.history.push(Event::MovementRoll {
                    player_id: *player_id,
                    roll,
                });

                self.consume(&Event::ChangeStage {
                    line: line!(),
                    stage: Stage::InGame(InGameStage::Movement),
                });
            }
            PurchaseEngine { player_id, engine } => {
                let player: &mut Player = self.players.get_mut(player_id).unwrap();

                player.engine = engine.clone();
                player.cash -= engine.cost() as i64;
            }
            PurchaseRail { player_id, rail } => {
                let player: &mut Player = self.players.get_mut(player_id).unwrap();

                player.cash -= rail.cost() as i64;

                self.rail_ledger.insert(*rail, Some(*player_id));

                if self.rail_ledger.iter().all(|(_, owner)| owner.is_some()) {
                    self.all_roads_bought = true;
                }

                // mark any players that are currently on the rail as grand-fathered
                for (_, player) in self.players.iter_mut().filter(|(id, _)| *id != player_id) {
                    // if the most recent rail is the one that was just bought then mark it as grand-fathered
                    let last_rail = player.route.last().map(|(_, rail)| rail).or_else(|| {
                        player
                            .route_history
                            .last()
                            .and_then(|last_route| last_route.last().map(|(_, rail)| rail))
                    });

                    if last_rail.is_some() && last_rail.unwrap() == rail {
                        player.grand_fathered_rail = Some(*rail);
                    }
                }
            }
            EndPurchaseStage { player_id } => {
                let player = self.players.get(player_id).unwrap();
                if player.cash >= self.declare_amount as i64 {
                    self.consume(&Event::ChangeStage {
                        line: line!(),
                        stage: Stage::InGame(InGameStage::DeclareOption),
                    });
                } else {
                    self.consume(&Event::ChangeStage {
                        line: line!(),
                        stage: Stage::InGame(InGameStage::MovementRoll),
                    });
                    self.advance_turn();
                }
            }
            SellRailToBank { player_id, rail } => {
                let player: &mut Player = self.players.get_mut(player_id).unwrap();

                // TODO: Don't hardcode the bank sell price here.
                player.cash += (rail.cost() / 2) as i64;

                self.rail_ledger.insert(*rail, None);

                // if the player has no more rails to sell and they are still bankrupt then they have lost the game
                if self
                    .rail_ledger
                    .iter()
                    .all(|(_, owner)| *owner != Some(*player_id))
                {
                    // TODO: Determine if we really should autoend the turn or require the user to manually end the turn
                    if player.cash <= 0 {
                        player.bankrupt = true;
                        self.advance_turn();
                        self.player_order.retain(|id| id != player_id);
                    } else if player.destination.is_none() {
                        self.consume(&Event::ChangeStage {
                            line: line!(),
                            stage: Stage::InGame(InGameStage::DestinationRoll),
                        });
                    } else {
                        self.advance_turn();
                        self.consume(&Event::ChangeStage {
                            line: line!(),
                            stage: Stage::InGame(InGameStage::MovementRoll),
                        });
                    }

                    // check if the game is over
                    if self.player_order.len() == 1 {
                        self.consume(&Event::ChangeStage {
                            line: line!(),
                            stage: Stage::Ended,
                        });
                        self.winner = Some(self.active_player_id.unwrap());
                    }
                }

                // if
            }
            EndBankruptcyHandling { player_id } => {
                let player = self.players.get_mut(player_id).unwrap();

                if player.cash <= 0 {
                    player.bankrupt = true;
                    self.advance_turn();
                    self.player_order.retain(|id| id != player_id);
                } else if player.destination.is_none() {
                    self.consume(&Event::ChangeStage {
                        line: line!(),
                        stage: Stage::InGame(InGameStage::DestinationRoll),
                    });
                } else {
                    self.advance_turn();
                    self.consume(&Event::ChangeStage {
                        line: line!(),
                        stage: Stage::InGame(InGameStage::MovementRoll),
                    });
                }
            }
            AuctionRail { player_id, rail } => {
                self.auction_state = Some(AuctionState {
                    player_id: *player_id,
                    rail: *rail,
                    // TODO: Don't hardcode the bank sell price here.
                    current_bid: rail.cost() / 2,
                    current_bidder: None,
                    auction_history: vec![],
                    dropped_out_bidders: HashSet::new(),
                });
            }
            Bid { player_id, bid } => {
                let auction_state = self.auction_state.as_mut().unwrap();

                auction_state.current_bid = *bid;
                auction_state.current_bidder = Some(*player_id);
                auction_state.auction_history.push((*player_id, *bid));
            }
            StopBid { player_id } => {
                let auction_state = self.auction_state.as_mut().unwrap();

                auction_state.dropped_out_bidders.insert(*player_id);

                // if all players have dropped out
                if auction_state.dropped_out_bidders.len() == self.players.len() - 1 {
                    let player = self.players.get_mut(&auction_state.player_id).unwrap();

                    // if the bank is the current bidder
                    if auction_state.current_bidder.is_none() {
                        // sell the rail to the bank
                        self.rail_ledger.insert(auction_state.rail, None);
                    } else {
                        // sell the rail to the current bidder
                        self.rail_ledger
                            .insert(auction_state.rail, auction_state.current_bidder);

                        // subtract the bid amount from the player
                        player.cash -= auction_state.current_bid as i64;

                        if player.going_home && player.cash < self.declare_amount as i64 {
                            player.going_home = false;
                        }
                    }

                    // grant the player the sell amount
                    player.cash += auction_state.current_bid as i64;

                    // reset the auction state
                    self.auction_state = None;

                    if player.cash >= 0
                        || self
                            .rail_ledger
                            .iter()
                            .all(|(_, owner)| *owner != Some(*player_id))
                    {
                        player.bankrupt = true;
                        // remove the player from the player order
                        self.advance_turn();
                        self.player_order.retain(|id| id != player_id);

                        self.consume(&Event::ChangeStage {
                            line: line!(),
                            stage: Stage::InGame(InGameStage::MovementRoll),
                        });

                        // check if the game is over
                        if self.player_order.len() == 1 {
                            self.stage = Stage::Ended;
                            self.winner = Some(self.active_player_id.unwrap());
                        }
                    } else {
                        self.consume(&ChangeStage {
                            line: line!(),
                            stage: Stage::InGame(InGameStage::BankruptcyHandling),
                        })
                    }
                }
            }
            DeclareChoice { player_id, declare } => {
                if *declare {
                    self.players.get_mut(player_id).unwrap().going_home = true;
                }

                self.consume(&Event::ChangeStage {
                    line: line!(),
                    stage: Stage::InGame(InGameStage::MovementRoll),
                });
                self.advance_turn();
            }
            // TODO: Remove to ensure all events are handled
            ChangeStage { line: _, stage } => {
                self.stage = *stage;
            }
            _ => {}
        }

        match valid_event {
            OrderRollRequest { player_id: _ } => {}
            HomeRollRequest { player_id: _ } => {}
            DestinationRollRequest { player_id: _ } => {}
            MovementRollRequest { player_id: _ } => {}
            _ => self.history.push(valid_event.clone()),
        }
    }

    fn advance_turn(&mut self) {
        for (index, player_id) in self.player_order.iter().enumerate() {
            if player_id == &self.active_player_id.expect("Active player should exist") {
                if index == self.player_order.len() - 1 {
                    self.active_player_id = Some(self.player_order[0]);
                } else {
                    self.active_player_id = Some(self.player_order[index + 1]);
                }

                break;
            }
        }
    }

    pub fn validate(&self, event: &Event) -> Result<(), String> {
        use Event::*;
        match event {
            Create { player_id } => {
                // Check that the current state is equal to the default state
                if *self != State::default() {
                    return Err("Game already exists".to_string());
                }

                // Check player doesn't already exist
                if self.players.contains_key(player_id) {
                    return Err("Player already exists".to_string());
                }
            }
            Start { player_id } => {
                // Check that the game hasn't already started
                if self.stage != Stage::PreGame {
                    return Err("Game has already started".to_string());
                }

                // Check that the calling player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // Check that the player is the host
                if self.game_host != Some(*player_id) {
                    return Err("Player is not the host".to_string());
                }

                // Check that the game has at least 2 players
                if self.players.len() < 2 {
                    return Err("Game does not have enough players (2)".to_string());
                }

                // Check that all players have a name
                if self.players.iter().any(|(_, player)| player.name.is_none()) {
                    let players_without_name: Vec<_> = self
                        .players
                        .iter()
                        .filter(|(_, player)| player.name.is_none())
                        .map(|(id, _)| id)
                        .collect();

                    if !players_without_name.is_empty() {
                        let mut players_without_name_sorted = players_without_name.clone();
                        players_without_name_sorted.sort();

                        let players_without_name_str = players_without_name_sorted
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        return Err(format!(
                            "Not all players have a name. Players without a name: {}",
                            players_without_name_str
                        ));
                    }
                }

                // Check that all players have a piece
                if self
                    .players
                    .iter()
                    .any(|(_, player)| player.piece.is_none())
                {
                    let players_without_piece: Vec<_> = self
                        .players
                        .iter()
                        .filter(|(_, player)| player.piece.is_none())
                        .map(|(id, _)| id)
                        .collect();

                    if !players_without_piece.is_empty() {
                        let players_without_piece_str = players_without_piece
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        return Err(format!(
                            "Not all players have a piece. Players without a piece: {}",
                            players_without_piece_str
                        ));
                    }
                }
            }
            SetPlayerAttributes {
                player_id,
                name,
                piece,
            } => {
                // check that we are in the pre-game stage
                if self.stage != Stage::PreGame {
                    return Err("Game has already started".to_string());
                }

                // check that the player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // check that the piece is not already taken
                if self
                    .players
                    .iter()
                    .any(|(_, player)| player.piece == Some(*piece))
                {
                    return Err("Piece is already taken".to_string());
                }

                // check that the name is not already taken
                if self
                    .players
                    .iter()
                    .any(|(_, player)| player.name == Some(name.clone()))
                {
                    return Err("Name is already taken".to_string());
                }

                // ensure the name is not blank
                if name.is_empty() {
                    return Err("Name cannot be blank".to_string());
                }
            }
            Move { player_id, route } => {
                // Verify the stage is movement
                if self.stage != Stage::InGame(InGameStage::Movement) {
                    return Err("It is not the movement stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }
                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // Verify that the user has a destination
                let player = self.players.get(player_id).unwrap();
                if player.destination.is_none() {
                    return Err("Player does not have a destination".to_string());
                }

                // Verify that the user has more moves left
                if player.spaces_left_to_move.is_none() {
                    return Err("Player has no more moves left".to_string());
                }

                let (city, _) = route;

                // Verify that the city that is being traveled to can be reached in 1 move from the player's location
                let current_city = player.current_city();
                if !RAILROAD_GRAPH.contains_edge(<C>::into(current_city.unwrap()), <C>::into(*city))
                {
                    return Err("City cannot be reached in one move".to_string());
                }

                // Verify that they have not already traveled this dot pattern before
                // Verify that player.current_city() and city do not appear next to each other in the player's route in any order
                if player.move_will_repeat_dot_pattern(
                    C::D(player.start.unwrap()),
                    current_city.unwrap(),
                    *city,
                    player.route.iter().map(|(c, _)| *c).collect(),
                ) {
                    return Err("Player has already traveled between these two cities".to_string());
                }
            }
            // These should only be sent from server to client
            HomeRoll {
                player_id: _,
                region_roll: _,
                city_roll: _,
                region: _,
                city: _,
            } => return Err("HomeRoll should only be sent from server to client".to_string()),
            DestinationRoll {
                player_id: _,
                region_roll: _,
                city_roll: _,
                region: _,
                city: _,
            } => {
                return Err("DestinationRoll should only be sent from server to client".to_string())
            }
            MovementRoll {
                player_id: _,
                roll: _,
            } => return Err("MovementRoll should only be sent from server to client".to_string()),
            OrderRoll {
                player_id: _,
                roll: _,
            } => return Err("OrderRoll should only be sent from server to client".to_string()),
            OrderRollRequest { player_id } => {
                // check that we are in the order roll stage
                if self.stage != Stage::InGame(InGameStage::OrderRoll) {
                    return Err("It is not the order roll stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }
            }
            HomeRollRequest { player_id } => {
                // verify that is the home roll stage
                if self.stage != Stage::InGame(InGameStage::HomeRoll) {
                    return Err("It is not the home roll stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // Verify that the user doesn't already have a home-city
                if self.players.get(player_id).unwrap().home_city.is_some() {
                    return Err("Player already has a home city".to_string());
                }
            }
            DestinationRollRequest { player_id } => {
                if self.stage != Stage::InGame(InGameStage::DestinationRoll) {
                    return Err("It is not the destination roll stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }
                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // Verify that the user doesn't already have a destination
                if self.players.get(player_id).unwrap().destination.is_some() {
                    return Err("Player already has a destination".to_string());
                }
            }
            MovementRollRequest { player_id } => {
                if self.stage != Stage::InGame(InGameStage::MovementRoll) {
                    return Err("It is not the movement roll stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }
                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // Check that the player isn't in the middle-of-moving
                if self
                    .players
                    .get(player_id)
                    .unwrap()
                    .spaces_left_to_move
                    .is_some()
                {
                    return Err("Player is in the middle of moving".to_string());
                }
            }
            PurchaseRail { player_id, rail } => {
                // ensure that it's the purchase stage
                if self.stage != Stage::InGame(InGameStage::Purchase) {
                    return Err("It is not the purchase stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }
                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // ensure that the rail is not owned
                if self.rail_ledger.get(rail).unwrap().is_some() {
                    return Err("Rail is already owned".to_string());
                }

                // ensure the player has enough money to purchase it
                if self.players.get(player_id).unwrap().cash < (rail.cost() as i64) {
                    return Err("Player does not have enough money to purchase rail".to_string());
                }
            }
            PurchaseEngine { player_id, engine } => {
                // ensure that it's the purchase stage
                if self.stage != Stage::InGame(InGameStage::Purchase) {
                    return Err("It is not the purchase stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }
                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // ensure the player has enough money to purchase it
                if self.players.get(player_id).unwrap().cash < (engine.cost() as i64) {
                    return Err("Player does not have enough money to purchase engine".to_string());
                }

                // a player shouldn't buy an engine they already have
                if self.players.get(player_id).unwrap().engine == *engine {
                    return Err("Player already has this engine".to_string());
                }

                // a player shouldn't buy a less-expensive engine then the one they already have
                if self.players.get(player_id).unwrap().engine.cost() >= engine.cost() {
                    return Err("Player already has a more expensive engine".to_string());
                }
            }
            EndPurchaseStage { player_id } => {
                // ensure that it's the purchase stage
                if self.stage != Stage::InGame(InGameStage::Purchase) {
                    return Err("It is not the purchase stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }
                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }
            }

            DeclareChoice {
                player_id,
                declare: _,
            } => {
                // ensure that it's the declare option stage
                if self.stage != Stage::InGame(InGameStage::DeclareOption) {
                    return Err("It is not the declare option stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }
            }
            PlayerJoined { player_id } => {
                // Check player doesn't already exist
                if self.players.contains_key(player_id) {
                    return Err("Player already exists".to_string());
                }
            }
            SellRailToBank { player_id, rail } => {
                // ensure that it's the BankruptcyHandling stage
                if self.stage != Stage::InGame(InGameStage::BankruptcyHandling) {
                    return Err("It is not the BankruptcyHandling stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // Check that the player owns the rail
                if *self.rail_ledger.get(rail).unwrap() != Some(*player_id) {
                    return Err("Player does not own the rail".to_string());
                }
            }
            AuctionRail { player_id, rail } => {
                // ensure that it's the BankruptcyHandling stage
                if self.stage != Stage::InGame(InGameStage::BankruptcyHandling) {
                    return Err("It is not the BankruptcyHandling stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // Check player is currently the one making their move
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // Check that the player owns the rail
                if *self.rail_ledger.get(rail).unwrap() != Some(*player_id) {
                    return Err("Player does not own the rail".to_string());
                }
            }
            Bid { player_id, bid } => {
                if self.stage != Stage::InGame(InGameStage::BankruptcyAuction) {
                    return Err("It is not the BankruptcyAuction stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // Check that the player is not the auctioneer
                if self.auction_state.as_ref().unwrap().player_id == *player_id {
                    return Err("Player can't bid in their own auction".to_string());
                }

                // Ensure the bidder isn't the current bidder
                if self.auction_state.as_ref().unwrap().current_bidder == Some(*player_id) {
                    return Err(
                        "Player is already the current bidder. Can't bid against yourself"
                            .to_string(),
                    );
                }

                // Ensure that the bid is higher than the current bid
                if self.auction_state.as_ref().unwrap().current_bid >= *bid {
                    return Err("Bid is not higher than the current bid".to_string());
                }

                // Ensure that the player has enough money to make the bid
                if self.players.get(player_id).unwrap().cash < *bid as i64 {
                    return Err("Player does not have enough money to make the bid".to_string());
                }
            }
            StopBid { player_id } => {
                // ensure that it's the BankrupctyAuction stage
                if self.stage != Stage::InGame(InGameStage::BankruptcyAuction) {
                    return Err("It is not the BankruptcyAuction stage".to_string());
                }

                // Check player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // Check that the player is not the auctioneer
                if self.auction_state.as_ref().unwrap().player_id == *player_id {
                    return Err("Player stop bidding in their own auction. The auction is done once the bidders decide it's done".to_string());
                }

                //
            }
            EndBankruptcyHandling { player_id } => {
                // ensure that it's the BankruptcyHandling stage
                if self.stage != Stage::InGame(InGameStage::BankruptcyHandling) {
                    return Err("It is not the BankruptcyHandling stage".to_string());
                }

                // check that the player exists
                if !self.players.contains_key(player_id) {
                    return Err("Player does not exist".to_string());
                }

                // ensure that the active player is the player that is ending the bankruptcy handling
                if self.active_player_id != Some(*player_id) {
                    return Err("It is not this player's turn".to_string());
                }

                // check that the player is no-longer bankrupt
                if self.players.get(player_id).unwrap().bankrupt {
                    return Err("Player is bankrupt still".to_string());
                }
            }
            ChangeStage { stage: _, line: _ } => {
                return Err("ChangeStage should only be sent from server to client".to_string())
            }
        }

        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            stage: Stage::PreGame,
            active_player_id: None,
            game_host: None,
            players: HashMap::new(),
            player_order: Vec::new(),
            history: Vec::new(),
            rail_ledger: Rail::iter().map(|rail| (rail, None)).collect(),
            all_roads_bought: false,
            winner: None,
            declare_amount: 200000,
            rover_reward: 50000,
            auction_bid_increment: 500,
            auction_state: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! consume {
        ($state:ident, $event:expr) => {
            assert_eq!($state.validate(&$event), Ok(()));
            $state.consume(&$event);
        };
    }

    // for these tests, come up with a game state, and then ensure that the events in the history create that game state

    #[test]
    fn create_a_game() {
        let mut state = State::default();

        let player_id = 101;
        let event = Event::Create { player_id };

        assert_eq!(state.validate(&event), Ok(()));
        state.consume(&event);

        assert_eq!(
            state,
            State {
                players: HashMap::from([(
                    player_id,
                    Player {
                        ..Player::default()
                    }
                )]),
                game_host: Some(player_id),
                player_order: vec![player_id],
                history: vec![event],
                ..State::default()
            }
        );
    }

    #[test]
    fn start_a_game() {
        let mut state = State::default();

        let player_ids = [70, 102, 130];
        consume!(
            state,
            Event::Create {
                player_id: player_ids[0]
            }
        );
        consume!(
            state,
            Event::PlayerJoined {
                player_id: player_ids[1],
            }
        );
        consume!(
            state,
            Event::PlayerJoined {
                player_id: player_ids[2],
            }
        );

        // ensure the game can't be started before all players have a name and piece
        assert_eq!(
            state.validate(&Event::Start {
                player_id: player_ids[0]
            }),
            Err("Not all players have a name. Players without a name: 70, 102, 130".to_string())
        );

        // grant all players a name and piece
        consume!(
            state,
            Event::SetPlayerAttributes {
                player_id: player_ids[0],
                name: "Archie Flagstaff".to_string(),
                piece: Piece::Red,
            }
        );

        // ensure the player can't set a blank name
        assert_eq!(
            state.validate(&Event::SetPlayerAttributes {
                player_id: player_ids[1],
                name: "".to_string(),
                piece: Piece::Blue,
            }),
            Err("Name cannot be blank".to_string())
        );

        // ensure the player can't choose a piece that is already taken
        assert_eq!(
            state.validate(&Event::SetPlayerAttributes {
                player_id: player_ids[1],
                name: "Bobby Flagstaff".to_string(),
                piece: Piece::Red,
            }),
            Err("Piece is already taken".to_string())
        );

        // have the 1st player join the game
        consume!(
            state,
            Event::SetPlayerAttributes {
                player_id: player_ids[1],
                name: "Bobby Flagstaff".to_string(),
                piece: Piece::Blue,
            }
        );

        // have the 2nd player join the game
        consume!(
            state,
            Event::SetPlayerAttributes {
                player_id: player_ids[2],
                name: "Cindy Flagstaff".to_string(),
                piece: Piece::Green,
            }
        );

        // ensure a non-host player can't start the game
        assert_eq!(
            state.validate(&Event::Start {
                player_id: player_ids[1]
            }),
            Err("Player is not the host".to_string())
        );

        // start the game
        consume!(
            state,
            Event::Start {
                player_id: player_ids[0]
            }
        );

        assert_eq!(
            State {
                history: Vec::new(),
                players: HashMap::new(),
                ..state
            },
            State {
                stage: Stage::InGame(InGameStage::OrderRoll),
                active_player_id: None,
                game_host: Some(player_ids[0]),
                player_order: vec![player_ids[0], player_ids[1], player_ids[2]],
                all_roads_bought: false,
                ..State::default()
            }
        );
    }

    // write a test ensuring the Create event can't be called on a non-default state
    #[test]
    fn cant_create_a_game_that_already_exists() {
        // Try to create a game on a non-default state
        let mut state = State::default();
        state.all_roads_bought = !state.all_roads_bought;

        let event = Event::Create { player_id: 102 };

        assert_eq!(
            state.validate(&event),
            Err("Game already exists".to_string())
        );
    }

    // test that a player can't create a game and then join it
    #[test]
    fn same_player_that_created_a_game_cant_join_it() {
        let mut state = State::default();

        let player_id = 103;
        let event = Event::Create { player_id };

        assert_eq!(state.validate(&event), Ok(()));
        state.consume(&event);

        let event = Event::PlayerJoined { player_id };

        assert_eq!(
            state.validate(&event),
            Err("Player already exists".to_string())
        );
    }
}
