use futures::channel::mpsc::Sender;

use leptos::*;
use leptos_meta::Title;
use store::{ClientMessage, Event};

use crate::{app::PlayerId, pre_game::layout::Layout};

/// Page where player's can create a new game or join an existing game.
#[component]
pub fn Home() -> impl IntoView {
    let tx = use_context::<Sender<ClientMessage>>().expect("Expected the tx sender");
    let player_id = use_context::<PlayerId>().expect("Expected a player ID");

    view! {
        <Title text="Home"/>
        <Layout>
            <input type="submit" value="Create Game" on:click={
                let mut tx = tx.clone();
                move |_| {
                    let _ = tx
                        .try_send(ClientMessage::Event(Event::Create {
                            player_id: player_id.0.get().unwrap(),
                        }));
                }
            }/>
            <input type="submit" value="Join Lobby" on:click={|_| {
                let navigate = leptos_router::use_navigate();
                navigate("/join", Default::default());
            }} />
            <input type="submit" value="Rules" on:click={|_| {
                let navigate = leptos_router::use_navigate();
                navigate("/rules", Default::default());
            }}/>
        </Layout>
    }
}
