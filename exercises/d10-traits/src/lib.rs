//! J10 — Traits et polymorphisme

use std::fmt;

pub trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;

    /// "<name> says <sound>", par exemple "Dog says Woof".
    fn describe(&self) -> String {
        todo!()
    }
}

pub struct Dog;

pub struct Cat {
    pub lives: u8,
}

impl Animal for Dog {
    fn name(&self) -> String {
        todo!()
    }
    fn sound(&self) -> String {
        todo!()
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        todo!()
    }
    fn sound(&self) -> String {
        todo!()
    }
}

/// Décrit chaque animal (via `describe`), dans l'ordre.
pub fn describe_all(animals: &[Box<dyn Animal>]) -> Vec<String> {
    todo!()
}

/// Plus grand élément de la tranche, ou `None` si elle est vide.
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    todo!()
}

// ---------- BONUS ----------

impl fmt::Display for Cat {
    /// Format attendu : "Cat(9 lives)".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
