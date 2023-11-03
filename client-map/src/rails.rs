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
            .map(|n| {
                let (c0, c1) = n.0;
                let rail_roads = n.1;

                rail_roads.into_iter().map(|rr| {
                    let lat_0 = c0.coordinates().latitude();
                    let lon_0 = c0.coordinates().longitude();
                    let lat_1 = c1.coordinates().latitude();
                    let lon_1 = c1.coordinates().longitude();

                    let color: String = rail_colors.get(&rr).unwrap().into();

                    view! {
                        <Polyline color=color positions=positions(&[(lat_0, lon_0), (lat_1, lon_1)]) />
                    }
                }).collect_view()
            }).collect_view();
        }
    }
}
