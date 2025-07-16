pub mod abrams_strogatz;
pub mod diaz_switkes;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve_abrams_strogatz(x: f64, y: f64, c: f64, a: f64, s: f64, t: f64) -> Vec<f64> {
    abrams_strogatz::Community { x, y, c, a, s }
        .solve(t)
}
