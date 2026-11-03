# J05 — Enums et pattern matching

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `enum` avec données, `match` exhaustif, `if let`, `split_once`, `parse`
**Lecture facultative (5 min)** : Rust Book, chapitre 6 (Enums and Pattern Matching)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `area` : aire d'une `Shape` (cercle = π r², rectangle, triangle = base × hauteur / 2)
- `parse_command` : `"quit"` → `Quit`, `"move 3 -2"` → `Move { dx: 3, dy: -2 }`, `"say hello world"` → `Say("hello world")`, tout le reste (ou arguments invalides) → `None`

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `apply` : applique une commande à une position `(x, y)` (seul `Move` la change)

## Vérifier

```bash
cargo test -p d05-enums-match                          # base
cargo test -p d05-enums-match -- --include-ignored     # base + bonus
cargo clippy -p d05-enums-match -- -D warnings         # style et pièges classiques
cargo fmt -p d05-enums-match                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi le compilateur refuse un `match` qui oublie une variante ? Qu'est-ce que ça t'apporte ?
2. Dans quels cas préférer `if let` à `match` ?
