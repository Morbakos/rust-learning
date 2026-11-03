# J08 — Vec et HashMap

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `Vec`, `HashMap`, `entry().or_insert()`, tri, déduplication

## Mini-leçon (3-5 min)

**`Vec<T>`** : tableau dynamique (taille variable), contigu en mémoire. **`HashMap<K, V>`** : association clé → valeur (l'ordre d'itération **n'est pas garanti**).

L'idiome le plus utile : **`entry`**, qui évite de vérifier si la clé existe déjà.

```rust
use std::collections::HashMap;

let mut counts: HashMap<String, usize> = HashMap::new();
for w in ["a", "b", "a"] {
    *counts.entry(w.to_string()).or_insert(0) += 1;   // insère 0 si absent, puis +1
}
// counts["a"] == 2

let mut v = vec![3, 1, 3, 2];
v.sort();            // [1, 2, 3, 3]
v.dedup();           // supprime les doublons CONSÉCUTIFS : [1, 2, 3]

// Trier des paires par un critère puis un second :
let mut pairs = vec![("b", 2), ("a", 2), ("c", 5)];
pairs.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));   // compte décroissant, puis nom croissant
```

- `.cmp()` compare des valeurs `Ord` ; `.then(...)` départage les égalités.
- `HashMap` → liste : `counts.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>()`.

**Pour l'exercice** : `text.split_whitespace()`, `to_lowercase()`, `entry().or_insert()`, `sort_by`, `truncate(n)` ou `take(n)`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 8.1 et 8.3 (Vectors, Hash Maps)

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
