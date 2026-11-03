use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Weather App"</h1>
        <p>"This is a simple weather app built with Leptos."</p>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
