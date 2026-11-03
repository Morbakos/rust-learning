# J14 — Mini projet : une matrice `f32`

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : stockage row-major dans un `Vec<f32>`, indexation 2D → 1D, tranches, produit matriciel

## Mini-leçon (3-5 min)

**Pourquoi des matrices ?** Les poids d'un LLM sont des matrices. À l'inférence, la grande majorité du temps est passée dans des produits **matrice × vecteur**. Si tu écris ça bien, tu as écrit l'essentiel d'un moteur.

**Stockage row-major** : on aplatit la matrice ligne après ligne dans un seul `Vec<f32>` (contigu en mémoire, donc très favorable au cache du processeur).

```text
[[1, 2, 3],        data = [1, 2, 3, 4, 5, 6]     rows = 2, cols = 3
 [4, 5, 6]]
élément (r, c)  →  data[r * cols + c]      ex. (1, 0) → 1*3 + 0 = 3 → 4
ligne r         →  &data[r * cols .. (r + 1) * cols]     (une tranche, sans copie)
```

**Produit matriciel** : si `A` est `n×k` et `B` est `k×m`, alors `C = A × B` est `n×m` avec
`C[i][j] = somme sur t de A[i][t] × B[t][j]`. Les dimensions doivent être compatibles (`A.cols == B.rows`), sinon on renvoie `None`.

**Pour l'exercice** :
- `vec![0.0; rows * cols]` crée un `Vec` de zéros.
- `get` / `set` : l'indexation d'un `Vec` panique déjà hors du tableau, mais `(0, cols)` tomberait sur la ligne suivante sans paniquer. Ajoute un `assert!(r < self.rows && c < self.cols)` pour respecter la doc.
- Trois boucles imbriquées suffisent pour `matmul` ; pense à celle qui parcourt `t` en dernier.
- `matvec` est un `matmul` avec une seule colonne : tu peux aussi réutiliser `row(i)` et le `dot` de J12.

*Pour aller plus loin (facultatif)* : Aucune : c'est le premier pas vers ton moteur d'inférence !

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
