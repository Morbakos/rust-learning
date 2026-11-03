# J12 — Itérateurs

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `iter()`, `filter`, `map`, `zip`, `sum`, `flat_map`, `collect`, `max_by`

## Mini-leçon (3-5 min)

**Un itérateur** produit des valeurs une par une. Les **adaptateurs** (`map`, `filter`, `zip`, `flat_map`, `take`...) sont **paresseux** : ils ne font rien tant qu'un **consommateur** (`sum`, `collect`, `count`, `max_by`, `for`...) ne tire dessus.

```rust
let v = vec![1, 2, 3, 4];

let total: i32 = v.iter()
    .filter(|&&x| x % 2 == 0)      // garde les pairs
    .map(|&x| x * x)               // les met au carré
    .sum();                        // consommateur : lance le calcul (20)

let pairs: Vec<(i32, char)> = v.iter().cloned().zip("abcd".chars()).collect();

let words: Vec<&str> = ["a b", "c"].iter().flat_map(|l| l.split_whitespace()).collect();
```

- `iter()` emprunte (donne des `&T`), `iter_mut()` donne des `&mut T`, `into_iter()` consomme la collection.
- `collect()` a besoin de connaître le type visé : `Vec<_>`, `String`, `HashMap<_, _>`... (annotation ou « turbofish » `collect::<Vec<_>>()`).
- **Zero-cost** : une chaîne d'itérateurs est compilée en boucle aussi rapide qu'une boucle écrite à la main. C'est ainsi qu'on écrit `dot`, `matvec`, `softmax` en Rust.

**Pour l'exercice** : `a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>()`, `assert_eq!(a.len(), b.len())` (provoque la panique voulue), `f32::max`, `fold`, `.enumerate()`, `max_by(|a, b| a.1.partial_cmp(b.1).unwrap())`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 13.2 (Iterators)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `sum_of_squares_of_evens` : somme des carrés des nombres pairs
- `flatten_lines` : tous les mots de toutes les lignes, en minuscules, dans l'ordre
- `dot` : produit scalaire de deux vecteurs `f32` (panique si les longueurs diffèrent). **C'est l'opération de base d'un moteur d'inférence !**
- **Règle : pas de boucle `for` classique, uniquement des itérateurs.**

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `argmax` : indice du plus grand `f32` (`None` si vide) — c'est le « sampling glouton » d'un LLM
- `softmax` : stable numériquement (soustrais le max avant `exp`) — c'est ce qui transforme les logits en probabilités

## Vérifier

```bash
cargo test -p d12-iterators                          # base
cargo test -p d12-iterators -- --include-ignored     # base + bonus
cargo clippy -p d12-iterators -- -D warnings         # style et pièges classiques
cargo fmt -p d12-iterators                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Les itérateurs sont « paresseux » : que veut dire cela ? À quel moment le calcul a-t-il vraiment lieu ?
2. Pourquoi `softmax` doit soustraire le maximum avant d'appliquer `exp` ?
