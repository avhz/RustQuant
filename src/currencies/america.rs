// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::Currency;

/// Argentina Peso (ARS)
/// The ISO three-letter code is ARS; the numeric code is 32.
/// It is divided into 100 centavos.
pub struct ARSCurrency {
    /// The currency data.
    pub data: Currency,
}

/// Brazilian real (BRL)
/// The ISO three-letter code is BRL; the numeric code is 986.
/// It is divided into 100 centavos.
pub struct BRLCurrency {
    /// The currency data.
    pub data: Currency,
}

/// Canadian dollar (CAD)
/// The ISO three-letter code is CAD; the numeric code is 124.
/// It is divided into 100 cents.
pub struct CADCurrency {
    /// The currency data.
    pub data: Currency,
}

/// Chilean peso (CLP)
/// The ISO three-letter code is CLP; the numeric code is 152.
/// It is divided into 100 centavos.
pub struct CLPCurrency {
    /// The currency data.
    pub data: Currency,
}

/// Colombian peso (COP)
/// The ISO three-letter code is COP; the numeric code is 170.
/// It is divided into 100 centavos.
pub struct COPCurrency {
    /// The currency data.
    pub data: Currency,
}
