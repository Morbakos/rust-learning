# J07 — Result et gestion d'erreurs

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `Result<T, E>`, opérateur `?`, type d'erreur personnalisé, `Display`, `std::error::Error`
**Lecture facultative (5 min)** : Rust Book, chapitre 9 (Error Handling)

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
