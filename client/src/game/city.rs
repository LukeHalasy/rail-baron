use futures::channel::mpsc::Sender;
use leptos::*;
use leptos_leaflet::{leaflet::MouseEvent, position, Circle, MouseEvents};
use store::{
    rail::{self, C},
    ClientMessage, Event,
};

use crate::app::PlayerId;

#[component]
pub fn City(city: C) -> impl IntoView {
    let latitude = city.coordinates().latitude();
    let longitude = city.coordinates().longitude();
    let radius = {
        match city {
            C::D(_) => 17000.0,
            C::P(_) => 9000.0,
        }
    };
    let color = {
        match city {
            C::D(_) => "black",
            C::P(_) => "gray",
        }
    };

    let tx = use_context::<Sender<ClientMessage>>().expect("Expected the tx sender");
    let player_id = use_context::<PlayerId>().expect("Expected a player id signal");

    let move_player = move |_event: MouseEvent| {
        let game_state = use_context::<ReadSignal<Option<store::State>>>()
            .expect("Expected a game state signal");
        let current_city = game_state
            .get()
            .unwrap()
            .players
            .get(&player_id.0.get().unwrap())
            .unwrap()
            .current_city()
            .unwrap();

        // See how many railroads
        // web_sys::console::log_1(&format!("clicked city! {:?}", city.to_string()).into());
        // let _ tx.clone().try_send
        // .update(|location| {
        //     *location = Position::new(event.latlng().lat(), event.latlng().lng());
        // });
        // update the player's location to be the clicked city
        // TODO: Send a player move request

        // let _ = tx
        //     .clone()
        //     .try_send(Event::DestinationCityRollRequest { player_id: 54 });

        // set_player_information.update(|_player| {
        // add the city to the player's route history
        // TODO: HANDLE RAILROAD SELECTION
        // player.unwrap().route_history.push(city);
        // player.route = Position::new(event.latlng().lat(), event.latlng().lng());
        // });

        // Get the list of railroads directly connecting the player's current city to the clicked city
        // If their is only one option, then automatically select it
        // Otherwise, show the user a pop-up with the options.
        // let rails: Vec<Rail> = RAILROAD_GRAPH
        //     .edges_connecting(<C>::into(current_city), <C>::into(city))

        let rails = rail::RAILROAD_EDGES
            .get(&(current_city, city))
            .or_else(|| rail::RAILROAD_EDGES.get(&(city, current_city)));

        if let Some(rails) = rails {
            if rails.len() == 1 {
                let mut tx = tx.clone();
                let _ = tx.try_send(ClientMessage::Event(Event::Move {
                    player_id: player_id.0.get().unwrap(),
                    route: (city, rails[0]),
                }));
            } else {
                // TODO: Show a pop-up with the options
            }
        }

        // TODO: In the future, if the user is riding along a rail that has two options for multiple stops, then after the
        // first time the user selects a railroad, automatically select the same railroad for the next stop.
    };

    view! {
        // TODO: Change to CircleMarker for consistent radius
        <Circle fill_opacity=1.0 mouse_events={MouseEvents::new().on_click(move_player)} fill_color={color} color="transparent" center=position!(latitude, longitude) radius={radius}>
        </Circle>
    }
}
