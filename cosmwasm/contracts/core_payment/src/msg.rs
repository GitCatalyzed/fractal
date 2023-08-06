use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Payment, Invoice};

/// Message type for `instantiate` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {

    pub admin: Option<String>,
    pub business_alias: String,
    pub usdc_address: Option<String>,
    pub bank_routing: u16,
    pub bank_account: u16,

}

/// Message type for `execute` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    PayInvoice{
        invoice_id: String,
        invoice_address: String,
        payment_amount: String,
        pay_unit: String,
    }

}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {

    #[returns(PaymentResponse)]
    Payment{
        invoice_id: String,
        address: String,
    },

    #[returns(OneInvoiceResponse)]
    OneInvoice{
        invoice_id: String,
    },

}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OneInvoiceResponse {
    pub invoice: Option<Invoice>,
}

// Previous code omitted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PaymentResponse {
    pub payment: Option<Payment>,
}