# Guess Game 🎯

Moje první program v Rustu! Není to nejlepší kód, ale učím se to :D

Používal jsem AI (Claude) jako pomocníka při učení, protože mi to přišlo jako lepší způsob, jak
pochopit, proč věci fungují (nebo nefungují XD). Vedle toho se učím i z videí a z oficiální
[Rust Book](https://doc.rust-lang.org/book/).

## O co jde

Klasická guess-the-number hra:
- na začátku si zvolíš obtížnost (maximální číslo, do kterého se generuje náhodné číslo)
- pak hádáš, dokud netrefíš správné číslo
- hra ti řekne, jestli je tvůj tip moc vysoký nebo moc nízký
- počítá i kolik pokusů jsi měl

## Jak spustit

Potřebuješ mít nainstalovaný [Rust](https://www.rust-lang.org/tools/install) (přes `rustup`).

```bash
git clone https://github.com/toafik/guess-game-rust.git
cd guess-game-rust
cargo run
```

## Co jsem se u toho naučil

- práci se stdin/stdout (`std::io`)
- `Result` a `match` na ošetřování chyb (žádné crashe při špatném vstupu!)
- základ knihovny `rand` (verze 0.10, kde se `thread_rng()` přejmenoval na `rng()` a je potřeba
  importovat `rand::RngExt`)
- funkce, návratové hodnoty a proč poslední výraz bez `;` je return
- `loop`, `break`, `continue`

## Co bych chtěl přidat příště

- lepší ošetření vstupu (třeba záporná čísla nebo prázdný vstup)
- možnost hrát znovu bez restartu programu
