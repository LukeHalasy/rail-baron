use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};

use leptos::*;
use leptos_leaflet::*;
use store::{main_city, Event};

use crate::{cities::Cities, rails::Rails};

#[component]
pub fn App() -> impl IntoView {
    let (player_location, set_player_location) = create_signal(Position::new(
        main_city::City::Albany_NY.coordinates().latitude(),
        main_city::City::Albany_NY.coordinates().longitude(),
    ));
    provide_context(set_player_location);

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

    view! {
        <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(39.8283, -98.5795) zoom=5.0 max_zoom=7.5 min_zoom=5.0 set_view=true>
            // TODO: need to add attribution
            <TileLayer url="https://{s}.basemaps.cartocdn.com/light_nolabels/{z}/{x}/{y}{r}.png"/>

            <Rails></Rails>
            <Cities></Cities>

            <Marker position={player_location}></Marker>
        </MapContainer>
    }
}
