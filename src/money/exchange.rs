// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use crate::RustQuantError;
use super::{Currency, Money};
use std::collections::HashMap;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Exchange struct to hold exchange rates.
#[derive(Debug, Clone)]
pub struct Exchange {
    /// Exchange rates hashmap.
    /// The key is a string of the form e.g. "USD_EUR",
    /// and the value is an ExchangeRate struct.
    /// The key is generated from the from_currency and to_currency of the ExchangeRate.
    pub rates: HashMap<String, ExchangeRate>,
}

/// ExchangeRate struct to hold exchange rate information.
#[derive(Debug, Clone, Copy)]
pub struct ExchangeRate {
    /// From currency
    pub from_currency: Currency,
    /// To currency
    pub to_currency: Currency,
    /// The actual exchange rate as a ratio from_currency/to_currency
    pub rate: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Exchange {
    /// Create a new empty Exchange.
    ///
    /// # Example
    /// ```
    /// use RustQuant::money::*;
    ///
    /// let exchange = Exchange::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            rates: HashMap::new(),
        }
    }

    /// Adds a new ExchangeRate to the Exchange.
    ///
    /// # Example
    /// ```
    /// use RustQuant::money::*;
    ///
    /// let mut exchange = Exchange::new();
    ///
    /// let usd_to_eur = ExchangeRate::new(USD, EUR, 0.85); // USD to EUR
    /// let eur_to_usd = ExchangeRate::new(EUR, USD, 1.18); // EUR to USD
    ///
    /// exchange.add_rate(usd_to_eur);
    /// exchange.add_rate(eur_to_usd);
    /// ```
    ///
    pub fn add_rate(&mut self, rate: ExchangeRate) {
        let key = format!(
            "{}/{}",
            rate.from_currency.code.alphabetic, rate.to_currency.code.alphabetic
        );
        self.rates.insert(key, rate);
    }

    /// Retrieves an ExchangeRate from the Exchange.
    ///
    /// # Example
    /// ```
    /// use RustQuant::money::*;
    ///
    /// let mut exchange = Exchange::new();
    ///
    /// let usd_to_eur = ExchangeRate::new(USD, EUR, 0.85); // USD to EUR
    /// let eur_to_usd = ExchangeRate::new(EUR, USD, 1.18); // EUR to USD
    ///
    /// exchange.add_rate(usd_to_eur);
    /// exchange.add_rate(eur_to_usd);
    ///
    /// let retrieved_usd_to_eur = exchange.get_rate(&USD, &EUR).expect("Rate not found");
    /// assert_eq!(retrieved_usd_to_eur.rate, 0.85);
    ///     
    /// let retrieved_eur_to_usd = exchange.get_rate(&EUR, &USD).expect("Rate not found");
    /// assert_eq!(retrieved_eur_to_usd.rate, 1.18);
    /// ```
    ///
    pub fn get_rate(
        &self,
        from_currency: &Currency,
        to_currency: &Currency,
    ) -> Option<&ExchangeRate> {
        let key = format!(
            "{}/{}",
            from_currency.code.alphabetic, to_currency.code.alphabetic
        );
        self.rates.get(&key)
    }

    /// Convert money from one currency to another using the exchange rate in the Exchange.
    /// It panics if the conversion rate is not found or if the money's currency doesn't match with from_currency.
    ///
    /// # Example
    /// ```
    /// use RustQuant::money::*;
    ///     
    /// let mut exchange = Exchange::new();
    ///
    /// let usd_to_eur = ExchangeRate::new(USD, EUR, 0.85); // USD to EUR
    /// let eur_to_usd = ExchangeRate::new(EUR, USD, 1.18); // EUR to USD
    ///     
    /// exchange.add_rate(usd_to_eur);
    /// exchange.add_rate(eur_to_usd);
    ///     
    /// let usd_100 = Money::new(USD, 100.0); // 100 USD
    /// let eur_85 = exchange.convert(usd_100, EUR); // Should be 85 EUR
    ///
    /// assert_eq!(eur_85.currency, EUR);
    /// assert_eq!(eur_85.amount, 85.0);
    /// ```
    pub fn convert(&self, money: Money, to_currency: Currency) -> Money {
        let rate = self
            .get_rate(&money.currency, &to_currency)
            .unwrap_or_else(|| {
                panic!(
                    "Exchange rate for converting {} to {} not found.",
                    money.currency.code.alphabetic, to_currency.code.alphabetic
                )
            });
        rate.convert(money)
    }
}

impl ExchangeRate {
    /// Create a new exchange rate.
    pub fn new(from_currency: Currency, to_currency: Currency, rate: f64) -> Self {
        Self {
            from_currency,
            to_currency,
            rate,
        }
    }

    /// Convert money from one currency to another using this exchange rate.
    /// It panics if the money's currency doesn't match with from_currency.
    ///
    /// # Example
    /// ```
    /// use RustQuant::money::*;
    /// use RustQuant::assert_approx_equal;
    ///
    /// // Use USD and EUR currency constants from the money module.
    /// let usd = Money::new(USD, 100.0);
    /// let eur_usd = ExchangeRate::new(USD, EUR, 0.9186955);  // 1 USD = 0.9186955 EUR
    /// let eur = eur_usd.convert(usd);
    ///
    /// assert_approx_equal!(eur.amount, 91.86955, 1e-5);
    /// assert_eq!(eur.currency, EUR);
    /// ```
    ///
    /// It panics if the money's currency doesn't match with from_currency.
    ///
    /// ```should_panic
    /// use RustQuant::money::*;
    ///
    /// let usd = Money::new(EUR, 100.0);                       // Notice the wrong currency
    /// let eur_usd = ExchangeRate::new(USD, EUR, 0.9186955);   // 1 USD = 0.9186955 EUR
    ///
    /// eur_usd.convert(usd);  // This will panic
    /// ```
    pub fn convert(&self, money: Money) -> Money {
        if money.currency == self.from_currency {
            let new_amount = money.amount * self.rate;
            Money::new(self.to_currency, new_amount)
        } else {
            panic!(
                "The currency of the money doesn't match with from_currency of the exchange rate."
            )
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// You can now add additional unit tests
#[cfg(test)]
mod test_exchange_rate {
    use crate::money::*;

    #[test]
    fn test_conversion() {
        // Initialize USD and EUR currencies and their ISO codes (simplified for example)
        let usd = Currency::new(
            "United States Dollar",
            "$",
            ISO_4217::new("USD", "840"),
            2,
            100,
        );

        let eur = Currency::new("Euro", "€", ISO_4217::new("EUR", "978"), 2, 100);

        // Create Money instance
        let usd_100 = Money::new(usd, 100.0);

        // Create ExchangeRate instance
        let usd_to_eur = ExchangeRate::new(usd, eur, 0.85); // 1 USD = 0.85 EUR as an example

        // Convert USD to EUR
        let eur_85 = usd_to_eur.convert(usd_100);

        // Verify the conversion
        assert_eq!(eur_85.currency, eur);
        assert_eq!(eur_85.amount, 85_f64);
    }

    #[test]
    fn test_add_and_get_rate() {
        let mut exchange = Exchange::new();

        let usd_to_eur = ExchangeRate::new(USD, EUR, 0.85); // USD to EUR
        let eur_to_usd = ExchangeRate::new(EUR, USD, 1.18); // EUR to USD

        exchange.add_rate(usd_to_eur);
        exchange.add_rate(eur_to_usd);

        let retrieved_usd_to_eur = exchange.get_rate(&USD, &EUR).expect("Rate not found");
        assert_eq!(retrieved_usd_to_eur.rate, 0.85);

        let retrieved_eur_to_usd = exchange.get_rate(&EUR, &USD).expect("Rate not found");
        assert_eq!(retrieved_eur_to_usd.rate, 1.18);
    }

    #[test]
    fn test_conversion_with_exchange() {
        let mut exchange = Exchange::new();

        let usd_to_eur = ExchangeRate::new(USD, EUR, 0.85); // USD to EUR
        let eur_to_usd = ExchangeRate::new(EUR, USD, 1.18); // EUR to USD

        exchange.add_rate(usd_to_eur);
        exchange.add_rate(eur_to_usd);

        let usd_100 = Money::new(USD, 100.0); // 100 USD
        let eur_85 = exchange.convert(usd_100, EUR); // Should be 85 EUR

        assert_eq!(eur_85.currency, EUR);
        assert_eq!(eur_85.amount, 85.0);
    }
}
