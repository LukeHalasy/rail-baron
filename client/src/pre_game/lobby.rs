use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

use crate::{app::PlayerId, pre_game::layout::Layout};

#[derive(Params, PartialEq)]
pub struct LobbyParams {
    pub id: usize,
}

#[component]
pub fn Lobby() -> impl IntoView {
    let lobby_id = move || {
        use_params::<LobbyParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    // pull in the game state
    let game_state =
        use_context::<ReadSignal<Option<store::State>>>().expect("Expected a game state signal");

    // pull in the player id
    let player_id = use_context::<PlayerId>().expect("Expected a player id signal");

    view! {
        <Title text={move || format!("Lobby {}", lobby_id())} />
        <Layout>
            <h1>{move || format!("Lobby {}", lobby_id())}</h1>
            <ul>
                <For
                    each=move || game_state.get().unwrap().players
                    key=|counter| counter.0
                    children=move |(id, _player)| {
                        let mut curr_id = id.to_string();
                        if game_state.get().unwrap().game_host == id {
                            curr_id += " (host)";
                        }

                        if id == player_id.0.get().unwrap() {
                            view! {
                                <li>
                                    <strong>{ curr_id }</strong>
                                </li>
                            }
                        } else {
                            view! {
                                <li>
                                    { curr_id }
                                </li>
                            }
                        }
                    }
                />
            </ul>
            <input type="submit" value="Start Game" />
        </Layout>
    }
}
