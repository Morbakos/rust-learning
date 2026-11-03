# J02 — Ownership : qui possède quoi ?

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : move, `String` vs `&str`, `&mut`, une seule référence mutable à la fois
**Lecture facultative (5 min)** : Rust Book, chapitre 4.1 (What is Ownership?)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `take_and_give_back` : prend une `String`, la renvoie avec sa longueur **en octets**
- `add_exclaim` : ajoute `!` à la fin d'une `String` empruntée en mutable
- `concat_owned` : consomme `a` et lui ajoute `b`

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `drain_evens` : retire les nombres pairs d'un `Vec` et les renvoie, `v` ne garde que les impairs

## Vérifier

```bash
cargo test -p d02-ownership                          # base
cargo test -p d02-ownership -- --include-ignored     # base + bonus
cargo clippy -p d02-ownership -- -D warnings         # style et pièges classiques
cargo fmt -p d02-ownership                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Que se passe-t-il si tu utilises `s` après l'avoir passée à `take_and_give_back` (sans récupérer la valeur de retour) ? Pourquoi ?
2. Pourquoi `"héllo".len()` vaut 6 et pas 5 ?
