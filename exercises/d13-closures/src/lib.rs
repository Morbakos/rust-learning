//! J13 — Closures

/// Applique `f` `n` fois à `x`. Avec n = 0, renvoie `x`.
pub fn apply_n<F: Fn(i32) -> i32>(f: F, n: usize, x: i32) -> i32 {
    todo!()
}

/// Renvoie une closure qui ajoute `k` à son argument.
pub fn make_adder(k: i32) -> impl Fn(i32) -> i32 {
    let _ = k;
    move |_x: i32| -> i32 { todo!() }
}

/// Trie par longueur croissante (tri stable). Indice : `sort_by_key`.
pub fn sort_by_len(words: &mut [String]) {
    todo!()
}

/// Renvoie une closure qui renvoie 1, puis 2, puis 3, ... à chaque appel.
pub fn counter() -> impl FnMut() -> u32 {
    move || -> u32 { todo!() }
}

// ---------- BONUS ----------

/// `compose(f, g)` renvoie une closure qui calcule `g(f(x))`.
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    let _ = (&f, &g);
    move |_a: A| -> C { todo!() }
}
