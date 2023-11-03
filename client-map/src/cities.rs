use leptos::*;

use store::{
    main_city,
    rail_road::C,
    sub_city::{self},
};

use crate::city::City;

#[component]
pub fn Cities() -> impl IntoView {
    let cities = main_city::City::cities();
    let sub_cities = sub_city::SubCity::sub_cities();

    view! {
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
    }
}
