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
                // TODO: Come up with a better key
                <For each=move || game_state.get().unwrap().history key=|event| event.player_id children=move |(index, event)| {
                    let event_clone = event;
                    view! {
                        <div style="display:inline-block; margin:0 1rem;">
                            <pre class="language-json">
                                <code class="language-jsonp">{serde_json::to_string_pretty(&event_clone).unwrap()}</code>
                            </pre>
                            <br/>
                        </div>
                    }
                }/>
            </div>
        </details>
    }
}
