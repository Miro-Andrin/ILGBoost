use pyo3::prelude::*;

use crate::config::{Config, self};

#[pyclass]
#[derive(FromPyObject, Debug)]
pub struct Model {
    name: String,
    config: Config,
    coef: Vec<f64>,
}

#[pymethods]
impl Model {
    fn __str__(slf: PyRef<'_, Self>) -> Option<String> {
        Some(slf.name.to_owned())
    }
    
    fn fit(mut slf: PyRefMut<'_, Self>, rows: Vec<f64>, response: Vec<f64>) -> PyResult<()> {
        if rows.len() == 0 {
            return Ok(());
        }
    
        if response.len() == 0 {
            return Ok(());
        }

        let config = &slf.config;
        println!("Fitting!");
        println!("{:?}", config);

        Ok(())
    }

    fn predict(mut slf: PyRefMut<'_, Self>, rows: Vec<f64>) -> PyResult<Vec<f64>> {
        let config = &slf.config;
        println!("predicting!");
        Ok(Vec::new())
    }
}

impl Model {
    pub fn new(config: Config) -> Model{
        Model{name: "Default".to_owned(), config: config, coef: Vec::new()}
    }
}