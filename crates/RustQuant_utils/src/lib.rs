// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Epsilon constant for use in testing.
// It is set to: f64::sqrt(f64::EPSILON)
// Once `f64::sqrt()` is `const`, this can be updated.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Epsilon constant for use in testing.
pub const RUSTQUANT_EPSILON: f64 = 0.000_000_014_901_161_193_847_656;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Helper macros.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Helper macro for tests to test floating point approximate equality.
#[macro_export]
macro_rules! assert_approx_equal {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            ($x - $y <= $d) && ($y - $x <= $d),
            "\nLeft: \t\t{}, \nRight: \t\t{}, \nPrecision: \t{}\n",
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
            .build_cartesian_2d(
                0f64..vec2d.len() as f64,
                (min * 0.95)..(max * 1.05), // 5% padding on y-axis
            )
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
    use std::f64::EPSILON as EPS;

    #[test]
    fn test_assert_approx_equal() {
        assert_approx_equal!(1_f64, 1.0, EPS);
        assert_approx_equal!(1_f64.exp(), std::f64::consts::E, EPS);
        assert_approx_equal!(1_f64.ln(), 0.0, EPS);
        assert_approx_equal!(1_f64.sin(), 0.841_470_984_807_896_5, EPS);
        assert_approx_equal!(1_f64.cos(), 0.540_302_305_868_139_8, EPS);
        assert_approx_equal!(1_f64.tan(), 1.557_407_724_654_902_3, EPS);
        assert_approx_equal!(1_f64.asin(), std::f64::consts::FRAC_PI_2, EPS);
        assert_approx_equal!(1_f64.acos(), 0.0, EPS);
        assert_approx_equal!(1_f64.atan(), std::f64::consts::FRAC_PI_4, EPS);
        assert_approx_equal!(1_f64.sinh(), 1.175_201_193_643_801_4, EPS);
        assert_approx_equal!(1_f64.cosh(), 1.543_080_634_815_243_7, EPS);
        assert_approx_equal!(1_f64.tanh(), 0.761_594_155_955_764_9, EPS);
        assert_approx_equal!(1_f64.asinh(), 0.881_373_587_019_543, EPS);
        assert_approx_equal!(1_f64.acosh(), 0.0, EPS);
    }

    #[test]
    fn test_plot_vector_macro() {
        let v = [1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 6.0, 3.0, 7.0, 2.0, 8.0, 1.0];
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
