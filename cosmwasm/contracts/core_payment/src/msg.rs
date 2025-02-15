use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//use crate::state::{Payment, Invoice};
use fractal_support::core::{Invoice, Payment};

/// Message type for `instantiate` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {

    pub admin: Option<String>,
    pub business_alias: String,
    pub usdc_address: Option<String>,
    pub bank_routing: String,
    pub bank_account: String,

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
        //address: String,
        payment_id: String,
    },

    #[returns(AllPaymentsResponse)]
    AllPayments{ },

    // #[returns(OneInvoiceResponse)]
    // OneInvoice{
    //     invoice_id: String,
    // },

}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OneInvoiceResponse {
    pub invoice: Option<Invoice>,
}

// Previous code omitted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PaymentResponse {
    pub payments: Option<Payment>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllPaymentsResponse {
    pub payments: Vec<Payment>,
}