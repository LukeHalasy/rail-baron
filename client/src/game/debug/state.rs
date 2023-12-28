use leptos::*;
use leptos_router::use_params;

use crate::{app::PlayerId, pre_game::lobby::LobbyParams};

#[component]
pub fn StateDebug() -> impl IntoView {
    let _id = move || {
        use_params::<LobbyParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    let game_state =
        use_context::<ReadSignal<Option<store::State>>>().expect("Expected a game state signal");

    let game_state_debug = move || {
        if game_state.get().is_some() {
            view! {
                <code class="language-json">{ serde_json::to_string_pretty(&game_state.get().unwrap()).unwrap() }</code>
            }
            .into_view()
        } else {
            view! {}.into_view()
        }
    };

    view! {
        <details>
            <summary>Game State</summary>
            <div style="height:40vh;">
                <pre class="language-json">
                    { game_state_debug }
                </pre>
            </div>
        </details>
    }
}
