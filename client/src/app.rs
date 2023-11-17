use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};

use leptos::*;

use leptos_router::{Router, Routes, Route};
use store::{Event, PlayerId, Player};

use crate::{login::Login, map::Map};

#[component]
pub fn App() -> impl IntoView {
    let ws = WebSocket::open("ws://127.0.0.1:8000").unwrap();
    let (mut write, _) = ws.split();

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

    let (player, set_player_information) = create_signal(None::<Player>);
    provide_context(player);
    provide_context(set_player_information);

    let (player_id, set_player_id) = create_signal(None::<PlayerId>);
    provide_context(player_id);
    provide_context(set_player_id);

    view! {
        <Router>
            <Routes>
                <Route path="/" view=Login />
                <Route path="/map" view=Map />
            </Routes>
        </Router>
    }
}
