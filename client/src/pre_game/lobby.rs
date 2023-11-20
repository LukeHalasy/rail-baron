use futures::channel::mpsc::Sender;

use leptos::{*, html::{Input, Select}};
use leptos_router::*;
use store::{Event, Piece, Player, PlayerId};
use web_sys::SubmitEvent;

use strum::IntoEnumIterator;

use crate::pre_game::layout::Layout;

#[derive(Params, PartialEq)]
struct LobbyParams {
    id: usize
}

#[component]
pub fn Lobby() -> impl IntoView {
    let id = move || {
        use_params::<LobbyParams>().with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .unwrap_or_default()
        })
    };

    view! {
        <Layout>
            // <input type="submit" value="Create Game" class="bg-blue-800 font-serif text-xl text-gray-300 w-64 p-4 cursor-pointer button" />
            // <input type="submit" value="Join Lobby" class="bg-blue-800 font-serif text-xl text-gray-300 w-64 p-4 cursor-pointer button" />
            <h1>{format!("Lobby {}", id())}</h1>
        </Layout>
    }
}
