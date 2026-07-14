use crate::io::read_file;
use crate::stats::{
    average, variance
};
use crate::bootstrap::sample_and_compute_c;

use clap::Parser;
use indicatif::ParallelProgressIterator;
use rayon::{ThreadPoolBuilder, prelude::*};

mod io; 
mod stats;
mod bootstrap;

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
    // Parse command line arugments
    let args = Args::parse();

    // Report the start of the program and some input parameters
    println!("Running C_boots");
    println!("Running on {} thread(s)", &args.num_threads);
    println!("Using file: {}", &args.file);
    
    // Build a ThreadPool based on user input or default values
    ThreadPoolBuilder::new()
        .num_threads(args.num_threads)
        .build_global()
        .unwrap();

    // Read the file given by the user
    let energies = read_file(&args.file)?;
    
    // Process sample size (the number of samples to take from the input data for a single sample)
    let mut sample_size = args.bootstrap_sample_size;
    if sample_size < 1 {
        sample_size = energies.len();
    } else if sample_size > energies.len() {
        sample_size = energies.len();
    }

    // Sample the heat capacity multiple times:
    // Create a range,
    // turn that range into a thread pool iterator,
    // create a progress bar iterator,
    // map a closure onto that iterator,
    // collect the result of calling that mapping.
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

    // Calculate the average and variance in heat capacity estimates
    let average_c = average(&cs);
    let variance_c = variance(&cs);
    
    // Report the estimated heat capacity and the error in that estimate
    println!("Average C | Standard Deviation in C");
    println!(    "{:.7} | {:.7}", average_c, variance_c.sqrt());
    Ok(())
}
