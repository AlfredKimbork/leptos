# Quick start: den eksisterende todo-app

Appen ligger i `todo-app/` og bruger Leptos 0.8 med client-side rendering (CSR) og Rust edition 2024. Du skal ikke oprette et nyt projekt eller erstatte kildefilerne.

## 1. Klargør værktøjerne

Forudsætning: Rust er installeret via rustup med en toolchain, der understøtter edition 2024 (Rust 1.85 eller nyere). Afhængighederne kan kræve en nyere version; brug gerne en opdateret stable-toolchain.

Tilføj WebAssembly-target og installér Trunk, hvis de mangler:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Installationen kan hente pakker fra nettet. Trunk bygger og serverer browserappen.

## 2. Start fra repository-roden

```sh
cd todo-app
trunk serve --open
```

Trunk bygger appen, starter udviklingsserveren og åbner browseren. Stop serveren med `Ctrl+C`.

`index.html` er allerede oprettet og henviser til `src/styles/main.css`. `src/main.rs` monterer `App` med `mount_to_body(App)`.

## 3. Kontrollér kode og lav release-build

Kør fra `todo-app/`, eventuelt i en anden terminal:

```sh
cargo check --target wasm32-unknown-unknown
trunk build --release
```

`cargo check` kontrollerer Rust-koden til browserens target, men tester ikke brugergrænsefladen. Release-buildet placeres som udgangspunkt i `dist/`. Første kørsel kan kræve netadgang til afhængigheder og værktøjer.

## 4. Find rundt i koden

- `src/main.rs`: `Vec<Todo>` i et signal, callbacks til tilføjelse, toggle og sletning samt en effekt til lagring.
- `src/models/todo.rs`: `Todo { id: Uuid, text: String, done: bool }` med Serde og `Clone`.
- `src/components/`: formular, liste, opgavevisning, statistik og panel.
- `src/services/storage.rs`: JSON via Serde og `localStorage` under nøglen `todos`. Manglende eller ulæselige data giver standardopgaver ved opstart.

Lagringen er lokal i browseren, ikke på en server. Data afhænger af browserens origin, herunder porten, og adgangen kan fejle.

**Forbehold ved visningen:** `<For>` bruger UUID som key og giver almindelige `Todo`-værdier til `TodoItem`. At toggle-callbacken ændrer signalets data er ikke en garanti for, at den eksisterende rækkes checkbox og tekstklasse opdateres korrekt.

Kommandoerne er vejledning; de er ikke kørt som del af denne dokumentationsopdatering, og appens browseradfærd er ikke testet her.
