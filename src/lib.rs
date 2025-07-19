pub mod abrams_strogatz;
pub mod diaz_switkes;

extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn solve_abrams_strogatz(x: f64, y: f64, c: f64, a: f64, s: f64, t: usize) -> Vec<f64> {
    abrams_strogatz::Community { x, y, c, a, s }
        .solve(t)
}

#[wasm_bindgen]
pub fn solve_diaz_switkes(x: Vec<u32>, m: Vec<f64>, g: Vec<f64>, alpha: f64, beta: f64, t: usize, i: usize) -> Vec<f64> {
    let x: [u32; 3] = x.try_into().expect("Expected x to have length 3");
    let m: [f64; 2] = m.try_into().expect("Expected m to have length 2");
    let g: [f64; 2] = g.try_into().expect("Expected g to have length 2");
    
    let solution = diaz_switkes::Community { x, m, g, alpha, beta }
        .solve(t);
    
    solution
        .iter()
        .map(|y| y[i])
        .collect()
}