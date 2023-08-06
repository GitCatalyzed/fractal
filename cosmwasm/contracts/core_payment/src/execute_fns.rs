use crate::error::ContractError;
use crate::state::{Invoice, config_write, Payment, PAYMENTS};
//use crate::msg::{AllInvoicesResponse, QueryInvoice, OneInvoiceResponse};
use core_invoice::msg::{QueryMsg, OneInvoiceResponse};

use cosmwasm_std::{
    QueryRequest, entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, Order, to_binary, Decimal, Timestamp, WasmQuery
};
use std::str::FromStr;

pub fn execute_pay_invoice(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    invoice_id: String,
    invoice_address: String,
    payment_amount: String,
    pay_unit: String,
) -> Result<Response, ContractError> {

    //Query INVOICE database for existence of invoice to be paid
    let query = WasmQuery::Smart {
        contract_addr: invoice_address;
        msg: to_binary(&QueryMsg::OneInvoice {invoice_id})?,
    }
    let res: OneInvoiceResponse = deps.querier.query(&QueryRequest::Wasm(query))?;
    let validated_invoice = unwrap_err(res.invoice, ContractError::InvalidInvoice{});

    //Tap into Config to get the information about payer, ensure payer is admin
    let payer = config_read(deps.storage).load()?;
    if payer.admin.to_string() != info.sender.to_string(){
        return Err(ContractError::Unauthorized{})
    }

    let payment_amount = Decimal::from_str(&payment_amount).unwrap();
    let zero_decimal = Decimal::from_str("0.0").unwrap();

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

    let current_payment = invoice.payment_history.len()+1;

    let payment_id = format!("{}_P{}", invoice_id, current_payment);

    let today = env.block.time;
    //let formatted_date = today.format("%Y-%m-%d").to_string();

    let validated_contract_address = deps.api.addr_validate(&invoice_address)?;

    //Accept the payment as valid and save it
    let payment = Payment{
        payment_id,
        payer_addr: payer.admin,
        payer_alias: payer.business_alias,
        invoice_id: invoice_id.clone(),
        invoice_address: validated_contract_address,
        payment_amount,
        pay_unit,
        pay_date: today,
    };

    PAYMENTS.save(deps.storage, (payer_addr.clone(), invoice_id.clone()), &payment)?;

    //Create sub-message to send back to Invoice to update for valid payment
    invoice.payment_history.push(payment.clone());
    invoice.balance_outstanding -= payment.payment_amount;
    if invoice.balance_outstanding == zero_decimal {
        invoice.status = "Closed".to_string();
    } else if invoice.balance_outstanding > zero_decimal {
        invoice.status = "Partially Paid".to_string();
    }

    let msg = WasmMsg::Execute {
        contract_addr: invoice_address,
        msg: to_binary(&ExecuteMsg::UpdateInvoice { 
            invoice_id: invoice_id,
            invoice: invoice,
        }).unwrap(),
        //This may be where we send money over!
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
 
    }
}