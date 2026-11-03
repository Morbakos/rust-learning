# J04 — Structs et méthodes

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `struct`, `impl`, `&self` vs `&mut self`, fonctions associées, `derive`

## Mini-leçon (3-5 min)

**`struct`** regroupe des champs nommés. Les **méthodes** s'écrivent dans un bloc `impl`.

- `&self` : la méthode **lit** l'objet.
- `&mut self` : elle le **modifie**.
- `self` : elle le **consomme** (rare).
- Pas de `self` : *fonction associée*, appelée avec `Type::nom(...)` (le « constructeur » est par convention `new`). `Self` désigne le type courant.

`#[derive(...)]` génère automatiquement des comportements : `Debug` (affichage `{:?}`), `Clone`, `PartialEq` (`==`)...

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {          // fonction associée
        Point { x, y }                        // raccourci : x: x, y: y
    }

    fn norm(&self) -> f64 {                   // lecture
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn shift(&mut self, dx: f64) {            // écriture
        self.x += dx;
    }
}

let mut p = Point::new(3.0, 4.0);
p.shift(1.0);
println!("{:?} {}", p, p.norm());
```

**Pour l'exercice** : `f64::max`, `a > b`, et pour `largest` : `iter().max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())` (les `f64` ne sont pas `Ord` à cause de `NaN`, d'où `partial_cmp`).

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 5 (Using Structs)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `Rectangle::new`, `Rectangle::square` (fonctions associées)
- `area` et `perimeter` (méthodes `&self`)
- `can_hold` : `self` est strictement plus grand que `other` en largeur ET en hauteur
- `scale` : multiplie les deux dimensions (méthode `&mut self`)

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `is_square`
- `largest` : le rectangle de plus grande aire d'une tranche (`None` si vide), sans copie

## Vérifier

```bash
cargo test -p d04-structs                          # base
cargo test -p d04-structs -- --include-ignored     # base + bonus
cargo clippy -p d04-structs -- -D warnings         # style et pièges classiques
cargo fmt -p d04-structs                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi `area` prend `&self` alors que `scale` prend `&mut self` ?
2. Que fait `#[derive(Clone, PartialEq)]` exactement ?
