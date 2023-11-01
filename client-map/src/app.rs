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

    view! {
        <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(38.8951100, -77.0363700) zoom=5.0 set_view=true>
            // TODO: need to add attribution
            <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"/>
            {
                cities
                .into_iter()
                .map(|n| {
                    let latitude = n.coordinates().latitude();
                    let longitude = n.coordinates().longitude();
                    let radius = 9000.0;
                    let city_name = n.to_string();

                    view! {
                        <Circle color="black" center=position!(latitude, longitude) radius=radius>
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
                    let radius = 9000.0;
                    let city_name = n.to_string();

                    view! {
                        <Circle color="grey" center=position!(latitude, longitude) radius=radius>
                            <Popup>
                                <strong>{city_name}</strong>
                            </Popup>
                        </Circle>
                    }
                }).collect_view()
            }
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
        </MapContainer>
    }
}
