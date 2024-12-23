pub const SECURE_URL: &str = "https://secure.culqi.com/v2";
pub const BASE_URL: &str = "https://api.culqi.com/v2";

pub const ORDER_URL: &str = "/orders";
pub const ORDER_CONFIRM_URL: &str = "/orders/{:id}/confirm";
pub const ORDER_CONFIRM_TYPE_URL: &str = "/orders/confirm";

pub const TOKEN_URL: &str = "/tokens";
pub const TOKEN_YAPE_URL: &str = "/tokens/yape";

pub const PLAN_URL: &str = "/recurrent/plans";
pub const PLAN_CREATE_URL: &str = "/recurrent/plans/create";

pub const SUBSCRIPTION_URL: &str = "/recurrent/subscriptions";
pub const SUBSCRIPTION_CREATE_URL: &str = "/recurrent/subscriptions/create";

pub const CUSTOMER_URL: &str = "/customers";

pub const CARD_URL: &str = "/cards";

pub const CHARGE_URL: &str = "/charges";
pub const CHARGE_CONFIRM_URL: &str = "/charges/{:id}/capture";

pub const REFUND_URL: &str = "/refunds";

pub fn get_url(path: &str, isTokenSecure: bool,) -> String {
    let base_url = if isTokenSecure { SECURE_URL } else { BASE_URL };
    format!("{}{}", base_url, path)
}

pub fn get_url_replace_id(path: &str, id: &str,) -> String {
    path.replace("{:id}", id,)
}
