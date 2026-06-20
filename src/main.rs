use rand::prelude::*;
use csv::Reader;

use rayon::prelude::*;
use clap::Parser;

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

    #[arg(short, long, default_value_t = 1)]
    molecules: usize,

    #[arg(short, long, default_value_t = 10_000)]
    num_samples: usize,

    #[arg(short, long, default_value_t = 0)]
    sample_size: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    
    let energies = read_file(&args.file);
    
    let mut sample_size = args.sample_size;
    if sample_size < 1 {
        sample_size = energies.len();
    } else if sample_size > energies.len() {
        sample_size = energies.len();
    }
    
    

    let cs: Vec<f64> = (0..args.num_samples)
        .into_par_iter()
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
    println!("Reading {}", filename);
    let mut rdr = Reader::from_path(filename).expect("Could not read file");
    let mut energies: Vec<f64> = Vec::new();
    let mut count = 0;
    for line in rdr.records() {
        if count % 10_000 == 0 {
            print!("\rLine {} read", count);
        }
        count += 1;
        let record = line.unwrap()[0].parse().unwrap();
        energies.push(record);
    }
    println!("\rFile Read                ");

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
