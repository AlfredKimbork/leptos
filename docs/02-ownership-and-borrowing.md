# Ownership og borrowing i todo-app

Rust styrer ejerskab og lån; Leptos-signaler styrer reaktivitet. Det er to forskellige mekanismer.

## Ejerskab og move

En `Todo` ejer sin `String`. I `todo-app/src/main.rs` flyttes teksten fra callbackens parameter ind i den nye opgave:

```rust
let add_todo = Callback::new(move |text: String| {
    set_todos.update(move |list| {
        list.push(Todo {
            id: Uuid::new_v4(),
            text,
            done: false,
        });
    });
});
```

Efter flytningen kan den oprindelige `text` ikke bruges igen. `move` får en closure til at indfange værdier ved værdi frem for at låne dem fra det omgivende scope. Det passer til callbacks, som gemmes til senere brug.

Den ydre closure indfanger signalets skrivehåndtag. Den indre overtager `text` og flytter den ind i `Todo`. `move` betyder ikke automatisk en dyr kopi: signalhåndtag og `Uuid` implementerer `Copy`, mens `String` flyttes.

## Borrowing: læs eller ændr

- `&T` låner en værdi til læsning; flere læselån kan eksistere samtidig.
- `&mut T` låner en værdi med eksklusiv adgang til ændring, mens lånet bruges.
- Et lån må ikke overleve den værdi, det peger på.

`save_todos(todos: &[Todo])` i `src/services/storage.rs` låner opgaverne til serialisering uden at overtage listen.

I `.update()` er `list` et `&mut Vec<Todo>`:

```rust
set_todos.update(move |list| {
    if let Some(todo) = list.iter_mut().find(|todo| todo.id == id) {
        todo.done = !todo.done;
    }
});
```

`iter_mut()` låner elementerne til ændring. Sletning bruger tilsvarende `list.retain(|todo| todo.id != id)`. Begge dele ændrer den eksisterende vektor uden først at klone hele listen.

## .get(), .with() og .update()

| Metode         | Adgang                                                                      | Eksempel fra appen                  |
| -------------- | --------------------------------------------------------------------------- | ----------------------------------- |
| `.get()`       | Returnerer en klon af værdien og registrerer en reaktiv læsning             | `todos.get()` i `TodoList`          |
| `.with(...)`   | Låner værdien til læsning i en closure og registrerer en reaktiv læsning    | Lagringseffekten i `App`            |
| `.update(...)` | Giver et mutabelt lån og underretter reaktive afhængigheder om opdateringen | `set_todos.update(...)` i callbacks |

En reaktiv læsning registreres, når den sker i en reaktiv kontekst, fx en `Effect`, `Memo` eller dynamisk view-closure. `.with()` undgår klonen, men lånet skal bruges inde i closuren.

`TodoForm` bruger `.get_untracked()` ved indsendelse: den kloner inputteksten uden at registrere en reaktiv afhængighed. Teksten flyttes derefter videre med `on_add.run(text)`.

## Clone og reaktivitet er ikke det samme

`Todo` har `#[derive(Clone, Deserialize, Serialize)]`. Når `todos.get()` kloner en `Vec<Todo>`, klones også opgaverne og deres tekst. Brug derfor ikke `.clone()` som automatisk løsning på ejerskabsfejl; overvej først et lån.

I `TodoList` leverer `.get()` almindelige `Todo`-værdier til `<For>` med UUID som key. `TodoItem` læser `todo.done` som en almindelig bool, ikke som et signal. En ændring af listens data er derfor ikke i sig selv en garanti for, at en eksisterende rækkes checkbox og tekstklasse opdateres ved toggle. Korrekt ejerskab er ikke det samme som korrekt reaktiv visning.
