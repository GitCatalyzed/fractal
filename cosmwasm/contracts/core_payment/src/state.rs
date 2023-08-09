// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Storage, Decimal, Timestamp};
use cw_storage_plus::{Map};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use fractal_structs::core::Payment;

static CONFIG_KEY: &[u8] = b"config";

pub fn config_write(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
   pub admin: Addr,
   pub business_alias: String,
   pub usdc_address: String,
   pub bank_routing: String,
   pub bank_account: String,

}


// //This either needs to be, or needs to create an NFT
// //It should interact with the payment struct to determine status
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Invoice {
//     pub creator: Addr,
//     pub payer_addr: Addr,
//     pub payer_alias: String,
//     pub invoice_id: String,
//     pub invoiced_value: Decimal,
//     pub balance_outstanding: Decimal,
//     pub date_due: String,
//     pub status: String,
//     pub pay_unit: String,
//     pub receipt_unit: String,
//     pub payment_history: Vec<Payment>,
// }

// // The payment struct needs to interact with cross-chain/cross-mode accounts (i.e. bank accounts) for fidelity
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Payment {
//     pub payment_id: String,
//     pub payer_addr: Addr,
//     pub payer_alias: String,
//     pub invoice_id: String,
//     pub invoice_address: Addr,
//     pub payment_amount: Decimal,
//     pub pay_unit: String,
//     pub pay_date: Timestamp,
// }



//pub const CONFIG: Item<Config> = Item::new("config");
//pub const INVOICES: Map<String, Invoice> = Map::new("invoices");
pub const PAYMENTS: Map<(Addr, String), Payment> = Map::new("payments");
