use crate::components::TodoItem;
use crate::models::Todo;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn TodoList(
    todos: Signal<Vec<Todo>>,
    on_toggle: Callback<Uuid>,
    on_delete: Callback<Uuid>,
) -> impl IntoView {
    view! {
        {move || if todos.get().is_empty() {
            view! {
                <div class="todo-list__empty">
                    <p>"Nothing to do yet."</p>
                </div>
            }.into_any()
        } else {
            view! {
                <ul class="todo-list">
                    <For
                        each=move || todos.get()
                        key=|todo| todo.id
                        children=move |todo: Todo| view! {
                            <li class="todo-list__entry">
                                <TodoItem
                                    todo=todo
                                    on_toggle=on_toggle
                                    on_delete=on_delete
                                />
                            </li>
                        }
                    />
                </ul>
            }.into_any()
        }}
    }
}
