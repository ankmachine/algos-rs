extern crate rand;
use rand::prelude::*;
use std::f64::consts::PI;

fn mc_importance_sampling<F>(f: F, a: f64, b: f64, samples: u64) -> f64
where
    F: Fn(f64) -> f64,
{
    //The key benefit of IS  is to reduce variance
    // by sampling more from "important" regions of
    // the function.
    let mut rng = rand::rng();
    let mut sum = 0.0;
    let range = b - a;

    // sum of f(x)
    for _ in 0..samples {
        let x = rng.random_range(a..b);
        // The PDF for a uniform distribution on [a, b] is 1/(b-a)
        let proposal_pdf = 1.0 / range;
        sum += f(x) / proposal_pdf;
    }

    sum / samples as f64
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let f = |x: f64| x.sin();
        let samples = 1_000_000;
        let est = mc_importance_sampling(f, 0.0, PI, samples);
        assert_eq!(2.0, est.round());
    }
}
