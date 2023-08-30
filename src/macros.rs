// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Helper macro for tests to test floating point approximate equality.
#[macro_export]
macro_rules! assert_approx_equal {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            ($x - $y <= $d) && ($y - $x <= $d),
            "\nActual: \t{}, \nExpected: \t{}, \nPrecision: \t{}\n",
            $x,
            $y,
            $d
        )
    };
}

/// Plot a vector of values.
#[macro_export]
macro_rules! plot_vector {
    ($v:expr, $file:expr) => {{
        // Macros are hygienic, so we need to import the libraries we want to use.
        use plotters::prelude::*;

        let mut min = $v[0];
        let mut max = $v[0];
        let mut vec2d = Vec::new();

        for (i, &val) in $v.iter().enumerate() {
            vec2d.push((i as f64, val));
            if val > max {
                max = val;
            } else if val < min {
                min = val;
            }
        }

        let root = BitMapBackend::new($file, (640, 480)).into_drawing_area();
        root.fill(&full_palette::WHITE).unwrap();

        let mut chart = plotters::prelude::ChartBuilder::on(&root)
            .caption($file, ("sans-serif", 30).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f64..vec2d.len() as f64, min..max)
            .unwrap();

        chart.configure_mesh().draw().unwrap();
        chart
            .draw_series(LineSeries::new(vec2d, RED))
            .unwrap()
            .label($file);
        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()
            .unwrap();
    }};
}

#[cfg(test)]
mod tests_plotters {

    #[test]
    fn test_assert_approx_equal() {
        assert_approx_equal!(1_f64, 1.0, 1e-10);
        assert_approx_equal!(1_f64.exp(), 2.718281828459045, 1e-10);
        assert_approx_equal!(1_f64.ln(), 0.0, 1e-10);
        assert_approx_equal!(1_f64.sin(), 0.8414709848078965, 1e-10);
        assert_approx_equal!(1_f64.cos(), 0.5403023058681398, 1e-10);
        assert_approx_equal!(1_f64.tan(), 1.5574077246549023, 1e-10);
        assert_approx_equal!(1_f64.asin(), 1.5707963267948966, 1e-10);
        assert_approx_equal!(1_f64.acos(), 0.0, 1e-10);
        assert_approx_equal!(1_f64.atan(), 0.7853981633974483, 1e-10);
        assert_approx_equal!(1_f64.sinh(), 1.1752011936438014, 1e-10);
        assert_approx_equal!(1_f64.cosh(), 1.5430806348152437, 1e-10);
        assert_approx_equal!(1_f64.tanh(), 0.7615941559557649, 1e-10);
        assert_approx_equal!(1_f64.asinh(), 0.881373587019543, 1e-10);
        assert_approx_equal!(1_f64.acosh(), 0.0, 1e-10);
    }

    #[test]
    fn test_plot_vector_macro() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 6.0, 3.0, 7.0, 2.0, 8.0, 1.0];
        let file = "plot_macro.png";

        // THIS WORKS.
        plot_vector!(v, file);

        // Check if the file exists
        if std::fs::metadata(file).is_ok() {
            println!("File exists. Attempting to remove...");

            // Remove the file
            if let Err(e) = std::fs::remove_file(file) {
                println!("Failed to remove file: {}", e);
            } else {
                println!("Successfully removed file.");
            }
        } else {
            println!("File does not exist.");
        }
    }
}
