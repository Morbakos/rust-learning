# J01 — Variables, types et fonctions

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `let`, `mut`, shadowing, types numériques, `Option`, tranches `&[T]`

## Mini-leçon (3-5 min)

**Variables.** Elles sont **immuables par défaut** ; `mut` autorise la modification. Le *shadowing* (`let x = ...` une seconde fois) crée une **nouvelle** variable du même nom, éventuellement d'un autre type.

**Types.** Entiers : `i32` (par défaut), `u32` (positif), `usize` (indices et tailles). Flottants : `f64` (par défaut), `f32`. Rust ne convertit **jamais** implicitement : on écrit `n as f64`. Attention : `7 / 2` vaut `3` (division entière).

**Fonctions.** La dernière expression, **sans `;`**, est la valeur renvoyée. `if` et `match` sont aussi des expressions.

```rust
let x = 5;              // immuable
let mut y = 5;          // modifiable
y += 1;
let x = x * 2;          // shadowing : nouvelle variable x (10)

fn double(n: i32) -> i32 {
    n * 2               // pas de `;` : c'est la valeur de retour
}

let label = if y > 5 { "grand" } else { "petit" };   // if = expression
let ratio = 3 as f64 / 2.0;                          // conversion explicite
```

**Tranches et `Option`.** `&[i32]` est une *vue* sur une suite d'entiers (tableau ou `Vec`). `Option<T>` vaut `Some(valeur)` ou `None` : c'est comme ça que Rust représente « il n'y a rien » (pas de `null`).

**Pour l'exercice** : `n % 3 == 0`, `values.is_empty()`, `for v in values { ... }`, `n.to_string()`, `format!("...")`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 3 (Common Programming Concepts)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `celsius_to_fahrenheit` : F = C × 9/5 + 32
- `fizzbuzz` : « Fizz » (multiple de 3), « Buzz » (multiple de 5), « FizzBuzz » (les deux), sinon le nombre en texte
- `min_max` : renvoie `Some((min, max))` d'une tranche, `None` si elle est vide

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `checked_average` : moyenne d'une tranche (`None` si vide)
- `fizzbuzz_up_to` : le FizzBuzz de 1 à n sous forme de `Vec<String>`

## Vérifier

```bash
cargo test -p d01-variables-types                          # base
cargo test -p d01-variables-types -- --include-ignored     # base + bonus
cargo clippy -p d01-variables-types -- -D warnings         # style et pièges classiques
cargo fmt -p d01-variables-types                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi `min_max` renvoie-t-elle une `Option` plutôt qu'un couple par défaut ?
2. Quelle différence entre `let x = 5; let x = x + 1;` (shadowing) et `let mut x = 5; x += 1;` ?
