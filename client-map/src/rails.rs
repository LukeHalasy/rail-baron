use std::collections::HashMap;

use leptos::*;

use leptos_leaflet::{positions, Polyline};
use store::{
    deed,
    rail_road::{self},
};

#[component]
pub fn Rails() -> impl IntoView {
    let mut rail_colors: HashMap<&deed::Deed, String> = HashMap::new();
    for rail in deed::Deed::rails() {
        let color = match rail {
            rail_road::Deed::B_AND_M => "#2C3073",
            rail_road::Deed::B_AND_O => "#0077B0",
            rail_road::Deed::NYC => "#295536",
            rail_road::Deed::NYNH_AND_H => "#662D2D",
            rail_road::Deed::PA => "#A12D2A",
            _ => "#ff007f",
        };
        rail_colors.insert(rail, color.into());
    }

    view! {
        {
            rail_road::Deed::get_edges()
            .into_iter()
            .map(|(city_pair, rail_roads)| {
                rail_roads.into_iter().enumerate().map(|(index, rail_road)| {
                    // Find slope between the cities
                    let c0_coors = (
                        city_pair.0.coordinates().latitude(),
                        city_pair.0.coordinates().longitude(),
                    );
                    let c1_coors = (
                        city_pair.1.coordinates().latitude(),
                        city_pair.1.coordinates().longitude(),
                    );

                    let slope = (c1_coors.1 - c0_coors.1) / (c1_coors.0 - c0_coors.0);

                    // need to move perpendicular to the slope of the line
                    // need to move a standard (distance) down (or up) the perpendicular line from each coordinate
                    // there are going to be different offsets (x and y)

                    // Calculate the perpendicular slope
                    let p_m = -1.0 / slope;

                    // Calculate the equation of the perpendicular line passing through (x0, y0)
                    // let p_b_0 = c0_coors.1 - (p_m * c0_coors.0);
                    // let p_b_1 = c0_coors.1 - (p_m * c0_coors.0);

                    let distance = (index as f64) * 0.01;
                    let lat_0 = c0_coors.0 + (distance / (1.0 + p_m * p_m)).sqrt();
                    let lon_0 = c0_coors.1 + p_m * (lat_0 - c0_coors.0);

                    let lat_1 = c1_coors.0 + (distance / (1.0 + p_m * p_m)).sqrt();
                    let lon_1 = c1_coors.1 + p_m * (lat_1 - c1_coors.0);

                    let color: String = rail_colors.get(&rail_road).unwrap().into();

                    view! {
                        <Polyline color={color} positions=positions(&[(lat_0, lon_0), (lat_1, lon_1)]) />
                    }
                }).collect_view()
            }).collect_view();
        }
    }
}
