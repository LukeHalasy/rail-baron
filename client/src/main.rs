use leptos::*;

mod app;
mod game;
mod pre_game;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {<App/>}
    });
}
