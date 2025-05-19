// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Trait to define financial quotes.
pub trait Quote {
    /// Quote value.
    fn value(&self) -> Option<f64>;
    /// Check if the quote is valid.
    fn is_valid(&self) -> bool;
}

/// Simple quote type.
pub struct SimpleQuote {
    value: Option<f64>,
}

impl SimpleQuote {
    /// Create a new simple quote.
    #[must_use]
    pub fn new(value: Option<f64>) -> Self {
        SimpleQuote { value }
    }

    /// Sets the value of the quote and returns the difference between the new value and the old value.
    ///
    /// # Arguments
    ///
    /// * `value` - An optional new value to set.
    ///
    /// # Returns
    ///
    /// * `f64` - The difference between the new value and the old value. If the new value is not present,
    ///  the difference will be 0.0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use RustQuant::cashflows::SimpleQuote;
    ///
    /// let mut quote = SimpleQuote::new(Some(10.0));
    /// let diff = quote.set_value(Some(15.0));
    /// assert_eq!(diff, 5.0);
    /// ```
    pub fn set_value(&mut self, value: Option<f64>) -> f64 {
        let diff = match (&self.value, &value) {
            (Some(old_value), Some(new_value)) => new_value - old_value,
            (None, Some(new_value)) => *new_value,
            _ => 0.0,
        };

        if diff != 0.0 {
            self.value = value;
            // Assuming notify_observers() function exists and does what it's supposed to
            // self.notify_observers();
        }

        diff
    }

    /// Resets the value of the quote to `None`.
    ///
    /// This method clears the current value of the quote, effectively making it invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use RustQuant::cashflows::{Quote, SimpleQuote};
    ///
    /// let mut quote = SimpleQuote::new(Some(10.0));
    /// assert!(quote.is_valid());
    ///
    /// quote.reset();
    /// assert!(!quote.is_valid());
    /// assert_eq!(quote.value(), None);
    /// ```
    pub fn reset(&mut self) {
        self.value = None;
    }
}

impl Quote for SimpleQuote {
    fn value(&self) -> Option<f64> {
        self.value
    }

    fn is_valid(&self) -> bool {
        self.value.is_some()
    }
}

/// Derived quote type.
pub struct DerivedQuote<F>
where
    F: Fn(f64) -> f64, // Box<dyn Fn() -> Option<f64>>,
{
    _value: Option<f64>,
    _function: F,
}
