//! J09 — Chaînes de caractères et UTF-8

/// Vrai si `s` est un palindrome en ignorant la casse et tout caractère
/// non alphanumérique. La chaîne vide est un palindrome.
pub fn is_palindrome(s: &str) -> bool {
    todo!()
}

/// Nombre de voyelles a, e, i, o, u (majuscules ou minuscules).
pub fn count_vowels(s: &str) -> usize {
    todo!()
}

/// Chiffre de César : décale chaque lettre ASCII de `shift` positions (cyclique),
/// en gardant la casse. Les autres caractères restent inchangés.
pub fn caesar(s: &str, shift: u8) -> String {
    todo!()
}

// ---------- BONUS ----------

/// `(nombre de caractères, nombre d'octets)` de `s`.
pub fn char_count_vs_byte_len(s: &str) -> (usize, usize) {
    todo!()
}

/// Les `n` premiers caractères de `s` (pas octets !).
pub fn truncate_chars(s: &str, n: usize) -> String {
    todo!()
}
