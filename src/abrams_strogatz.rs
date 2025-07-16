use differential_equations::prelude::*;

#[derive(Copy, Clone)]
pub struct Community {
    pub x: f64,
    pub y: f64,
    pub c: f64,
    pub a: f64,
    pub s: f64,
}

impl ODE for Community {
    fn diff(&self, _t: f64, y: &f64, dydt: &mut f64) {
        *dydt = (1.0 - y) * self.c * y.powf(self.a) * self.s - y * self.c * (1.0 - y).powf(self.a) * (1.0 - self.s);
    }
}

impl Community {
    pub fn n(&self) -> u32 {
        (self.x + self.y) as u32
    }

    pub fn solve(&self, t: f64) -> Vec<f64> {
        let problem = ODEProblem::new(
            *self,
            0.0,
            t,
            self.y / (self.n() as f64),
        );

        let mut solver = ExplicitRungeKutta::dop853()
            .rtol(1e-8)
            .atol(1e-6);

        match problem.solve(&mut solver) {
            Ok(y) => y,
            Err(e) => panic!("{}", e),
        }.iter().map(|(_, y)| *y).collect()
    }
}
