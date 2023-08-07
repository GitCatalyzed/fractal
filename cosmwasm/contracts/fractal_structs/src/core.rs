use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Storage, Decimal, Timestamp};

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
    pub pay_unit: String,
    pub receipt_unit: String,
    pub payment_history: Vec<Payment>,
    pub tokenized_status: String,
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
}