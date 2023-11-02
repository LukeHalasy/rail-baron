use std::collections::HashMap;


use leptos::*;
use leptos_leaflet::{leaflet::MouseEvent, *};
use store::{
    deed, main_city,
    rail_road::{self, C},
    sub_city::{self},
};

use crate::city::City;

#[component]
pub fn App() -> impl IntoView {
    let cities = main_city::City::cities();
    let sub_cities = sub_city::SubCity::sub_cities();

    let mut rs: HashMap<&deed::Deed, String> = HashMap::new();

    // create a reactive signal with the initial value
    let (_value, _set_value) = create_signal("".to_string());
    let (location, set_location) = create_signal(Position::new(
        main_city::City::Albany_NY.coordinates().latitude(),
        main_city::City::Albany_NY.coordinates().longitude(),
    ));

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    // let clear = move |_| set_value.set("");

    // let rf = move |event: MouseEvent| set_value.update(|value| *value = event.to_string().into());
    let _rf = move |event: MouseEvent| {
        set_location.update(|location| {
            *location = Position::new(event.latlng().lat(), event.latlng().lng());
        })
    };

    for de in deed::Deed::rails() {
        // let color = RandomColor::new()
        //     .luminosity(random_color::Luminosity::Dark)
        //     .to_hex();
        let color = match de {
            rail_road::Deed::B_AND_M => "#2C3073",
            rail_road::Deed::B_AND_O => "#0077B0",
            rail_road::Deed::NYC => "#295536",
            rail_road::Deed::NYNH_AND_H => "#662D2D",
            rail_road::Deed::PA => "#A12D2A",
            _ => "#ff007f",
        };
        rs.insert(de, color.into());
    }

    let edges = rail_road::Deed::get_edges();
    let _player_one = main_city::City::Cleveland_OH;

    view! {
        <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(39.8283, -98.5795) zoom=5.0 min_zoom=5.0 set_view=true>
            // TODO: need to add attribution
            <TileLayer url="https://{s}.basemaps.cartocdn.com/light_nolabels/{z}/{x}/{y}{r}.png"/>
            {
                edges
                .into_iter()
                .map(|n| {
                    let (c0, c1) = n.0;
                    let rail_roads = n.1;

                    rail_roads.into_iter().map(|rr| {
                        let lat_0 = c0.coordinates().latitude();
                        let lon_0 = c0.coordinates().longitude();
                        let lat_1 = c1.coordinates().latitude();
                        let lon_1 = c1.coordinates().longitude();

                        let color: String = rs.get(&rr).unwrap().into();

                        view! {
                            <Polyline color=color positions=positions(&[(lat_0, lon_0), (lat_1, lon_1)]) />
                        }
                    }).collect_view()
                }).collect_view();
            }
            {
                cities
                .into_iter()
                .map(|n| {
                    view! {
                        <City city={C::D(*n)}></City>
                    }
                }).collect_view()
            }
            {
                sub_cities
                .into_iter()
                .map(|n| {
                    view! {
                        <City city={C::P(*n)}></City>
                    }
                }).collect_view()
            }
            <Marker title="player_one" position={location}>
            </Marker>
        </MapContainer>
    }
}
