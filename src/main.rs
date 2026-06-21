use rand::prelude::*;
use csv::ReaderBuilder;

use clap::Parser;
use indicatif::{
    ParallelProgressIterator, ProgressBar
};
use rayon::{ThreadPoolBuilder, prelude::*};

const R: f64 = 0.00198720425864083;
// const N_SAMPLES: usize = 10000;
// const N: usize = 900;
// const N_RESAMPLE: usize = 10000;

#[derive(Parser, Debug)]
#[command(
    author="Josh Dunn",
    version="0.1",
    about="Computes heat capacity using a bootstrapping method."
)]
struct Args {
    file: String,
    temperature: f64,

    #[arg(short, long, default_value_t = 4)]
    num_threads: usize,

    #[arg(short, long, default_value_t = 1)]
    molecules: usize,

    #[arg(short, long, default_value_t = 10_000)]
    samples: usize,

    #[arg(short, long, default_value_t = 0)]
    bootstrap_sample_size: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Running C_boots");
    println!("Running on {} thread(s)", &args.num_threads);
    println!("Using file: {}", &args.file);
    
    ThreadPoolBuilder::new()
        .num_threads(args.num_threads)
        .build_global()
        .unwrap();

    let energies = read_file(&args.file);
    
    let mut sample_size = args.bootstrap_sample_size;
    if sample_size < 1 {
        sample_size = energies.len();
    } else if sample_size > energies.len() {
        sample_size = energies.len();
    }

    let cs: Vec<f64> = (0..args.samples)
        .into_par_iter()
        .progress_count(args.samples as u64)
        .map(|_|{
            let mut rng = rand::rng();
            sample_and_compute_c(
                &energies,
                &mut rng,
                &args.temperature,
                &sample_size,
                &(args.molecules as f64))
        })
        .collect();

    let average_c = average(&cs);
    let variance_c = variance(&cs);
    
    println!("Average C | Standard Deviation in C");
    println!(    "{:.7} | {:.7}", average_c, variance_c.sqrt());
    Ok(())
}

fn read_file(filename: &str) -> Vec<f64> {
    let bar = ProgressBar::new_spinner();
    bar.set_message("Reading file...");
    bar.enable_steady_tick(std::time::Duration::from_micros(500));

    let mut rdr = ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_path(filename)
        .expect("Could not read file");
    let mut energies: Vec<f64> = Vec::new();
    for line in rdr.records() {
        let record = line.unwrap()[0].parse().unwrap();
        energies.push(record);
    }

    bar.finish_and_clear();
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

fn sample_and_compute_c(
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
