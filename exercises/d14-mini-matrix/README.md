# J14 — Mini projet : une matrice `f32`

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : stockage row-major dans un `Vec<f32>`, indexation 2D → 1D, tranches, produit matriciel
**Lecture facultative (5 min)** : Aucune : c'est le premier pas vers ton moteur d'inférence !

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- Stockage **row-major** : l'élément (ligne r, colonne c) est à l'indice `r * cols + c`
- `zeros`, `from_vec` (`Err` si la taille ne correspond pas), `rows`, `cols`, `get`, `set`
- `row` : renvoie une ligne sous forme de tranche `&[f32]` (sans copie)
- `transpose` et `matmul` (`None` si les dimensions sont incompatibles)

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `matvec` : produit matrice × vecteur (`None` si dimensions incompatibles). C'est **l'opération dominante** d'un transformer à l'inférence : chaque couche en fait plusieurs

## Vérifier

```bash
cargo test -p d14-mini-matrix                          # base
cargo test -p d14-mini-matrix -- --include-ignored     # base + bonus
cargo clippy -p d14-mini-matrix -- -D warnings         # style et pièges classiques
cargo fmt -p d14-mini-matrix                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi le stockage row-major est plus rapide quand on parcourt une ligne que quand on parcourt une colonne ?
2. Quelle est la complexité de ton `matmul` ? Comment pourrais-tu l'accélérer (indice : ordre des boucles, cache) ?
