use ndarray::Array;

pub fn normalize(tensor: &Array<f32, ndarray::IxDyn>) -> Array<f32, ndarray::IxDyn> {
    let dim = tensor.ndim() - 1;
    let norm = tensor.map_axis(ndarray::Axis(dim), |row| {
        row.mapv(|x| x.powi(2)).sum().sqrt()
    });
    let eps = 1e-12;
    let normalized_tensor = tensor / norm.mapv(|v| if v > eps { v } else { 1.0 });
    normalized_tensor
}
