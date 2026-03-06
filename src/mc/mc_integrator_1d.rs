extern crate rand;
use rand::prelude::*;

fn monte_carlo_integrate<F>(f: F, a: f64, b: f64, samples: u64) -> f64
where
    F: Fn(f64) -> f64,
{
    // montercarlo integration 1d
    // intgeral = (b-a) * avg
    // avg = sum of f(x)/samples

    let mut rng = rand::rng();
    let mut sum = 0.0;

    // sum of f(x)
    for _ in 0..samples {
        let x = rng.random_range(a..b);
        sum += f(x);
    }

    let avg = sum / (samples as f64);
    // integrate
    (b - a) * avg
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let f = |x: f64| x * x;
        let samples = 1_000_000;
        let est = monte_carlo_integrate(f, 0.0, 1.0, samples);
        assert!(0.32 < est);
        assert!(0.34 > est);
    }
}
