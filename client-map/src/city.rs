use leptos::*;
use leptos_leaflet::{position, Circle};
use store::rail_road::C;

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

    view! {
        // TODO: Change to CircleMarker for consistent radius
        <Circle fill_opacity=1.0 fill_color={color} color="transparent" center=position!(latitude, longitude) radius={radius}>
        </Circle>
    }
}
