# J05 — Enums et pattern matching

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `enum` avec données, `match` exhaustif, `if let`, `split_once`, `parse`

## Mini-leçon (3-5 min)

**Enum.** Contrairement à beaucoup de langages, chaque variante peut **porter des données**. C'est parfait pour modéliser « une chose parmi plusieurs possibilités ».

**`match`** examine la variante et en extrait les données. Il doit être **exhaustif** : si tu oublies un cas, ça ne compile pas. `_` capture « tout le reste ». `if let` est un `match` à un seul cas.

```rust
enum Message {
    Ping,
    Text(String),
    Resize { w: u32, h: u32 },
}

fn describe(m: &Message) -> String {
    match m {
        Message::Ping => "ping".to_string(),
        Message::Text(s) => format!("texte : {s}"),
        Message::Resize { w, h } => format!("{w}x{h}"),
    }
}

if let Message::Text(s) = &msg {
    println!("{s}");
}
```

**Parser du texte.** `"a b".split_once(' ')` donne `Some(("a", "b"))`. `"42".parse::<i32>()` donne un `Result` ; `.ok()` le transforme en `Option`. On peut aussi faire un `match` sur un tuple ou sur une chaîne : `match name { "quit" => ..., _ => None }`.

**Pour l'exercice** : `std::f64::consts::PI`, `split_once`, `split_whitespace().collect::<Vec<_>>()`, et `?` marche aussi dans une fonction qui renvoie `Option`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 6 (Enums and Pattern Matching)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `area` : aire d'une `Shape` (cercle = π r², rectangle, triangle = base × hauteur / 2)
- `parse_command` : `"quit"` → `Quit`, `"move 3 -2"` → `Move { dx: 3, dy: -2 }`, `"say hello world"` → `Say("hello world")`, tout le reste (ou arguments invalides) → `None`

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `apply` : applique une commande à une position `(x, y)` (seul `Move` la change)

## Vérifier

```bash
cargo test -p d05-enums-match                          # base
cargo test -p d05-enums-match -- --include-ignored     # base + bonus
cargo clippy -p d05-enums-match -- -D warnings         # style et pièges classiques
cargo fmt -p d05-enums-match                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi le compilateur refuse un `match` qui oublie une variante ? Qu'est-ce que ça t'apporte ?
2. Dans quels cas préférer `if let` à `match` ?
