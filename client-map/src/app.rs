use std::collections::HashMap;

use leptos::*;
use leptos_leaflet::*;
use store::{
    deed, main_city,
    rail_road::{self},
    sub_city::{self},
};

use crate::{cities::Cities, rails::Rails};

#[component]
pub fn App() -> impl IntoView {
    let (player_location, set_player_location) = create_signal(Position::new(
        main_city::City::Albany_NY.coordinates().latitude(),
        main_city::City::Albany_NY.coordinates().longitude(),
    ));
    provide_context(set_player_location);

    view! {
        <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(39.8283, -98.5795) zoom=5.0 min_zoom=5.0 set_view=true>
            // TODO: need to add attribution
            <TileLayer url="https://{s}.basemaps.cartocdn.com/light_nolabels/{z}/{x}/{y}{r}.png"/>

            <Rails></Rails>
            <Cities></Cities>

            <Marker position={player_location}></Marker>
        </MapContainer>
    }
}
