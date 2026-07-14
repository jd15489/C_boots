use csv::ReaderBuilder;
use indicatif::ProgressBar;

pub fn read_file(filename: &str) -> std::io::Result<Vec<f64>> {
    let bar = ProgressBar::new_spinner();
    bar.set_message("Reading file...");
    bar.enable_steady_tick(std::time::Duration::from_micros(500));

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_path(filename)?;
    let mut energies: Vec<f64> = Vec::new();
    for line in rdr.records() {
        let record = line.unwrap()[0].parse().unwrap();
        energies.push(record);
    }

    bar.finish_and_clear();
    Ok(energies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let result = read_file("tests/energies.csv");
        assert_eq!(result.get(0..10), Some(&[1.,2.,3.,4.,5.,6.,7.,8.,9.,10.][..]));
    }
}