use leptos::*;

use store::{
    main_city,
    rail_road::C,
    sub_city::{self},
};

use crate::city::City;

#[component]
pub fn Cities() -> impl IntoView {
    view! {
        {
            main_city::City::cities()
            .iter()
            .map(|n| {
                view! {
                    <City city={C::D(*n)}></City>
                }
            }).collect_view()
        }
        {
            sub_city::SubCity::sub_cities()
            .iter()
            .map(|n| {
                view! {
                    <City city={C::P(*n)}></City>
                }
            }).collect_view()
        }
    }
}
