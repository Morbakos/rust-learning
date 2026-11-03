# J06 — Option et combinateurs

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `Option<T>`, `?` sur Option, `map`, `and_then`, `unwrap_or`, `checked_*`

## Mini-leçon (3-5 min)

**`Option<T>`** remplace `null` : soit `Some(valeur)`, soit `None`. Le compilateur t'**oblige** à traiter le cas `None`, donc plus de plantage « null pointer ».

Façons de l'utiliser (de la plus explicite à la plus concise) :

```rust
// 1. match
match find(&v, 3) {
    Some(i) => println!("trouvé en {i}"),
    None => println!("absent"),
}

// 2. combinateurs
let doubled: Option<i32> = Some(4).map(|x| x * 2);                 // Some(8)
let chained = Some(4).and_then(|x| if x > 0 { Some(x) } else { None });
let n = None.unwrap_or(0);                                         // valeur par défaut

// 3. l'opérateur ? : « si None, sors de la fonction avec None »
fn first_double(v: &[i32]) -> Option<i32> {
    let first = v.first()?;       // si v est vide, on renvoie None ici
    Some(first * 2)
}
```

- `map` transforme la valeur **si elle existe**.
- `and_then` enchaîne une opération qui **peut elle-même échouer** (elle renvoie une `Option`).
- `.unwrap()` plante si `None` : à éviter, d'où la règle du jour.

**Pour l'exercice** : `iter().position(|&x| x == target)`, `iter().find(...)`, `a.checked_div(b)` (renvoie `None` si `b == 0` ou en cas de dépassement), `s.parse::<i32>().ok()`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 6.1 (Option) — et la doc de `Option` dans la std

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `find_index` : indice de la première occurrence de `target`
- `safe_div` : division entière, `None` si diviseur nul (ou dépassement : indice → `checked_div`)
- `first_even` : premier nombre pair
- `double_first_even` : le double du premier pair, en utilisant `map` ou `?`
- **Règle : aucun `.unwrap()` ni `.expect()` dans ton code.**

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `parse_and_halve` : parse une chaîne en `i32` puis divise par 2 via `safe_div`
- `first_char_upper` : premier caractère en majuscule ASCII (`None` si chaîne vide)

## Vérifier

```bash
cargo test -p d06-option                          # base
cargo test -p d06-option -- --include-ignored     # base + bonus
cargo clippy -p d06-option -- -D warnings         # style et pièges classiques
cargo fmt -p d06-option                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Quelle différence entre `map` et `and_then` sur une `Option` ?
2. Pourquoi Rust n'a pas de `null` ? Qu'est-ce qui remplace le « null pointer exception » ?
