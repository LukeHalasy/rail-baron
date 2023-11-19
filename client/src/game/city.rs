use futures::channel::mpsc::Sender;
use leptos::*;
use leptos_leaflet::{leaflet::MouseEvent, position, Circle, MouseEvents, Position};
use store::{rail::C, Event, Player};

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

    let set_player_information = use_context::<WriteSignal<Option<Player>>>().expect("Expected a player information setter");

    let tx = use_context::<Sender<Event>>().expect("Expected the tx sender");
    let move_player = move |event: MouseEvent| {
        // .update(|location| {
        //     *location = Position::new(event.latlng().lat(), event.latlng().lng());
        // });
        // update the player's location to be the clicked city
        // TODO: Send a player move request

        // let _ = tx
        //     .clone()
        //     .try_send(Event::DestinationCityRollRequest { player_id: 54 });

        set_player_information.update(|player| {
            // add the city to the player's route history
            // TODO: HANDLE RAILROAD SELECTION
            // player.unwrap().route_history.push(city);
            // player.route = Position::new(event.latlng().lat(), event.latlng().lng());
        });

    };

    view! {
        // TODO: Change to CircleMarker for consistent radius
        <Circle fill_opacity=1.0 mouse_events={MouseEvents::new().on_click(move_player)} fill_color={color} color="transparent" center=position!(latitude, longitude) radius={radius}>
        </Circle>
    }
}