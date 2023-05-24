// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// Write vector to file.
pub fn write_vector(v: &[f64]) -> Result<(), Box<dyn Error>> {
    let strings: Vec<String> = v.iter().map(|n| n.to_string()).collect();

    let mut file = File::create("vector.out")?;
    writeln!(file, "{}", strings.join(", "))?;
    Ok(())
}

/// Prepare vector for plotting in `plot_vector()`.
fn prepare_vec(vals: Vec<f64>) -> (Vec<(f64, f64)>, f64, f64) {
    let mut out = vec![(0.0, 0.0); vals.len()];
    let mut min = vals[0];
    let mut max = vals[0];

    for i in 0..vals.len() {
        out[i] = (i as f64, vals[i]);
        if vals[i] > max {
            max = vals[i]
        } else if vals[i] < min {
            min = vals[i]
        }
    }
    (out, min, max)
}

/// Plot a vector of values.
pub fn plot_vector(v: Vec<f64>, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (vec2d, min, max) = prepare_vec(v);

    let root = BitMapBackend::new(file, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(file, ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..vec2d.len() as f64, min..max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(vec2d, RED))?.label(file);

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}

#[cfg(test)]
mod tests_plotters {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_write_vector() -> Result<(), Box<dyn std::error::Error>> {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        write_vector(&v)?;

        // Check the file was created and has content.
        let contents = fs::read_to_string("vector.out")?;
        assert!(!contents.is_empty());

        // Clean up after the test.
        fs::remove_file("vector.out")?;

        Ok(())
    }

    #[test]
    fn test_prepare_vec() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (out, min, max) = prepare_vec(v);

        assert_eq!(out.len(), 5);
        assert_eq!(min, 1.0);
        assert_eq!(max, 5.0);
    }

    #[test]
    fn test_plot_vector() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;

        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let file_path = dir.path().join("plot.png");
        let file_str = file_path.to_str().unwrap();
        plot_vector(v, file_str)?;

        // Check the file was created.
        assert!(file_path.exists());

        dir.close()?;

        Ok(())
    }
}
