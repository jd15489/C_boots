pub fn average(data: &[f64]) -> f64 {
    let mut sum = 0.;
    for e in data.iter() {
        sum += e
    }
    sum / (data.len() as f64)
}

pub fn variance(data: &[f64]) -> f64 {
    let mut data_sq = Vec::with_capacity(data.len());
    for e in data.iter() {
        data_sq.push(e * e)
    }

    let data_average = average(data);
    average(&data_sq) - (data_average * data_average)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert_eq!(2.5, average(&Vec::from([1., 2., 3., 4.])));
    }

    #[test]
    fn test_variance() {
        assert_eq!(1.25, variance(&Vec::from([1., 2., 3., 4.])));
    }
}
