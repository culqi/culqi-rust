pub const REQUEST_PLAN_CREATE: &str = r#"{
    "short_name": "cp-prueb2442",
    "description": "Cypress PCI | ERRROR NO USAR",
    "amount": 300,
    "currency": "PEN",
    "interval_unit_time": 1,
    "interval_count": 1,
    "initial_cycles": {
      "count": 1,
      "has_initial_charge": true,
      "amount": 400,
      "interval_unit_time": 1
    },
    "name": "CY PCI - ERROR 100018",
    "metadata":{
        "key": "value"
    }
}"#;

pub const REQUEST_PLAN_UPDATE: &str = r#"{
    "short_name": "cp-prueb2442",
    "description": "Cypress PCI | ERRROR NO USAR",
    "name": "CY PCI - ERROR 100018",
}"#;

pub const REQUEST_PLAN_ALL: &str = r#"{
    "limit": 100,
    "status": 1,
    "before": "pln_live_oUr88s1vYacQ4wI9"
}"#;