use pyo3::prelude::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RUSTQUANT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[pymodule]
fn RustQuant(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_data_module(py, m)?;
    register_instruments_module(py, m)?;
    register_time_module(py, m)?;

    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant.data module
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn register_data_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let name = "data";
    let module = PyModule::new(parent_module.py(), name)?;

    // Add:
    module.add_class::<::RustQuant::data::Curve>()?;
    module.add_class::<::RustQuant::data::CurveType>()?;
    module.add_class::<::RustQuant::data::InterpolationMethod>()?;
    //

    parent_module.add_submodule(&module)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item(format!("RustQuant.{name}"), module)?;
    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant.instruments module
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn register_instruments_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let name = "instruments";
    let module = PyModule::new(parent_module.py(), name)?;

    // Add:
    // module.add_class::<::RustQuant::time::Calendar>()?;
    // module.add_class::<::RustQuant::time::Market>()?;
    //

    parent_module.add_submodule(&module)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item(format!("RustQuant.{name}"), module)?;
    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant.time module
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn register_time_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let name = "time";
    let module = PyModule::new(parent_module.py(), name)?;

    // Add:
    module.add_class::<::RustQuant::time::Calendar>()?;
    module.add_class::<::RustQuant::time::Market>()?;
    //

    parent_module.add_submodule(&module)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item(format!("RustQuant.{name}"), module)?;
    Ok(())
}
