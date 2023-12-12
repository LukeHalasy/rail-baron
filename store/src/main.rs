use std::io::Write;

use store::{Event, Piece, State};

// TODO: Think about a better way to test out the computer
fn main() {
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
    println!("Game state: {:#?}", game_state);

    let rounds = 500;
    let mut i = 0;
    let player_to_max_id = 2;
    while i < rounds * game_state.players.len() {
        let active_player = game_state.active_player_id.unwrap();
        let (_, event) = game_state.minimax(
            4,
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
                    // if an invalid event occurs print out the event history to a json file
                    let filename = "event_history.json".to_string();
                    let mut file = std::fs::File::create(filename.clone()).unwrap();

                    let json = serde_json::to_string_pretty(&game_state.history).unwrap();
                    // wirite the json to the file
                    println!("Writing event history to file: {}", filename.clone());
                    file.write_all(json.as_bytes()).unwrap();

                    println!("STATE: {:#?}", game_state);
                    panic!("Invalid event: {:#?}, {:?}", event, err);
                }
            }
        }

        println!("Round finished: {}, Player_id: {}", i, active_player);
        println!("Event: {:#?}", event);

        game_state.consume(&event);

        i += 1;
    }
    println!("Game state: {:#?}", game_state);

    let mut file = std::fs::File::create("state.json").unwrap();
    let json = serde_json::to_string_pretty(&game_state).unwrap();
    // wirite the json to the file
    println!("Writing state to file: state.json");
    file.write_all(json.as_bytes()).unwrap();
}
