use leptos::prelude::*;

#[component]
pub fn Panel(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="panel">
            <h2 class="panel__title">{title}</h2>
            {children()}
        </section>
    }
}
