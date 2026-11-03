//! J14 — Mini projet : une matrice f32 (stockage row-major)

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f32>, // row-major : (r, c) est à l'indice r * cols + c
}

impl Matrix {
    /// Matrice `rows x cols` remplie de zéros.
    pub fn zeros(rows: usize, cols: usize) -> Self {
        todo!()
    }

    /// Construit depuis des données row-major.
    /// `Err(message)` si `data.len() != rows * cols`.
    pub fn from_vec(rows: usize, cols: usize, data: Vec<f32>) -> Result<Self, String> {
        todo!()
    }

    pub fn rows(&self) -> usize {
        todo!()
    }

    pub fn cols(&self) -> usize {
        todo!()
    }

    /// Élément (r, c). Panique si hors limites.
    pub fn get(&self, r: usize, c: usize) -> f32 {
        todo!()
    }

    /// Modifie l'élément (r, c). Panique si hors limites.
    pub fn set(&mut self, r: usize, c: usize, value: f32) {
        todo!()
    }

    /// La ligne `r` sous forme de tranche, sans copie. Panique si hors limites.
    pub fn row(&self, r: usize) -> &[f32] {
        todo!()
    }

    pub fn transpose(&self) -> Matrix {
        todo!()
    }

    /// Produit matriciel `self x other`. `None` si `self.cols != other.rows`.
    pub fn matmul(&self, other: &Matrix) -> Option<Matrix> {
        todo!()
    }

    // ---------- BONUS ----------

    /// Produit matrice x vecteur. `None` si `v.len() != self.cols`.
    pub fn matvec(&self, v: &[f32]) -> Option<Vec<f32>> {
        todo!()
    }
}
