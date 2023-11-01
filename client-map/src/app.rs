use std::collections::HashMap;
use std::time::Duration;

use leptos::*;
use leptos_leaflet::*;
use store::{deed, main_city, rail_road, sub_city};

#[component]
pub fn App() -> impl IntoView {
    let cities = main_city::City::cities();
    let sub_cities = sub_city::SubCity::sub_cities();

    let mut rs: HashMap<&deed::Deed, String> = HashMap::new();
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

    let d = rail_road::Deed::get_edges();
    let player_one = main_city::City::Cleveland_OH;

    view! {
        <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(39.8283, -98.5795) zoom=5.0 min_zoom=5.0 set_view=true>
            // TODO: need to add attribution
            <TileLayer url="https://{s}.basemaps.cartocdn.com/light_nolabels/{z}/{x}/{y}{r}.png"/>
            {
                d
                .into_iter()
                .map(|n| {
                    n.1.into_iter().map(|h| {
                        let c0_coors = (
                            n.0.0.coordinates().latitude(),
                            n.0.0.coordinates().longitude(),
                        );
                        let c1_coors = (
                            n.0.1.coordinates().latitude(),
                            n.0.1.coordinates().longitude(),
                        );

                        let lat_0 = c0_coors.0;
                        let lon_0 = c0_coors.1;
                        let lat_1 = c1_coors.0;
                        let lon_1 = c1_coors.1;

                        let color: String = rs.get(&h).unwrap().into();

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
                    let latitude = n.coordinates().latitude();
                    let longitude = n.coordinates().longitude();
                    let city_name = n.to_string();

                    view! {
                        // TODO: Change to CircleMarker for consistent radius
                        <Circle fill_opacity=1.0 fill_color="black" color="transparent" center=position!(latitude, longitude) radius=17000.0>
                            <Popup>
                                <strong>{city_name}</strong>
                            </Popup>
                        </Circle>
                    }
                }).collect_view()
            }
            {
                sub_cities
                .into_iter()
                .map(|n| {
                    let latitude = n.coordinates().latitude();
                    let longitude = n.coordinates().longitude();
                    let city_name = n.to_string();

                    view! {
                        // TODO: Change to CircleMarker for consistent radius
                        <Circle fill_opacity=1.0 fill_color="grey" color="transparent" center=position!(latitude, longitude) radius=9000.0>
                            <Popup>
                                <strong>{city_name}</strong>
                            </Popup>
                        </Circle>
                    }
                }).collect_view()
            }
            <Marker title="player_one" position=position!({player_one.coordinates().latitude()}, {player_one.coordinates().longitude()})>
            </Marker>
        </MapContainer>
    }
}
