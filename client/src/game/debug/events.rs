use leptos::*;
use leptos_router::use_params;

use crate::{app::PlayerId, pre_game::lobby::LobbyParams};

#[component]
pub fn EventHistoryDebug() -> impl IntoView {
    let _id = move || {
        use_params::<LobbyParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    let _player_id = use_context::<PlayerId>().expect("Expected a player id signal");
    let game_state =
        use_context::<ReadSignal<Option<store::State>>>().expect("Expected a game state signal");

    view! {
        <details>
            <summary>Event History</summary>
            <div style="height:40vh;">
                <pre class="language-json">
                    <code class="language-json">{serde_json::to_string_pretty(&game_state.get().unwrap().history).unwrap()}</code>
                </pre>
            </div>
        </details>
    }
}
