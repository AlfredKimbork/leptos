# Status og repetition

Denne oversigt beskriver, hvad den nuværende kode arbejder med. Den er ikke en testprotokol eller en afsluttende evaluering.

## Det, der er skrevet kode til

- En `Todo`-model med UUID, tekst og status.
- En komponentopdelt brugergrænseflade med fælles state i `App`.
- Tilføjelse, ændring af status og sletning via callbacks.
- Inputvalidering og en valgfri fejlbesked.
- En tom-tilstand for listen.
- Statistik med en afledt closure og en `Memo`.
- Lagring og indlæsning via localStorage og JSON.
- CSS-klasser og en genanvendelig `Panel`-komponent.

## Det, fejlene har vist

| Problem undervejs                    | Relevant læring                                                       |
| ------------------------------------ | --------------------------------------------------------------------- |
| Semikolon efter en attribut          | `view!` har markup-syntaks, selv om attributværdierne indeholder Rust |
| Samme handler til klik og submit     | Events har forskellige konkrete typer                                 |
| Opgaven forsvandt ved submit         | Browserens standardhandling kan genindlæse siden                      |
| Forskellige typer fra `if` og `else` | Begge grene skal returnere samme type, fx gennem `.into_any()`        |
| Input skal nulstilles visuelt        | En closure gør bindingen til signalet reaktiv                         |

## Aktuelle kontrolpunkter

- `TodoStats` har en logging-effect med `move ||`; Leptos 0.8 forventer ét closure-argument, fx `move |_|`.
- `TodoItem` modtager en almindelig `Todo` i en keyed `<For>`. Kontrollér reaktiv visning af ændret `done`, ikke kun om callbacken ændrer listen.
- Den nuværende inputkomponent bruger klik og ikke HTML-formularens submit. Tilføjelse med Enter er derfor ikke implementeret gennem en formular.
- Lagringsfejl vises ikke til brugeren i brugergrænsefladen.

Dokumentationen ændrer ikke disse kodepunkter. Se [fejlfindingsguiden](06-common-problems.md) for forklaringer.

## Manuel kontrol af appen

Når koden kan bygges, kan følgende bruges som en lille tjekliste:

- [ ] Tilføj en opgave og kontrollér, at inputfeltet tømmes.
- [ ] Prøv tom tekst og kun mellemrum; kontrollér fejlbeskeden.
- [ ] Markér en opgave som færdig; kontrollér checkbox, tekstens udseende og statistik.
- [ ] Slet en opgave og kontrollér listen og statistikken.
- [ ] Genindlæs siden og kontrollér, at opgaver og status indlæses igen.
- [ ] Slet alle opgaver, genindlæs og kontrollér, at listen stadig er tom.
- [ ] Kontrollér browserkonsollen for fejl.

Punkterne er forslag til test, ikke en liste over beståede tests.

## Kan du forklare det uden at kigge?

1. Hvorfor ligger listen i `App`, mens inputtet har sit eget signal?
2. Hvad er forskellen på et signal, en almindelig `Todo` og en `Memo`?
3. Hvordan kommer teksten fra `TodoForm` til listen?
4. Hvad gør `move`, og hvorfor er det ikke det samme som `.clone()`?
5. Hvorfor skal en key være stabil og unik?
6. Hvorfor gør en stabil key ikke automatisk alle felter reaktive?
7. Hvad sker der fra `.update()` til lagring i localStorage?
8. Hvad er forskellen på en reaktiv opdatering og en genindlæsning?

Der er endnu ikke kode til weather-app, API-kald, routing eller server-side rendering i disse projekter. Det er derfor ikke en del af denne studieguide.
