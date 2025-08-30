// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! calendars and holidays for different countries.

/// Argentina holidays and calendars.
pub mod argentina;
pub(crate) use argentina::*;

/// Australia holidays and calendars.
pub mod australia;
pub(crate) use australia::*;

/// Austria holidays and calendars.
pub mod austria;
pub(crate) use austria::*;

/// Botswana holidays and calendars.
pub mod botswana;
pub(crate) use botswana::*;

/// Brazil holidays and calendars.
pub mod brazil;
pub(crate) use brazil::*;

/// Canada holidays and calendars.
pub mod canada;
pub(crate) use canada::*;

/// Chile holidays and calendars.
pub mod chile;
pub(crate) use chile::*;

/// China holidays and calendars.
pub mod china;
pub(crate) use china::*;

/// Czech Republic holidays and calendars.
pub mod czech_republic;
pub(crate) use czech_republic::*;

/// Denmark holidays and calendars.
pub mod denmark;
pub(crate) use denmark::*;

/// Finland holidays and calendars.
pub mod finland;
pub(crate) use finland::*;

/// France holidays and calendars.
pub mod france;
pub(crate) use france::*;

/// Germany holidays and calendars.
pub mod germany;
pub(crate) use germany::*;

/// Hong Kong holidays and calendars.
pub mod hong_kong;
pub(crate) use hong_kong::*;

/// Hungary holidays and calendars.
pub mod hungary;
pub(crate) use hungary::*;

/// Iceland holidays and calendars.
pub mod iceland;
pub(crate) use iceland::*;

/// India holidays and calendars.
pub mod india;
pub(crate) use india::*;

/// Indonesia holidays and calendars.
pub mod indonesia;
pub(crate) use indonesia::*;

/// Israeli (Jewish) holidays and calander, implemented with an external API.
pub mod israel;
pub(crate) use israel::*;

/// Italy holidays and calendars.
pub mod italy;
pub(crate) use italy::*;

/// Mexico holidays and calendars
pub mod mexico;
pub(crate) use mexico::*;

/// Netherlands holidays and calendars.
pub mod netherlands;
pub(crate) use netherlands::*;

/// New Zealand holidays and calendars.
pub mod new_zealand;
pub(crate) use new_zealand::*;

/// Singapore holidays and calendars.
pub mod singapore;
pub(crate) use singapore::*;

/// United Kingdom holidays and calendars.
pub mod united_kingdom;
pub(crate) use united_kingdom::*;

/// United States holidays and calendars.
pub mod united_states;
pub(crate) use united_states::*;
