use leptos::*;
use leptos_leaflet::{position, Circle, Marker, Position};
use store::{rail::C, travel_payout::City};

#[component]
pub fn Player(player: store::Player) -> impl IntoView {
    let color = match player.piece {
        Some(store::Piece::Red) => "red",
        Some(store::Piece::Blue) => "blue",
        Some(store::Piece::Green) => "green",
        Some(store::Piece::Yellow) => "yellow",
        Some(store::Piece::Purple) => "violet",
        Some(store::Piece::Orange) => "orange",
        // TODO: Handle no piece
        _ => "black",
    };

    // let home_city = move || {
    //     if let Some(city) = player.home_city {
    //         view! {
    //             <Marker
    //                 position=Position::new(
    //                     city.coordinates().latitude(),
    //                     city.coordinates().longitude(),
    //                 )
    //                 icon_url=format!("https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-{}.png", color)
    //                 // needed to make the icon appear in the correct location
    //                 icon_size=(25.0, 41.0)
    //                 icon_anchor=(12.0, 41.0)>
    //             ></Marker>
    //         }
    //     } else {
    //         view! {}.into_view()
    //     }
    // };

    let destination_city = move || {
        if let Some(city) = player.destination {
            view! {
                <Marker
                    position=Position::new(
                        city.coordinates().latitude(),
                        city.coordinates().longitude(),
                    )
                    icon_url=format!("https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-{}.png", color)
                    // needed to make the icon appear in the correct location
                    icon_size=(25.0, 41.0)
                    icon_anchor=(12.0, 41.0)>
                ></Marker>
            }
        } else {
            view! {}.into_view()
        }
    };

    let current_city = move || {
        if let Some(city) = player.clone().current_city() {
            view! {
                <Circle fill_opacity=1.0 fill_color={color} color="black" center=position!(city.coordinates().latitude(), city.coordinates().longitude()) radius={27000.0}>
                </Circle>
            }
        } else {
            view! {}.into_view()
        }
    };

    view! {
        // { home_city }
        { current_city }
        { destination_city }
    }
}
