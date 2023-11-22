use futures::channel::mpsc::Sender;
use leptos::{*, html::Input};
use web_sys::console;

use crate::pre_game::layout::Layout;
use store::ClientMessage;

#[component]
pub fn Join() -> impl IntoView {
    let lobby_input: NodeRef<Input> = create_node_ref();
    let tx = use_context::<Sender<ClientMessage>>().expect("Expected the tx sender");

    view! {
        <Layout>
            <input type="number" placeholder="Lobby #" autofocus node_ref=lobby_input />
            <input type="submit" value="Join" on:click={
                let mut tx = tx.clone(); 
                move |_| {
                    console::log_1(&"attempting it".into());
                    let value = lobby_input().expect("<input> to exist").value();
                    let _ = tx
                        .try_send(ClientMessage::JoinGame(
                            value.parse::<u64>().expect("Expected a valid lobby number"),
                        ));
                }
            }/>
        </Layout>
    }
}
