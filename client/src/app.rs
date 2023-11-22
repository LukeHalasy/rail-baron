use futures::{SinkExt, StreamExt};
use leptos_meta::{provide_meta_context, Stylesheet};
use reqwasm::websocket::{futures::WebSocket, Message};

use leptos::*;

use leptos_router::{Router, Routes, Route};
use store::{Event, PlayerId, Player, ServerMessage};
// use server::ServerMessage;

use crate::pre_game::home::Home;
use crate::game::game::Game;
use crate::pre_game::join::Join;
use crate::pre_game::lobby::Lobby;
use crate::pre_game::rules::Rules;

#[component]
pub fn App() -> impl IntoView {
    let ws = WebSocket::open("ws://127.0.0.1:8000").unwrap();
    let (mut write, mut read) = ws.split();

    let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<Event>(1000);
    provide_context(in_tx);

    let (player, set_player_information) = create_signal(None::<Player>);
    provide_context(player);
    provide_context(set_player_information);

    let (player_id, set_player_id) = create_signal(None::<PlayerId>);
    provide_context(player_id);
    provide_context(set_player_id);
    
    leptos::spawn_local(async move {
        while let Some(event) = in_rx.next().await {
            // log::debug!("got event from channel! {}", s);
            write
                .send(Message::Bytes(bincode::serialize(&event).unwrap()))
                .await
                .unwrap();
        }
    });

    leptos::spawn_local(async move {
        while let Some(msg) = read.next().await {
            match msg.unwrap() {
                // Message::Text(message) => {
                //     // if the player doesn't have an id yet this must be the server sending their id 
                //     web_sys::console::log_1(&format!("got message from server! {}", message).into());
                // },
                Message::Bytes(bytes) => {
                    match bincode::deserialize::<ServerMessage>(&bytes).unwrap() {
                        ServerMessage::Event(event) => {
                            web_sys::console::log_1(&format!("got event from server! {:?}", event).into());
                        },
                        ServerMessage::Error(error) => {
                            web_sys::console::error_1(&format!("got error from server! {:?}", error).into());
                        },
                        ServerMessage::Connection(player_id) => {
                            web_sys::console::log_1(&format!("got connection from server! {:?}", player_id).into());
                            set_player_id.update(|id| {
                                *id = Some(player_id);
                            });
                            web_sys::console::log_1(&format!("got player id from server! {:?}", player_id).into());
                        },
                        ServerMessage::GameCreated(game_id) => {
                            web_sys::console::log_1(&format!("got game created from server! {:?}", game_id).into());
                            // navigate to the lobby
                            let navigate = leptos_router::use_navigate();
                            navigate(&format!("/lobby/{}", game_id), Default::default());
                        }
                    }
                    // web_sys::console::log_1(&format!("got event from server! {:?}", event).into());
                },
                _ => {}
            }
        }
    });

    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
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
