# Leptos i todo-appen

Leptos er et Rust-framework til reaktive brugergrænseflader. Appen bruger Leptos 0.8 med **client-side rendering (CSR)**: Rust-koden kompileres til WebAssembly og kører i browseren.

## Komponenter og ansvar

En komponent er en funktion med `#[component]`, der returnerer `impl IntoView`. `view!` beskriver HTML og komponenter. `main()` monterer `App` i sidens `<body>` med `mount_to_body(App)`.

| Komponent   | Ansvar i appen                                                      |
| ----------- | ------------------------------------------------------------------- |
| `App`       | Ejer listen og handlingerne til at tilføje, ændre og slette opgaver |
| `TodoForm`  | Håndterer input og fejlbeskeder                                     |
| `TodoList`  | Viser listen eller en besked, når den er tom                        |
| `TodoItem`  | Viser en opgave med checkbox og sletteknap                          |
| `TodoStats` | Beregner antal resterende opgaver og status                         |
| `Panel`     | Genanvendelig ramme med titel og indhold                            |

`Panel` modtager `children: Children`. Indholdet mellem `<Panel>` og `</Panel>` vises med `{children()}`.

## Signaler: data, som brugergrænsefladen kan følge

I `src/main.rs` oprettes listen sådan:

```rust
let (todos, set_todos) = signal(load_todos().unwrap_or_else(default_todos));
```

- `todos` giver læseadgang til listen.
- `set_todos` giver skriveadgang.
- Hvis indlæsning fejler eller intet er gemt, bruges standardopgaverne.

| Metode             | Brug                                                                      |
| ------------------ | ------------------------------------------------------------------------- |
| `.get()`           | Læser og kloner værdien; registrerer en afhængighed i en reaktiv kontekst |
| `.with(...)`       | Læser gennem en reference uden at klone hele værdien                      |
| `.get_untracked()` | Læser uden at registrere en reaktiv afhængighed                           |
| `.set(...)`        | Erstatter værdien                                                         |
| `.update(...)`     | Ændrer den eksisterende værdi og giver besked til afhængige beregninger   |

## Props ned, callbacks op

`App` sender data og callbacks til underkomponenterne:

```rust
<TodoForm on_add=add_todo />
```

`TodoForm` modtager `on_add: Callback<String>` og kalder:

```rust
on_add.run(text);
```

Forløbet er:

1. Brugeren skriver tekst og klikker på knappen.
2. `TodoForm` validerer teksten.
3. Callbacken sender teksten til `App`.
4. `App` tilføjer en `Todo` med `.update()` og `.push()`.
5. Reaktive visninger og lagring kan reagere på ændringen.

Formularen behøver derfor ikke adgang til hele listen. `Signal<Vec<Todo>>` bruges som læsbar prop til listen og statistikken; `todos.into()` konverterer læsehandlet til denne type.

## Hvorfor `move ||` er vigtigt

Inputfeltet er bundet til et signal:

```rust
prop:value=move || input_value.get()
on:input=move |event| {
    set_input_value.set(event_target_value(&event))
}
```

`on:input` opdaterer signalet, når brugeren skriver. Closuren ved `prop:value` lader inputfeltet følge signalet, også når teksten nulstilles efter tilføjelse.

Uden closuren ville `.get()` blive læst ved oprettelsen af viewet i stedet for at etablere den ønskede reaktive binding.

`move ||` tager ingen argumenter. `move |event|` modtager et event. `move |_|` modtager et argument, men ignorerer det.

## Validering med `Option`

`TodoForm` bruger `signal(None::<String>)` til en valgfri fejlbesked:

- `None`: ingen fejlbesked.
- `Some(...)`: vis beskeden.

`text.trim().is_empty()` afviser også input, der kun består af mellemrum. `trim()` bruges til kontrollen; den oprindelige tekst sendes videre uden at blive trimmet.

## Lister, keys og forskellige views

I `TodoList` bruges:

```rust
<For
    each=move || todos.get()
    key=|todo| todo.id
    children=move |todo: Todo| view! {
        <li>{todo.text}</li>
    }
/>
```

- `each` læser listen reaktivt.
- `key` identificerer opgaven med dens stabile UUID.
- `children` opretter visningen for et element.

Listen og tom-beskeden har forskellige konkrete view-typer. Derfor bruger begge grene i appens `if`-udtryk `.into_any()`, så de returnerer samme type, `AnyView`.

**Vigtig begrænsning:** En stabil key gør ikke automatisk alle felter reaktive. Den nuværende `TodoItem` modtager en almindelig `Todo`, og `<For>` genbruger elementer med samme key. Ændringer i `done` er derfor ikke automatisk en reaktiv binding til checkbox og CSS-klasse. Skeln mellem, at listen ændres i state, og at den enkelte opgaves visning følger med.

## Afledte værdier, `Memo` og `Effect`

`TodoStats` beregner antal resterende opgaver med en closure:

```rust
let remaining = move || todos.get().iter().filter(|todo| !todo.done).count();
```

Det er afledt state: værdien kan beregnes fra listen og behøver ikke sit eget skrivbare signal.

`all_done` er en `Memo`, der gemmer resultatet af en reaktiv beregning. Kontrollen for en ikke-tom liste er vigtig: `.all(...)` alene returnerer også `true` for en tom iterator.

`Effect` bruges til handlinger uden for selve beregningen af et view:

- `App` gemmer listen i localStorage.
- `TodoStats` indeholder logging af status.

I Leptos 0.8 modtager en `Effect::new`-closure ét argument, som kan ignoreres med `move |_|`. Den nuværende logging i `TodoStats` er skrevet med `move ||`; det er en kodeforskel, der bør rettes før build, ikke et alternativt API.

**Husk:** Brug closures eller `Memo` til beregninger og `Effect` til sideeffekter. En reaktiv opdatering er ikke det samme som en genindlæsning af siden.
