# J13 — Closures

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `Fn` / `FnMut` / `FnOnce`, `move`, `impl Fn`, closures qui capturent leur environnement
**Lecture facultative (5 min)** : Rust Book, chapitre 13.1 (Closures)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `apply_n` : applique `f` `n` fois à `x` (`n = 0` renvoie `x`)
- `make_adder` : renvoie une closure qui ajoute `k` à son argument
- `sort_by_len` : trie des `String` par longueur croissante (tri **stable**), avec une closure
- `counter` : renvoie une closure qui renvoie 1, puis 2, puis 3… à chaque appel

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `compose` : `compose(f, g)(x) == g(f(x))`

## Vérifier

```bash
cargo test -p d13-closures                          # base
cargo test -p d13-closures -- --include-ignored     # base + bonus
cargo clippy -p d13-closures -- -D warnings         # style et pièges classiques
cargo fmt -p d13-closures                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi `counter` doit renvoyer un `FnMut` et pourquoi faut-il `move` ?
2. Quelle différence entre `Fn`, `FnMut` et `FnOnce` ? Donne un exemple de chacun.
