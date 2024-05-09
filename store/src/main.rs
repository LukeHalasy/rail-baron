use std::io::Write;

use store::{Event, Piece, State};

// TODO: Think about a better way to test out the computer
fn main() {
    // set the number of games to test
    let num_games = 100;
    let game_vec = vec![0; num_games];

    // run num_games counting the number of times the computer wins
    let mut computer_wins = 0;
    for (count, _) in game_vec.iter().enumerate() {
        println!("STARTING GAME # {}", count);
        // Initialize a new game
        let mut game_state = State::default();

        let setup_events = [
            Event::Create { player_id: 0 },
            Event::PlayerJoined { player_id: 1 },
            Event::PlayerJoined { player_id: 2 },
            Event::PlayerJoined { player_id: 3 },
            Event::SetPlayerAttributes {
                player_id: 0,
                name: "C1".to_string(),
                piece: Piece::Red,
            },
            Event::SetPlayerAttributes {
                player_id: 1,
                name: "C2".to_string(),
                piece: Piece::Blue,
            },
            Event::SetPlayerAttributes {
                player_id: 2,
                name: "C3".to_string(),
                piece: Piece::Green,
            },
            Event::SetPlayerAttributes {
                player_id: 3,
                name: "C4".to_string(),
                piece: Piece::Purple,
            },
            Event::Start { player_id: 0 },
            Event::OrderRollRequest { player_id: 0 },
            Event::OrderRollRequest { player_id: 1 },
            Event::OrderRollRequest { player_id: 2 },
            Event::OrderRollRequest { player_id: 3 },
        ];

        for event in setup_events.iter() {
            if let Err(err) = game_state.validate(event) {
                panic!("Invalid event: {:?}", err);
            }
            game_state.consume(event);
        }

        // Now, let the computer play four rounds
        let player_to_max_id = 2;
        while game_state.stage != store::Stage::Ended {
            // println!("Game: {}, Round: {}", count, i);
            // io::stdout().flush().unwrap();

            let _active_player = game_state.active_player_id.unwrap();
            // Thought.. what happens if the player gets eliminated...
            let (_, event) = game_state.minimax(
                15,
                player_to_max_id,
                false,
                setup_events.last().unwrap().clone(),
            );
            // ensure the event is valid
            match event {
                Event::HomeRoll { .. } => {}
                Event::DestinationRoll { .. } => {}
                Event::OrderRoll { .. } => {}
                _ => {
                    if let Err(err) = game_state.validate(&event) {
                        // if an invalid event occurs print out e event history to a json file
                        let filename = "event_history.json".to_string();
                        let mut file = std::fs::File::create(filename.clone()).unwrap();

                        let json = serde_json::to_string_pretty(&game_state.history).unwrap();
                        // wirite the json to the file
                        println!("Writing event history to file: {}", filename.clone());
                        file.write_all(json.as_bytes()).unwrap();

                        // write the game state to a json file
                        let filename = "state.json".to_string();
                        let mut file = std::fs::File::create(filename.clone()).unwrap();

                        let json = serde_json::to_string_pretty(&game_state).unwrap();
                        file.write_all(json.as_bytes()).unwrap();

                        // println!("STATE: {:#?}", game_state);
                        panic!("Invalid event: {:#?}, {:?}", event, err);
                    }
                }
            }

            // println!("Event: {:#?}", event);
            game_state.consume(&event);

            // check if the player to max id has been eliminted
            if !game_state.player_order.contains(&player_to_max_id) {
                println!(
                    "The AI Lost! Player {} has been eliminated!",
                    player_to_max_id
                );

                break;
                // find the next player to max id
                // let mut next_player_to_max_id = player_to_max_id + 1;
                // while game_state.players.contains_key(&next_player_to_max_id) == false {
                //     next_player_to_max_id += 1;
                // }
                // println!("Next player to max id: {}", next_player_to_max_id);
                // player_to_max_id = next_player_to_max_id;
            }
        }

        if game_state.winner.is_some() && game_state.winner.unwrap() == player_to_max_id {
            computer_wins += 1;
            println!("The computer won!");
        }
    }

    println!(
        "The computer won {} out of {} games",
        computer_wins,
        game_vec.len()
    );
}
