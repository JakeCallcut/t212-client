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

/// from GET /equity/positions
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Position {
    pub instrument: Instrument,
    pub quantity: f64,
    pub quantity_available_for_trading: f64,
    pub quantity_in_pies: f64,
    pub average_price_paid: f64,
    pub current_price: f64,
    pub created_at: String, // ISO 8601 timestamp; kept as String for now
    pub wallet_impact: PositionWalletImpact,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Instrument {
    pub ticker: String,
    pub name: String,
    pub isin: String,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PositionWalletImpact {
    pub currency: String,
    pub current_value: f64,
    pub total_cost: f64,
    pub unrealized_profit_loss: f64,
    pub fx_impact: Option<f64>, // null on most rows; a number when FX applies
}