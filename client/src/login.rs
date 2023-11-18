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
        <div style="position: relative; height: 100vh; width: 100vw; display: flex; justify-content: center; align-items: center;">
            <div style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; background-image: url('assets/images/rail-riches.png'); background-size: 100% 100%;"></div>
            <div style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0, 0, 0, 0.5); display: flex; justify-content: center; align-items: center;">
                <div style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0, 0, 0, 0.5); display: flex; justify-content: center; align-items: flex-start;">
                    <form on:submit=join_game style="display: flex; flex-direction: column; align-items: center; margin-top: 20px;">
                        <h1 style="text-align: center; margin-bottom: 20px; font-family: 'OldTimey', sans-serif; font-size: 88px; color: #1A4176; text-decoration: underline dashed; text-shadow: -2px -2px 0 #999999, 2px -2px 0 #999999, -2px 2px 0 #999999, 2px 2px 0 #999999; color: #0000a0;">Railway Riches</h1>
                        <div style="background-color: rgba(153, 153, 153, 1); padding: 30px;">
                            // <input type="text" node_ref=player_id_input placeholder="Enter Player ID" pattern="[0-9]*" style="margin-bottom: 10px;"/>
                            // <input type="text" node_ref=name_input placeholder="Enter your name" style="margin-bottom: 10px;"/>
                            // <select node_ref=piece_input style="margin-bottom: 10px;">
                            //     {
                            //         Piece::iter()
                            //         .map(move |p| {
                            //             view! {
                            //                 <option value=p.to_string()>{p.to_string()}</option>
                            //             }
                            //         }).collect_view()
                            //     }
                            // </select>
                            // <input type="submit" value="Join Game"/>
                            <div style="display: flex; flex-direction: column; align-items: center; justify-content: center;">
                                <div style="display: flex; flex-direction: row; align-items: center; justify-content: center; margin-bottom: 10px;">
                                    <input type="text" placeholder="Enter Lobby #" style="background-color: #1A4176; color: white; font-size: 16px; padding: 10px 20px; margin-right: 10px; border: none; cursor: pointer;"/>
                                    <input type="submit" value="Join Lobby" style="background-color: #1A4176; color: white; font-size: 16px; padding: 10px 20px; border: none; cursor: pointer;"/>
                                </div>
                                <hr style="width: 100%; margin-bottom: 20px;" />
                                <button type="button" style="background-color: #1A4176; color: white; font-size: 16px; padding: 10px 20px; border: none; cursor: pointer;">Create Game</button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
