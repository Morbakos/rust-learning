# J03 — Emprunts et tranches (slices)

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `&T`, `&mut T`, `&str`, `&[T]`, `&mut [T]`, indices et sous-tranches

## Mini-leçon (3-5 min)

**Règle d'or des emprunts** : à un instant donné, tu as soit **plusieurs** `&` (lecture), soit **un seul** `&mut` (écriture), jamais les deux. Cela supprime toute une classe de bugs (modifier une donnée pendant qu'on la lit).

**Tranche (slice).** Une `&[T]` ou un `&str` est un « pointeur + longueur » vers une portion de données **sans copie**. Un `&str` est une tranche de `String`.

```rust
let v = vec![10, 20, 30, 40, 50];
let a: &[i32] = &v[1..4];      // [20, 30, 40] (début inclus, fin exclue)
let b = &v[..2];               // [10, 20]
let c = &v[3..];               // [40, 50]

let s = String::from("bonjour monde");
let mot: &str = &s[0..7];      // "bonjour" : aucune copie, juste une vue

fn somme(xs: &[i32]) -> i32 { xs.iter().sum() }
somme(&v);                     // &Vec<i32> se convertit en &[i32]
somme(&[1, 2, 3]);             // un tableau aussi
```

**`&mut [T]`** : on peut modifier les éléments en place (mais pas changer la taille).

**Pour l'exercice** : `s.find(' ')` (renvoie une `Option<usize>`), `s.split_whitespace().next()`, `values.len()`, `values.swap(i, j)`, `values.windows(k)`, `values.iter().sum()`. Pas besoin de `.clone()`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 4.2 et 4.3 (References, Slices)

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
