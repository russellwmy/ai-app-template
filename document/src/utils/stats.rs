use std::collections::HashMap;

pub fn mean(values: &Vec<f32>) -> f32 {
    let s = values.iter().sum::<f32>();

    s / values.len() as f32
}

pub fn stdev(values: &Vec<f32>) -> f32 {
    let u = mean(values);
    let n = values.len();

    values
        .iter()
        .map(|x| (x - u).powi(2) as f32 / n as f32)
        .sum::<f32>()
}

pub fn mode(values: &Vec<f32>) -> f32 {
    let mut counter = HashMap::new();

    for value in values {
        let key = value.to_string();
        counter
            .entry(key)
            .and_modify(|count| *count += 1)
            .or_insert(0);
    }
    let mut counter_vec: Vec<_> = counter.iter().collect();
    counter_vec.sort_by(|a, b| a.1.cmp(b.1));

    match counter_vec.last() {
        Some(item) => item.0.parse::<f32>().unwrap(),
        None => 0.0,
    }
}
