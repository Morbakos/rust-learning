//! J07 — Result et gestion d'erreurs

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingSeparator,
    EmptyKey,
    InvalidValue(String),
}

/// "age=42" -> Ok(("age".to_string(), 42))
/// Pas de '=' -> Err(MissingSeparator) ; clé vide -> Err(EmptyKey) ;
/// valeur qui n'est pas un i32 -> Err(InvalidValue(<la valeur>)).
/// Indice : `split_once('=')`, `ok_or(...)`, `map_err(...)`, `?`.
pub fn parse_kv(line: &str) -> Result<(String, i32), ParseError> {
    todo!()
}

/// Somme des valeurs de toutes les lignes. Stoppe à la première erreur.
pub fn sum_values(lines: &[&str]) -> Result<i32, ParseError> {
    todo!()
}

// ---------- BONUS ----------

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ParseError {}
