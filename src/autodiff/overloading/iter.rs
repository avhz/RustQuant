// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::variables::variable::Variable;
use std::iter::{Product, Sum};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: ITERATORS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> Sum<Variable<'v>> for Variable<'v> {
    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let params = (0..100).map(|x| g.var(x as f64)).collect::<Vec<_>>();
    ///
    /// let sum = params.iter().copied().sum::<Variable>();
    ///
    /// let derivs = sum.accumulate();
    ///
    /// for i in derivs.wrt(&params) {
    ///     assert_eq!(i, 1.0);
    /// }
    /// ```
    #[inline]
    fn sum<I: Iterator<Item = Variable<'v>>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y)
            .expect("Cannot call sum() since vector is empty. Exiting ...")
    }
}

impl<'v> Product<Variable<'v>> for Variable<'v> {
    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let params = (1..=5).map(|x| g.var(x as f64)).collect::<Vec<_>>();
    ///
    /// let prod = params.iter().copied().product::<Variable>();
    ///
    /// let derivs = prod.accumulate();
    /// let true_gradient = vec![120.0, 60.0, 40.0, 30.0, 24.0];
    ///
    /// let n = derivs.wrt(&params).len();
    /// let m = true_gradient.len();
    /// assert_eq!(n, m);
    ///
    /// for i in 0..n {
    ///     assert_eq!(derivs.wrt(&params)[i], true_gradient[i]);
    /// }
    /// ```
    #[inline]
    fn product<I: Iterator<Item = Variable<'v>>>(iter: I) -> Self {
        iter.reduce(|x, y| x * y)
            .expect("Cannot call product() since vector is empty. Exiting ...")
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overload {
    use crate::autodiff::{Accumulate, Gradient, Graph, Variable};

    #[test]
    fn test_sum() {
        let g = Graph::new();

        let params = (0..100).map(|x| g.var(x as f64)).collect::<Vec<_>>();
        let sum = params.iter().copied().sum::<Variable>();
        let derivs = sum.accumulate();

        for i in derivs.wrt(&params) {
            assert_eq!(i, 1.0);
        }
    }

    #[test]
    fn test_product() {
        let g = Graph::new();

        let params = (1..=5).map(|x| g.var(x as f64)).collect::<Vec<_>>();
        let prod = params.iter().copied().product::<Variable>();

        let derivs = prod.accumulate();
        let true_gradient = [120.0, 60.0, 40.0, 30.0, 24.0];

        let expects = derivs.wrt(&params);
        let n = expects.len();
        let m = true_gradient.len();
        assert_eq!(n, m);

        for (&expect, &gradient) in expects.iter().zip(true_gradient.iter()) {
            assert_eq!(expect, gradient);
        }
    }
}
