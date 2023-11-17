use leptos::{*};
use leptos_leaflet::{Position, MapContainer, TileLayer};
use leptos_router::A;

use crate::{rails::Rails, cities::Cities, player::Player};

#[component]
pub fn Map() -> impl IntoView {
    // TODO: throw an error if the player is not set (which maens a user navigated to /map without logging in)
    // in which case navigate them back to the login page
    let player = use_context::<ReadSignal<Option<store::Player>>>().expect("A player signal should exist");
    
    // create_effect(move |_| {
    //     if player.clone().is_none() {
    //         let navigate = leptos_router::use_navigate();
    //         navigate("/", Default::default());
    //     }
    // });
    
    view! {
        <main>
        {
            move || 
                if player.get().is_none() {
                    view! {
                        // <div>"You are not logged in. Please "<a href="/">log in</a>" to view the map."</div>
                        <div>
                            <p>you are not logged in</p>
                            <A href="/">log in</A>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <MapContainer style="top:0;left:0;height:100vh;width:100vh,position:absolute" center=Position::new(39.8283, -98.5795) zoom=5.0 max_zoom=7.5 min_zoom=5.0 set_view=true>
                            // TODO: need to add attribution
                            <TileLayer url="https://{s}.basemaps.cartocdn.com/light_nolabels/{z}/{x}/{y}{r}.png"/>

                            <Rails></Rails>
                            <Cities></Cities>

                            {
                                if player.get().unwrap().route_history.len() > 0 {
                                    view! {
                                        <Player player={player.get().unwrap()}></Player>
                                    }.into_view()
                                } else { 
                                    view! {}.into_view()
                                }
                            }
                        </MapContainer>
                    }.into_view()
                }
            
        }
        </main>
    }
}
