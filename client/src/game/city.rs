use leptos::*;
use leptos_leaflet::{leaflet::MouseEvent, position, Circle, MouseEvents};
use store::rail::C;

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

    // let _tx = use_context::<Sender<ClientMessage>>().expect("Expected the tx sender");
    let move_player = move |event: MouseEvent| {
        web_sys::console::log_1(
            &format!("clicked city! {:?}", city.to_string()).into(),
        );
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
    };

    view! {
        // TODO: Change to CircleMarker for consistent radius
        <Circle fill_opacity=1.0 mouse_events={MouseEvents::new().on_click(move_player)} fill_color={color} color="transparent" center=position!(latitude, longitude) radius={radius}>
        </Circle>
    }
}
