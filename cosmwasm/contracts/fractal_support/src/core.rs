use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Decimal, Timestamp};
use crate::route_payment::{Route};
use std::str;

pub static ONCHAIN_LIST: &'static [&str] = &["uosmo","uatom","uusdc"];

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Invoice {
    pub creator: Addr,
    pub payer_addr: Addr,
    pub payer_alias: String,
    pub invoice_id: String,
    pub invoiced_value: Decimal,
    pub balance_outstanding: Decimal,
    pub date_due: String,
    pub status: String,
    pub receipt_unit: String,
    pub payment_history: Vec<Payment>,
    pub tokenized_status: String,
    //Need to check if a CCF governs the creator-payer pair
    pub ccf_status: String,
    
}

// The payment struct needs to interact with cross-chain/cross-mode accounts (i.e. bank accounts) for fidelity
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Payment {
    pub payment_id: String,
    pub payer_addr: Addr,
    pub payer_alias: String,
    pub invoice_id: String,
    pub invoice_address: Addr,
    pub payment_amount: Decimal,
    pub pay_unit: String,
    pub pay_date: Timestamp,
    pub send_unit: String,
    //Routing through - if a USD payment is happening, we would verify through bank account check
    pub settlement_status: String,
}

pub fn conversion(pay_unit: &str, receipt_unit: &str) -> String {
    if ONCHAIN_LIST.contains(&pay_unit) && ONCHAIN_LIST.contains(&receipt_unit) {
        format!("{}_P{}", pay_unit, receipt_unit)
    } else {
        format!("{}_{}", "onchain", "onchain")
    }
}


//NOT COMPLETE. Need cross chain and error
pub fn payment_route(send_unit: String, receipt_unit: String) -> Route {

    let send_unit_str: &str = &send_unit;
    let receipt_unit_str: &str = &receipt_unit;

    if ONCHAIN_LIST.contains(&send_unit_str) && ONCHAIN_LIST.contains(&receipt_unit_str){
        return Route::DirectOnchain
    } else if send_unit.to_uppercase() == "USD" || receipt_unit.to_uppercase() == "USD" {
        return Route::CircleRoute
    } else {
        return Route::CrossChain
    }
    
}