use leptos::*;
use leptos_router::*;

use crate::pre_game::layout::Layout;

#[derive(Params, PartialEq)]
struct LobbyParams {
    id: usize,
}

#[component]
pub fn Lobby() -> impl IntoView {
    let id = move || {
        use_params::<LobbyParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    view! {
        <Layout>
            // <input type="submit" value="Create Game" class="w-64 p-4 font-serif text-xl text-gray-300 bg-blue-800 cursor-pointer button" />
            // <input type="submit" value="Join Lobby" class="w-64 p-4 font-serif text-xl text-gray-300 bg-blue-800 cursor-pointer button" />
            <h1>{format!("Lobby {}", id())}</h1>
        </Layout>
    }
}