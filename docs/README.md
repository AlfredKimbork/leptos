# Studieguide: Rust og Leptos

Guiden følger arbejdet med [todo-appen](../todo-app/README.md). Fokus er på begreber, der findes i den aktuelle kode, og de fejl, der er opstået undervejs — ikke på en fuld gennemgang af Rust eller Leptos.

## Læserækkefølge

1. [Rust i projektet](01-rust-basics.md) — typer, datamodel, moduler og iteratorer.
2. [Ownership og borrowing](02-ownership-and-borrowing.md) — ejerskab, referencer og closures.
3. [Leptos i todo-appen](03-leptos-fundamentals.md) — komponenter, signaler, callbacks og reaktivitet.
4. [React og Leptos](04-react-vs-leptos.md) — en kort sammenligning af de relevante mønstre.
5. [localStorage og JSON](05-localstorage.md) — hvordan opgaver gemmes og indlæses.
6. [Fejl og læring](06-common-problems.md) — konkrete problemer og deres årsager.
7. [Kør projektet](07-quick-start.md) — værktøjer og kommandoer.
8. [Status og repetition](08-status-og-repetition.md) — kontrolpunkter, testforslag og spørgsmål.

## Sådan bruges guiden

Læs et afsnit sammen med den tilhørende kode i `todo-app/src`. Prøv derefter at forklare forløbet med egne ord: brugerhandling → callback → ændret signal → reaktiv visning og lagring.

Guiden skelner mellem kode, der er skrevet, og funktioner, der er kontrolleret. Den aktuelle kode har fortsat kontrolpunkter ved `Effect::new` og opdatering af enkelte listeelementer; de er beskrevet under fejlfinding og status. Dokumentationen i sig selv bekræfter ikke, at appen er fejlfri.

## Opslagsværker

- [Rust Book](https://doc.rust-lang.org/book/)
- [Leptos Book](https://book.leptos.dev/)
- [Trunk](https://trunkrs.dev/)
