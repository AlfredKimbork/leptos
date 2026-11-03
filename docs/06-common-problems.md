# Fejl og læring fra todo-appen

## 1. `invalid tag name or attribute key`

I en tidligere udgave sluttede en attribut i `view!` med semikolon:

```rust
// Forkert i markup:
on:input=move |event| set_input_value.set(event_target_value(&event));
```

Attributten er ikke en selvstændig Rust-statement. Fjern det afsluttende semikolon, eller skriv handlerens kode inde i `{ ... }`.

**Læring:** En lille syntaksfejl i en makro kan give flere fejl andre steder. Start med den første konkrete parserfejl frem for de lange afledte typefejl.

## 2. `SubmitEvent` og `MouseEvent` er forskellige typer

Tidligere blev samme closure brugt til både `on:submit` og `on:click`.

En closure har én bestemt argumenttype. Et submit-event og et klik-event kan derfor ikke sendes til samme closure på den måde.

Ved en rigtig HTML-formular kan tilføjelsen håndteres udelukkende gennem formularen:

```rust
let submit = move |event: leptos::ev::SubmitEvent| {
    event.prevent_default();
    // Valider input og kald callbacken her.
};

view! {
    <form on:submit=submit>
        <button type="submit">"Tilføj"</button>
    </form>
}
```

Dette er et forklarende eksempel. Den nuværende `TodoForm` bruger en `<div>` og en `on:click`-handler, ikke en `<form>`.

## 3. Opgaven kom frem og forsvandt igen

Årsagen i den tidligere formular var browserens standardhandling ved submit: siden blev genindlæst, og state i hukommelsen blev nulstillet.

`event.prevent_default()` forhindrer den almindelige navigation, når man bruger `on:submit`.

Det er vigtigt at skelne mellem:

- **Reaktiv opdatering:** Leptos opdaterer de dele af brugergrænsefladen, som afhænger af ændret state.
- **Genindlæsning:** Browseren starter appen forfra.
- **Lagring:** localStorage gør det muligt at gendanne state efter en genindlæsning.

Den nuværende app har også localStorage. Det løser lagring mellem besøg, men er ikke en erstatning for korrekt eventhåndtering.

## 4. `if` og `else` returnerer forskellige typer

Et view med en tom-besked og et view med `<ul>` har forskellige konkrete Rust-typer. Tidligere brugte kun den ene gren `.into_any()`.

Begge grene skal returnere samme type:

```rust
if is_empty {
    view! { <p>"Ingen opgaver"</p> }.into_any()
} else {
    view! { <ul></ul> }.into_any()
}
```

`is_empty` repræsenterer her resultatet af tom-kontrollen. I appen udføres kontrollen med `todos.get().is_empty()`.

## 5. Inputfeltet skal følge signalet

Brug en closure til inputfeltets værdi:

```rust
prop:value=move || input_value.get()
```

Så bliver feltet også tømt visuelt, når handleren kalder `set_input_value.set(String::new())`.

## 6. State ændres, men en opgave ser uændret ud

I den nuværende liste identificerer `<For>` hver opgave med `todo.id`. `TodoItem` modtager samtidig en almindelig `Todo`, ikke et signal.

Når `done` ændres, bliver ID'et ved med at være det samme. `<For>` kan derfor genbruge det eksisterende view uden at genoprette det med nye feltværdier. En stabil key alene gør ikke `todo.done` reaktiv.

**Ved fejlfinding:** Kontrollér både state, statistikken og opgavens tekstklasse. Et checkbox-klik kan ændre browserens checkbox visuelt uden at bevise, at hele komponenten er reaktivt forbundet.

En senere løsning kan lade hvert element læse sin aktuelle værdi reaktivt, eksempelvis via ID og adgang til listens signal. Dette er ikke implementeret som del af dokumentationen.

## 7. `Effect::new` skal have den rigtige closure-signatur

Lagringseffekten i `App` bruger `move |_|`. Logging-effekten i `TodoStats` bruger aktuelt `move ||`.

I Leptos 0.8 forventer `Effect::new` en closure med ét argument. Brug `move |_|`, når argumentet ikke skal bruges. Dette er et kendt kontrolpunkt i den nuværende kode, ikke en rettelse foretaget af guiden.

## En enkel fejlsøgningsrutine

1. Læs den første relevante fejl og find linjen i koden.
2. Kontrollér syntaks og typer, før du ændrer logikken.
3. Skeln mellem compilerfejl, reaktivitet og browserens standardhandlinger.
4. Kør `cargo check --target wasm32-unknown-unknown` fra `todo-app`.
5. Brug browserens konsol og localStorage-visning til browserrelaterede problemer.

En forklaring på en fejl er ikke det samme som en bestået test. Kontrollér ændringen i browseren bagefter.
