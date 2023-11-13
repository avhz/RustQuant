// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// ISO 3166 country codes enum.
///
/// Format:
///     - `alpha_2`: ISO 3166-1 alpha-2 country code. e.g. US = United States
///     - `alpha_3`: ISO 3166-1 alpha-3 country code. e.g. USA = United States
///    - `numeric`: ISO 3166-1 numeric code. e.g. USA = 840
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct ISO_3166 {
    /// The ISO 3166-1 alpha-2 country code.
    pub alpha_2: &'static str,

    /// The ISO 3166-1 alpha-3 country code.
    pub alpha_3: &'static str,

    /// The ISO 3166-1 numeric code.
    pub numeric: &'static str,
}

impl ISO_3166 {
    /// Returns the ISO 3166-1 alpha-2 country code.
    pub fn alpha_2(&self) -> &'static str {
        self.alpha_2
    }

    /// Returns the ISO 3166-1 alpha-3 country code.
    pub fn alpha_3(&self) -> &'static str {
        self.alpha_3
    }

    /// Returns the ISO 3166-1 numeric code.
    pub fn numeric(&self) -> &'static str {
        self.numeric
    }

    /// Convert ISO 3166-1 alpha-2 country code to `ISO_3166` struct.
    pub fn from_alpha_2(alpha_2: &str) -> Option<Self> {
        match alpha_2 {
            "AF" => Some(AFGHANISTAN),
            "AX" => Some(ÅLAND_ISLANDS),
            "AL" => Some(ALBANIA),
            "DZ" => Some(ALGERIA),
            "AS" => Some(AMERICAN_SAMOA),
            "AD" => Some(ANDORRA),
            "AO" => Some(ANGOLA),
            "AI" => Some(ANGUILLA),
            "AQ" => Some(ANTARCTICA),
            "AG" => Some(ANTIGUA_AND_BARBUDA),
            "AR" => Some(ARGENTINA),
            "AM" => Some(ARMENIA),
            "AW" => Some(ARUBA),
            "AU" => Some(AUSTRALIA),
            "AT" => Some(AUSTRIA),
            "AZ" => Some(AZERBAIJAN),
            "BS" => Some(BAHAMAS),
            "BH" => Some(BAHRAIN),
            "BD" => Some(BANGLADESH),
            "BB" => Some(BARBADOS),
            "BY" => Some(BELARUS),
            "BE" => Some(BELGIUM),
            "BZ" => Some(BELIZE),
            "BJ" => Some(BENIN),
            "BM" => Some(BERMUDA),
            "BT" => Some(BHUTAN),
            "BO" => Some(BOLIVIA),
            "BQ" => Some(BONAIRE_SINT_EUSTATIUS_AND_SABA),
            "BA" => Some(BOSNIA_AND_HERZEGOVINA),
            "BW" => Some(BOTSWANA),
            "BV" => Some(BOUVET_ISLAND),
            "BR" => Some(BRAZIL),
            "IO" => Some(BRITISH_INDIAN_OCEAN_TERRITORY),
            "BN" => Some(BRUNEI_DARUSSALAM),
            "BG" => Some(BULGARIA),
            "BF" => Some(BURKINA_FASO),
            "BI" => Some(BURUNDI),
            "CV" => Some(CABO_VERDE),
            "KH" => Some(CAMBODIA),
            "CM" => Some(CAMEROON),
            "CA" => Some(CANADA),
            "KY" => Some(CAYMAN_ISLANDS),
            "CF" => Some(CENTRAL_AFRICAN_REPUBLIC),
            "TD" => Some(CHAD),
            "CL" => Some(CHILE),
            "CN" => Some(CHINA),
            "CX" => Some(CHRISTMANS_ISLAND),
            "CC" => Some(COCOS_ISLANDS),
            "CO" => Some(COLOMBIA),
            "KM" => Some(COMOROS),
            "CG" => Some(CONGO),
            "CD" => Some(CONGO_DEMOCRATIC_REPUBLIC),
            "CK" => Some(COOK_ISLANDS),
            "CR" => Some(COSTA_RICA),
            "CI" => Some(CÔTE_DIVOIRE),
            "HR" => Some(CROATIA),
            "CU" => Some(CUBA),
            "CW" => Some(CURAÇAO),
            "CY" => Some(CYPRUS),
            "CZ" => Some(CZECHIA),
            "DK" => Some(DENMARK),
            "DJ" => Some(DJIBOUTI),
            "DM" => Some(DOMINICA),
            "DO" => Some(DOMINICAN_REPUBLIC),
            "EC" => Some(ECUADOR),
            "EG" => Some(EGYPT),
            "SV" => Some(EL_SALVADOR),
            "GQ" => Some(EQUATORIAL_GUINEA),
            "ER" => Some(ERITREA),
            "EE" => Some(ESTONIA),
            "SZ" => Some(ESWATINI),
            "ET" => Some(ETHIOPIA),
            "FK" => Some(FALKLAND_ISLANDS),
            "FO" => Some(FAROE_ISLANDS),
            "FJ" => Some(FIJI),
            "FI" => Some(FINLAND),
            "FR" => Some(FRANCE),
            "GF" => Some(FRENCH_GUINEA),
            "PF" => Some(FRENCH_POLYNESIA),
            "TF" => Some(FRENCH_SOUTHERN_TERRITORIES),
            "GA" => Some(GABON),
            "GM" => Some(GAMBIA),
            "GE" => Some(GEORGIA),
            "DE" => Some(GERMANY),
            "GH" => Some(GHANA),
            "GI" => Some(GIBRALTAR),
            "GR" => Some(GREECE),
            "GL" => Some(GREENLAND),
            "GD" => Some(GRENADA),
            "GP" => Some(GUADELOUPE),
            "GU" => Some(GUAM),
            "GT" => Some(GUATEMALA),
            "GG" => Some(GUERNSEY),
            "GN" => Some(GUINEA),
            "GW" => Some(GUINEA_BISSAU),
            "GY" => Some(GUYANA),
            "HT" => Some(HAITI),
            "HM" => Some(HEARD_ISLAND_AND_MCDONALD_ISLANDS),
            "VA" => Some(HOLY_SEE),
            "HN" => Some(HONDURAS),
            "HK" => Some(HONG_KONG),
            "HU" => Some(HUNGARY),
            "IS" => Some(ICELAND),
            "IN" => Some(INDIA),
            "ID" => Some(INDONESIA),
            "IR" => Some(IRAN),
            "IQ" => Some(IRAQ),
            "IE" => Some(IRELAND),
            "IM" => Some(ISLE_OF_MAN),
            "IL" => Some(ISRAEL),
            "IT" => Some(ITALY),
            "JM" => Some(JAMAICA),
            "JP" => Some(JAPAN),
            "JE" => Some(JERSEY),
            "JO" => Some(JORDAN),
            "KZ" => Some(KAZAKHSTAN),
            "KE" => Some(KENYA),
            "KI" => Some(KIRIBATI),
            "KP" => Some(KOREA_DEMOCRATIC_PEOPLE_REPUBLIC),
            "KR" => Some(KOREA_REPUBLIC),
            "KW" => Some(KUWAIT),
            "KG" => Some(KYRGYZSTAN),
            "LA" => Some(LAO_PEOPLE_DEMOCRATIC_REPUBLIC),
            "LV" => Some(LATVIA),
            "LB" => Some(LEBANON),
            "LS" => Some(LESOTHO),
            "LR" => Some(LIBERIA),
            "LY" => Some(LIBYA),
            "LI" => Some(LIECHTENSTEIN),
            "LT" => Some(LITHUANIA),
            "LU" => Some(LUXEMBOURG),
            "MO" => Some(MACAO),
            "MG" => Some(MADAGASCAR),
            "MW" => Some(MALAWI),
            "MY" => Some(MALAYSIA),
            "MV" => Some(MALDIVES),
            "ML" => Some(MALI),
            "MT" => Some(MALTA),
            "MH" => Some(MARSHALL_ISLANDS),
            "MQ" => Some(MARTINIQUE),
            "MR" => Some(MAURITANIA),
            "MU" => Some(MAURITIUS),
            "YT" => Some(MAYOTTE),
            "MX" => Some(MEXICO),
            "FM" => Some(MICRONESIA),
            "MD" => Some(MOLDOVA),
            "MC" => Some(MONACO),
            "MN" => Some(MONGOLIA),
            "ME" => Some(MONTENEGRO),
            "MS" => Some(MONTSERRAT),
            "MA" => Some(MOROCCO),
            "MZ" => Some(MOZAMBIQUE),
            "MM" => Some(MYANMAR),
            "NA" => Some(NAMIBIA),
            "NR" => Some(NAURU),
            "NP" => Some(NEPAL),
            "NL" => Some(NETHERLANDS),
            "NC" => Some(NEW_CALEDONIA),
            "NZ" => Some(NEW_ZEALAND),
            "NI" => Some(NICARAGUA),
            "NE" => Some(NIGER),
            "NG" => Some(NIGERIA),
            "NU" => Some(NIUE),
            "NF" => Some(NORFOLK_ISLAND),
            "MK" => Some(NORTH_MACEDONIA),
            "MP" => Some(NORTHERN_MARIANA_ISLANDS),
            "NO" => Some(NORWAY),
            "OM" => Some(OMAN),
            "PK" => Some(PAKISTAN),
            "PW" => Some(PALAU),
            "PS" => Some(PALESTINE),
            "PA" => Some(PANAMA),
            "PG" => Some(PAPUA_NEW_GUINEA),
            "PY" => Some(PARAGUAY),
            "PE" => Some(PERU),
            "PH" => Some(PHILIPPINES),
            "PN" => Some(PITCAIRN),
            "PL" => Some(POLAND),
            "PT" => Some(PORTUGAL),
            "PR" => Some(PUERTO_RICO),
            "QA" => Some(QATAR),
            "RE" => Some(RÉUNION),
            "RO" => Some(ROMANIA),
            "RU" => Some(RUSSIAN_FEDERATION),
            "RW" => Some(RWANDA),
            "BL" => Some(SAINT_BARTHELEMY),
            "SH" => Some(SAINT_HELENA_ASCENSION_AND_TRISTAN_DA_CUNHA),
            "KN" => Some(SAINT_KITTS_AND_NEVIS),
            "LC" => Some(SAINT_LUCIA),
            "MF" => Some(SAINT_MARTIN),
            "PM" => Some(SAINT_PIERRE_AND_MIQUELON),
            "VC" => Some(SAINT_VINCENT_AND_THE_GRENADINES),
            "WS" => Some(SAMOA),
            "SM" => Some(SAN_MARINO),
            "ST" => Some(SAO_TOME_AND_PRINCIPE),
            "SA" => Some(SAUDI_ARABIA),
            "SN" => Some(SENEGAL),
            "RS" => Some(SERBIA),
            "SC" => Some(SEYCHELLES),
            "SL" => Some(SIERRA_LEONE),
            "SG" => Some(SINGAPORE),
            "SX" => Some(SINT_MAARTEN),
            "SK" => Some(SLOVAKIA),
            "SI" => Some(SLOVENIA),
            "SB" => Some(SOLOMON_ISLANDS),
            "SO" => Some(SOMALIA),
            "ZA" => Some(SOUTH_AFRICA),
            "GS" => Some(SOUTH_GEORGIA_AND_THE_SOUTH_SANDWICH_ISLANDS),
            "SS" => Some(SOUTH_SUDAN),
            "ES" => Some(SPAIN),
            "LK" => Some(SRI_LANKA),
            "SD" => Some(SUDAN),
            "SR" => Some(SURINAME),
            "SJ" => Some(SVALBARD_AND_JAN_MAYEN),
            "SE" => Some(SWEDEN),
            "CH" => Some(SWITZERLAND),
            "SY" => Some(SYRIAN_ARAB_REPUBLIC),
            "TW" => Some(TAIWAN),
            "TJ" => Some(TAJIKISTAN),
            "TZ" => Some(TANZANIA),
            "TH" => Some(THAILAND),
            "TL" => Some(TIMOR_LESTE),
            "TG" => Some(TOGO),
            "TK" => Some(TOKELAU),
            "TO" => Some(TONGA),
            "TT" => Some(TRINIDAD_AND_TOBAGO),
            "TN" => Some(TUNISIA),
            "TR" => Some(TURKEY),
            "TM" => Some(TURKMENISTAN),
            "TC" => Some(TURKS_AND_CAICOS_ISLANDS),
            "TV" => Some(TUVALU),
            "UG" => Some(UGANDA),
            "UA" => Some(UKRAINE),
            "AE" => Some(UNITED_ARAB_EMIRATES),
            "GB" => Some(UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND),
            "US" => Some(UNITED_STATES_OF_AMERICA),
            "UM" => Some(UNITED_STATES_MINOR_OUTLYING_ISLANDS),
            "UY" => Some(URUGUAY),
            "UZ" => Some(UZBEKISTAN),
            "VU" => Some(VANUATU),
            "VE" => Some(VENEZUELA),
            "VN" => Some(VIETNAM),
            "VG" => Some(VIRGIN_ISLANDS_BRITISH),
            "VI" => Some(VIRGIN_ISLANDS_US),
            "WF" => Some(WALLIS_AND_FUTUNA),
            "EH" => Some(WESTERN_SAHARA),
            "YE" => Some(YEMEN),
            "ZM" => Some(ZAMBIA),
            "ZW" => Some(ZIMBABWE),
            _ => None,
        }
    }
}

/// Macro to generate all ISO 3166 country codes.
macro_rules! iso_3166 {
    ($($name:ident: $alpha_2:literal, $alpha_3:literal, $numeric:literal,)*) => {
        $(
            /// $name
            pub const $name: ISO_3166 = ISO_3166 {
                alpha_2: $alpha_2,
                alpha_3: $alpha_3,
                numeric: $numeric,
            };
        )*
    };
}

iso_3166! {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // A
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    AFGHANISTAN: "AF", "AFG", "004",
    ÅLAND_ISLANDS: "AX", "ALA", "248",
    ALBANIA: "AL", "ALB", "008",
    ALGERIA: "DZ", "DZA", "012",
    AMERICAN_SAMOA: "AS", "ASM", "016",
    ANDORRA: "AD", "AND", "020",
    ANGOLA: "AO", "AGO", "024",
    ANGUILLA: "AI", "AIA", "660",
    ANTARCTICA: "AQ", "ATA", "010",
    ANTIGUA_AND_BARBUDA: "AG", "ATG", "028",
    ARGENTINA: "AR", "ARG", "032",
    ARMENIA: "AM", "ARM", "051",
    ARUBA: "AW", "ABW", "533",
    AUSTRALIA: "AU", "AUS", "036",
    AUSTRIA: "AT", "AUT", "040",
    AZERBAIJAN: "AZ", "AZE", "031",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // B
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    BAHAMAS: "BS", "BHS", "044",
    BAHRAIN: "BH", "BHR", "048",
    BANGLADESH: "BD", "BGD", "050",
    BARBADOS: "BB", "BRB", "052",
    BELARUS: "BY", "BLR", "112",
    BELGIUM: "BE", "BEL", "056",
    BELIZE: "BZ", "BLZ", "084",
    BENIN: "BJ", "BEN", "204",
    BERMUDA: "BM", "BMU", "060",
    BHUTAN: "BT", "BTN", "064",
    BOLIVIA: "BO", "BOL", "068",
    BONAIRE_SINT_EUSTATIUS_AND_SABA: "BQ", "BES", "535",
    BOSNIA_AND_HERZEGOVINA: "BA", "BIH", "070",
    BOTSWANA: "BW", "BWA", "072",
    BOUVET_ISLAND: "BV", "BVT", "074",
    BRAZIL: "BR", "BRA", "076",
    BRITISH_INDIAN_OCEAN_TERRITORY: "IO", "IOT", "086",
    BRUNEI_DARUSSALAM: "BN", "BRN", "096",
    BULGARIA: "BG", "BGR", "100",
    BURKINA_FASO: "BF", "BFA", "854",
    BURUNDI: "BI", "BDI", "108",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // C
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    CABO_VERDE: "CV", "CPV", "132",
    CAMBODIA: "KH", "KHM", "116",
    CAMEROON: "CM", "CMR", "120",
    CANADA: "CA", "CAN", "124",
    CAYMAN_ISLANDS: "KY", "CYM", "136",
    CENTRAL_AFRICAN_REPUBLIC: "CF", "CAF", "140",
    CHAD: "TD", "TCD", "148",
    CHILE: "CL", "CHL", "152",
    CHINA: "CN", "CHN", "156",
    CHRISTMANS_ISLAND: "CX", "CXR", "162",
    COCOS_ISLANDS: "CC", "CCK", "166",
    COLOMBIA: "CO", "COL", "170",
    COMOROS: "KM", "COM", "174",
    CONGO: "CG", "COG", "178",
    CONGO_DEMOCRATIC_REPUBLIC: "CD", "COD", "180",
    COOK_ISLANDS: "CK", "COK", "184",
    COSTA_RICA: "CR", "CRI", "188",
    CÔTE_DIVOIRE: "CI", "CIV", "384",
    CROATIA: "HR", "HRV", "191",
    CUBA: "CU", "CUB", "192",
    CURAÇAO: "CW", "CUW", "531",
    CYPRUS: "CY", "CYP", "196",
    CZECHIA: "CZ", "CZE", "203",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // D
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    DENMARK: "DK", "DNK", "208",
    DJIBOUTI: "DJ", "DJI", "262",
    DOMINICA: "DM", "DMA", "212",
    DOMINICAN_REPUBLIC: "DO", "DOM", "214",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // E
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ECUADOR: "EC", "ECU", "218",
    EGYPT: "EG", "EGY", "818",
    EL_SALVADOR: "SV", "SLV", "222",
    EQUATORIAL_GUINEA: "GQ", "GNQ", "226",
    ERITREA: "ER", "ERI", "232",
    ESTONIA: "EE", "EST", "233",
    ESWATINI: "SZ", "SWZ", "748",
    ETHIOPIA: "ET", "ETH", "231",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // F
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    FALKLAND_ISLANDS: "FK", "FLK", "238",
    FAROE_ISLANDS: "FO", "FRO", "234",
    FIJI: "FJ", "FJI", "242",
    FINLAND: "FI", "FIN", "246",
    FRANCE: "FR", "FRA", "250",
    FRENCH_GUINEA: "GF", "GUF", "254",
    FRENCH_POLYNESIA: "PF", "PYF", "258",
    FRENCH_SOUTHERN_TERRITORIES: "TF", "ATF", "260",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // G
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    GABON: "GA", "GAB", "266",
    GAMBIA: "GM", "GMB", "270",
    GEORGIA: "GE", "GEO", "268",
    GERMANY: "DE", "DEU", "276",
    GHANA: "GH", "GHA", "288",
    GIBRALTAR: "GI", "GIB", "292",
    GREECE: "GR", "GRC", "300",
    GREENLAND: "GL", "GRL", "304",
    GRENADA: "GD", "GRD", "308",
    GUADELOUPE: "GP", "GLP", "312",
    GUAM: "GU", "GUM", "316",
    GUATEMALA: "GT", "GTM", "320",
    GUERNSEY: "GG", "GGY", "831",
    GUINEA: "GN", "GIN", "324",
    GUINEA_BISSAU: "GW", "GNB", "624",
    GUYANA: "GY", "GUY", "328",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // H
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    HAITI: "HT", "HTI", "332",
    HEARD_ISLAND_AND_MCDONALD_ISLANDS: "HM", "HMD", "334",
    HOLY_SEE: "VA", "VAT", "336",
    HONDURAS: "HN", "HND", "340",
    HONG_KONG: "HK", "HKG", "344",
    HUNGARY: "HU", "HUN", "348",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // I
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ICELAND: "IS", "ISL", "352",
    INDIA: "IN", "IND", "356",
    INDONESIA: "ID", "IDN", "360",
    IRAN: "IR", "IRN", "364",
    IRAQ: "IQ", "IRQ", "368",
    IRELAND: "IE", "IRL", "372",
    ISLE_OF_MAN: "IM", "IMN", "833",
    ISRAEL: "IL", "ISR", "376",
    ITALY: "IT", "ITA", "380",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // J
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    JAMAICA: "JM", "JAM", "388",
    JAPAN: "JP", "JPN", "392",
    JERSEY: "JE", "JEY", "832",
    JORDAN: "JO", "JOR", "400",
    KAZAKHSTAN: "KZ", "KAZ", "398",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // K
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    KENYA: "KE", "KEN", "404",
    KIRIBATI: "KI", "KIR", "296",
    KOREA_DEMOCRATIC_PEOPLE_REPUBLIC: "KP", "PRK", "408",
    KOREA_REPUBLIC: "KR", "KOR", "410",
    KUWAIT: "KW", "KWT", "414",
    KYRGYZSTAN: "KG", "KGZ", "417",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // L
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    LAO_PEOPLE_DEMOCRATIC_REPUBLIC: "LA", "LAO", "418",
    LATVIA: "LV", "LVA", "428",
    LEBANON: "LB", "LBN", "422",
    LESOTHO: "LS", "LSO", "426",
    LIBERIA: "LR", "LBR", "430",
    LIBYA: "LY", "LBY", "434",
    LIECHTENSTEIN: "LI", "LIE", "438",
    LITHUANIA: "LT", "LTU", "440",
    LUXEMBOURG: "LU", "LUX", "442",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // M
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    MACAO: "MO", "MAC", "446",
    MADAGASCAR: "MG", "MDG", "450",
    MALAWI: "MW", "MWI", "454",
    MALAYSIA: "MY", "MYS", "458",
    MALDIVES: "MV", "MDV", "462",
    MALI: "ML", "MLI", "466",
    MALTA: "MT", "MLT", "470",
    MARSHALL_ISLANDS: "MH", "MHL", "584",
    MARTINIQUE: "MQ", "MTQ", "474",
    MAURITANIA: "MR", "MRT", "478",
    MAURITIUS: "MU", "MUS", "480",
    MAYOTTE: "YT", "MYT", "175",
    MEXICO: "MX", "MEX", "484",
    MICRONESIA: "FM", "FSM", "583",
    MOLDOVA: "MD", "MDA", "498",
    MONACO: "MC", "MCO", "492",
    MONGOLIA: "MN", "MNG", "496",
    MONTENEGRO: "ME", "MNE", "499",
    MONTSERRAT: "MS", "MSR", "500",
    MOROCCO: "MA", "MAR", "504",
    MOZAMBIQUE: "MZ", "MOZ", "508",
    MYANMAR: "MM", "MMR", "104",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // N
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    NAMIBIA: "NA", "NAM", "516",
    NAURU: "NR", "NRU", "520",
    NEPAL: "NP", "NPL", "524",
    NETHERLANDS: "NL", "NLD", "528",
    NEW_CALEDONIA: "NC", "NCL", "540",
    NEW_ZEALAND: "NZ", "NZL", "554",
    NICARAGUA: "NI", "NIC", "558",
    NIGER: "NE", "NER", "562",
    NIGERIA: "NG", "NGA", "566",
    NIUE: "NU", "NIU", "570",
    NORFOLK_ISLAND: "NF", "NFK", "574",
    NORTH_MACEDONIA: "MK", "MKD", "807",
    NORTHERN_MARIANA_ISLANDS: "MP", "MNP", "580",
    NORWAY: "NO", "NOR", "578",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // O
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    OMAN: "OM", "OMN", "512",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // P
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    PAKISTAN: "PK", "PAK", "586",
    PALAU: "PW", "PLW", "585",
    PALESTINE: "PS", "PSE", "275",
    PANAMA: "PA", "PAN", "591",
    PAPUA_NEW_GUINEA: "PG", "PNG", "598",
    PARAGUAY: "PY", "PRY", "600",
    PERU: "PE", "PER", "604",
    PHILIPPINES: "PH", "PHL", "608",
    PITCAIRN: "PN", "PCN", "612",
    POLAND: "PL", "POL", "616",
    PORTUGAL: "PT", "PRT", "620",
    PUERTO_RICO: "PR", "PRI", "630",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Q
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    QATAR: "QA", "QAT", "634",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // R
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    RÉUNION: "RE", "REU", "638",
    ROMANIA: "RO", "ROU", "642",
    RUSSIAN_FEDERATION: "RU", "RUS", "643",
    RWANDA: "RW", "RWA", "646",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // S
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    SAINT_BARTHELEMY: "BL", "BLM", "652",
    SAINT_HELENA_ASCENSION_AND_TRISTAN_DA_CUNHA: "SH", "SHN", "654",
    SAINT_KITTS_AND_NEVIS: "KN", "KNA", "659",
    SAINT_LUCIA: "LC", "LCA", "662",
    SAINT_MARTIN: "MF", "MAF", "663",
    SAINT_PIERRE_AND_MIQUELON: "PM", "SPM", "666",
    SAINT_VINCENT_AND_THE_GRENADINES: "VC", "VCT", "670",
    SAMOA: "WS", "WSM", "882",
    SAN_MARINO: "SM", "SMR", "674",
    SAO_TOME_AND_PRINCIPE: "ST", "STP", "678",
    SAUDI_ARABIA: "SA", "SAU", "682",
    SENEGAL: "SN", "SEN", "686",
    SERBIA: "RS", "SRB", "688",
    SEYCHELLES: "SC", "SYC", "690",
    SIERRA_LEONE: "SL", "SLE", "694",
    SINGAPORE: "SG", "SGP", "702",
    SINT_MAARTEN: "SX", "SXM", "534",
    SLOVAKIA: "SK", "SVK", "703",
    SLOVENIA: "SI", "SVN", "705",
    SOLOMON_ISLANDS: "SB", "SLB", "090",
    SOMALIA: "SO", "SOM", "706",
    SOUTH_AFRICA: "ZA", "ZAF", "710",
    SOUTH_GEORGIA_AND_THE_SOUTH_SANDWICH_ISLANDS: "GS", "SGS", "239",
    SOUTH_SUDAN: "SS", "SSD", "728",
    SPAIN: "ES", "ESP", "724",
    SRI_LANKA: "LK", "LKA", "144",
    SUDAN: "SD", "SDN", "729",
    SURINAME: "SR", "SUR", "740",
    SVALBARD_AND_JAN_MAYEN: "SJ", "SJM", "744",
    SWEDEN: "SE", "SWE", "752",
    SWITZERLAND: "CH", "CHE", "756",
    SYRIAN_ARAB_REPUBLIC: "SY", "SYR", "760",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // T
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    TAIWAN: "TW", "TWN", "158",
    TAJIKISTAN: "TJ", "TJK", "762",
    TANZANIA: "TZ", "TZA", "834",
    THAILAND: "TH", "THA", "764",
    TIMOR_LESTE: "TL", "TLS", "626",
    TOGO: "TG", "TGO", "768",
    TOKELAU: "TK", "TKL", "772",
    TONGA: "TO", "TON", "776",
    TRINIDAD_AND_TOBAGO: "TT", "TTO", "780",
    TUNISIA: "TN", "TUN", "788",
    TURKEY: "TR", "TUR", "792",
    TURKMENISTAN: "TM", "TKM", "795",
    TURKS_AND_CAICOS_ISLANDS: "TC", "TCA", "796",
    TUVALU: "TV", "TUV", "798",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // U
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    UGANDA: "UG", "UGA", "800",
    UKRAINE: "UA", "UKR", "804",
    UNITED_ARAB_EMIRATES: "AE", "ARE", "784",
    UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND: "GB", "GBR", "826",
    UNITED_STATES_OF_AMERICA: "US", "USA", "840",
    UNITED_STATES_MINOR_OUTLYING_ISLANDS: "UM", "UMI", "581",
    URUGUAY: "UY", "URY", "858",
    UZBEKISTAN: "UZ", "UZB", "860",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // V
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    VANUATU: "VU", "VUT", "548",
    VENEZUELA: "VE", "VEN", "862",
    VIETNAM: "VN", "VNM", "704",
    VIRGIN_ISLANDS_BRITISH: "VG", "VGB", "092",
    VIRGIN_ISLANDS_US: "VI", "VIR", "850",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // W
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    WALLIS_AND_FUTUNA: "WF", "WLF", "876",
    WESTERN_SAHARA: "EH", "ESH", "732",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Y
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    YEMEN: "YE", "YEM", "887",
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Z
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ZAMBIA: "ZM", "ZMB", "894",
    ZIMBABWE: "ZW", "ZWE", "716",
}
