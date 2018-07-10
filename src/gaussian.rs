// This is based of code from https://athemathmo.github.io/2016/06/24/using-gmm-in-rust.html
// TODO: Currently waiting on confirmation of license.

use rand::distributions::{Normal, Distribution};
use rand::Rng;
use rand::thread_rng;

/// This gets samples from a Gaussian Mixture Model
/// count: The number of samples
/// distribution: A vec of tuples (mean, sd, weight)
/// Returns: A vec of samples
pub fn get_samples_from_gmm(
    count: usize,
    distributions: Vec<(f64, f64, f64)>)
    -> Vec<f64>
{
    let gaussians: Vec<Normal> = distributions
        .iter()
        .map(|(mean, sd, _)| Normal::new(*mean, *sd))
        .collect();

    let weights: Vec<f64> = distributions
        .iter()
        .map(|(_, _, weight)| *weight)
        .collect();
    
    let mut rng = thread_rng();
    let mut samples = Vec::with_capacity(count);

    for _ in 0..count {
        let chosen_guassian = gaussians[pick_distribution_id(&weights, &mut rng)];

        samples.push(chosen_guassian.sample(&mut rng));

    }

    samples
}

fn pick_distribution_id<R: Rng>(weights: &[f64], rng: &mut R) -> usize {
    let total_weight = weights
        .iter()
        .sum();

    // Generate a number between 0 and total_weight
    let random_number = rng.gen_range(0.0f64, total_weight);

    let mut cdf = 0.0f64;
    for (i, weight) in weights.iter().enumerate() {
        cdf += *weight;

        if random_number < cdf {
            return i;
        }
    }

    panic!("No random value was sampled!")
}