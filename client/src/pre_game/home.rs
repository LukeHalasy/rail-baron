use futures::channel::mpsc::Sender;

use leptos::{*, html::{Input, Select}};
use leptos_router::{Router, Routes, Route};
use store::{Event, Piece, Player, PlayerId};
use web_sys::{SubmitEvent, console};

use strum::IntoEnumIterator;

use crate::pre_game::layout::Layout;

/// Page where player's can create a new game or join an existing game.
#[component]
pub fn Home() -> impl IntoView {
    let player_id_input: NodeRef<Input> = create_node_ref();
    let name_input: NodeRef<Input> = create_node_ref();
    let piece_input: NodeRef<Select> = create_node_ref();
    
    let tx = use_context::<Sender<Event>>().expect("Expected the tx sender");
    let player_id = use_context::<ReadSignal<Option<PlayerId>>>().expect("Expected a player ID");
    let set_player_information = use_context::<WriteSignal<Option<Player>>>().expect("Expected a player information setter");
    let set_player_id = use_context::<WriteSignal<Option<PlayerId>>>().expect("Expected a player ID setter");

   
    // this all would be much easier with accounts...

    // by the end of this PR, I want to be able to do this:
    // Clicking the "Create Game" button will call a method within the server to create a new game.
    // The server will return a game ID and take the user to /lobby/<game_id>.
    // so as not to need to deal with names for now (and to make it easier to test), the game ID will be the player ID.
    // each subsequent player will be able to join the game by entering the game ID into the "Join Lobby" input.
    // the server will then add the player to the game and take them to /lobby/<game_id>.
    // their name, and all other connected players, will be displayed on the page.
    // the host will be able to start the game, which will take them to /map/<game_id>.

    // within the page

    // let create_game = move |_| {
        // print that the player is attempting to create a game


        // Update the player information
        // set_player_information.update(|player| {
        //     *player = Some(Player {
        //         name: name.clone(),
        //         piece: Piece::from_str(&piece).unwrap(),
        //         ..Default::default()
        //     });
        // });

        // set_player_id.update(|id| {
        //     *id = Some(player_id);
        // });
        
        // let navigate = leptos_router::use_navigate();
        // navigate("/lobby", Default::default());
    // };

    view! {
        <Layout>
            <input type="submit" value="Create Game" on:click={let mut tx = tx.clone(); move |_| {
                let _ = tx
                    .try_send(Event::Create {
                        player_id: player_id.get().unwrap(),
                    });
            }}/>
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
