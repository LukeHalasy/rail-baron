use leptos::*;
use leptos_meta::Title;
use leptos_router::*;
use store::PlayerId;

use crate::pre_game::layout::Layout;

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
    let player_id =
        use_context::<ReadSignal<Option<PlayerId>>>().expect("Expected a player id signal");

    view! {
        <Title text={move || format!("Lobby {}", lobby_id())} />
        <Layout>
            // <input type="submit" value="Create Game" class="w-64 p-4 font-serif text-xl text-gray-300 bg-blue-800 cursor-pointer button" />
            // <input type="submit" value="Join Lobby" class="w-64 p-4 font-serif text-xl text-gray-300 bg-blue-800 cursor-pointer button" />
            <h1>{move || format!("Lobby {}", lobby_id())}</h1>
            // display the players
            <ul>
            // {
            //     game_state.get().unwrap().players.into_iter()
            //         .map(|n| view! { <li>{n.0}</li>})
            //         .collect_view()
            // }
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=move || game_state.get().unwrap().players
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, _player)| {
                        let mut curr_id = id.to_string();
                        if game_state.get().unwrap().game_host == id {
                            curr_id += " (host)";
                        }

                        if id == player_id.get().unwrap() {
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
