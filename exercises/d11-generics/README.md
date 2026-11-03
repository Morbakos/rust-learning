# J11 — Génériques : une pile `Stack<T>`

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : types génériques, `impl<T>`, bornes de traits, `Default`

## Mini-leçon (3-5 min)

**Génériques** : écrire du code valable pour **n'importe quel type** `T`, vérifié à la compilation. Tu en utilises déjà : `Vec<T>`, `Option<T>`, `HashMap<K, V>`.

- `struct Stack<T>` : une pile d'éléments de type `T`.
- `impl<T> Stack<T> { ... }` : méthodes disponibles pour tous les `T`.
- On peut ajouter des **blocs `impl` conditionnels** : `impl<T: Display> Stack<T>` n'existe que si `T` sait s'afficher.

```rust
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self { Pair { a, b } }
    fn first(&self) -> &T { &self.a }         // renvoie une RÉFÉRENCE : on ne vole rien
}

impl<T: PartialOrd + Copy> Pair<T> {
    fn max(&self) -> T {                       // seulement si T est comparable et copiable
        if self.a > self.b { self.a } else { self.b }
    }
}
```

**Pourquoi `peek` renvoie `Option<&T>`** : on veut *regarder* sans retirer ni déplacer l'élément. Renvoyer `T` obligerait à le déplacer hors de la pile.

**Performance** : le compilateur génère une version spécialisée par type utilisé (monomorphisation), donc un `Stack<f32>` est aussi rapide qu'une version écrite à la main.

**Pour l'exercice** : `Vec::push`, `pop`, `last()`, `len()`, `is_empty()`, `iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 10.1 (Generic Data Types)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `Stack<T>` avec `new`, `push`, `pop`, `peek` (renvoie `Option<&T>`), `len`, `is_empty`
- Comportement LIFO : le dernier entré est le premier sorti

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `render` (pour `T: Display`) : `"[1, 2, 3]"` du bas vers le haut de la pile, `"[]"` si vide
- `max` (pour `T: PartialOrd + Copy`) : plus grand élément (`None` si vide)

## Vérifier

```bash
cargo test -p d11-generics                          # base
cargo test -p d11-generics -- --include-ignored     # base + bonus
cargo clippy -p d11-generics -- -D warnings         # style et pièges classiques
cargo fmt -p d11-generics                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi `peek` renvoie-t-elle `Option<&T>` et pas `Option<T>` ?
2. Que se passe-t-il à la compilation si tu appelles `stack.render()` avec un `T` qui n'implémente pas `Display` ?
