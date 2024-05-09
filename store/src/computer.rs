use std::{io::Write, vec};

use crate::{
    main_city::Rail,
    rail::{C, RAILS_TO_CITIES_MAP},
    travel_payout::City,
    Engine, Event, InGameStage, PlayerId, Stage, State,
};

use petgraph::algo::astar;
use strum::IntoEnumIterator;

impl State {
    fn evaluate(&self, player_to_max_id: PlayerId) -> f64 {
        // evaluate the current game state and consider how good it is for the player_to_max_id
        // return a f64 between -1 and 1 where -1 means the player lost and 1 means the player won

        // check for wins and losses
        if self.stage == Stage::Ended {
            if self.winner == Some(player_to_max_id) {
                return 1.0;
            } else {
                return -1.0;
            }
        };

        // check if the player has been eliminated
        if !self.player_order.contains(&player_to_max_id) {
            return -1.0;
        }

        // TODO: Evaluate railroad network connectivity.
        // let network_connectivity = self.evaluate_network_connectivity(player_id);

        // TODO: Evaluate how much money the player had at the end of the game.
        let money = self.evaluate_money(player_to_max_id);

        // TODO: Evaluate how many cities the player made it to.
        // TODO: Evaluate the directness of each player's routes.

        // average the evaluations
        // let parameters = [network_connectivity, money];
        let parameters = [money];

        parameters.iter().sum::<f64>() / parameters.len() as f64
    }

    // TODO: This evaluation function is a big bottleneck
    #[allow(dead_code)]
    fn evaluate_network_connectivity(&self, player_id: u64) -> f64 {
        // For what percentage of source / destination pairs can the player get from source to destination without using another player's rails ?
        // Get a list of all source / destination pairs

        let source_destination_pairs = City::cities()
            .iter()
            .flat_map(|&source| {
                City::cities()
                    .iter()
                    .filter(move |&&destination| source != destination)
                    .map(move |&destination| (source, destination))
            })
            .collect::<Vec<_>>();
        let num_total_pairs = source_destination_pairs.len();

        let mut num_connected_pairs = 0;
        for (source, destination) in source_destination_pairs {
            // loop through all the rails and check if the player can get from source to destination without using another player's rails
            for (rail, owner) in self.rail_ledger.iter() {
                if *owner == Some(player_id) {
                    let mut contains_source = false;
                    let mut contains_destination = false;

                    let graph = &*RAILS_TO_CITIES_MAP;

                    if let Some(connected_cities) = graph.get(rail) {
                        for city in connected_cities {
                            if *city == C::D(source) {
                                contains_source = true;
                            }

                            if *city == C::D(destination) {
                                contains_destination = true;
                            }
                        }
                    }

                    if contains_source && contains_destination {
                        num_connected_pairs += 1;
                        break;
                    }
                }
            }
        }

        // < 50% will be negative, > 50% will be positive
        ((num_connected_pairs as f64 / num_total_pairs as f64) - 0.5).tanh()
    }

    fn evaluate_money(&self, player_id: PlayerId) -> f64 {
        let player = self.players.get(&player_id).unwrap();
        (player.cash as f64 / 1000.0).tanh()
    }

    fn find_all_valid_moves(&self) -> Vec<Event> {
        // find all valid moves for the current player
        // this is used to generate the game tree
        let player_id = self.active_player_id.unwrap();

        // if the game is over, return an empty vec
        use Stage::*;

        // if the player is bankrupt then they have no valid moves
        if self.players.get(&player_id).unwrap().bankrupt {
            return vec![];
        }

        use InGameStage::*;

        match self.stage {
            Ended => vec![],
            PreGame => vec![],
            InGame(BankruptcyAuction) => {
                // TODO: Need to change auctioning to be turn based
                // if the player is the auctioneer then they have no valid moves, they must wait for the auction to end
                let auction_state = self.auction_state.as_ref().unwrap();
                if auction_state.player_id.eq(&player_id)
                    || auction_state.dropped_out_bidders.contains(&player_id)
                {
                    vec![]
                } else {
                    // the player can bid self.auction_bid_increment + (auction_state.current_bid * n) where n is any number up to the point where they wouldn't have enough money to bid
                    let auction_state = self.auction_state.as_ref().unwrap();
                    let player = self.players.get(&player_id).unwrap();
                    let mut valid_moves = vec![Event::StopBid { player_id }];
                    let mut n = 1;
                    while player.cash
                        >= auction_state.current_bid as i64
                            + (self.auction_bid_increment * n) as i64
                    {
                        valid_moves.push(Event::Bid {
                            player_id,
                            bid: (self.auction_bid_increment * n) + auction_state.current_bid,
                        });

                        n += 1;
                    }

                    valid_moves
                }
            }
            InGame(BankruptcyHandling) => {
                // The player could sell any of their rails to the bank, or auction any of their rails
                let mut valid_moves = vec![];
                for (rail, owner) in self.rail_ledger.iter() {
                    if *owner == Some(player_id) {
                        valid_moves.push(Event::SellRailToBank {
                            player_id,
                            rail: *rail,
                        });
                        // TODO: Uncomment once auctioning is turn based
                        // valid_moves.push(Event::AuctionRail {
                        //     player_id,
                        //     rail: *rail,
                        // });
                    }
                }

                let player = self.players.get(&player_id).unwrap();
                if !player.bankrupt {
                    valid_moves.push(Event::EndBankruptcyHandling { player_id });
                }

                valid_moves
            }
            InGame(OrderRoll) => {
                println!("Order roll request going to be suggested");
                vec![Event::OrderRollRequest { player_id }]
            }
            InGame(HomeRoll) => {
                vec![Event::HomeRollRequest { player_id }]
            }
            InGame(DestinationRoll) => {
                vec![Event::DestinationRollRequest { player_id }]
            }
            InGame(MovementRoll) => {
                vec![Event::MovementRollRequest { player_id }]
            }
            InGame(Movement) => {
                // The player can move to any city that is connected to their current city
                let player = self.players.get(&player_id).unwrap();
                let mut valid_moves = vec![];
                let current_city = player.current_city();

                // check if the player has a destination
                if player.destination.is_none() {
                    println!("Goingto panic!");
                    println!("Player: {:#?}", player);
                    println!("Will save the state to a file");

                    let mut file = std::fs::File::create("state.json").unwrap();
                    let json = serde_json::to_string_pretty(&self).unwrap();
                    // wirite the json to the file
                    println!("Writing state to file: state.json");
                    file.write_all(json.as_bytes()).unwrap();

                    panic!("Player didn't have a destination while moving");
                }

                // Calculate the A* path to the players destination
                // Dont need to worry about cycles just yet since for now we are giving every edge the same weight
                let path = astar(
                    &Rail::get_railroad_graph(),
                    C::into(current_city.unwrap()),
                    |c| {
                        c == C::into(C::D(
                            player
                                .destination
                                .expect("Expected player to have a destination"),
                        ))
                    },
                    |_| 1,
                    |_| 0,
                );
                if path.clone().unwrap().1.len() == 1 {
                    println!("Goingto panic!");
                    println!("Player: {:#?}", player);
                    let mut file = std::fs::File::create("state.json").unwrap();
                    let json = serde_json::to_string_pretty(&self).unwrap();
                    // wirite the json to the file
                    println!("Writing state to file: state.json");
                    file.write_all(json.as_bytes()).unwrap();
                    panic!("Wasn't able to find a path to the destination");
                }

                let first_city_on_optimal_path: C = path.unwrap().1[1].into(); // first city that is not the current city

                // get all the edges between current_city and path.unwrap().1[0] converted to a city
                for edge in Rail::get_railroad_graph().edges_connecting(
                    C::into(current_city.unwrap()),
                    C::into(first_city_on_optimal_path),
                ) {
                    // get the rail from the edge
                    let rail = *edge.weight();
                    valid_moves.push(Event::Move {
                        player_id,
                        route: (first_city_on_optimal_path, rail),
                    });
                }

                valid_moves
            }
            InGame(DeclareOption) => {
                vec![
                    Event::DeclareChoice {
                        player_id,
                        declare: true,
                    },
                    Event::DeclareChoice {
                        player_id,
                        declare: false,
                    },
                ]
            }
            InGame(Purchase) => {
                // the player can purchase any rail that is not owned, for which they have enough money
                let player = self.players.get(&player_id).unwrap();
                let mut valid_moves = vec![];
                for (rail, owner) in self.rail_ledger.iter() {
                    if owner.is_none() && player.cash >= (rail.cost() as i64) {
                        valid_moves.push(Event::PurchaseRail {
                            player_id,
                            rail: *rail,
                        });
                    }
                }

                // TODO: They can also buy an engine upgrade if they have enough money
                // iterate through the Engine enum and check if validate passes and if it does then add it to the valid moves
                for engine in Engine::iter() {
                    if self
                        .validate(&Event::PurchaseEngine {
                            player_id,
                            engine: engine.clone(),
                        })
                        .is_err()
                    {
                        continue;
                    }

                    valid_moves.push(Event::PurchaseEngine { player_id, engine });
                }

                // they can also end their turn
                valid_moves.push(Event::EndPurchaseStage { player_id });

                valid_moves
            }
        }
    }

    pub fn minimax(
        &self,
        depth: u64,
        player_to_max_id: PlayerId,
        minning_for_other: bool,
        last_event: Event,
    ) -> (f64, Event) {
        if self.stage == Stage::Ended
            || depth == 0
            || !self.player_order.contains(&player_to_max_id)
        {
            return (self.evaluate(player_to_max_id), last_event);
        };

        if minning_for_other {
            let mut min_eval = f64::MAX;
            let mut min_event = None;

            if self.find_all_valid_moves().is_empty() {
                println!("Event: {:?}", last_event);
                // save the state to a file
                let mut file = std::fs::File::create("state.json").unwrap();
                let json = serde_json::to_string_pretty(&self).unwrap();
                // wirite the json to the file
                println!("Writing state to file: state.json");
                file.write_all(json.as_bytes()).unwrap();

                panic!("No valid moves!");
            }

            for event in self.find_all_valid_moves() {
                let mut new_state = self.clone();
                //validate the event
                if let Err(err) = new_state.validate(&event) {
                    println!("State: {:#?}", self);
                    println!("Event: {:?}", event);
                    println!("INVALID!");

                    // save the state to a file
                    let mut file = std::fs::File::create("state.json").unwrap();
                    let json = serde_json::to_string_pretty(&self).unwrap();
                    // wirite the json to the file
                    println!("Writing state to file: state.json");
                    file.write_all(json.as_bytes()).unwrap();
                    panic!("Invalid event: {:?}", err);
                }

                new_state.consume(&event);

                let next_player_is_player_to_max =
                    new_state.active_player_id == Some(player_to_max_id);
                let (eval, _) = new_state.minimax(
                    depth - 1,
                    player_to_max_id,
                    !next_player_is_player_to_max,
                    event.clone(),
                );

                if eval < min_eval {
                    min_eval = eval;
                    min_event = Some(event.clone());
                }
            }

            (
                min_eval,
                min_event.expect("A min event should have been set"),
            )
        } else {
            let mut max_eval = f64::MIN;
            let mut max_event = None;

            if self.find_all_valid_moves().is_empty() {
                println!("Event: {:?}", last_event);

                // save the state to a file
                let mut file = std::fs::File::create("state.json").unwrap();
                let json = serde_json::to_string_pretty(&self).unwrap();
                // wirite the json to the file
                println!("Writing state to file: state.json");
                file.write_all(json.as_bytes()).unwrap();
                panic!("No valid moves!");
            }

            for event in self.find_all_valid_moves() {
                let mut new_state = self.clone();
                // validate the event
                if let Err(err) = new_state.validate(&event) {
                    println!("State: {:#?}", self);
                    println!("Event: {:?}", event);
                    println!("INVALID!");
                    panic!("Invalid event: {:?}", err);
                }

                new_state.consume(&event);

                let next_player_is_player_to_max =
                    new_state.active_player_id == Some(player_to_max_id);
                let (eval, _) = new_state.minimax(
                    depth - 1,
                    player_to_max_id,
                    !next_player_is_player_to_max,
                    event.clone(),
                );

                if eval > max_eval {
                    max_eval = eval;
                    max_event = Some(event);
                }
            }

            (
                max_eval,
                max_event.expect("A max event should have been set."),
            )
        }
    }
}
