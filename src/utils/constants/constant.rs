pub const CUSTOMER_KEY: &str = "cus";
pub const TOKEN_KEY: &str = "tkn";
pub const CARD_KEY: &str = "crd";
pub const ORDER_KEY: &str = "ord";
pub const PLAN_KEY: &str = "pln";
pub const REFUND_KEY: &str = "ref";
pub const SUBSCRIPTION_KEY: &str = "sxn";
pub const CHARGE_KEY: &str = "chr";
pub const CUSTOMER_ID: &str = "customer_id";
pub const TOKEN_ID: &str = "token_id";
pub const CHARGE_ID: &str = "charge_id";
pub const SOURCER_ID: &str = "source_id";
pub const ORDER_ID: &str = "order_id";
pub const CURRENCY_CODE: &str = "currency_code";
pub const CARD_ID: &str = "card_id";
pub const PLAN_ID: &str = "plan_id";

pub const BEFORE: &str = "before";
pub const LIMIT: &str = "limit";
pub const AFTER: &str = "after";
pub const STATUS: &str = "status";
pub const REASON: &str = "reason";
pub const ALLOW_VALUES_CURRENCY_CODE: &[&str] = &["PEN", "USD",];

pub const CREATION_DATE_FROM: &str = "creation_date_from";
pub const CREATION_DATE_TO: &str = "creation_date_to";
pub const COUNTRY_CODE: &str = "country_code";
pub const DEVICE_TYPE: &str = "device_type";
pub const EMAIL: &str = "email";
pub const METADATA: &str = "metadata";
pub const CARD_NUMBER: &str = "card_number";
pub const CVV: &str = "cvv";
pub const EXPIRATION_MONTH: &str = "expiration_month";
pub const EXPIRATION_YEAR: &str = "expiration_year";
pub const EXPIRATION_DATE: &str = "expiration_date";

pub const ALLOW_VALUES_COUNTRY_CODE: &[&str] = &[
    "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW", "AX", "AZ",
    "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN", "BO", "BQ", "BR", "BS",
    "BT", "BV", "BW", "BY", "BZ", "CA", "CC", "CD", "CF", "CG", "CH", "CI", "CK", "CL", "CM", "CN",
    "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ", "DE", "DJ", "DK", "DM", "DO", "DZ", "EC", "EE",
    "EG", "EH", "ER", "ES", "ET", "FI", "FJ", "FK", "FM", "FO", "FR", "GA", "GB", "GD", "GE", "GF",
    "GG", "GH", "GI", "GL", "GM", "GN", "GP", "GQ", "GR", "GS", "GT", "GU", "GW", "GY", "HK", "HM",
    "HN", "HR", "HT", "HU", "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM",
    "JO", "JP", "KE", "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC",
    "LI", "LK", "LR", "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MF", "MG", "MH", "MK",
    "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV", "MW", "MX", "MY", "MZ", "NA",
    "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP", "NR", "NU", "NZ", "OM", "PA", "PE", "PF", "PG",
    "PH", "PK", "PL", "PM", "PN", "PR", "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW",
    "SA", "SB", "SC", "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS",
    "ST", "SV", "SX", "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO",
    "TR", "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE", "VG", "VI",
    "VN", "VU", "WF", "WS", "YE", "YT", "ZA", "ZM", "ZW",
];

pub const ALLOW_VALUES_DEVICE_TYPE: &[&str] = &[
    "escritorio",
    "movil",
    "tablet",
    "desktop",
    "tablet",
    "mobile",
];

pub const ALLOW_VALUES_ORDER_TYPES: &[&str] = &["cip", "cuotealo",];
