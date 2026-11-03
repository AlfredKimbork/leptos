use crate::models::Todo;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn TodoItem(todo: Todo, on_toggle: Callback<Uuid>, on_delete: Callback<Uuid>) -> impl IntoView {
    let id = todo.id;
    view! {
        <div class="todo-item">
            <input
                class="todo-item__checkbox"
                type="checkbox"
                checked=todo.done
                on:change=move |_| on_toggle.run(id)
            />

            <span
                class="todo-item__text"
                class:todo-item__text--done=todo.done
            >
                {todo.text}
            </span>

            <button
                class="todo-item__delete"
                on:click=move |_| on_delete.run(id)
                aria-label="Delete task"
            >
                "Delete"
            </button>
        </div>
    }
}
