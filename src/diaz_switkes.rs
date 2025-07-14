use diffsol::{
    CraneliftJitModule, NalgebraMat, OdeBuilder, OdeSolverMethod, MatrixCommon
};
use plotly::{layout::Axis, layout::Layout, Plot, Scatter};

type M = diffsol::NalgebraMat<f64>;
type LS = diffsol::NalgebraLU<f64>;
type CG = CraneliftJitModule;

pub struct Community {
    pub x: [u32; 3],
    pub m: [f64; 2],
    pub g: [f64; 2],
    pub alpha: f64,
    pub beta: f64
}

impl Community {
    pub fn n(&self) -> u32 {
        self.x.iter().sum()
    }

    pub fn solve(&self, t: f64) -> NalgebraMat<f64> {
        let problem = OdeBuilder::<M>::new()
            .build_from_diffsl::<CG>(
                &format!("
                    m1 {{ {} }} m2 {{ {} }} g1 {{ {} }} g2 {{ {} }} a {{ {} }} c {{ {} }}
                    u_i {{
                        x1 = {},
                        b = {},
                        x2 = {},
                    }}
                    F_i {{
                        m1 * x1 * b - g1 * x1 * x2 + a * x1,
                        - m1 * x1 * b + m2 * x2 * b + g1 * x1 * x2 + g2 * x2 * x1 - a * x1 - c * x2,
                        - m2 * x2 * b - g2 * x2 * x1 + c * x2,
                    }}
                ", self.m[0], self.m[1], self.g[0], self.g[1], self.alpha, self.beta, self.x[0] as f64 / self.n() as f64, self.x[1] as f64 / self.n() as f64, self.x[2] as f64 / self.n() as f64)
            )
            .unwrap();

            let mut solver = problem.bdf::<LS>().unwrap();
            let (ys, ts) = solver.solve(t).unwrap();

            let mono_dom = ys.inner().row(0).into_iter().copied().collect::<Vec<_>>();
            let bilingual = ys.inner().row(1).into_iter().copied().collect::<Vec<_>>();
            let mono_under = ys.inner().row(2).into_iter().copied().collect::<Vec<_>>();
            let time = ts.into_iter().collect::<Vec<_>>();

            let mut plot = Plot::new();
            let mono_d = Scatter::new(time.clone(), mono_dom.clone().iter().map(|x| x * self.n() as f64).collect()).name("Monolingual Dominant");
            let bil = Scatter::new(time.clone(), bilingual.clone().iter().map(|x| x * self.n() as f64).collect()).name("Bilingual");
            let mono_u = Scatter::new(time.clone(), mono_under.clone().iter().map(|x| x * self.n() as f64).collect()).name("Monolingual Underrepresented");
            plot.add_trace(mono_d);
            plot.add_trace(bil);
            plot.add_trace(mono_u);

            let layout = Layout::new()
                .x_axis(Axis::new()
                    .title("Time")
                )
                .y_axis(Axis::new()
                    .title("Population")
                );
            plot.set_layout(layout);

            plot.write_html("plot.html");

            // Generate phase plot comparing x_0 and x_1
            let mut plot = Plot::new();
            let mono = Scatter::new(mono_dom.clone(), mono_under.clone()).name("Monolingual Underrepresented");
            plot.add_trace(mono);
            let layout = Layout::new()
                .x_axis(Axis::new()
                    .title("Monolingual Dominant")
                )
                .y_axis(Axis::new()
                    .title("Monolingual Underrepresented")
                );
            plot.set_layout(layout);
            plot.write_html("plot2.html");

            ys
    }
}
