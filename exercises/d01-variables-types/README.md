# J01 — Variables, types et fonctions

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `let`, `mut`, shadowing, types numériques, `Option`, tranches `&[T]`
**Lecture facultative (5 min)** : Rust Book, chapitre 3 (Common Programming Concepts)

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
