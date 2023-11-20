use leptos::*;

use crate::pre_game::layout::Layout;

#[component]
pub fn Join() -> impl IntoView {
    view! {
        <Layout>
            <input type="text" placeholder="Lobby #" autofocus />
            <input type="submit" value="Join" />
        </Layout>
    }
}
