# J09 — Chaînes de caractères et UTF-8

**Durée** : 15 min (base) · +15 min (bonus)
**Notions** : `String` / `&str`, `chars()`, octets vs caractères, `u8`, `as`
**Lecture facultative (5 min)** : Rust Book, chapitre 8.2 (Strings)

## Consigne

Ouvre `src/lib.rs` et remplace chaque `todo!()` par ton implémentation.
Les tests sont dans `tests/base.rs` : **ne les modifie pas**.

- `is_palindrome` : ignore la casse et tout ce qui n'est pas alphanumérique
- `count_vowels` : nombre de voyelles `aeiou` (insensible à la casse)
- `caesar` : chiffre de César sur les lettres ASCII (casse conservée, le reste inchangé). Indice : réduis d'abord `shift % 26`

## Bonus (facultatif, +15 min)

Les tests bonus sont dans `tests/bonus.rs` (marqués `#[ignore]`). Les fonctions
correspondantes sont à la fin de `src/lib.rs`.

- `char_count_vs_byte_len` : renvoie `(nombre de caractères, nombre d'octets)`
- `truncate_chars` : garde les `n` premiers **caractères** (sans jamais couper un caractère UTF-8 en deux)

## Vérifier

```bash
cargo test -p d09-strings                          # base
cargo test -p d09-strings -- --include-ignored     # base + bonus
cargo clippy -p d09-strings -- -D warnings         # style et pièges classiques
cargo fmt -p d09-strings                           # formatage
```

## Questions de compréhension

Ton agent tuteur te posera ces questions (ou d'autres) une fois les tests au vert :

1. Pourquoi ne peut-on pas écrire `s[0]` sur une `String` en Rust ?
2. Quelle différence entre `char`, `u8` et `&str` ?
