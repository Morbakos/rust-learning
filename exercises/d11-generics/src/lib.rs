//! J11 — Génériques : une pile `Stack<T>`

#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, item: T) {
        todo!()
    }

    /// Retire et renvoie l'élément du dessus.
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// Regarde l'élément du dessus sans le retirer.
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------- BONUS ----------

impl<T: std::fmt::Display> Stack<T> {
    /// "[1, 2, 3]" (du bas vers le haut), "[]" si la pile est vide.
    pub fn render(&self) -> String {
        todo!()
    }
}

impl<T: PartialOrd + Copy> Stack<T> {
    /// Plus grand élément de la pile, ou `None` si elle est vide.
    pub fn max(&self) -> Option<T> {
        todo!()
    }
}
