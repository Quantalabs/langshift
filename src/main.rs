pub mod diaz_switkes;
pub mod abrams_strogatz;

fn main() {
    let community = diaz_switkes::Community {
        x: [330, 340, 330],
        m: [0.2, 0.4],
        g: [0.8, 0.01],
        alpha: 0.01,
        beta: 0.2,
    };
    
    community.solve(600.0);

    let community = abrams_strogatz::Community {
        x: 80.0,
        y: 20.0,
        c: 1.0,
        a: 1.31,
        s: 0.33
    };
    
    community.solve(20.0);
}