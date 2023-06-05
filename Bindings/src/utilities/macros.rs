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
            "\nLeft: \t{}, \nRight: \t{}, \nDelta: \t{}\n",
            $x,
            $y,
            $d
        )
    };
}
