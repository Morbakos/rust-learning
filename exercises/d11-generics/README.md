# J11 — Génériques : une pile `Stack<T>`

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : types génériques, `impl<T>`, bornes de traits, `Default`
**Lecture facultative (5 min)** : Rust Book, chapitre 10.1 (Generic Data Types)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `Stack<T>` avec `new`, `push`, `pop`, `peek` (renvoie `Option<&T>`), `len`, `is_empty`
- Comportement LIFO : le dernier entré est le premier sorti

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `render` (pour `T: Display`) : `"[1, 2, 3]"` du bas vers le haut de la pile, `"[]"` si vide
- `max` (pour `T: PartialOrd + Copy`) : plus grand élément (`None` si vide)

## Vérifier

```bash
cargo test -p d11-generics                          # base
cargo test -p d11-generics -- --include-ignored     # base + bonus
cargo clippy -p d11-generics -- -D warnings         # style et pièges classiques
cargo fmt -p d11-generics                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi `peek` renvoie-t-elle `Option<&T>` et pas `Option<T>` ?
2. Que se passe-t-il à la compilation si tu appelles `stack.render()` avec un `T` qui n'implémente pas `Display` ?
