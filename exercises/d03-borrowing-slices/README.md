# J03 — Emprunts et tranches (slices)

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `&T`, `&mut T`, `&str`, `&[T]`, `&mut [T]`, indices et sous-tranches
**Lecture facultative (5 min)** : Rust Book, chapitre 4.2 et 4.3 (References, Slices)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `first_word` : premier mot d'un `&str` (sans allocation, tu renvoies une sous-tranche)
- `sum_slice` : somme d'une tranche d'entiers
- `reverse_in_place` : inverse une tranche mutable **sans** créer de nouveau `Vec`

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `split_halves` : coupe une tranche en deux (première moitié = len/2) sans copier
- `largest_window_sum` : plus grande somme de `k` éléments consécutifs (`None` si k = 0 ou k > len)

## Vérifier

```bash
cargo test -p d03-borrowing-slices                          # base
cargo test -p d03-borrowing-slices -- --include-ignored     # base + bonus
cargo clippy -p d03-borrowing-slices -- -D warnings         # style et pièges classiques
cargo fmt -p d03-borrowing-slices                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi `first_word` peut renvoyer un `&str` sans allouer de mémoire ? À quoi ce `&str` « pointe » ?
2. Pourquoi ne peut-on pas avoir une référence `&mut` et une `&` sur la même donnée en même temps ?
