use pyo3::prelude::*;
use rand_distr::Distribution;
use statrs::distribution::Normal;
use rand::SeedableRng;


#[pyfunction]
fn mc_price() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let normal = Normal::new(1.0, 1.0).unwrap();

    let num_paths: usize = 200000;
    let num_steps: usize = 700;

    // Initialize the vector
    let mut _v: Vec<f64> = Vec::with_capacity(num_paths);

    // This takes about 0.9 to 1.1 secs. which matches numpy with same parameters, and 'out' option.
    for _step in 1..num_steps {
        _v = normal.sample_iter(&mut rng).take(num_paths).collect();
    }
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in `Cargo.toml`
#[pymodule]
fn rustmc(_py: Python, m: &PyModule) -> PyResult<()> {
    // pyo3_log::init();
    m.add_function(wrap_pyfunction!(mc_price, m)?)?;
    // m.add_class::<RustStruct>()?;

    Ok(())
}