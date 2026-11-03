use leptos::prelude::*;

#[component]
pub fn TodoForm(on_add: Callback<String>) -> impl IntoView {
    let (input_value, set_input_value) = signal(String::new());
    let (error, set_error) = signal(None::<String>);

    let submit = move |_| {
        let text = input_value.get_untracked();
        if text.trim().is_empty() {
            set_error.set(Some("Todo text cannot be empty".to_string()));
            return;
        }
        on_add.run(text);
        set_input_value.set(String::new());
        set_error.set(None);
    };

    view! {
        <div class="todo-form">
            <div class="todo-form__controls">
                <input
                    class="todo-form__input"
                    type="text"
                    placeholder="What needs to be done?"
                    prop:value=move || input_value.get()
                    on:input=move |event| {
                        set_input_value.set(event_target_value(&event))
                    }
                />
                <button
                    class="todo-form__button"
                    on:click=submit
                >
                    "Add task"
                </button>
            </div>

            {move || error.get().map(|msg| view! {
                <p class="todo-form__error">{msg}</p>
            }).into_any()}
        </div>
    }
}
