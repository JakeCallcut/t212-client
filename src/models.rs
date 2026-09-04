// Typed response models for the Trading 212 API.

use serde::Deserialize;

/// from GET /equity/account/summary
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccountSummary {
    pub id: i64,
    pub currency: String,
    pub total_value: f64,
    pub cash: Cash,
    pub investments: Investments,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cash {
    pub available_to_trade: f64,
    pub in_pies: f64,
    pub reserved_for_orders: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Investments {
    pub current_value: f64,
    pub realized_profit_loss: f64,
    pub total_cost: f64,
    pub unrealized_profit_loss: f64,
}