// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::{Month, OffsetDateTime};

/// France calendar.
pub struct France;

impl crate::time::Calendar for France {
    fn name(&self) -> &'static str {
        "France"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::FRANCE
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XPAR
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (_, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            // Jour de l'An
            || (d == 1 && m == Month::January)
            // Lundi de Paques
            || (dd == em)
            // Fete du Travail
            || (d == 1 && m == Month::May)
            // Victoire 1945
            || (d == 8 && m == Month::May)
            // Ascension
            || (d == 10 && m == Month::May)
            // Pentecote
            || (d == 21 && m == Month::May)
            // Fete nationale
            || (d == 14 && m == Month::July)
            // Assomption
            || (d == 15 && m == Month::August)
            // Toussaint
            || (d == 1 && m == Month::November)
            // Armistice 1918
            || (d == 11 && m == Month::November)
            // Noel
            || (d == 25 && m == Month::December)
        {
            return false;
        }

        true
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {}
