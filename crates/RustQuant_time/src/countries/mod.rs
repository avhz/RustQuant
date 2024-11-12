// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! calendars and holidays for different countries.

/// Calendars implemented for African countries.
pub mod africa {
    /// Botswana holidays and calendars.
    pub mod botswana;
}

/// Calendars implemented for Asian countries.
pub mod asia {
    /// China holidays and calendars.
    pub mod china;
    /// Hong Kong holidays and calendars.
    pub mod hong_kong;
    /// India holidays and calendars.
    pub mod india;
    /// Indonesia holidays and calendars.
    pub mod indonesia;
    /// Singapore holidays and calendars.
    pub mod singapore;
}

/// Calendars implemented for European countries.
pub mod europe {
    /// Austria holidays and calendars.
    pub mod austria;
    /// Czech Republic holidays and calendars.
    pub mod czech_republic;
    /// Denmark holidays and calendars.
    pub mod denmark;
    /// Finland holidays and calendars.
    pub mod finland;
    /// France holidays and calendars.
    pub mod france;
    /// Germany holidays and calendars.
    pub mod germany;
    /// Hungary holidays and calendars.
    pub mod hungary;
    /// Iceland holidays and calendars.
    pub mod iceland;
    /// Netherlands holidays and calendars.
    pub mod netherlands;
    /// United Kingdom holidays and calendars.
    pub mod united_kingdom;
}

/// Calendars implemented for North American countries.
pub mod north_america {
    /// Canada holidays and calendars.
    pub mod canada;
    /// United States holidays and calendars.
    pub mod united_states;
	/// Mexico holidays and calendars
	pub mod mexico;
}

/// Calendars implemented for Oceanian countries.
pub mod oceania {
    /// Australia holidays and calendars.
    pub mod australia;
    /// New Zealand holidays and calendars.
    pub mod new_zealand;
}

/// Calendars implemented for South American countries.
pub mod south_america {
    /// Argentina holidays and calendars.
    pub mod argentina;
    /// Brazil holidays and calendars.
    pub mod brazil;
    /// Chile holidays and calendars.
    pub mod chile;
}

/// Calanders implemented for Middle Eastern countries. 
pub mod middle_east {
    /// Israeli (Jewish) holidays and calander, implemented with an external API.
    pub mod israel;
}