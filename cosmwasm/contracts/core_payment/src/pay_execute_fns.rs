use fractal_support::error::ContractError;
use crate::state::{
    config_read, PAYMENTS
};

use fractal_support::core::{Payment, conversion, payment_route};
use fractal_support::route_payment::{Route, payment_route_direct_onchain};
use core_invoice::msg::{ExecuteMsg, QueryMsg, OneInvoiceResponse};

use cosmwasm_std::{
    QueryRequest, WasmMsg, DepsMut, Env, MessageInfo, Response, to_binary, Decimal, WasmQuery, BankMsg
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
        contract_addr: invoice_address.clone(),
        msg: to_binary(&QueryMsg::OneInvoice {invoice_id: invoice_id.clone()})?,
    };
    let res: OneInvoiceResponse = deps.querier.query(&QueryRequest::Wasm(query))?;
    let unvalidated_invoice = res.invoice;
    if unvalidated_invoice.is_none(){
        return Err(ContractError::InvalidInvoice{});
    }

    let mut validated_invoice = unvalidated_invoice.unwrap();

    //Tap into Config to get the information about payer, ensure payer is admin
    let payer = config_read(deps.storage).load()?;
    if payer.admin.to_string() != info.sender.to_string(){
        return Err(ContractError::Unauthorized{})
    }

    let payment_amount = Decimal::from_str(&payment_amount).unwrap();
    let zero_decimal = Decimal::from_str("0.0").unwrap();

    //Validate payer against invoice expectation and ensure address and payment is valid
    if validated_invoice.payer_addr.to_string() != info.sender.to_string(){
        return Err(ContractError::IncorrectPayer{})
    }
    if payment_amount <= zero_decimal {
        return Err(ContractError::InvalidPaymentValueNegative{})
    }
    if payment_amount > validated_invoice.balance_outstanding{
        return Err(ContractError::InvalidPaymentValueAboveBalance{})
    }
    
    let validated_contract_address = deps.api.addr_validate(&invoice_address)?;

    let current_payment = validated_invoice.payment_history.len()+1;

    let payment_id = format!("{}_P{}", invoice_id, current_payment);

    let today = env.block.time;
    //let formatted_date = today.format("%Y-%m-%d").to_string();

    //Accept the payment as valid to this point
    let mut payment = Payment{
        payment_id,
        payer_addr: payer.admin,
        payer_alias: payer.business_alias,
        invoice_id: invoice_id.clone(),
        invoice_address: validated_contract_address.clone(),
        payment_amount,
        pay_unit: pay_unit.clone(),
        pay_date: today,
        send_unit: String::new(),
        settlement_status: "payment initiated".to_string(),
    };
    
    //BRIDGE CONVERSION ZONE
    if pay_unit != validated_invoice.receipt_unit {
        //Bridge conversion pair routes
        match conversion(&pay_unit, &validated_invoice.receipt_unit).as_str(){
            "usdc_usd" => unimplemented!(),
            "usd_usdc" => unimplemented!(),
            "onchain_onchain" => unimplemented!(),
            &_ => unimplemented!(),
        }

    } else if pay_unit == validated_invoice.receipt_unit {

        payment.send_unit = payment.pay_unit.clone();

    } else {
        return Err(ContractError::InvalidPaymentPayUnit{})
    }

    //ROUTING OF ACTUAL VALUE HAPPENS HERE; WILL NEED TO BE AWARE OF CCF
    let bank_msg: BankMsg;
    if payment.send_unit != validated_invoice.receipt_unit {
        return Err(ContractError::ConversionFailure{})
    } else {

        match payment_route(payment.send_unit.clone(),  validated_invoice.receipt_unit.clone()) {
            //USDC-USDC, OSMO-ATOM, Etc.
            Route::DirectOnchain => {
                let bank_msg_response = payment_route_direct_onchain(
                    payment.send_unit.clone(), 
                    payment.payment_amount.clone(), 
                    payment.invoice_address.clone(), 
                    info.clone())?;
                bank_msg = bank_msg_response;
                //Update Settlement Status
                payment.settlement_status = "settled".to_string();                
            },
            
            //IBC/ETH/BTC
            Route::CrossChain=> unimplemented!(),
            //Fiat
            Route::CircleRoute => unimplemented!(),

            //_ => return Err(ContractError::FailedRouting{}),
        }

    }

    PAYMENTS.save(deps.storage, (info.sender.clone(), invoice_id.clone()), &payment)?;

    validated_invoice.payment_history.push(payment.clone());
    validated_invoice.balance_outstanding -= payment.payment_amount;
    if validated_invoice.balance_outstanding == zero_decimal {
        validated_invoice.status = "Closed".to_string();
    } else if validated_invoice.balance_outstanding > zero_decimal {
        validated_invoice.status = "Partially Paid".to_string();
    }

    let msg = WasmMsg::Execute {
        contract_addr: validated_contract_address.clone().to_string(),
        msg: to_binary(&ExecuteMsg::UpdateInvoice { 
            invoice_id: invoice_id,
            invoice: validated_invoice,
        }).unwrap(),
        //This may be where we send money over!
        funds: vec![],
    };

    // let denom = pay_unit.clone();
    // let onchain_payment = cw_utils::must_pay(&info, &denom)?.u128();

    //Because this is a u128, need to get creative to check payment_amount = funds sent
    // if onchain_payment.to_string != payment_amount {
    //     return Err(ContractError::InvalidPaymentValue{})
    // }
    // let bank_message = BankMsg::Send{
    //     to_address: validated_contract_address.clone().to_string(),
    //     amount: coins(onchain_payment, &denom),
    // };

    //I removed the second bank message in the response for now, lets see if the routing function handled it
    let resp = Response::new().add_message(msg).add_message(bank_msg);
    Ok(resp)
 
}