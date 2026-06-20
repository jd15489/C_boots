use std::env;
use std::io::Error;

use rand::prelude::*;
use csv::Reader;

const R: f64 = 0.00198720425864083;
const N_SAMPLES: usize = 1000;
const N: usize = 900;
const N_RESAMPLE: usize = 100000;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Usage: <program> <string>"
        ));
    }

    let filename = &args[1];
    let temperature: &f64 = &args[2].parse().expect("Not a valid temperature");

    let mut rng = rand::rng();
    let energies = read_file(filename);
    let mut Cs: Vec<f64> = Vec::with_capacity(N_SAMPLES);

    for i in 0..N_SAMPLES {
        println!("\r{} / {}", i, N_SAMPLES);
        let C = sample_and_compute_c(
            &energies,
            &mut rng,
            temperature,
        );
        Cs.push(C);
    }

    let average_c = average(&Cs);
    let variance_c = variance(&Cs);

    println!("{} {}", average_c, variance_c);
    Ok(())
}

fn read_file(filename: &str) -> Vec<f64> {
    println!("Reading {}", filename);
    let mut rdr = Reader::from_path(filename).expect("Could not read file");
    let mut energies: Vec<f64> = Vec::new();
    let mut count = 0;
    for line in rdr.records() {
        if count % 10 == 0 {
            print!("\rLine {} read", count);
        }
        count += 1;
        let record = line.unwrap()[0].parse().unwrap();
        energies.push(record);
    }
    println!("\rFile Read");

    energies
}

fn average(vector: &Vec<f64>) -> f64 {
    let mut sum = 0.;
    for e in vector.iter() {
        sum += e
    }
    let average = sum / (vector.len() as f64);
    average
}

fn variance(vector: &Vec<f64>) -> f64 {
    let mut vector_sq = Vec::with_capacity(vector.len());
    for e in vector.iter() {
        vector_sq.push(e*e)
    };

    let vector_average = average(&vector);
    let variance =  average(&vector_sq) - (vector_average * vector_average);
    variance
}

fn sample_and_compute_c(vector: &Vec<f64>, generator: &mut ThreadRng, temperature: &f64) -> f64 {
    let mut sum = 0.;
    let mut sq_sum = 0.;

    let mut n_resample = N_RESAMPLE;
    if n_resample > vector.len() {
        n_resample = vector.len();
    }

    for i in 0..n_resample {
        if i % 10 == 0 {
            print!("\r{} / {}", i, n_resample);
        }
        let index = generator.random_range(0..vector.len());
        sum += vector[index];
        sq_sum += vector[index].powf(2.);
    }
    println!("");

    let average = sum / n_resample as f64;
    let sq_average = sq_sum / n_resample as f64;
    let variance = sq_average - average.powf(2.);

    variance / (R * (temperature * temperature)) / (N as f64)
}
