//! J02 — Ownership : qui possède quoi ?

/// Prend possession de `s` et la renvoie avec sa longueur **en octets**.
pub fn take_and_give_back(s: String) -> (String, usize) {
    todo!()
}

/// Ajoute "!" à la fin de la chaîne, sans en créer une nouvelle.
pub fn add_exclaim(s: &mut String) {
    todo!()
}

/// Consomme `a` et lui ajoute `b` à la fin. Renvoie le résultat.
pub fn concat_owned(a: String, b: &str) -> String {
    todo!()
}

// ---------- BONUS ----------

/// Retire de `v` tous les nombres pairs et les renvoie (dans l'ordre d'origine).
/// Après l'appel, `v` ne contient plus que les impairs.
pub fn drain_evens(v: &mut Vec<i32>) -> Vec<i32> {
    todo!()
}
