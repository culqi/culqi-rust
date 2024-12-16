pub const REQUEST_CHARGUE_BODY: &str = r#"{
    "amount": 600,
    "currency_code": "PEN",
    "email": "review@culqi.com",
    "source_id": "tkn_test_IctezQFcWKhvOHyQ",
    "antifraud_details": {
        "first_name": "Fernando",
        "last_name": "Chullo",
        "email": "review134@culqi.com",
        "phone_number": "945737476",
        "device_finger_print_id": "8b17f1dc-e616-46cf-b416-ec7ef63730e9"
    }
}"#;

pub const CUSTOM_HEADERS: &str = r#"{
    "X-Charge-Channel": "recurrent",
    "X-Plan-Type": 1,
    "X-Header-Config": true
}"#;