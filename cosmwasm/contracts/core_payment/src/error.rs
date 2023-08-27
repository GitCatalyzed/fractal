use cosmwasm_std::StdError;
use thiserror::Error;
use cw_utils::PaymentError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("Paying address doesn't match expected payer")]
    IncorrectPayer{},

    #[error("Payment value is negative")]
    InvalidPaymentValue{},

    #[error("Incorrect Date Format")]
    InvalidDate{},

    #[error("Invoice doe not exist")]
    InvalidInvoice{},

    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),

}
