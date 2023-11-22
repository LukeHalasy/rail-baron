use leptos::*;
use leptos_leaflet::{Marker, Position};

#[component]
pub fn Player(player: store::Player) -> impl IntoView {
    let (current_city, _) = player
        .route_history
        .last()
        .expect("The Player should have a route history");
    let position = Position::new(
        current_city.coordinates().latitude(),
        current_city.coordinates().longitude(),
    );

    view! {
        <Marker position={position}></Marker>
    }
}
