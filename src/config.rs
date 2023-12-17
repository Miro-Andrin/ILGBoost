use pyo3::prelude::*;

#[derive(FromPyObject, Debug)]
pub struct Config {
    /// Number of iterations to run.
    pub steps: usize,

    /// Learning rate (0 to 1)
    pub gamma: f32,

    // If the data is supplied in row order or column order
    pub data_order: DataOrder,

    // Fitting intervept or not
    pub fit_intercept: bool,

    // Verbose printing during fitting
    pub verbose: bool,
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum DataOrder {
    RowMajor,
    ColumnMajor
}