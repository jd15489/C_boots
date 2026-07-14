

pub fn average(vector: &Vec<f64>) -> f64 {
    let mut sum = 0.;
    for e in vector.iter() {
        sum += e
    }
    let average = sum / (vector.len() as f64);
    average
}

pub fn variance(vector: &Vec<f64>) -> f64 {
    let mut vector_sq = Vec::with_capacity(vector.len());
    for e in vector.iter() {
        vector_sq.push(e*e)
    };

    let vector_average = average(&vector);
    let variance =  average(&vector_sq) - (vector_average * vector_average);
    variance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert_eq!(
            2.5,
            average(&Vec::from([1.,2.,3.,4.]))
        );
    }

    #[test]
    fn test_variance() {
        assert_eq!(
            1.25,
            variance(&Vec::from([1.,2.,3.,4.]))
        );
    }
}