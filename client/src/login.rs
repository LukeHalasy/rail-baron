use futures::channel::mpsc::Sender;

use leptos::{*, html::{Input, Select}};
use store::{Event, Piece, Player, PlayerId};
use web_sys::SubmitEvent;

use strum::IntoEnumIterator;

#[component]
pub fn Login() -> impl IntoView {
    let player_id_input: NodeRef<Input> = create_node_ref();
    let name_input: NodeRef<Input> = create_node_ref();
    let piece_input: NodeRef<Select> = create_node_ref();
    
    let tx = use_context::<Sender<Event>>().expect("Expected the tx sender");
    let set_player_information = use_context::<WriteSignal<Option<Player>>>().expect("Expected a player information setter");
    let set_player_id = use_context::<WriteSignal<Option<PlayerId>>>().expect("Expected a player ID setter");

    let join_game = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let player_id = player_id_input()
            .expect("id <input> to exist")
            .value()
            .parse::<PlayerId>()
            .expect("Failed to parse player ID");

        let name = name_input().expect("name <input> to exist").value();

        let piece = piece_input()
            .expect("<select> to exist")
            .value();

        let _ = tx
            .clone()
            .try_send(Event::PlayerJoined {
                player_id,
                name: name.clone(),
                piece: Piece::from_str(&piece).unwrap(),
            });

        // TODO: Add error handling in case request fails
        // Update the player information
        set_player_information.update(|player| {
            *player = Some(Player {
                name: name.clone(),
                piece: Piece::from_str(&piece).unwrap(),
                ..Default::default()
            });
        });

        set_player_id.update(|id| {
            *id = Some(player_id);
        });
        
        let navigate = leptos_router::use_navigate();
        navigate("/map", Default::default());
    };
    
    view! {
        <div>
            <h1>"Welcome to the game!"</h1>
            <form on:submit=join_game>
                <input type="text" node_ref=player_id_input placeholder="Enter Player ID" pattern="[0-9]*"/>
                <input type="text" node_ref=name_input placeholder="Enter your name"/>
                <select node_ref=piece_input>
                    {
                        Piece::iter()
                        .map(move |p| {
                            view! {
                                <option value=p.to_string()>{p.to_string()}</option>
                            }
                        }).collect_view()
                    }
                </select>
                <input type="submit" value="Join Game"/>
            </form>
        </div>
    }
}
