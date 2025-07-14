use diffsol::{CraneliftJitModule, MatrixCommon, NalgebraMat, OdeBuilder, OdeSolverMethod};
use plotly::{Plot, Scatter, layout::Axis, layout::Layout};

type M = diffsol::NalgebraMat<f64>;
type CG = CraneliftJitModule;

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

    pub fn solve(&self, t: f64) -> NalgebraMat<f64> {
        let problem = OdeBuilder::<M>::new()
            .build_from_diffsl::<CG>(&format!(
                "
                    c {{ {} }} a {{ {} }} s {{ {} }}
                    u_i {{
                        x = {},
                    }}
                    F_i {{
                        (1 - x) * c * pow(x, a) * s - x * c * pow(1 - x, a) * (1 - s),
                    }}
                ",
                self.c,
                self.a,
                self.s,
                self.x / self.n() as f64
            ))
            .unwrap();

        let mut solver = problem.tsit45().unwrap();
        let (ys, ts) = solver.solve(t).unwrap();

        let x = ys.inner().row(0).into_iter().copied().collect::<Vec<_>>();
        let time = ts.into_iter().collect::<Vec<_>>();

        let mut plot = Plot::new();
        let x_plot = Scatter::new(
            time.clone(),
            x
                .clone()
                .iter()
                .map(|x| x * self.n() as f64)
                .collect(),
        )
        .name("Monolingual Dominant");
        plot.add_trace(x_plot);

        let layout = Layout::new()
            .x_axis(Axis::new().title("Time"))
            .y_axis(Axis::new().title("Population"));
        plot.set_layout(layout);

        plot.write_html("plot.html");

        // Generate phase plot comparing x_0 and x_1
        let mut plot = Plot::new();
        let mono =
            Scatter::new(x.clone().iter().map(|x| x * self.n() as f64).collect(), x.clone().iter().map(|x| self.n() as f64 - x * self.n() as f64).collect()).name("Monolingual Underrepresented");
        plot.add_trace(mono);
        let layout = Layout::new()
            .x_axis(Axis::new().title("Monolingual Underrepresented"))
            .y_axis(Axis::new().title("Monolingual Dominant"));
        plot.set_layout(layout);
        plot.write_html("plot2.html");

        ys
    }
}
