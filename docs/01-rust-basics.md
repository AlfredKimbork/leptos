# Rust i todo-app

Kort studieguide til den eksisterende `todo-app`: Rust edition 2024 og Leptos 0.8 med client-side rendering (CSR).

## Typer og struct

Fra `todo-app/src/models/todo.rs`:

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub done: bool,
}
```

- `Uuid` identificerer en opgave. Nye opgaver får `Uuid::new_v4()`, ikke et løbenummer.
- `String` er ejet tekst. `"Add your first task".to_string()` laver en `String` fra en strengliteral af typen `&str`.
- `bool` er `true` eller `false`.
- `Vec<Todo>` er en liste af opgaver. `vec![...]` opretter en liste.
- `usize` er blandt andet returtypen fra `.len()` og `.count()`.
- `derive` genererer implementeringer: `Clone` gør kopiering mulig, og Serdes `Serialize`/`Deserialize` bruges til at omsætte opgaver til og fra JSON.

## Moduler og synlighed

I `src/main.rs` erklærer `mod models;` modulet. I `src/models/mod.rs` står:

```rust
mod todo;
pub use todo::Todo;
```

`mod` erklærer et modul; `use` gør et navn tilgængeligt i det aktuelle scope. `pub use` eksporterer navnet videre, så `main.rs` kan skrive `use models::Todo;`. `pub` på struct og felter giver adgang fra andre moduler inden for modulets synlighed.

## Funktioner, Option og Result

Storage-funktionerne i `src/services/storage.rs` har disse signaturer:

```rust
pub fn load_todos() -> Option<Vec<Todo>>
pub fn save_todos(todos: &[Todo])
```

`fn` definerer en funktion, og `->` angiver returtypen. `&[Todo]` er et lånt udsnit af opgaver. Uden angivet returtype returneres `()`.

- `Option<T>` er enten `Some(værdi)` eller `None`. Indlæsningen kan mangle data; `main.rs` bruger derfor `load_todos().unwrap_or_else(default_todos)` som fallback.
- `Result<T, E>` er enten `Ok(værdi)` eller `Err(fejl)`. Browserens storage-API og `serde_json` kan fejle.

Et aktuelt eksempel fra indlæsningen:

```rust
let storage = window.local_storage().ok()??;
```

`local_storage()` returnerer `Result<Option<Storage>, _>`. `.ok()` omdanner en fejl til `None`. De to `?` pakker de to `Option`-lag ud og returnerer tidligt med `None`, hvis et lag mangler. `serde_json::from_str(&todos_json).ok()` behandler også ugyldig JSON som manglende data.

Ved lagring bruges `if let Ok(json) = serde_json::to_string(todos)` til kun at fortsætte ved succes. En fejl fra `storage.set_item(...)` logges.

## Lister og iteratorer

I `src/main.rs` ændres listen gennem `set_todos.update(...)`:

```rust
// Tilføj: text flyttes ind i den nye opgave.
list.push(Todo {
    id: Uuid::new_v4(),
    text,
    done: false,
});

// Find en opgave, og skift dens status.
if let Some(todo) = list.iter_mut().find(|todo| todo.id == id) {
    todo.done = !todo.done;
}

// Behold alle andre opgaver: den valgte fjernes.
list.retain(|todo| todo.id != id);
```

`iter_mut()` giver adgang til at ændre elementerne. `find()` returnerer en `Option`, som `if let Some(...)` pakker ud. `|todo| ...` er en closure, altså en anonym funktion.

I `src/components/todo_stats.rs` tælles resterende opgaver med:

```rust
todos.get().iter().filter(|todo| !todo.done).count()
```

`iter()` låner elementerne, `filter()` vælger dem med `done == false`, og `count()` tæller dem. `.all(|todo| todo.done)` undersøger, om alle er færdige; koden kontrollerer også, at listen ikke er tom.
