# J09 — Chaînes de caractères et UTF-8

**Durée** : 3-5 min de mini-leçon + ~10 min d'exercice · +10-15 min de bonus
**Notions** : `String` / `&str`, `chars()`, octets vs caractères, `u8`, `as`

## Mini-leçon (3-5 min)

**Une `String` est du texte UTF-8** : un caractère peut occuper de 1 à 4 octets. Conséquences :

- `s.len()` compte des **octets**, pas des caractères (`"é".len() == 2`).
- On ne peut pas écrire `s[0]` : ce serait ambigu. On passe par `s.chars()` (itérateur de `char`) ou par `s.bytes()` (itérateur d'octets).
- `&str` = vue sur du texte ; `String` = texte possédé et modifiable.

```rust
let s = "héllo";
s.len();                       // 6 octets
s.chars().count();             // 5 caractères
s.chars().rev().collect::<String>();      // "olléh"

let c = 'g';
c.is_alphanumeric();           // vrai pour lettres et chiffres
c.to_ascii_lowercase();        // 'g'

// Arithmétique sur les lettres ASCII :
let base = b'a';                               // 97 (un u8)
let shifted = ((c as u8 - base + 3) % 26 + base) as char;   // décalage de 3 : 'j'
```

- `char::is_ascii_lowercase()` / `is_ascii_uppercase()` pour tester la casse.
- `collect::<String>()` reconstruit une chaîne depuis un itérateur de `char`.
- Pour le décalage : réduis `shift % 26` d'abord pour éviter un dépassement de `u8`.

**Pour l'exercice** : `chars()`, `filter`, `map`, `eq`/`==` entre itérateurs ou `Vec<char>`, `take(n)`.

*Pour aller plus loin (facultatif)* : Rust Book, chapitre 8.2 (Strings)

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
