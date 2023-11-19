use futures::{SinkExt, StreamExt};
use leptos_meta::{provide_meta_context, Stylesheet};
use reqwasm::websocket::{futures::WebSocket, Message};

use leptos::*;

use leptos_router::{Router, Routes, Route};
use store::{Event, PlayerId, Player};

use crate::pre_game::home::Home;
use crate::game::game::Game;

#[component]
pub fn App() -> impl IntoView {
    let ws = WebSocket::open("ws://127.0.0.1:8000").unwrap();
    let (mut write, mut read) = ws.split();

    let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<Event>(1000);
    provide_context(in_tx);

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
                Message::Text(error_message) => {
                    web_sys::console::error_1(&format!("got error from server! {}", error_message).into());
                },
                Message::Bytes(bytes) => {
                    let event: Event = bincode::deserialize(&bytes).unwrap();
                    web_sys::console::log_1(&format!("got event from server! {:?}", event).into());
                },
            }
        }
    });

    let (player, set_player_information) = create_signal(None::<Player>);
    provide_context(player);
    provide_context(set_player_information);

    let (player_id, set_player_id) = create_signal(None::<PlayerId>);
    provide_context(player_id);
    provide_context(set_player_id);
    
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/map" view=Game />
            </Routes>
        </Router>
    }
}
