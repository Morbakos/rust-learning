//! J05 — Enums et pattern matching

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

/// Aire de la forme. Indice : `std::f64::consts::PI`.
pub fn area(shape: &Shape) -> f64 {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    Move { dx: i32, dy: i32 },
    Say(String),
}

/// "quit" -> Quit
/// "move 3 -2" -> Move { dx: 3, dy: -2 }
/// "say hello world" -> Say("hello world")
/// Tout le reste (commande inconnue, arguments manquants ou invalides) -> None.
/// Indices : `split_once(' ')`, `split_whitespace()`, `.parse::<i32>().ok()`.
pub fn parse_command(input: &str) -> Option<Command> {
    todo!()
}

// ---------- BONUS ----------

/// Applique la commande à une position. Seul `Move` modifie la position.
pub fn apply(pos: (i32, i32), cmd: &Command) -> (i32, i32) {
    todo!()
}
