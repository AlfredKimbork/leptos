# Lagring med localStorage og JSON

Appen gemmer opgaver i browseren, så de kan indlæses igen efter en genindlæsning. Koden ligger i `todo-app/src/services/storage.rs`.

## Fra `Todo` til JSON

Datamodellen har:

```rust
#[derive(Clone, Deserialize, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub done: bool,
}
```

- `Serialize` gør det muligt at omdanne data til et format som JSON.
- `Deserialize` gør det muligt at oprette Rust-værdier fra data som JSON.
- `Clone` bruges blandt andet, når listen læses med `.get()`.

`serde` leverer traits og derive-makroer. `serde_json` omdanner mellem Rust-værdier og JSON-tekst. UUID-bibliotekets `serde`-feature gør det muligt også at gemme ID'erne.

localStorage gemmer tekst, ikke Rust-objekter. Derfor skal listen konverteres, før den gemmes.

## Indlæsning ved opstart

```rust
pub fn load_todos() -> Option<Vec<Todo>> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let todos_json = storage.get_item(STORAGE_KEY).ok()??;

    serde_json::from_str(&todos_json).ok()
}
```

Forløbet er:

1. Find browserens `window`.
2. Find localStorage.
3. Hent teksten under nøglen `"todos"`.
4. Konvertér JSON-teksten til `Vec<Todo>`.

`?` afslutter funktionen tidligt med `None`, hvis en nødvendig værdi mangler. `.ok()` omdanner `Result` til `Option`, så fejldetaljerne bliver fravalgt.

`local_storage()` returnerer `Result<Option<Storage>, ...>`. Efter `.ok()` er der to lag af `Option`; derfor bruges `??` til at håndtere begge lag. Det samme mønster bruges ved `get_item()`.

I `App` bruges:

```rust
let (todos, set_todos) = signal(load_todos().unwrap_or_else(default_todos));
```

`unwrap_or_else` kalder kun `default_todos`, når resultatet er `None`. En gemt tom liste er `Some(vec![])` og bliver derfor ikke erstattet af standardopgaverne.

## Gem automatisk, når listen ændres

I `App` følger en effect listens signal:

```rust
Effect::new(move |_| {
    todos.with(|list| {
        save_todos(list);
    });
});
```

`.with()` læser gennem en reference og registrerer afhængigheden. Effekten kører i browseren efter oprettelsen og igen, når signalet giver besked om ændringer.

`save_todos(todos: &[Todo])` låner opgaverne, konverterer dem med `serde_json::to_string()` og gemmer resultatet med `storage.set_item()`.

Her passer en effect godt: lagring er en sideeffekt, ikke en værdi, der skal vises i et view.

## Begrænsninger i den nuværende løsning

- Data er knyttet til browser og origin, herunder port. En anden port kan derfor vise en anden liste.
- Ingen synkronisering mellem enheder og ingen database på en server.
- Brugeren kan slette data gennem browseren.
- Browseren kan afvise adgang eller løbe tør for lagerplads.
- Indlæsningsfejl bliver til `None`, så manglende data og ugyldig JSON giver samme fallback.
- En ugyldig gemt værdi kan blive overskrevet af standardlisten, når lagringseffekten kører.
- Kun fejl fra selve `set_item()` logges; andre fejl ved lagring kan medføre en stille afslutning.

## Kontrollér forståelsen

- Hvorfor skal listen omdannes til tekst?
- Hvad er forskellen på `None` og `Some(vec![])`?
- Hvorfor bruges `.with()` i stedet for at klone hele listen?
- Hvorfor er localStorage noget andet end blot at undgå en genindlæsning?
