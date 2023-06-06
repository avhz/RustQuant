// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! RustQuant: A Rust library for quantitative finance.
//!
//! Contact: rustquantcontact@gmail.com
//!
//! This library is a work in progress.
//! Any contributions are greatly appreciated.

//! This project contains the Python bindings for RustQuant.

#![deny(missing_docs)]
#![allow(non_snake_case)]

use pyo3::prelude::*;

#[pymodule]
fn RustQuant(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // m.add_class::<>()?;
    // m.add_wrapped(wrap_pyfunction!( ).unwrap();

    Ok(())
}
