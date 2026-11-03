# J10 — Traits et polymorphisme

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `trait`, méthode par défaut, `impl Trait for Type`, `dyn Trait`, bornes génériques

## Mini-leçon (3-5 min)

**Un `trait`** décrit un comportement que des types peuvent implémenter (comme une interface). Il peut fournir des **méthodes par défaut**.

```rust
trait Greet {
    fn name(&self) -> String;

    fn hello(&self) -> String {               // méthode par défaut
        format!("Bonjour, {} !", self.name())
    }
}

struct English;
impl Greet for English {
    fn name(&self) -> String { "Sam".to_string() }
}
```

**Deux façons de les utiliser :**

1. **Génériques** `fn f<T: Greet>(x: &T)` : le compilateur génère une version par type (« monomorphisation ») : aucun coût à l'exécution. Le type doit être connu à la compilation.
2. **Objets-traits** `&dyn Greet` ou `Box<dyn Greet>` : **un seul type de conteneur pour des types différents** (liste hétérogène), avec un petit coût d'appel dynamique.

```rust
fn all_hello(items: &[Box<dyn Greet>]) -> Vec<String> {
    items.iter().map(|g| g.hello()).collect()
}

fn largest<T: PartialOrd + Copy>(xs: &[T]) -> Option<T> { /* ... */ }
//                ^ « borne » : T doit pouvoir être comparé ET copié
```

**Pour l'exercice** : `format!("{} says {}", self.name(), self.sound())`, `iter().map(...).collect()`, et pour `largest` : boucle avec comparaison, ou `fold`. Pour `Display`, utilise `write!(f, "...")`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 10.2 (Traits) (et la section sur les trait objects)

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
