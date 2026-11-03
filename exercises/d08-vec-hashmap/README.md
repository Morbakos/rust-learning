# J08 — Vec et HashMap

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `Vec`, `HashMap`, `entry().or_insert()`, tri, déduplication
**Lecture facultative (5 min)** : Rust Book, chapitre 8.1 et 8.3 (Vectors, Hash Maps)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `word_count` : compte les mots d'un texte (insensible à la casse, séparés par des espaces)
- `top_n` : les `n` mots les plus fréquents, triés par fréquence décroissante puis par ordre alphabétique
- `dedup_sorted` : valeurs uniques triées par ordre croissant

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `group_by_len` : regroupe des mots par longueur en gardant l'ordre d'apparition

## Vérifier

```bash
cargo test -p d08-vec-hashmap                          # base
cargo test -p d08-vec-hashmap -- --include-ignored     # base + bonus
cargo clippy -p d08-vec-hashmap -- -D warnings         # style et pièges classiques
cargo fmt -p d08-vec-hashmap                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. À quoi sert `entry().or_insert(0)` et pourquoi est-ce mieux qu'un `get` suivi d'un `insert` ?
2. L'ordre d'itération d'un `HashMap` est-il garanti ? Que faire si tu as besoin d'un ordre ?
