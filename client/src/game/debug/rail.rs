use leptos::*;
use leptos_router::use_params;

use crate::{app::PlayerId, pre_game::lobby::LobbyParams};

#[component]
pub fn RailDebug() -> impl IntoView {
    let _id = move || {
        use_params::<LobbyParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    let _player_id = use_context::<PlayerId>().expect("Expected a player id signal");
    let game_state =
        use_context::<ReadSignal<Option<store::State>>>().expect("Expected a game state signal");

    view! {
        <details>
            <summary>Rails</summary>
            <div style="height:40vh;">
                <ul>
                    <For each=move || game_state.get().unwrap().rail_ledger key=|ledger_map| ledger_map.0 children=move |(rail, _player)| {
                        let rail_clone = rail;
                        view! {
                            <li class="text-center text-gray-300" style="background-color:#272822;">
                                {move || format!("{} - ${}", rail_clone.full_name(), rail_clone.cost())}
                            </li>
                        }
                    }/>
                </ul>
            </div>
        </details>
    }
}
