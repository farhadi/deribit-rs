use crate::models::{Currency, Direction, LiquidationType, LiquidityType, OrderState, OrderType};
use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserTradesData {
    pub amount: f64,
    pub api: bool,
    pub combo_id: Option<String>,
    pub contracts: Option<f64>,
    pub direction: Direction,
    pub fee: f64,
    pub fee_currency: Currency,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub label: Option<String>,
    pub liquidity: LiquidityType,
    pub liquidation: Option<LiquidationType>,
    pub mark_price: Option<f64>,
    pub matching_id: Option<String>,
    pub mmp: bool,
    pub order_id: String,
    pub order_type: OrderType,
    pub original_order_type: Option<String>,
    pub price: f64,
    pub profit_loss: Option<f64>,
    pub reduce_only: Option<bool>,
    pub risk_reducing: Option<bool>,
    pub self_trade: bool,
    pub state: OrderState,
    pub tick_direction: i64,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: i64,
    pub post_only: bool,
}

#[derive(Debug, Clone)]
pub enum UserTradesChannel {
    ByInstrument {
        instrument_name: String,
        interval: String,
    },
    ByKind {
        kind: String,
        currency: String,
        interval: String,
    },
}

impl<'de> Deserialize<'de> for UserTradesChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["user", "trades", instrument_name, interval] => Ok(UserTradesChannel::ByInstrument {
                instrument_name: instrument_name.to_string(),
                interval: interval.to_string(),
            }),
            ["user", "trades", kind, currency, interval] => Ok(UserTradesChannel::ByKind {
                kind: kind.to_string(),
                currency: currency.to_string(),
                interval: interval.to_string(),
            }),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"user.trades.{instrument_name}.{interval} or trades.{kind}.{currency}.{interval}"
            )),
        }
    }
}
impl Serialize for UserTradesChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for UserTradesChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserTradesChannel::ByInstrument {
                instrument_name,
                interval,
            } => write!(f, "user.trades.{}.{}", instrument_name, interval),
            UserTradesChannel::ByKind {
                kind,
                currency,
                interval,
            } => {
                write!(f, "user.trades.{}.{}.{}", kind, currency, interval)
            }
        }
    }
}
