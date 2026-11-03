# J13 — Closures

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `Fn` / `FnMut` / `FnOnce`, `move`, `impl Fn`, closures qui capturent leur environnement

## Mini-leçon (3-5 min)

**Une closure** est une fonction anonyme qui peut **capturer** des variables de son environnement : `|x| x + offset`.

Selon ce qu'elle fait de ce qu'elle capture, elle implémente un ou plusieurs traits :

- **`Fn`** : lit seulement ce qu'elle capture (appelable plusieurs fois, même partagée).
- **`FnMut`** : **modifie** ce qu'elle capture (il faut la déclarer `mut`).
- **`FnOnce`** : **consomme** ce qu'elle capture (appelable une seule fois).

`move` force la closure à **prendre possession** des variables capturées (indispensable quand elle doit survivre à la fonction qui l'a créée).

```rust
let offset = 10;
let add = |x: i32| x + offset;            // Fn : lit `offset`
add(1);                                   // 11

let mut total = 0;
let mut accumulate = |x: i32| total += x; // FnMut : modifie `total`
accumulate(3);
accumulate(4);                            // total vaut 7 après la fin de l'emprunt

fn make_scaler(k: i32) -> impl Fn(i32) -> i32 {
    move |x| x * k                        // `move` : k est copié DANS la closure
}

let mut names = vec!["bb", "a", "ccc"];
names.sort_by_key(|s| s.len());           // closure passée en argument
```

`impl Fn(i32) -> i32` (en argument ou en retour) dit « n'importe quelle closure ou fonction de cette forme ».

**Pour l'exercice** : pour `counter`, la closure garde un compteur interne : `let mut n = 0; move || { n += 1; n }`. Pour `apply_n` : une simple boucle `for _ in 0..n`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 13.1 (Closures)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `apply_n` : applique `f` `n` fois à `x` (`n = 0` renvoie `x`)
- `make_adder` : renvoie une closure qui ajoute `k` à son argument
- `sort_by_len` : trie des `String` par longueur croissante (tri **stable**), avec une closure
- `counter` : renvoie une closure qui renvoie 1, puis 2, puis 3… à chaque appel

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `compose` : `compose(f, g)(x) == g(f(x))`

## Vérifier

```bash
cargo test -p d13-closures                          # base
cargo test -p d13-closures -- --include-ignored     # base + bonus
cargo clippy -p d13-closures -- -D warnings         # style et pièges classiques
cargo fmt -p d13-closures                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi `counter` doit renvoyer un `FnMut` et pourquoi faut-il `move` ?
2. Quelle différence entre `Fn`, `FnMut` et `FnOnce` ? Donne un exemple de chacun.
