use ndarray::Array1;

pub fn cosine_similarity(a: Vec<f32>, b: Vec<f32>) -> f32 {
    let a = Array1::from(a);
    let b = Array1::from(b);
    let dot_product = a.dot(&b);
    let norm_a = a.dot(&a).sqrt();
    let norm_b = b.dot(&b).sqrt();
    dot_product / (norm_a * norm_b)
}
