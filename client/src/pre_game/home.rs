use futures::channel::mpsc::Sender;

use leptos::*;
use store::{ClientMessage, Event, PlayerId};

use crate::pre_game::layout::Layout;

/// Page where player's can create a new game or join an existing game.
#[component]
pub fn Home() -> impl IntoView {
    let tx = use_context::<Sender<ClientMessage>>().expect("Expected the tx sender");
    let player_id = use_context::<ReadSignal<Option<PlayerId>>>().expect("Expected a player ID");

    view! {
        <Layout>
            <input type="submit" value="Create Game" on:click={
                let mut tx = tx.clone();
                move |_| {
                    let _ = tx
                        .try_send(ClientMessage::Event(Event::Create {
                            player_id: player_id.get().unwrap(),
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
