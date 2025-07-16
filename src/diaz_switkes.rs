// use diffsol::{
//     NalgebraMat, OdeBuilder, OdeSolverMethod
// };

// type M = diffsol::NalgebraMat<f64>;
// type LS = diffsol::NalgebraLU<f64>;

use differential_equations::prelude::*;
use nalgebra::{SVector, vector};

#[derive(Copy, Clone)]
pub struct Community {
    pub x: [u32; 3],
    pub m: [f64; 2],
    pub g: [f64; 2],
    pub alpha: f64,
    pub beta: f64,
}

impl ODE<f64, SVector<f64, 3>> for Community {
    fn diff(&self, _t: f64, y: &SVector<f64, 3>, dydt: &mut SVector<f64, 3>) {
        dydt[0] = self.m[0] * y[0] * y[2] - self.m[1] * y[0] * y[1] + self.alpha * y[0];
        dydt[1] = -self.m[0] * y[0] * y[2]
            + self.m[0] * y[1] * y[2]
            + self.m[1] * y[0] * y[1]
            + self.m[1] * y[1] * y[0]
            - self.alpha * y[0]
            - self.beta * y[1];
        dydt[2] = -self.m[0] * y[1] * y[2] - self.m[1] * y[1] * y[0] + self.beta * y[1];
    }
}

impl Community {
    pub fn n(&self) -> u32 {
        self.x.iter().sum()
    }

    pub fn solve(&self, t: f64) -> Vec<Vec<f64>> {
        let problem = ODEProblem::new(
            self.clone(),
            0.0,
            t,
            vector![self.x[0] as f64, self.x[1] as f64, self.x[2] as f64],
        );

        let mut solver = ExplicitRungeKutta::dop853()
            .rtol(1e-8)
            .atol(1e-6);

        let solution = match problem.solve(&mut solver) {
            Ok(y) => y,
            Err(e) => panic!("{}", e),
        };

        solution.iter().map(|(_, y)| {
            vec![y[0], y[1], y[2]]
        }).collect()
    }
}
