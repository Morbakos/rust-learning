# J12 — Itérateurs

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `iter()`, `filter`, `map`, `zip`, `sum`, `flat_map`, `collect`, `max_by`
**Lecture facultative (5 min)** : Rust Book, chapitre 13.2 (Iterators)

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
