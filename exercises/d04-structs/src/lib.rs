//! J04 — Structs et méthodes

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        todo!()
    }

    /// Un carré de côté `side`.
    pub fn square(side: f64) -> Self {
        todo!()
    }

    pub fn area(&self) -> f64 {
        todo!()
    }

    pub fn perimeter(&self) -> f64 {
        todo!()
    }

    /// Vrai si `self` est strictement plus large ET plus haut que `other`.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    /// Multiplie largeur et hauteur par `factor`.
    pub fn scale(&mut self, factor: f64) {
        todo!()
    }

    // ---------- BONUS ----------

    pub fn is_square(&self) -> bool {
        todo!()
    }
}

/// Le rectangle d'aire maximale, ou `None` si la tranche est vide.
pub fn largest(rects: &[Rectangle]) -> Option<&Rectangle> {
    todo!()
}
