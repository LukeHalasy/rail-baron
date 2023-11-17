use leptos::*;

mod app;
mod cities;
mod city;
mod rails;
mod login;
mod player;
mod map;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {<App/>}
    });
}
