use leptos::*;
use leptos_leaflet::{MapContainer, Position, TileLayer};
use leptos_meta::Title;
use leptos_router::use_params;

use crate::{
    app::PlayerId,
    game::{cities::Cities, player::Player, rails::Rails},
    pre_game::lobby::LobbyParams,
};

#[component]
pub fn Game() -> impl IntoView {
    let id = move || {
        use_params::<LobbyParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    let _player_id = use_context::<PlayerId>().expect("Expected a player id signal");
    let game_state =
        use_context::<ReadSignal<Option<store::State>>>().expect("Expected a game state signal");

    view! {
        <Title text={move || format!("Game {}", id())} />
        <main>
            <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(39.8283, -98.5795) zoom=5.0 max_zoom=7.5 min_zoom=5.0 set_view=true>
                // TODO: need to add attribution
                <TileLayer url="https://{s}.basemaps.cartocdn.com/light_nolabels/{z}/{x}/{y}{r}.png"/>

                <Rails></Rails>
                <Cities></Cities>

                <For
                    each=move || game_state.get().unwrap().players
                    key=|player_map| player_map.0
                    children=move |(_id, player)| {
                        if player.route_history.is_empty() {
                            view! {}.into_view()
                        } else {
                            view! {
                                <Player player={player.clone()}></Player>
                            }
                        }
                    }
                />
            </MapContainer>
        </main>
    }
}
