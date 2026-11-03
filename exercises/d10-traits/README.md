# J10 — Traits et polymorphisme

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `trait`, méthode par défaut, `impl Trait for Type`, `dyn Trait`, bornes génériques
**Lecture facultative (5 min)** : Rust Book, chapitre 10.2 (Traits) (et la section sur les trait objects)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- Méthode par défaut `describe` du trait `Animal` : `"<name> says <sound>"`
- Implémente `Animal` pour `Dog` (« Dog » / « Woof ») et `Cat` (« Cat » / « Meow »)
- `describe_all` : décrit une liste d'animaux hétérogènes (`Box<dyn Animal>`)
- `largest` : plus grand élément d'une tranche de `T: PartialOrd + Copy` (`None` si vide)

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `impl Display for Cat` : `"Cat(9 lives)"` pour un chat avec 9 vies

## Vérifier

```bash
cargo test -p d10-traits                          # base
cargo test -p d10-traits -- --include-ignored     # base + bonus
cargo clippy -p d10-traits -- -D warnings         # style et pièges classiques
cargo fmt -p d10-traits                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Quelle différence entre `&dyn Animal` (dispatch dynamique) et `fn f<T: Animal>(a: &T)` (génériques) ?
2. Pourquoi `largest` exige-t-elle `Copy` ici ? Comment s'en passer ?
