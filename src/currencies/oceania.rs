use crate::currencies::currency::*;

/// Australian dollar
/// The ISO three-letter code is AUD; the numeric code is 36.
/// It is divided into 100 cents.
struct AUDCurrency {
    data: Currency,
}

impl AUDCurrency {
    fn new() -> Self {
        Self {
            data: Currency {
                name: "Australian Dollar".to_string(),
                symbol: "AU$".to_string(),
                code: ISO_4217::AUD,
                minor: 2,
                fractions: 100,
            },
        }
    }
}
