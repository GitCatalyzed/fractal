use crate::error::ContractError;
use crate::state::{Invoice, config_write, Payment, PAYMENTS};
use crate::msg::AllInvoicesResponse;
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, Order, to_binary, Decimal, Timestamp
};
use std::str::FromStr;

fn execute_pay_invoice(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    invoice_id: String,
    payment_amount: String,
    pay_unit: String,
) -> Result<Response, ContractError> {
    //load invoice and check that it exists
    let invoice = INVOICES.may_load(deps.storage, invoice_id.to_string())?;
    
    let payment_amount = Decimal::from_str(&payment_amount).unwrap();
    let zero_decimal = Decimal::from_str("0.0").unwrap();

    match invoice{
        //If there is an invoice, we need to make sure it is a valid payment from payer
        //and determine what payment this is
        //NEED TO REWRITE THIS CODE
        Some(mut invoice) => {
            //Validate payer against invoice expectation and ensure address and payment is valid
            if invoice.payer_addr.to_string() != info.sender.to_string(){
                return Err(ContractError::IncorrectPayer{})
            }
            if payment_amount <= zero_decimal {
                return Err(ContractError::InvalidPaymentValue{})
            }
            if payment_amount > invoice.balance_outstanding{
                return Err(ContractError::InvalidPaymentValue{})
            }

            if pay_unit != invoice.pay_unit {
                return Err(ContractError::InvalidPaymentValue{})
            }

            let unvalidated_payer = info.sender.to_string();
            let payer_addr = deps.api.addr_validate(&unvalidated_payer)?;
            let current_payment = invoice.payment_history.len()+1;

            let payment_id = format!("{}_P{}", invoice_id, current_payment);

            let today = env.block.time;
            //let formatted_date = today.format("%Y-%m-%d").to_string();

            //Accept the payment as valid and save it
            let payment = Payment{
                payment_id,
                payer_addr: payer_addr.clone(),
                payer_alias,
                invoice_id: invoice_id.clone(),
                payment_amount,
                pay_unit,
                pay_date: today,
            };

            PAYMENTS.save(deps.storage, (payer_addr.clone(), invoice_id.clone()), &payment)?;

            //Update the Invoice and store the payment in the history
            invoice.payment_history.push(payment.clone());
            invoice.balance_outstanding -= payment.payment_amount;
            if invoice.balance_outstanding == zero_decimal {
                invoice.status = "Closed".to_string();
            } else if invoice.balance_outstanding > zero_decimal {
                invoice.status = "Partially Paid".to_string();
            }

            INVOICES.save(deps.storage, invoice_id.to_string(), &invoice)?;
            Ok(Response::new())

        },
        None => Err(ContractError::Unauthorized{}),
    }
}