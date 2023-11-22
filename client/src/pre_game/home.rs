use futures::channel::mpsc::Sender;

use leptos::{*, html::{Input, Select}};
use leptos_router::{Router, Routes, Route};
use store::{Event, Piece, Player, PlayerId, ClientMessage};
use web_sys::{SubmitEvent, console};

use strum::IntoEnumIterator;

use crate::pre_game::layout::Layout;

/// Page where player's can create a new game or join an existing game.
#[component]
pub fn Home() -> impl IntoView {
    let player_id_input: NodeRef<Input> = create_node_ref();
    let name_input: NodeRef<Input> = create_node_ref();
    let piece_input: NodeRef<Select> = create_node_ref();
    
    let tx = use_context::<Sender<ClientMessage>>().expect("Expected the tx sender");
    let player_id = use_context::<ReadSignal<Option<PlayerId>>>().expect("Expected a player ID");
    let set_player_information = use_context::<WriteSignal<Option<Player>>>().expect("Expected a player information setter");
    let set_player_id = use_context::<WriteSignal<Option<PlayerId>>>().expect("Expected a player ID setter");

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
