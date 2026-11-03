//! J08 — Vec et HashMap

use std::collections::HashMap;

/// Compte les occurrences de chaque mot (en minuscules, séparés par des espaces).
pub fn word_count(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Les `n` mots les plus fréquents : tri par compte décroissant,
/// puis par mot croissant en cas d'égalité.
pub fn top_n(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    todo!()
}

/// Valeurs uniques, triées par ordre croissant.
pub fn dedup_sorted(values: &[i32]) -> Vec<i32> {
    todo!()
}

// ---------- BONUS ----------

/// Regroupe les mots par longueur (en caractères), en gardant l'ordre d'apparition.
pub fn group_by_len(words: &[&str]) -> HashMap<usize, Vec<String>> {
    todo!()
}
