use futures::{SinkExt, StreamExt};

use leptos_meta::{provide_meta_context, Stylesheet, Title};
use reqwasm::websocket::{futures::WebSocket, Message};

use leptos::*;

use leptos_router::{Route, Router, Routes};
use store::{ClientMessage, Event, Player, PlayerId, ServerMessage, State};
use web_sys::console;
// use server::ServerMessage;

use crate::game::game::Game;
use crate::pre_game::home::Home;
use crate::pre_game::join::Join;
use crate::pre_game::lobby::Lobby;
use crate::pre_game::rules::Rules;

pub type Error = String;

#[component]
pub fn App() -> impl IntoView {
    let ws = WebSocket::open("ws://127.0.0.1:8000").unwrap();
    let (mut write, mut read) = ws.split();

    let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<ClientMessage>(1000);
    provide_context(in_tx);

    let (player, set_player_information) = create_signal(None::<Player>);
    provide_context(player);
    provide_context(set_player_information);

    let (state, set_state) = create_signal(None::<State>);
    provide_context(state);
    provide_context(set_state);

    let (player_id, set_player_id) = create_signal(None::<PlayerId>);
    provide_context(player_id);
    provide_context(set_player_id);

    let (error, set_error) = create_signal(None::<Error>);
    provide_context(error);

    leptos::spawn_local(async move {
        while let Some(msg) = in_rx.next().await {
            // log::debug!("got event from channel! {}", s);
            write
                .send(Message::Bytes(bincode::serialize(&msg).unwrap()))
                .await
                .unwrap();
        }
    });

    leptos::spawn_local(async move {
        while let Some(msg) = read.next().await {
            if let Message::Bytes(bytes) = msg.unwrap() {
                if let Ok(server_message) = bincode::deserialize::<ServerMessage>(&bytes) {
                    match server_message {
                        ServerMessage::Event(event) => {
                            web_sys::console::log_1(
                                &format!("got event from server! {:?}", event).into(),
                            );
                            match event {
                                Event::PlayerJoined {
                                    player_id: joined_player_id,
                                } => {
                                    // add the joined player to the game state
                                    console::log_1(
                                        &format!("got player joined from server! {:?}", event)
                                            .into(),
                                    );

                                    set_state.update(|state| {
                                        let state = state.as_mut().unwrap();
                                        state.players.insert(
                                            joined_player_id,
                                            Player {
                                                name: joined_player_id.to_string(),
                                                ..Player::default()
                                            },
                                        );

                                        state.player_order.push(joined_player_id);
                                    });
                                }
                                Event::Create { player_id } => {
                                    // navigate to the lobby
                                    // let navigate = leptos_router::use_navigate();
                                    // navigate(
                                    //     &format!("/lobby/{}", player_id.get().unwrap()),
                                    //     Default::default(),
                                    // )
                                    console::log_1(
                                        &format!("got player creation from server! {:?}", event)
                                            .into(),
                                    );

                                    set_state.update(|state| {
                                        let state = state.as_mut().unwrap();

                                        state.players.insert(
                                            player_id,
                                            Player {
                                                name: player_id.to_string(),
                                                ..Player::default()
                                            },
                                        );
                                        state.player_order.push(player_id);
                                        state.active_player_id = player_id;
                                        state.game_host = player_id;
                                    });
                                }
                                Event::Start { player_id: _ } => {
                                    // navigate to the game
                                    let navigate = leptos_router::use_navigate();
                                    navigate(
                                        &format!("/game/{}", player_id.get().unwrap()),
                                        Default::default(),
                                    )
                                }
                                _ => {}
                            }
                        }
                        ServerMessage::Error(error) => {
                            web_sys::console::error_1(
                                &format!("got error from server! {:?}", error).into(),
                            );

                            set_error.update(|e| *e = Some(error));
                        }
                        ServerMessage::Connection(player_id) => {
                            web_sys::console::log_1(
                                &format!("got connection from server! {:?}", player_id).into(),
                            );
                            set_player_id.update(|id| {
                                *id = Some(player_id);
                            });
                            web_sys::console::log_1(
                                &format!("got player id from server! {:?}", player_id).into(),
                            );
                        }
                        ServerMessage::GameCreated(game_id) => {
                            web_sys::console::log_1(
                                &format!("got game created from server! {:?}", game_id).into(),
                            );

                            // initialize the game state
                            set_state.update(|state| {
                                *state = Some(State::default());
                            });

                            // navigate to the lobby
                            let navigate = leptos_router::use_navigate();
                            navigate(&format!("/lobby/{}", game_id), Default::default());
                        }
                        ServerMessage::GameJoined(game_id) => {
                            web_sys::console::log_1(
                                &format!("succesfully joined game! {:?}", game_id).into(),
                            );

                            // initialize the game state
                            set_state.update(|state| {
                                *state = Some(State::default());
                            });

                            // navigate to the lobby
                            let navigate = leptos_router::use_navigate();
                            navigate(&format!("/lobby/{}", game_id), Default::default());
                        }
                    }
                }
            }
        }
    });

    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Title formatter=|text| format!("Railway Riches - {text}")/>
        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/join" view=Join />
                <Route path="/rules" view=Rules />
                <Route path="/lobby/:id" view=Lobby />
                <Route path="/game/:id" view=Game />
            </Routes>
        </Router>
    }
}
