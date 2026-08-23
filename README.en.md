# Guess Game 🎯

🇨🇿 [Česká verze](README.md)

My first program in Rust! It's not the best code out there, but I'm learning :D

I used AI (Claude and Gemini) as a learning aid, since it felt like a better way to actually
understand *why* things work (or don't XD). Alongside that, I'm also learning from videos and the
official [Rust Book](https://doc.rust-lang.org/book/).

## What it does

A classic guess-the-number game:
- at the start you pick a difficulty (the maximum number the random number can go up to)
- then you keep guessing until you hit the right number
- the game tells you if your guess was too high or too low
- it also counts how many attempts you took

## How to run it

You need [Rust](https://www.rust-lang.org/tools/install) installed (via `rustup`).

```bash
git clone https://github.com/toafik/guess-game-rust.git
cd guess-game-rust
cargo run
```

## What I learned making this

- working with stdin/stdout (`std::io`)
- `Result` and `match` for error handling (no crashes on bad input!)
- the basics of the `rand` crate (version 0.10, where `thread_rng()` was renamed to `rng()` and
  you need to import `rand::RngExt`)
- functions, return values, and why the last expression without a `;` is the return value
- `loop`, `break`, `continue`, `if else`

## Resources I'm learning from

- [Rust Book](https://doc.rust-lang.org/book/) – official docs/book
- [Rust for beginners (2025)](https://youtube.com/playlist?list=PLAscMa3kKhHjBYOTrT20kpYMNQX95KU6i) – YouTube playlist by Rustfully
