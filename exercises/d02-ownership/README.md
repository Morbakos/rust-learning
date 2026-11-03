# J02 — Ownership : qui possède quoi ?

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : move, `String` vs `&str`, `&mut`, une seule référence mutable à la fois

## Mini-leçon (3-5 min)

**C'est LE concept de Rust.** Pas de ramasse-miettes : la mémoire est libérée automatiquement, au moment précis où son propriétaire sort de portée. C'est ce qui rend Rust rapide **et** sûr (utile pour gérer de gros tenseurs en mémoire).

**Trois règles** : chaque valeur a un **propriétaire** ; il n'y en a qu'**un** à la fois ; quand il disparaît, la valeur est libérée.

**Move.** Affecter ou passer une valeur à une fonction **déplace** la propriété : l'ancienne variable n'est plus utilisable. Les petits types simples (`i32`, `f64`, `bool`, `char`) sont *copiés* à la place. `.clone()` fait une copie explicite.

**Emprunter.** `&T` prête en lecture, `&mut T` prête en écriture, sans céder la propriété.

```rust
let s1 = String::from("salut");
let s2 = s1;                 // move : s1 n'est plus utilisable
// println!("{s1}");         // ❌ erreur de compilation

fn consume(s: String) {}              // prend possession (s est libérée à la fin)
fn read(s: &String) -> usize { s.len() }          // emprunt en lecture
fn edit(s: &mut String) { s.push('?'); }          // emprunt en écriture

let mut t = String::from("hey");
edit(&mut t);                // on prête t en écriture, on le récupère après
```

**Pour l'exercice** : `s.len()` compte des **octets** ; `push_str`, `push`, `Vec::retain`, `Vec::drain(..)`. Le compilateur t'expliquera ses refus : lis bien ses messages, ils sont excellents.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 4.1 (What is Ownership?)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `take_and_give_back` : prend une `String`, la renvoie avec sa longueur **en octets**
- `add_exclaim` : ajoute `!` à la fin d'une `String` empruntée en mutable
- `concat_owned` : consomme `a` et lui ajoute `b`

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `drain_evens` : retire les nombres pairs d'un `Vec` et les renvoie, `v` ne garde que les impairs

## Vérifier

```bash
cargo test -p d02-ownership                          # base
cargo test -p d02-ownership -- --include-ignored     # base + bonus
cargo clippy -p d02-ownership -- -D warnings         # style et pièges classiques
cargo fmt -p d02-ownership                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Que se passe-t-il si tu utilises `s` après l'avoir passée à `take_and_give_back` (sans récupérer la valeur de retour) ? Pourquoi ?
2. Pourquoi `"héllo".len()` vaut 6 et pas 5 ?
