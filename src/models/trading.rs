use crate::models::{
    AdvanceOption, AssetKind, Currency, Direction, Either, OrderState, OrderType, Request, Role,
    TimeInForce, Trigger, VoidRequest,
};
use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;

#[derive(Deserialize, Serialize, Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct BuyRequest(pub TradeRequest);

#[derive(Deserialize, Serialize, Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct BuyResponse(pub TradeResponse);

impl BuyRequest {
    pub fn market(instrument_name: &str, amount: f64) -> BuyRequest {
        BuyRequest(TradeRequest::market(instrument_name, amount))
    }
    pub fn limit(instrument_name: &str, price: f64, amount: f64) -> BuyRequest {
        BuyRequest(TradeRequest::limit(instrument_name, amount, price))
    }
}

impl Request for BuyRequest {
    const METHOD: &'static str = "private/buy";
    type Response = BuyResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct SellRequest(pub TradeRequest);

#[derive(Deserialize, Serialize, Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct SellResponse(pub TradeResponse);

impl SellRequest {
    pub fn market(instrument_name: &str, amount: f64) -> SellRequest {
        SellRequest(TradeRequest::market(instrument_name, amount))
    }
    pub fn limit(instrument_name: &str, price: f64, amount: f64) -> SellRequest {
        SellRequest(TradeRequest::limit(instrument_name, amount, price))
    }
}

impl Request for SellRequest {
    const METHOD: &'static str = "private/sell";
    type Response = SellResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EditRequest {
    pub order_id: String,
    pub amount: f64,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<AdvanceOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
}

impl EditRequest {
    pub fn new(order_id: &str, price: f64, amount: f64) -> Self {
        Self {
            order_id: order_id.to_string(),
            amount: amount,
            price: price,
            post_only: None,
            advanced: None,
            stop_price: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct EditResponse(pub TradeResponse);

impl Request for EditRequest {
    const METHOD: &'static str = "private/edit";
    type Response = EditResponse;
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeRequest {
    pub instrument_name: String,
    pub amount: f64,
    pub r#type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    pub time_in_force: TimeInForce,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_show: Option<f64>,
    pub post_only: bool,
    pub reduce_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<AdvanceOption>,
}

impl TradeRequest {
    pub fn market(instrument_name: &str, amount: f64) -> TradeRequest {
        TradeRequest {
            instrument_name: instrument_name.into(),
            amount: amount,
            r#type: OrderType::Market,
            label: None,
            price: None,
            time_in_force: TimeInForce::GoodTilCancelled,
            max_show: None,
            post_only: false,
            reduce_only: false,
            stop_price: None,
            trigger: None,
            advanced: None,
        }
    }

    pub fn limit(instrument_name: &str, amount: f64, price: f64) -> TradeRequest {
        TradeRequest {
            instrument_name: instrument_name.into(),
            amount: amount,
            r#type: OrderType::Limit,
            label: None,
            price: Some(price),
            time_in_force: TimeInForce::GoodTilCancelled,
            max_show: None,
            post_only: false,
            reduce_only: false,
            stop_price: None,
            trigger: None,
            advanced: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TradeResponse {
    pub trades: Vec<Trade>,
    pub order: Order,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Trade {
    pub trade_seq: i64,
    pub trade_id: String,
    pub timestamp: u128,
    pub tick_direction: i64,
    pub state: OrderState,
    pub self_trade: bool,
    pub price: f64,
    pub order_type: OrderType,
    pub order_id: String,
    pub matching_id: Option<String>,
    pub liquidity: Role,
    pub label: Option<String>,
    pub instrument_name: String,
    pub index_price: f64,
    pub fee_currency: Currency,
    pub fee: f64,
    pub direction: Direction,
    pub amount: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Order {
    pub time_in_force: TimeInForce,
    pub reduce_only: bool,
    pub profit_loss: f64,
    #[serde(deserialize_with = "deserialize_price")]
    pub price: Option<f64>, // None for stop_market
    pub post_only: bool,
    pub order_type: OrderType,
    pub order_state: OrderState,
    pub order_id: String,
    pub max_show: f64,
    pub last_update_timestamp: u128,
    pub label: Option<String>,
    pub is_liquidation: bool,
    pub instrument_name: String,
    pub filled_amount: f64,
    pub direction: Direction,
    pub creation_timestamp: u128,
    pub commission: f64,
    pub average_price: f64,
    pub api: bool,
    pub amount: f64,
}

fn deserialize_price<'de, D>(de: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let p: Either<String, f64> = Deserialize::deserialize(de)?;
    Ok(p.right())
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CancelOrderType {
    All,
    Limit,
    Stop,
}

impl Default for CancelOrderType {
    fn default() -> Self {
        CancelOrderType::All
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CancelRequest {
    order_id: String,
}

impl CancelRequest {
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CancelResponse {
    pub advanced: Option<AdvanceOption>,
    pub amount: f64,
    pub api: bool,
    pub average_price: f64,
    pub commission: f64,
    pub creation_timestamp: u64,
    pub direction: Direction,
    pub filled_amount: f64,
    pub implv: Option<f64>,
    pub instrument_name: String,
    pub is_liquidation: bool,
    pub label: String,
    pub last_update_timestamp: u64,
    pub max_show: f64,
    pub order_id: String,
    pub order_state: OrderState,
    pub order_type: OrderType,
    pub post_only: bool,
    pub price: f64,
    pub profit_loss: f64,
    pub reduce_only: bool,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub trigger: Option<Trigger>,
    pub triggered: Option<bool>,
    pub usd: Option<f64>,
}

impl Request for CancelRequest {
    const METHOD: &'static str = "private/cancel";
    type Response = CancelResponse;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CancelAllRequest;

impl Request for CancelAllRequest {
    const METHOD: &'static str = "private/cancel_all";
    type Response = CancelAllResponse;
}

impl VoidRequest for CancelAllRequest {
    #[inline]
    fn empty(&self) -> bool {
        true
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CancelAllByInstrumentRequest {
    pub instrument_name: String,
    pub r#type: CancelOrderType,
}

impl Request for CancelAllByInstrumentRequest {
    const METHOD: &'static str = "private/cancel_all_by_instrument";
    type Response = CancelAllResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct CancelAllByCurrencyRequest {
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AssetKind>,
    pub r#type: CancelOrderType,
}

impl Request for CancelAllByCurrencyRequest {
    const METHOD: &'static str = "private/cancel_all_by_currency";
    type Response = CancelAllResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CancelAllResponse {
    Ok,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetOrderStateRequest {
    order_id: String,
}

impl GetOrderStateRequest {
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetOrderStateResponse {
    pub advanced: Option<AdvanceOption>,
    pub amount: f64,
    pub api: bool,
    pub average_price: f64,
    pub commission: f64,
    pub creation_timestamp: u64,
    pub direction: Direction,
    pub filled_amount: f64,
    pub implv: Option<f64>,
    pub instrument_name: String,
    pub is_liquidation: bool,
    pub label: String,
    pub last_update_timestamp: u64,
    pub max_show: f64,
    pub order_id: String,
    pub order_state: OrderState,
    pub order_type: OrderType,
    pub post_only: bool,
    pub price: f64,
    pub profit_loss: f64,
    pub reduce_only: bool,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub trigger: Option<Trigger>,
    pub triggered: Option<bool>,
    pub usd: Option<f64>,
}

impl Request for GetOrderStateRequest {
    const METHOD: &'static str = "private/get_order_state";
    type Response = GetOrderStateResponse;
}
