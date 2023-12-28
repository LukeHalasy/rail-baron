use futures::channel::mpsc::Sender;
use leptos::*;
use leptos_leaflet::{MapContainer, Position, TileLayer};
use leptos_meta::Title;
use leptos_router::use_params;
use store::{ClientMessage, Event};

use crate::{
    app::PlayerId,
    game::{
        cities::Cities,
        debug::{
            events::EventHistoryDebug, player::PlayerDebug, rail::RailDebug, state::StateDebug,
        },
        player::Player,
        rails::Rails,
    },
    pre_game::lobby::LobbyParams,
};

#[component]
pub fn Game() -> impl IntoView {
    let id = move || {
        use_params::<LobbyParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    let player_id = use_context::<PlayerId>().expect("Expected a player id signal");
    let game_state =
        use_context::<ReadSignal<Option<store::State>>>().expect("Expected a game state signal");
    let tx = use_context::<Sender<ClientMessage>>().expect("Expected the tx sender");

    let players = move || {
        if game_state.get().is_some() {
            view! {
                <For
                    each=move || game_state.get().unwrap().players
                    key=|player_map| player_map.0
                    children=move |(_id, player)| {
                        view! {
                            <Player player={player.clone()}></Player>
                        }
                    }
                />
            }
            .into_view()
        } else {
            view! {}.into_view()
        }
    };

    let player_info = move || {
        if game_state.get().is_some() {
            view! {
                <For
                    each=move || game_state.get().unwrap().players
                    key=|player_map| player_map.0
                    children=move |(id, player)| {
                        let home_city = player.home_city.map(|h| h.to_string()).unwrap_or_else(|| "None".to_string());
                        let start = player.start.map(|s| s.to_string()).unwrap_or_else(|| "None".to_string());
                        let destination = player.destination.map(|d| d.to_string()).unwrap_or_else(|| "None".to_string());
                        let piece = player.piece.unwrap().to_string();
                        let spaces_left_to_move = player.spaces_left_to_move.unwrap_or(0);

                        view! {
                            <div style="width:25%; float:left;">
                                <h1>{player.name.clone()}</h1>
                                <h1>{player.cash}</h1>
                                <h1>{piece}</h1>
                                <h1>Home: {home_city}</h1>
                                <h1>Route: {start} -> {destination}</h1>
                                <h1>Spaces Left: {spaces_left_to_move}</h1>
                                // <h1>{move || player.piece.unwrap().clone()}</h1>
                                // <h1>{player.piece.unwrap().clone()}</h1>
                            </div>
                        }
                    }
                />
            }
            .into_view()
        } else {
            view! {}.into_view()
        }
    };

    let status = move || {
        if game_state.get().unwrap().active_player_id == Some(player_id.0.get().unwrap()) {
            if let store::Stage::InGame(stage) = game_state.get().unwrap().stage {
                match stage {
                    store::InGameStage::OrderRoll => view! {
                        <input type="submit" value="Roll for order" on:click={
                            let mut tx = tx.clone();
                            move |_| {
                                let _ = tx
                                    .try_send(ClientMessage::Event(Event::OrderRollRequest {
                                        player_id: player_id.0.get().unwrap(),
                                    }));
                            }
                        }/>
                    }
                    .into_view(),
                    store::InGameStage::HomeRoll => view! {
                        <input type="submit" value="Roll for home" on:click={
                            let mut tx = tx.clone();
                            move |_| {
                                let _ = tx
                                    .try_send(ClientMessage::Event(Event::HomeRollRequest {
                                        player_id: player_id.0.get().unwrap(),
                                    }));
                            }
                        }/>
                    }
                    .into_view(),
                    store::InGameStage::DestinationRoll => view! {
                        <input type="submit" value="Roll for destination" on:click={
                            let mut tx = tx.clone();
                            move |_| {
                                let _ = tx
                                    .try_send(ClientMessage::Event(Event::DestinationRollRequest {
                                        player_id: player_id.0.get().unwrap(),
                                    }));
                            }
                        }/>
                    }
                    .into_view(),
                    store::InGameStage::MovementRoll => view! {
                        <input type="submit" value="Roll for movement" on:click={
                            let mut tx = tx.clone();
                            move |_| {
                                let _ = tx
                                    .try_send(ClientMessage::Event(Event::MovementRollRequest {
                                        player_id: player_id.0.get().unwrap(),
                                    }));
                            }
                        }/>
                    }
                    .into_view(),
                    store::InGameStage::BankruptcyHandling => todo!(),
                    store::InGameStage::BankruptcyAuction => todo!(),
                    store::InGameStage::DeclareOption => todo!(),
                    store::InGameStage::Movement => view! {
                        <p>Selecting city to move to..</p>
                    }
                    .into_view(),
                    store::InGameStage::Purchase => todo!(),
                }
            } else {
                view! {
                    <p>Unexpected Stage..</p>
                }
                .into_view()
            }

            // view! {
            //     <input type="submit" value="Roll Dice" on:click={
            //         let mut tx = tx.clone();
            //         move |_| {
            //             let _ = tx
            //                 .try_send(ClientMessage::RollDice(id().try_into().unwrap()));
            //         }
            //     }/>
            // }.into_view()
        } else {
            view! {
                <p>Waiting for other players moves..</p>
            }
            .into_view()
        }
    };

    view! {
        <Title text={move || format!("Game {}", id())} />
        <main>
            <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(39.8283, -98.5795) zoom=4.0 max_zoom=7.5 min_zoom=4.0 set_view=true>
                // TODO: need to add attribution
                <TileLayer url="https://{s}.basemaps.cartocdn.com/light_nolabels/{z}/{x}/{y}{r}.png"/>

                <Rails></Rails>
                <Cities></Cities>

                { players }
            </MapContainer>
            // create a div that displays the active_player_id + the current stage
            <div style="position:absolute; left:0; top: 0; width:100vw; background-color:rgba(0,0,0,0.9); color:white; z-index:9999; overflow:auto;">
                <h1>{move || format!("Stage: {:?}", game_state.get().unwrap().stage)}</h1>
                <h1>{move || format!("Active Player: {:?}", game_state.get().unwrap().active_player_id)}</h1>
                { status }
                // add a div that has four divs within it each of which takes up 25% of the width
                <div>
                    { player_info }
                </div>
            </div>

            // create a div that displays each player's name and cash at the bottom of the screen
            // <div style="position:absolute; left:0; bottom: 0; width:100vw; background-color:rgba(0,0,0,0.5); color:white; z-index:9999; overflow:auto;">
            //     // <RailDebug></RailDebug>
            //     // <PlayerDebug></PlayerDebug>
            //     // <EventHistoryDebug></EventHistoryDebug>
            //     // <StateDebug></StateDebug>
            //     { player_info }
            // </div>
        </main>
    }
}
