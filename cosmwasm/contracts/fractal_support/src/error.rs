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
    InvalidPaymentValueNegative{},

    #[error("Payment value is greater than balance outstanding")]
    InvalidPaymentValueAboveBalance{},

    #[error("Uncompatible Pay Unit")]
    InvalidPaymentPayUnit{},

    #[error("Incorrect Date Format")]
    InvalidDate{},

    #[error("Invoice doe not exist")]
    InvalidInvoice{},

    #[error("Send unit does not match receipt unit")]
    ConversionFailure{},

    #[error("Unable to succesfully route the onchain payment")]
    FailedOnchainRoute{},

    #[error("No viable payment routing")]
    FailedRouting{},

    #[error("Unable to parse input payment amount")]
    ParseError{},

    #[error("Routed Payment Amount does not equal input Payment Amount")]
    UnmatchingPayment{},

    #[error("Error accessing wallet funds to route onchain payment")]
    MustPayError{},

    #[error("Error accessing wallet funds to route onchain payment")]
    NoInvoiceValue{},

    #[error("Bank message not received by payment execute function")]
    BankMsgError{},

    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),

}