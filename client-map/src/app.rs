use std::time::Duration;

use leptos::*;
use leptos_leaflet::*;
use store::{deed, main_city, rail_road, sub_city};

#[component]
pub fn App() -> impl IntoView {
    let cities = main_city::City::cities();
    let sub_cities = sub_city::SubCity::sub_cities();

    create_effect(move |_| {
        set_interval_with_handle(
            move || {
                set_marker_position.update(|pos| {
                    pos.lat += 0.001;
                    pos.lng += 0.001;
                });
            },
            Duration::from_millis(200),
        )
        .ok()
    });

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
          //   <Marker position=marker_position >
          //       <Popup>
          //           <strong>{"A pretty CSS3 popup"}</strong>
          //       </Popup>
          //   </Marker>
          //     <Marker position=position!(51.5, -0.065) draggable=true >
          //       <Popup>
          //           <strong>{"A pretty CSS3 popup"}</strong>
          //       </Popup>
          //   </Marker>
          //   <Tooltip position=position!(51.5, -0.06) permanent=true direction="top">
          //       <strong>{"And a tooltip"}</strong>
          //   </Tooltip>
          //   <Polyline positions=positions(&[(51.505, -0.09), (51.51, -0.1), (51.51, -0.12)])/>
          //   <Polygon color="purple" positions=positions(&[ (51.515, -0.09), (51.52, -0.1), (51.52, -0.12)]) >
          //     <Tooltip sticky=true direction="top">
          //         <strong>{"I'm a polygon"}</strong>
          //     </Tooltip>
          // </Polygon>
          // <Circle center=position!(51.505, -0.09) color="blue" radius=200.0>
          //     <Tooltip sticky=true>{"I'm a circle"}</Tooltip>
          // </Circle>
        </MapContainer>
    }
}
