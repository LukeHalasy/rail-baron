//! A chat server that broadcasts a message to all connections.
//!
//! This is a simple line-based server which accepts WebSocket connections,
//! reads lines from those connections, and broadcasts the lines to all other
//! connected clients.
//!
//! You can test this out by running:
//!
//!     cargo run --example server 127.0.0.1:12345
//!
//! And then in another window run:
//!
//!     cargo run --example client ws://127.0.0.1:12345/
//!
//! You can run the second command in multiple windows and then chat between the
//! two, seeing the messages from the other client as they're received. For all
//! connected clients they'll all join the same room and see everyone else's
//! messages.

use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use serde::{Deserialize, Serialize};
use store::{State, PlayerId, GameId, ServerMessage};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;

// each game has a list of peers
type PeerMaps = Arc<Mutex<HashMap<GameId, HashMap<SocketAddr, Tx>>>>;
type PeerToGameMap = Arc<Mutex<HashMap<SocketAddr, GameId>>>;

type GameStates = Arc<Mutex<HashMap<GameId, State>>>;

async fn handle_connection(peer_maps: PeerMaps, peer_to_game_map: PeerToGameMap, game_states: GameStates, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    let (outgoing, incoming) = ws_stream.split();

    // send a message to the client to let them know they've connected (and their ID)
    // tx.unbounded_send(Message::Text(addr.port().to_string()));
    // send a ServerMessage::Connection to the client to let them know they've connected (and their ID)
    // let msg = ServerMessage::Connection(addr.port().into());
    tx.unbounded_send(Message::Binary(bincode::serialize(&ServerMessage::Connection(addr.port().into())).unwrap()));

    // tx.unbounded_send(Message::Binary(addr.port().to_string().into_bytes()));

    let broadcast_incoming = incoming.try_for_each(|msg| {
        // deserialize message into an Event
        let event: store::Event = bincode::deserialize(&msg.clone().into_data()).expect("Received a non-event");
        println!("Received a message from {}: {:?}", addr, event);

        // if the event is to create a new game, first ensure the peer isn't already in a game
        match event  {
            store::Event::Create { .. } => {
                if peer_to_game_map.lock().unwrap().contains_key(&addr) {
                    println!("{} is already in a game", &addr);

                    match bincode::serialize(&ServerMessage::Error("Player is already in a game".to_string())) {
                        Ok(serialized) => {
                            match tx.unbounded_send(Message::Binary(serialized)) {
                                Ok(_) => println!("Error message sent to {:?}", addr),
                                Err(e) => println!("Error sending error message to {:?}: {}", addr, e),
                            }
                        }
                        Err(e) => println!("Error serializing error message: {}", e),
                    }

                    return future::ok(())
                } else {
                    println!("{} is creating a new game", &addr);
                    let game_id = game_states.lock().unwrap().len() as u64;
                    let game_state = State::default();

                    game_states.lock().unwrap().insert(game_id, game_state);
                    peer_maps.lock().unwrap().insert(game_id, HashMap::from([
                        (addr, tx.clone())
                    ]));
                    peer_to_game_map.lock().unwrap().insert(addr, game_id);
                    println!("{} is now in game {}", &addr, game_id);

                    // send a ServerMessage::GameCreated to the client to let them know their game was created
                    tx.unbounded_send(Message::Binary(bincode::serialize(&ServerMessage::GameCreated(game_id)).unwrap()));
                }
            }
            _ => {}
        }

        // ensure the user is in a game
        if !peer_to_game_map.lock().unwrap().contains_key(&addr) {
            println!("{} is not in a game", &addr);

            match tx.unbounded_send(Message::Binary(bincode::serialize(&ServerMessage::Error("Player is not in a game".to_string())).unwrap())) {
                Ok(_) => println!("Error message sent to {:?}", addr),
                Err(e) => println!("Error sending error message to {:?}: {}", addr, e),
            }

            return future::ok(())
        } 

        // get the user's game 
        let binding = peer_to_game_map.lock().unwrap();
        let game_id = binding.get(&addr).unwrap().to_owned();
        println!("hreh");

        // validate the event. if the event is invalid print an error message
        if let Err(e) = game_states.lock().unwrap().get(&game_id).unwrap().validate(&event) {
            println!("Invalid event: {:?}, err: {}", event, e);

            match tx.unbounded_send(Message::Binary(bincode::serialize(&ServerMessage::Error(e.to_string())).unwrap())) {
                Ok(_) => println!("Error message sent to {:?}", addr),
                Err(e) => println!("Error sending error message to {:?}: {}", addr, e),
            }
        } else {
            println!("HEYO");
            let game_states_clone = game_states.clone();
            let event_clone = event.clone();
            tokio::spawn(async move {
                println!("Attempting to consume event: {:?}", event_clone);
                game_states_clone.lock().unwrap().get_mut(&game_id).unwrap().consume(&event_clone);
                println!("Consumed event: {:?}", event_clone);
            });
                // .map(|(_, ws_sink)| ws_sink);
            // broadcat the event to all peers in the game except the sender
            let peers = peer_maps.lock().unwrap();
            let broadcast_recipients = peers.get(&game_id).unwrap();

            for (peer_addr, recp) in broadcast_recipients {
                match recp.unbounded_send(Message::Binary(bincode::serialize(&ServerMessage::Event(event.clone())).unwrap())) {
                    Ok(_) => println!("Event sent to {:?}", peer_addr),
                    Err(e) => println!("Error sending event to {:?}: {}", peer_addr, e),
                }
            }
        }

        println!("DOWN HERE");
    
        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("Player {} disconnected", &addr);
    // if the player is in a game, remove them from the game
    if let Some(game_id) = peer_to_game_map.lock().unwrap().remove(&addr) {
        println!("Removing player {} from game {}", &addr, game_id);
        peer_maps.lock().unwrap().get_mut(&game_id).unwrap().remove(&addr);
        peer_to_game_map.lock().unwrap().remove(&addr);

        // if the game is now empty, remove it
        if peer_maps.lock().unwrap().get(&game_id).unwrap().is_empty() {
            println!("Game {} is now empty. Deleting game..", game_id);
            peer_maps.lock().unwrap().remove(&game_id);
            game_states.lock().unwrap().remove(&game_id);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8000".to_string());

    let peer_maps = PeerMaps::new(Mutex::new(HashMap::new()));
    let game_states = GameStates::new(Mutex::new(HashMap::new()));
    let peer_to_game_map = PeerToGameMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(peer_maps.clone(), peer_to_game_map.clone(), game_states.clone(), stream, addr));
    }

    Ok(())
}
