// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines calendars and holidays for different countries.

/// Calendars implemented for African countries.
pub mod africa {
    /// This module defines Botswana holidays and calendars.
    pub mod botswana;
}

/// Calendars implemented for Asian countries.
pub mod asia {
    /// This module defines China holidays and calendars.
    pub mod china;
    /// This module defines Hong Kong holidays and calendars.
    pub mod hong_kong;
    /// This module defines India holidays and calendars.
    pub mod india;
    /// This module defines Indonesia holidays and calendars.
    pub mod indonesia;
    /// This module defines Singapore holidays and calendars.
    pub mod singapore;
}

/// Calendars implemented for European countries.
pub mod europe {
    /// This module defines Austria holidays and calendars.
    pub mod austria;
    /// This module defines Czech Republic holidays and calendars.
    pub mod czech_republic;
    /// This module defines Denmark holidays and calendars.
    pub mod denmark;
    /// This module defines Finland holidays and calendars.
    pub mod finland;
    /// This module defines France holidays and calendars.
    pub mod france;
    /// This module defines Germany holidays and calendars.
    pub mod germany;
    /// This module defines Hungary holidays and calendars.
    pub mod hungary;
    /// This module defines Iceland holidays and calendars.
    pub mod iceland;
    /// This module defines United Kingdom holidays and calendars.
    pub mod united_kingdom;
}

/// Calendars implemented for North American countries.
pub mod north_america {
    /// This module defines Canada holidays and calendars.
    pub mod canada;
    /// This module defines United States holidays and calendars.
    pub mod united_states;
}

/// Calendars implemented for Oceanian countries.
pub mod oceania {
    /// This module defines Australia holidays and calendars.
    pub mod australia;
    /// This module defines New Zealand holidays and calendars.
    pub mod new_zealand;
}

/// Calendars implemented for South American countries.
pub mod south_america {
    /// This module defines Argentina holidays and calendars.
    pub mod argentina;
    /// This module defines Brazil holidays and calendars.
    pub mod brazil;
    /// This module defines Chile holidays and calendars.
    pub mod chile;
}
