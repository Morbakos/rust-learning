# J04 — Structs et méthodes

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `struct`, `impl`, `&self` vs `&mut self`, fonctions associées, `derive`
**Lecture facultative (5 min)** : Rust Book, chapitre 5 (Using Structs)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `Rectangle::new`, `Rectangle::square` (fonctions associées)
- `area` et `perimeter` (méthodes `&self`)
- `can_hold` : `self` est strictement plus grand que `other` en largeur ET en hauteur
- `scale` : multiplie les deux dimensions (méthode `&mut self`)

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `is_square`
- `largest` : le rectangle de plus grande aire d'une tranche (`None` si vide), sans copie

## Vérifier

```bash
cargo test -p d04-structs                          # base
cargo test -p d04-structs -- --include-ignored     # base + bonus
cargo clippy -p d04-structs -- -D warnings         # style et pièges classiques
cargo fmt -p d04-structs                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi `area` prend `&self` alors que `scale` prend `&mut self` ?
2. Que fait `#[derive(Clone, PartialEq)]` exactement ?
