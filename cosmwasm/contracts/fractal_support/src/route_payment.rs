use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{coins, BankMsg, DepsMut, MessageInfo, Response};
use crate::core::{Payment};
use crate::error::ContractError as core_payment::error::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Route {
 
    DirectOnchain,
    CircleRoute,
    CrossChain,

}

pub fn payment_route_direct_onchain (
    payment: Payment,
    _deps: DepsMut, 
    info: MessageInfo) -> Result<Response, ContractError> {

    let denom = payment.send_unit.clone();
    let onchain_payment_result = cw_utils::must_pay(&info, &denom);

    let onchain_payment = match onchain_payment_result {
        Ok(onchain_payment) => onchain_payment.u128(),
        Err(_) => return Err(ContractError::MustPayError{})
    };

    let payment_amount_str = payment.payment_amount.to_string();

    let payment_amount_validation = match payment_amount_str.parse::<u128>() {
        Ok(payment_amount_validation) => payment_amount_validation,
        Err(_) => return Err(ContractError::ParseError{}),
    };

    if payment_amount_validation != onchain_payment {
        return Err(ContractError::UnmatchingPayment{});
    }
    
    let bank_message = BankMsg::Send{
        to_address: payment.invoice_address.clone().to_string(),
        amount: coins(onchain_payment, &denom),
    };

    let resp = Response::new().add_message(bank_message);
    Ok(resp)

}