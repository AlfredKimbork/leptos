use crate::models::Todo;
use leptos::prelude::*;

#[component]
pub fn TodoStats(todos: Signal<Vec<Todo>>) -> impl IntoView {
    let remaining = move || todos.get().iter().filter(|todo| !todo.done).count();
    let all_done =
        Memo::new(move |_| !todos.get().is_empty() && todos.get().iter().all(|todo| todo.done));

    Effect::new(move || {
        leptos::logging::log! {
          "Todo list changed: {} remaining, all done ={}",
          remaining(),
          all_done.get()
        }
    });

    view! {
        <div class="todo-stats">
            <p class="todo-stats__count">
                "Remaining: " <strong>{remaining}</strong>
            </p>

            <p class="todo-stats__message">
                {move ||
                    if todos.get().is_empty() { "" }
                    else if all_done.get() { "All tasks completed." }
                    else { "Still working on it." }
                }
            </p>
        </div>
    }
}
