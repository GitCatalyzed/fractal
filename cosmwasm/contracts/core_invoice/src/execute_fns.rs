use crate::error::ContractError;
use crate::state::{Invoice, config_write, Payment, INVOICES};
use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, Decimal, Timestamp
};
use std::str::FromStr;


pub fn create_invoice(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    payer_addr: String,
    payer_alias: String,
    invoice_id: String,
    invoiced_value: String,
    date_due: String, 
    pay_unit: String,
    receipt_unit: String,
) -> Result<Response, ContractError>{
    
    //Ensure the invoice is created by owner of contract
    let mut config = config_write(deps.storage).load()?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized{});
    }

    let decimal_invoiced_value = Decimal::from_str(&invoiced_value).unwrap();
    let zero_decimal = Decimal::from_str("0.0").unwrap();

    //1. Ensure Valid Invoice Value
    if decimal_invoiced_value <= zero_decimal {
        return Err(ContractError::NoInvoiceValue{});
    }
    
    //2. Accept payer_addr as a String from msg, and convert to Addr and validate
    //IF YOU GET AN ERROR HERE YOU MAY NEED TO UNWRAP THE MESSAGE
    let unvalidated_payer = payer_addr;
    let validated_payer = deps.api.addr_validate(&unvalidated_payer)?;

    //3. Accept date_due as a String and convert to NaiveDate
    // let format_date = "%Y-%m-%d";
    // let parsed_date = match NaiveDate::parse_from_str(&date_due, format_date) {
    //     Ok(date) => date,
    //     Err(_err) => return Err(ContractError::InvalidDate{}),
    // };

    let payment_history: Vec<Payment> = vec![];
    let balance_outstanding = Decimal::from_str(&invoiced_value).unwrap();

    let invoice = Invoice {
        creator: info.sender,
        payer_addr: validated_payer,
        payer_alias,
        invoice_id: invoice_id.clone(),
        invoiced_value: decimal_invoiced_value,
        balance_outstanding,
        date_due,
        status: "Open".to_string(),
        pay_unit,
        receipt_unit,
        payment_history,
        tokenized_status: "0".to_string(),
    };
    
    INVOICES.save(deps.storage, invoice_id, &invoice)?;

    Ok(Response::new()
        .add_attribute("action", "create_invoice"))
}

//Probably just need to pass invoice. Mark for Cleanup
pub fn update_invoice (
    deps: DepsMut,
    _env: Env,
    invoice_id: String,
    invoice: Invoice,
) -> Result <Response, ContractError> {

    INVOICES.save(deps.storage, invoice_id, &invoice)?;
    Ok(Response::new()
        .add_attribute("action", "update_invoice"))

}