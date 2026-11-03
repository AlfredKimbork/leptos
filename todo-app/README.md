# Todo-app med Rust og Leptos

En lille læringsapp, der bruges til at undersøge Rust og Leptos gennem en konkret opgaveliste. Appen bruger Leptos 0.8 med client-side rendering og kører i browseren som WebAssembly.

## Hvad appen indeholder

- Tilføjelse af opgaver og validering af tomt input.
- Handlinger til at markere opgaver som færdige og slette dem.
- Antal resterende opgaver og en statusbesked.
- Lagring i browserens localStorage og indlæsning ved opstart.
- Opdeling i komponenter med signaler, props og callbacks.
- UUID'er til opgavernes ID'er og almindelig CSS til styling.

Der er ingen backend eller brugerkonti. Opgaverne gemmes lokalt i den aktuelle browser og på det aktuelle origin; de synkroniseres ikke mellem enheder.

## Kør lokalt

Du skal have Rust, WebAssembly-targetet og Trunk installeret:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Kør fra denne mappe (`todo-app`):

```sh
trunk serve --open
```

Kontrollér Rust-koden eller lav et produktionsbuild med:

```sh
cargo check --target wasm32-unknown-unknown
trunk build --release
```

## Projektstruktur

- `src/main.rs`: fælles state, callbacks og opstart.
- `src/components/`: input, liste, opgave, statistik og panel.
- `src/models/todo.rs`: datamodellen for en opgave.
- `src/services/storage.rs`: JSON og localStorage.
- `src/styles/main.css`: styling.

## Status

Projektet er under læring og udvikling. Logging-effekten i `TodoStats` skal bruge `move |_|` i stedet for `move ||` til Leptos 0.8. Desuden skal opdatering af eksisterende opgavers visning undersøges: `TodoItem` modtager almindelige værdier i en keyed `<For>`, så ændringer af `done` er ikke automatisk reaktive i elementets view.

Denne dokumentationsopdatering ændrer ikke appens kode og er ikke en bekræftelse på, at build eller browsertest er bestået.

Se [studieguiden](../docs/README.md) for korte forklaringer af koden og de fejl, der er opstået undervejs.
