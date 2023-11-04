use futures::channel::mpsc::Sender;
use leptos::*;
use leptos_leaflet::{leaflet::MouseEvent, position, Circle, MouseEvents, Position};
use store::{rail_road::C, Event};

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

    let set_player_location =
        use_context::<WriteSignal<Position>>().expect("Expected a player location setter");

    // let add_todo_action = create_action(|input: &String| {
    //     let input = input.to_owned();
    //     async move { add_todo_request(&input).await }
    // });

    let tx = use_context::<Sender<Event>>().expect("Expected the tx sender");
    let move_player = move |event: MouseEvent| {
        set_player_location.update(|location| {
            *location = Position::new(event.latlng().lat(), event.latlng().lng());
        });
        tx.clone()
            .try_send(Event::DestinationCityRollRequest { player_id: 54 });
    };

    // let move_player = create_action(|input: &MouseEvent| async move {
    //     tx.clone()
    //         .try_send(Event::DestinationCityRollRequest { player_id: 54 })
    // });

    // let move_player = leptos::spawn_local(async move |event: MouseEvent| {
    //     tx.clone()
    //         .try_send(Event::DestinationCityRollRequest { player_id: 54 })
    // });

    view! {
        // TODO: Change to CircleMarker for consistent radius
        <Circle fill_opacity=1.0 mouse_events={MouseEvents::new().on_click(move_player)} fill_color={color} color="transparent" center=position!(latitude, longitude) radius={radius}>
        </Circle>
    }
}
