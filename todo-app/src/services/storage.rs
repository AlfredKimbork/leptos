use crate::models::Todo;

const STORAGE_KEY: &str = "todos";

pub fn load_todos() -> Option<Vec<Todo>> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let todos_json = storage.get_item(STORAGE_KEY).ok()??;

    serde_json::from_str(&todos_json).ok()
}

pub fn save_todos(todos: &[Todo]) {
    let Some(window) = web_sys::window() else {
        return;
    };

    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };

    if let Ok(json) = serde_json::to_string(todos) {
        if let Err(error) = storage.set_item(STORAGE_KEY, &json) {
            leptos::logging::log!("Failed to save todos to local storage: {:?}", error);
        }
    }
}
