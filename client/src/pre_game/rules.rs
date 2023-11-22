use leptos::*;
use leptos_meta::Title;

use crate::pre_game::layout::Layout;

#[component]
pub fn Rules() -> impl IntoView {
    view! {
        <Title text="Rules"/>
        <Layout>
            <h1>Rules coming soon!</h1>
            <input type="submit" value="Home" on:click={|_| {
                let navigate = leptos_router::use_navigate();
                navigate("/", Default::default());
            }} />
        </Layout>
    }
}
