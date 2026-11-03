//! J06 — Option et combinateurs
//! Règle du jour : pas de `.unwrap()` ni de `.expect()` dans ton code.

/// Indice de la première occurrence de `target`, ou `None`.
pub fn find_index(values: &[i32], target: i32) -> Option<usize> {
    todo!()
}

/// `a / b`, ou `None` si `b == 0` (ou si ça déborde : `i32::MIN / -1`).
/// Indice : `i32::checked_div`.
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    todo!()
}

/// Premier nombre pair de la tranche.
pub fn first_even(values: &[i32]) -> Option<i32> {
    todo!()
}

/// Le double du premier nombre pair (ou `None`). Utilise `map` ou `?`.
pub fn double_first_even(values: &[i32]) -> Option<i32> {
    todo!()
}

// ---------- BONUS ----------

/// Parse `s` en i32 puis divise par 2 avec `safe_div`. `None` si le parsing échoue.
pub fn parse_and_halve(s: &str) -> Option<i32> {
    todo!()
}

/// Premier caractère de `s` en majuscule ASCII, ou `None` si `s` est vide.
pub fn first_char_upper(s: &str) -> Option<char> {
    todo!()
}
