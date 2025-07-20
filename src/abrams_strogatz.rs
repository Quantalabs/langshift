use diffeq::ode::{problem::OdeProblem, Ode};

#[derive(Copy, Clone)]
pub struct Community {
    pub x: f64,
    pub y: f64,
    pub c: f64,
    pub a: f64,
    pub s: f64,
}

impl Community {
    pub fn n(&self) -> u32 {
        (self.x + self.y) as u32
    }

    pub fn system(&self, _: f64, v: &Vec<f64>) -> Vec<f64> {
        let x = v[0];

        vec![
            (1.0 - x) * self.c * x.powf(self.a) * self.s - x * self.c * (1.0 - x).powf(self.a) * (1.0 - self.s),
        ]
    }

    pub fn solve(&self, t: usize) -> Vec<f64> {
        let problem = OdeProblem::builder()
            .tspan_linspace(0.0, t as f64, t * 100)
            .fun(|_, y| self.system(0.0, y))
            .init(vec![self.x / (self.n() as f64)])
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
            .map(|(_, y)| y[0] * (self.n() as f64))
            .collect()
    }
}
