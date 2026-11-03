mod components;
mod models;
mod services;
use components::{Panel, TodoForm, TodoList, TodoStats};
use models::Todo;
use services::{load_todos, save_todos};

use leptos::prelude::*;
use uuid::Uuid;

#[component]
fn App() -> impl IntoView {
    let default_todos = || {
        vec![
            Todo {
                id: Uuid::new_v4(),
                text: "Add your first task".to_string(),
                done: false,
            },
            Todo {
                id: Uuid::new_v4(),
                text: "Mark a task as completed".to_string(),
                done: false,
            },
            Todo {
                id: Uuid::new_v4(),
                text: "Delete a task".to_string(),
                done: false,
            },
        ]
    };

    let (todos, set_todos) = signal(load_todos().unwrap_or_else(default_todos));

    Effect::new(move |_| {
        todos.with(|list| {
            save_todos(list);
        });
    });

    let add_todo = Callback::new(move |text: String| {
        set_todos.update(move |list| {
            list.push(Todo {
                id: Uuid::new_v4(),
                text,
                done: false,
            });
        });
    });

    let toggle = Callback::new(move |id: Uuid| {
        set_todos.update(move |list| {
            if let Some(todo) = list.iter_mut().find(|todo| todo.id == id) {
                todo.done = !todo.done;
            }
        });
    });

    let delete = Callback::new(move |id: Uuid| {
        set_todos.update(move |list| {
            list.retain(|todo| todo.id != id);
        });
    });

    view! {
        <main class="app">
            <header class="app__header">
                <span class="app__eyebrow">"YOUR WORKSPACE"</span>
                <h1 class="app__title">"My Todos"</h1>
                <p class="app__subtitle">
                    "A simple place to keep track of your tasks."
                </p>
            </header>

            <TodoForm on_add=add_todo />

            <Panel title="Today's Todos">
                <TodoStats todos=todos.into() />
                <TodoList
                    todos=todos.into()
                    on_toggle=toggle
                    on_delete=delete
                />
            </Panel>
        </main>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
