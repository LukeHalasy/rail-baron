use std::collections::HashMap;

use futures::{SinkExt, StreamExt};

use leptos_meta::{provide_meta_context, Stylesheet, Title};
use reqwasm::websocket::{futures::WebSocket, Message};

use leptos::*;

use leptos_router::{Route, Router, Routes};
use store::rail::Rail;
use store::travel_payout::City;
use store::{ClientMessage, Event, Piece, Player, ServerMessage, State};
use strum::IntoEnumIterator;
use web_sys::console;
// use server::ServerMessage;

use crate::game::game::Game;
use crate::pre_game::home::Home;
use crate::pre_game::join::Join;
use crate::pre_game::lobby::{Lobby, LobbyParams};
use crate::pre_game::rules::Rules;

pub type Error = String;

#[derive(Copy, Clone, Debug)]
pub struct GameId(ReadSignal<Option<store::GameId>>);

#[derive(Copy, Clone, Debug)]
pub struct PlayerId(pub ReadSignal<Option<store::PlayerId>>);

#[component]
pub fn App() -> impl IntoView {
    let ws = WebSocket::open("ws://127.0.0.1:8000").unwrap();
    let (mut write, mut read) = ws.split();

    let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<ClientMessage>(1000);
    provide_context(in_tx);

    // TODO: Uncomment
    // let (state, set_state) = create_signal(None::<State>);

    let (state, set_state) = create_signal(Some({
        State {
            active_player_id: Some(1),
            game_host: Some(2),
            players: HashMap::from([
                (
                    1,
                    store::Player {
                        cash: 20000,
                        name: "Luke".to_string(),
                        piece: store::Piece::Blue,
                        home_city: Some(City::Baltimore_MD),
                        start: Some(City::Albany_NY),
                        destination: Some(City::Buffalo_NY),
                        ..store::Player::default()
                    },
                ),
                (
                    2,
                    store::Player {
                        cash: 2000,
                        name: "Kyle".to_string(),
                        piece: store::Piece::Red,
                        home_city: Some(City::Butte_MT),
                        start: Some(City::Atlanta_GA),
                        destination: Some(City::Las_Vegas_NV),
                        ..store::Player::default()
                    },
                ),
                (
                    3,
                    store::Player {
                        cash: 10000,
                        name: "Simon".to_string(),
                        piece: store::Piece::Yellow,
                        home_city: Some(City::Los_Angeles_CA),
                        start: Some(City::Tampa_FL),
                        destination: Some(City::Casper_WY),
                        ..store::Player::default()
                    },
                ),
            ]),
            rail_ledger: Rail::iter()
                .map(|rail| match rail {
                    Rail::C_AND_O => (rail, Some(1)),
                    Rail::ACL => (rail, Some(2)),
                    Rail::SAL => (rail, Some(2)),
                    _ => (rail, None),
                })
                .collect(),
            ..State::default()
        }
    }));

    provide_context(state);
    provide_context(set_state);

    let (player_id, set_player_id) = create_signal(None::<store::PlayerId>);
    provide_context(PlayerId(player_id));

    let (game_id, _set_game_id) = create_signal(None::<store::GameId>);
    provide_context(GameId(game_id));

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
                                        state.active_player_id = Some(player_id);
                                        state.game_host = Some(player_id);
                                    });
                                }
                                Event::Start { player_id: _ } => {
                                    // get the lobby id from the url
                                    let lobby_id =
                                        leptos_router::use_params::<LobbyParams>().with(|params| {
                                            params
                                                .as_ref()
                                                .map(|params| params.id)
                                                .unwrap_or_default()
                                        });

                                    // navigate to the game
                                    let navigate = leptos_router::use_navigate();
                                    navigate(&format!("/game/{}", lobby_id), Default::default())
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
