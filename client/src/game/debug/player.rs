use leptos::*;
use leptos_router::use_params;
use serde_json;

use crate::{app::PlayerId, pre_game::lobby::LobbyParams};

#[component]
pub fn PlayerDebug() -> impl IntoView {
    let _id = move || {
        use_params::<LobbyParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    let _player_id = use_context::<PlayerId>().expect("Expected a player id signal");
    let game_state =
        use_context::<ReadSignal<Option<store::State>>>().expect("Expected a game state signal");

    view! {
        <details>
            <summary>Players</summary>
            <div style="height:40vh;">
                <For each=move || game_state.get().unwrap().players key=|player_map| player_map.0 children=move |(_player_id, player)| {
                    view! {
                        <div style="display:inline-block; margin:0 1rem;">
                            <h2 class="text-center text-gray-300" style="background-color:#272822;">{player.name.clone()}</h2>
                            <br/>
                            <pre class="language-json">
                                <code class="language-jsonp">{serde_json::to_string_pretty(&player).unwrap()}</code>
                            </pre>
                            <br/>
                        </div>
                    }
                }/>
            </div>
        </details>
    }
}
