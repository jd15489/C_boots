use rand::prelude::*;

const R: f64 = 0.00198720425864083;

pub fn sample_and_compute_c(
    vector: &Vec<f64>,
    generator: &mut ThreadRng,
    temperature: &f64,
    sample_size: &usize,
    molecules: &f64
) -> f64 {
    let mut sum = 0.;
    let mut sq_sum = 0.;

    for _ in 0..*sample_size {
        let index = generator.random_range(0..vector.len());
        sum += vector[index];
        sq_sum += vector[index].powf(2.);
    }

    let average = sum / *sample_size as f64;
    let sq_average = sq_sum / *sample_size as f64;
    let variance = sq_average - average.powf(2.);

    variance / (R * (temperature * temperature)) / molecules
}