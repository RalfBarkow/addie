use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct AddieUser {
    #[serde(default)]
    pub pub_key: String,
    pub uuid: String,
    #[serde(rename = "stripeAccountId")]
    #[serde(default)]
    pub stripe_account_id: String
}

impl Default for AddieUser {
    fn default() -> Self {
        AddieUser {
            pub_key: "".to_string(),
            uuid: "".to_string(),
            stripe_account_id: "".to_string()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Gateway {
    timestamp: String,
    uuid: String,
    minimum_cost: u32,
    ordinal: u64,
    signature: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Spell {
    pub timestamp: String,
    pub spell: String,
    #[serde(rename = "casterUUID")]
    pub caster_uuid: String,
    pub total_cost: u32, 
    pub mp: bool,
    pub ordinal: u32,
    pub caster_signature: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
    pub gateways: Vec<Gateway>
}

impl Default for Spell {
    fn default() -> Self {
	Spell {
	    timestamp: "now".to_string(),
	    spell: "test".to_string(),
	    caster_uuid: "".to_string(),
	    total_cost: 200,
	    mp: true,
	    ordinal: 1,
	    caster_signature: "".to_string(),
	    extra: HashMap::<String, Value>::new(),
	    gateways: Vec::<Gateway>::new()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SpellResult {
    pub success: bool,
    // arbitrary json somehow?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuccessResult {
    pub success: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Nineum {
    pub nineum: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PaymentIntent {
    pub payment_intent: String,
    pub ephemeral_key: String,
    pub customer: String,
    pub publishable_key: String
}

impl PaymentIntent {
    pub fn new() -> Self {
        PaymentIntent {
            payment_intent: "".to_string(),
            ephemeral_key: "".to_string(),
            customer: "".to_string(),
            publishable_key: "".to_string()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Payee {
    pub pubKey: String,
    pub amount: i32
}
