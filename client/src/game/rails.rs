use std::collections::HashMap;

use leptos::*;

use leptos_leaflet::{positions, Polyline};
use store::rail::{self};

#[component]
pub fn Rails() -> impl IntoView {
    let mut rail_colors: HashMap<&rail::Rail, String> = HashMap::new();
    for rail in rail::Rail::rails() {
        let color = match rail {
            rail::Rail::B_AND_M => "#2C3073",
            rail::Rail::B_AND_O => "#0077B0",
            rail::Rail::NYC => "#295536",
            rail::Rail::NYNH_AND_H => "#A12D2A",
            rail::Rail::UP => "#A12D2A",
            rail::Rail::PA => "#FF0000",
            rail::Rail::C_AND_O => "#9E913F",
            rail::Rail::SP => "#9E913F",
            rail::Rail::WP => "#000000",
            rail::Rail::RF_AND_P => "#5631AF",
            rail::Rail::ACL => "#77130D",
            rail::Rail::N_AND_W => "#000000",
            rail::Rail::SAL => "#FFA500",
            rail::Rail::SOU => "#00A86B",
            rail::Rail::T_AND_P => "#800080",
            rail::Rail::L_AND_N => "#0077B0",
            rail::Rail::IC => "#88100D",
            rail::Rail::D_AND_RGW => "#FF0000",
            rail::Rail::CMSTP_AND_P => "#DA70D6",
            rail::Rail::AT_AND_SF => "#0000FF",
            rail::Rail::NP => "#5631AF",
            rail::Rail::CB_AND_Q => "#FFA500",
            rail::Rail::GN => "#0077B0",
            rail::Rail::C_AND_NW => "#85ae2b",
            rail::Rail::MP => "#85ae2b",
            rail::Rail::SLSF => "#DA70D6",
            rail::Rail::GM_AND_O => "#5631AF",
            rail::Rail::CRI_AND_P => "#000000",
        };
        rail_colors.insert(rail, color.into());
    }

    view! {
        {
            rail::Rail::get_edges()
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
            }).collect_view()
        }
    }
}
