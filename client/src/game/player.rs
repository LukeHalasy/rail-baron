use leptos::*;
use leptos_leaflet::{Marker, Position};
use store::{rail::C, travel_payout::City};

#[component]
pub fn Player(player: store::Player) -> impl IntoView {
    let current_city = {
        if let Some((city, _)) = player.route_history.last() {
            *city
        } else if let Some(city) = player.start {
            C::D(city)
        } else if let Some(city) = player.home_city {
            C::D(city)
        } else {
            // TODO: This is a hack to get the game to compile. Instead throw an error message.
            C::D(City::Albany_NY)
        }
    };

    let position = Position::new(
        current_city.coordinates().latitude(),
        current_city.coordinates().longitude(),
    );

    let color = match player.piece {
        store::Piece::Red => "red",
        store::Piece::Blue => "blue",
        store::Piece::Green => "green",
        store::Piece::Yellow => "yellow",
        store::Piece::Purple => "violet",
        store::Piece::Orange => "orange",
    };

    view! {
        // <Marker position={position} icon_class={icon_class}></Marker>
        <Marker
            position={position}
            icon_url=format!("https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-{}.png", color)
            // needed to make the icon appear in the correct location
            icon_size=(25.0, 41.0)
            icon_anchor=(12.0, 41.0)>
        </Marker>
    }
}
