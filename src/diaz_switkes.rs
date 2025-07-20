use diffeq::ode::{Ode, problem::OdeProblem};

#[derive(Copy, Clone)]
pub struct Community {
    pub x: [u32; 3],
    pub m: [f64; 2],
    pub g: [f64; 2],
    pub alpha: f64,
    pub beta: f64,
}

impl Community {
    pub fn n(&self) -> u32 {
        self.x.iter().sum()
    }

    pub fn system(&self, _: f64, v: &Vec<f64>) -> Vec<f64> {
        let (x_1, b, x_2) = (v[0], v[1], v[2]);

        vec![
            self.m[0] * x_1 * b - self.g[0] * x_1 * x_2 + self.alpha * x_1,
            -self.m[0] * x_1 * b + self.m[1] * x_2 * b + self.g[0] * x_1 * x_2 
                + self.g[1] * x_2 * x_1 - self.beta * x_2 - self.alpha * x_1,
            - self.m[1] * x_2 * b - self.g[1] * x_2 * x_1 + self.beta * x_2
        ]
    }

    pub fn solve(&self, t: usize) -> Vec<Vec<f64>> {
        let problem = OdeProblem::builder()
            .tspan_linspace(0.0, t as f64, t * 100)
            .fun(|_, y| self.system(0.0, y))
            .init(vec![(self.x[0] as f64) / (self.n() as f64), (self.x[1] as f64) / (self.n() as f64), (self.x[2] as f64) / (self.n() as f64)])
            .build()
            .unwrap();

        let solution = problem
            .solve(Ode::Feuler, Default::default())
            .unwrap();

        solution
            .yout
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 100 == 0)
            .map(|(_, y)| y.clone().iter().map(|y| y * (self.n() as f64)).collect())
            .collect()
    }
}
