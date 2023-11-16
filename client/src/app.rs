use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use futures::channel::mpsc::Sender;

use leptos::{*, html::{Input, Select}};
use leptos_leaflet::*;
use leptos_router::{Router, Routes, Route};
use store::{main_city, Event, Piece, PlayerId};
use web_sys::SubmitEvent;

use strum::IntoEnumIterator;

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

    let (name, set_name) = create_signal("".to_string());
    let (player_id, set_player_id) = create_signal(PlayerId::default());

    // we'll use a NodeRef to store a reference to the input element
    // this will be filled when the element is created
    let player_id_input: NodeRef<Input> = create_node_ref();
    let name_input: NodeRef<Input> = create_node_ref();
    let color_input: NodeRef<Select> = create_node_ref();
    
    // fires when the form `submit` event happens
    // this will store the value of the <input> in our signal
    let tx = use_context::<Sender<Event>>().expect("Expected the tx sender");
    let join_game = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        
        set_player_id(player_id_input()
            .expect("id <input> to exist")
            .value()
            .parse::<u64>()
            .expect("Failed to parse player ID")
        );

        set_name(name_input()
            .expect("name <input> to exist")
            .value());


        let color = color_input()
            .expect("<select> to exist")
            .value();

        // send Event::PlayerJoined { player_id: 64, name: value, color: Piece::Red } to the server
        let _ = tx
            .clone()
            .try_send(Event::PlayerJoined {
                player_id: player_id.clone().get(),
                name: name.clone().get(),
                color: Piece::from_str(&color).unwrap(),
            });
        
        let navigate = leptos_router::use_navigate();
        navigate("/map", Default::default());
    };
    
    let home_page = view! {
        <div>
            <h1>"Welcome to the game!"</h1>
            <form on:submit=join_game>
                <input type="text" value=player_id node_ref=player_id_input placeholder="Enter Player ID" pattern="[0-9]*"/>
                <input type="text" value=name node_ref=name_input placeholder="Enter your name"/>
                <select node_ref=color_input>
                    {
                        Piece::iter()
                        .map(move |p| {
                            view! {
                                <option value=p.to_string()>{p.to_string()}</option>
                            }
                        }).collect_view()
                    }
                </select>
                <input type="submit" value="Join Game"/>
            </form>
        </div>
    };

    let main_page = view! {
        <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(39.8283, -98.5795) zoom=5.0 max_zoom=7.5 min_zoom=5.0 set_view=true>
            // TODO: need to add attribution
            <TileLayer url="https://{s}.basemaps.cartocdn.com/light_nolabels/{z}/{x}/{y}{r}.png"/>

            <Rails></Rails>
            <Cities></Cities>

            <Marker position={player_location}></Marker>
        </MapContainer>
    };

    view! {
        <Router>
            <Routes>
                <Route path="/" view={move || home_page.clone()} />
                <Route path="/map" view={move || main_page.clone()} />
            </Routes>
        </Router>
    }
}
