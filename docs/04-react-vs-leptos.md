# React og Leptos: sammenligning i todo-app

Projektet bruger Rust edition 2024 og Leptos 0.8 med CSR. Sammenligningen handler om mønstrene i den eksisterende kode, ikke om målte resultater eller personlige erfaringer.

## Samme opdeling, forskellig reaktivitet

| Område          | React                                     | Denne Leptos-app                                         |
| --------------- | ----------------------------------------- | -------------------------------------------------------- |
| Komponenter     | Funktioner med JSX/TSX                    | `#[component]` og `view!`                                |
| State           | Fx `useState<Todo[]>([])`                 | `signal(...)` med en `Vec<Todo>` i `App`                 |
| Props og events | Props og callback-funktioner              | Fx `Signal<Vec<Todo>>` og `Callback<Uuid>`               |
| Afledte værdier | Beregning under rendering, evt. `useMemo` | Closure for `remaining`, `Memo` for `all_done`           |
| Sideeffekter    | Fx `useEffect` med afhængighedsliste      | `Effect` sporer signaler læst under kørslen              |
| Listevisning    | `.map()` med `key`                        | `<For>` med `todo.id` som key                            |
| Browserbuild    | JavaScript, evt. fra TypeScript           | WebAssembly med JavaScript-integration, bygget med Trunk |

React genkører normalt komponentfunktionen ved en state-opdatering og sammenligner renderingsresultatet. Leptos kører normalt komponentfunktionen ved oprettelse og opdaterer derefter de reaktive beregninger og views, der læser signalet.

## Opdatering af listen

Et tilsvarende React-mønster opretter et nyt array:

```tsx
setTodos((current) => [...current, newTodo]);
```

I `todo-app/src/main.rs` ændrer Leptos listen gennem et mutabelt lån:

```rust
set_todos.update(move |list| {
    list.push(Todo {
        id: Uuid::new_v4(),
        text,
        done: false,
    });
});
```

Toggle bruger `iter_mut().find(...)`, og sletning bruger `retain(...)`. Det kræver ikke en klon af hele listen ved selve ændringen. Læsninger med `todos.get()` kloner dog listen, fx i `TodoList` og `TodoStats`.

Rusts `move` i callbacks styrer indfangning og ejerskab. Det er en anden mekanisme end Reacts rendering og state-håndtering.

## Lagring og listevisning

I `App` læser en `Effect` signalet med `.with()` og kalder `save_todos`. Serde omsætter `Todo` til JSON, og `web-sys` giver adgang til `localStorage`. I React kan en tilsvarende sideeffekt bruge `useEffect`, `JSON.stringify` og browserens `localStorage`.

En vigtig forskel ved keyed lister: React kan ved en ny rendering give en eksisterende komponent nye props med samme key. Leptos' `<For>` genbruger rækker efter key; det gør ikke automatisk felterne i en almindelig `Todo` reaktive.

Her modtager `TodoItem` en almindelig `Todo`, og både `checked` og tekstklassen læser en almindelig `bool`. Toggle-callbacken ændrer data i signalet, men det dokumenterer ikke, at en eksisterende rækkes visning følger med. Reaktive felter eller en anden reaktiv adgang til opgaven kan være nødvendige, hvis listen skal understøtte den opdatering sikkert.

Der er ikke grundlag her for at konkludere noget om relativ hastighed, bundle-størrelse eller samlet udviklingstid.
