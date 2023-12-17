#![allow(non_snake_case)]

use std::ops::Sub;

use nalgebra::{OMatrix, Dyn, DVector};
use pyo3::prelude::*;

mod config;
use config::Config;
use config::DataOrder;


#[pyfunction]
fn calculate(config: Config, rows: Vec<f64>, response: Vec<f64>) -> PyResult<Option<Vec<f64>>> {
    if rows.len() == 0 {
        return Ok(Some(Vec::new()));
    }

    if response.len() == 0 {
        return Ok(Some(Vec::new()));
    }

    // There should be row_count*column_count number of ellements in rows.
    // row_count has to be the same as response.len().  
    let row_count = response.len();
    let col_count = rows.len() / row_count;

    if col_count * row_count != rows.len(){
        panic!("Miss formated data")
    }

    
    println!("{:?}", config);

    type DynamicMatrix = OMatrix<f64, Dyn, Dyn>;    // stores data
    
    let data = match config.data_order {
        DataOrder::RowMajor =>  DynamicMatrix::from_row_slice(row_count, col_count, &rows),
        DataOrder::ColumnMajor =>  DynamicMatrix::from_column_slice(row_count, col_count, &rows)
    };

                
        
    DynamicMatrix::from_row_slice(row_count, col_count, &rows);
    let mut y = DVector::from_row_slice(&response);

    let mut y_mean = 0.0;

    if config.fit_intercept {
        let len = y.len() as f64;
     y_mean = y.sum()/len;   
    }

    y = y.add_scalar(-y_mean);
    
    println!("{data}");
    println!("{y}");
    
    // Simple linear regression:
    // beta = (X.T @ X).inv() @ X.T @ y

    let beta = (data.transpose() * &data).try_inverse().map(|d| { d * &data.transpose()* y });

    if beta.is_none() {
        return Ok(None);
    }

    let beta = beta.unwrap();


    let result  = beta.iter().cloned().collect::<Vec<_>>();
    Ok(Some(result))
}


// fn norm(col1: ColumnView, col2: ColumnView){
//     // returns sum(col1[i] * col2[i])
// }


/// A Python module implemented in Rust.
#[pymodule]
fn ILGBoost(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DataOrder>();
    m.add_function(wrap_pyfunction!(calculate, m)?)?;
    Ok(())
}
