use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use fractal_support::core::{Invoice};

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

    CreateInvoice{
        payer_addr: String,
        payer_alias: String,
        invoice_id: String,
        invoiced_value: String,
        date_due: String,
        receipt_unit: String,
    },

    UpdateInvoice{
        invoice_id: String,
        invoice: Invoice,
    },
    
    //DeleteInvoice{},

    

}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(AllInvoicesResponse)]
    AllInvoices{ },
    
    #[returns(OneInvoiceResponse)]
    OneInvoice{
        invoice_id: String,
    },

    // #[returns(PaymentResponse)]
    // Payment{
    //     invoice_id: String,
    //     address: String,
    // },

}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]

pub struct AllInvoicesResponse {
    pub invoices: Vec<Invoice>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OneInvoiceResponse {
    pub invoice: Option<Invoice>,
}

// // Previous code omitted
// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// pub struct PaymentResponse {
//     pub payment: Option<Payment>,
// }