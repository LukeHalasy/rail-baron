use futures::channel::mpsc::Sender;

use leptos::{*, html::{Input, Select}};
use store::{Event, Piece, Player, PlayerId};
use web_sys::SubmitEvent;

use strum::IntoEnumIterator;

#[component]
pub fn Home() -> impl IntoView {
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
        <div class="relative h-screen w-screen flex justify-center items-center">
            <div class="absolute top-0 left-0 w-full h-full bg-cover bg-center" style="background-image: url('assets/images/rail-riches.png');"></div>
            <div class="fixed top-0 left-0 w-full h-full bg-black bg-opacity-40 flex justify-center items-center">
                <div class="fixed top-0 left-0 w-full h-full bg-black bg-opacity-50 flex justify-center items-start">
                    <form on:submit=join_game class="flex flex-col items-center mt-20">
                        <h1 class="text-center mb-10 text-8xl font-oldtimey underline-dashed text-blue-800" style="text-shadow: -1px -1px 0 #fff, 1px -1px 0 #fff, -1px 1px 0 #fff, 1px 1px 0 #fff;">Railway Riches</h1>
                        <div class="bg-gray-400 p-6">
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
                            <div class="flex flex-col items-center justify-center space-y-3">
                                <input type="submit" value="Create Game" class="bg-blue-800 text-gray-300 w-64 font-bold p-4 cursor-pointer button" />
                                <input type="submit" value="Join Lobby" class="bg-blue-800 text-gray-300 w-64 font-bold p-4 cursor-pointer button" />
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
