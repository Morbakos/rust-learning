# J06 — Option et combinateurs

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `Option<T>`, `?` sur Option, `map`, `and_then`, `unwrap_or`, `checked_*`
**Lecture facultative (5 min)** : Rust Book, chapitre 6.1 (Option) — et la doc de `Option` dans la std

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `find_index` : indice de la première occurrence de `target`
- `safe_div` : division entière, `None` si diviseur nul (ou dépassement : indice → `checked_div`)
- `first_even` : premier nombre pair
- `double_first_even` : le double du premier pair, en utilisant `map` ou `?`
- **Règle : aucun `.unwrap()` ni `.expect()` dans ton code.**

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `parse_and_halve` : parse une chaîne en `i32` puis divise par 2 via `safe_div`
- `first_char_upper` : premier caractère en majuscule ASCII (`None` si chaîne vide)

## Vérifier

```bash
cargo test -p d06-option                          # base
cargo test -p d06-option -- --include-ignored     # base + bonus
cargo clippy -p d06-option -- -D warnings         # style et pièges classiques
cargo fmt -p d06-option                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Quelle différence entre `map` et `and_then` sur une `Option` ?
2. Pourquoi Rust n'a pas de `null` ? Qu'est-ce qui remplace le « null pointer exception » ?
