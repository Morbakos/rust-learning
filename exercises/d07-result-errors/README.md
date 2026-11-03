# J07 — Result et gestion d'erreurs

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `Result<T, E>`, opérateur `?`, type d'erreur personnalisé, `Display`, `std::error::Error`

## Mini-leçon (3-5 min)

**`Result<T, E>`** : `Ok(valeur)` ou `Err(erreur)`. C'est la façon idiomatique de signaler qu'une opération peut échouer, sans exceptions. Le compilateur t'oblige à gérer l'erreur.

**L'opérateur `?`** : « si c'est une erreur, sors de la fonction en la renvoyant ; sinon, déballe la valeur ». Il remplace des `match` répétitifs.

**Type d'erreur personnalisé.** On définit souvent un `enum` qui liste les causes d'échec possibles.

```rust
#[derive(Debug)]
enum ConfigError {
    Missing,
    BadNumber(String),
}

fn read_port(s: &str) -> Result<u16, ConfigError> {
    let raw = s.strip_prefix("port=").ok_or(ConfigError::Missing)?;  // Option -> Result
    raw.parse::<u16>()
        .map_err(|_| ConfigError::BadNumber(raw.to_string()))        // change le type d'erreur
}

fn total(a: &str, b: &str) -> Result<u16, ConfigError> {
    Ok(read_port(a)? + read_port(b)?)     // la 1re erreur arrête tout
}
```

- `ok_or(e)` : `Option` → `Result` (`None` devient `Err(e)`).
- `map_err(f)` : transforme l'erreur.
- Pour qu'une erreur soit « citoyenne » : implémenter `Display` (message lisible) et `std::error::Error`.

**`panic!`** (ou `unwrap`) est réservé aux bugs « impossibles », pas aux entrées utilisateur invalides.

**Pour l'exercice** : `line.split_once('=')`, `ok_or`, `map_err`, `?`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 9 (Error Handling)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `parse_kv` : `"age=42"` → `Ok(("age".to_string(), 42))`. Erreurs : pas de `=` → `MissingSeparator`, clé vide → `EmptyKey`, valeur non entière → `InvalidValue(<la valeur>)`
- `sum_values` : somme les valeurs de plusieurs lignes, s'arrête à la **première** erreur (utilise `?`)

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- Implémente `Display` pour `ParseError` (messages lisibles en français ou anglais)
- L'impl `std::error::Error` est déjà là : vérifie qu'elle compile avec ton `Display`

## Vérifier

```bash
cargo test -p d07-result-errors                          # base
cargo test -p d07-result-errors -- --include-ignored     # base + bonus
cargo clippy -p d07-result-errors -- -D warnings         # style et pièges classiques
cargo fmt -p d07-result-errors                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Que fait exactement l'opérateur `?` ? Que se passe-t-il si l'erreur n'est pas du même type que celui de la fonction ?
2. Pourquoi utiliser `Result` plutôt que `panic!` pour une entrée utilisateur invalide ?
