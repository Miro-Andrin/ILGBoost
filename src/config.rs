use pyo3::prelude::*;

#[derive(FromPyObject, Debug)]
pub struct Config {
    /// Number of iterations to run.
    pub steps: usize,

    /// Learning rate (0 to 1)
    pub gamma: f32,


    pub data_order: DataOrder,
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum DataOrder {
    RowMajor,
    ColumnMajor
}