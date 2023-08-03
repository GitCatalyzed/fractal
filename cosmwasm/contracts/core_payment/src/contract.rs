#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, Order, to_binary, Decimal, Timestamp
};
use cw2::set_contract_version;
use std::str::FromStr;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{config_write, Config, Invoice, Payment};
use crate::execute_fns::create_payment;
use crate::query_fns::query_all_payments;


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:payment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    //Message metadata, contains the sender of the message (Addr) and the funds sent with it a Vec<Coin>.
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg.admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;

    //Payer's USDC address; from address 
    let usdc_address = msg.usdc_address.unwrap_or("_".to_string());
    let validated_usdc_address = deps.api.addr_validate(&usdc_address)?;

    let config = Config{
        admin: validated_admin.clone(),
        business_alias: msg.business_alias,
        usdc_address: validated_usdc_address.clone(),
        bank_routing: msg.bank_routing,
        bank_account: msg.bank_account,
    };

    config_write(deps.storage).save(&config)?;

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string())
        .add_attribute("business_alias", config.business_alias.to_string())
        .add_attribute("usdc_address", validated_usdc_address.to_string())
        .add_attribute("bank_routing", config.bank_routing.to_string())
        .add_attribute("bank_account", config.bank_account.to_string())
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
       
        ExecuteMsg::PayInvoice{
            invoice_id,
            payment_amount,
            pay_unit,
        } => execute_pay_invoice(deps, env, info, invoice_id, payer_alias, payment_amount, pay_unit),
    }
}

fn execute_pay_invoice(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    invoice_id: String,
    payer_alias: String,
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

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Find matched incoming message variant and query them your custom logic
        // and then construct your query response with the type usually defined
        // `msg.rs` alongside with the query message itself.
        //
        // use `cosmwasm_std::to_binary` to serialize query response to json binary.
        
        QueryMsg::Payment{address, invoice_id} => query_payment(deps, env, address, invoice_id),
    }
}


fn query_payment(deps: Deps, _env: Env, address: String, invoice_id: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    let payment = PAYMENTS.may_load(deps.storage, (validated_address, invoice_id.to_string()))?;

    to_binary(&PaymentResponse { payment })
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}

// Previous code omitted
#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, from_binary}; // helper to construct an attribute e.g. ("action", "instantiate")
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info}; // mock functions to mock an environment, message info, dependencies
    use crate::contract::{instantiate, execute, query}; 
    use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, PaymentResponse}; 
    //use cosmwasm_std::Addr;

    // Two fake addresses we will use to mock_info
    pub const ADDR1: &str = "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks";
    pub const ADDR2: &str = "osmo18s5lynnmx37hq4wlrw9gdn68sg2uxp5rgk26vv";

    //TODO: MAKE SURE PAYMENT PAYER MATCHES INVOICE PAYER

    #[test]
    fn test_instantiate() {

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg {admin: None};
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )

    }

    // #[test]
    // fn test_execute_payment_valid(){

    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let info1 = mock_info(ADDR1, &vec![]);
    //     let info2 = mock_info(ADDR2, &vec![]);
    //     //Instantiate the invoice
    //     let msg = InstantiateMsg {admin: None};
    //     let _res = instantiate(deps.as_mut(), env.clone(), info1.clone(), msg).unwrap();

    //     let msg = ExecuteMsg::CreateInvoice{
    //         payer_addr: ADDR2.to_string(),
    //         payer_alias: "Ditobanx".to_string(),
    //         invoice_id: "I00000001".to_string(),
    //         invoiced_value: "1000000.0".to_string(),
    //         date_due: "2023-07-01".to_string(),
    //         pay_unit: "USDC".to_string(),
    //         receipt_unit: "USD".to_string(),
    //     };
    //     //unwrap to assert success
    //     let _res = execute(deps.as_mut(), env.clone(), info1.clone(), msg).unwrap();      
     
        
    //     //New payment in full
    //     let msg = ExecuteMsg::PayInvoice{
    //         invoice_id: "I00000001".to_string(),
    //         payer_alias: "Ditobanx".to_string(),
    //         payment_amount: "1000000.0".to_string(),
    //         pay_unit: "USDC".to_string(),
    //     };

    //     let _res = execute(deps.as_mut(), env.clone(), info2.clone(), msg).unwrap();
   
    // }

    //Have a lot more tests to write but I want to get to deploy

    // #[test]
    // fn test_query_payment(){

    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let info1 = mock_info(ADDR1, &vec![]);
    //     let info2 = mock_info(ADDR2, &vec![]);
    //     //Instantiate the invoice
    //     let msg = InstantiateMsg {admin: None};
    //     let _res = instantiate(deps.as_mut(), env.clone(), info1.clone(), msg).unwrap();

    //     let msg = ExecuteMsg::CreateInvoice{
    //         payer_addr: ADDR2.to_string(),
    //         payer_alias: "Ditobanx".to_string(),
    //         invoice_id: "I00000001".to_string(),
    //         invoiced_value: "1000000.0".to_string(),
    //         date_due: "2023-07-01".to_string(),
    //         pay_unit: "USDC".to_string(),
    //         receipt_unit: "USD".to_string(),
    //     };
    //     //unwrap to assert success
    //     let _res = execute(deps.as_mut(), env.clone(), info1.clone(), msg).unwrap();      
        
    //     //New payment in full
    //     let msg = ExecuteMsg::PayInvoice{
    //         invoice_id: "I00000001".to_string(),
    //         payer_alias: "Ditobanx".to_string(),
    //         payment_amount: "1000000.0".to_string(),
    //         pay_unit: "USDC".to_string(),
    //     };

    //     let _res = execute(deps.as_mut(), env.clone(), info2.clone(), msg).unwrap();

    //     let msg = QueryMsg::Payment{
    //         invoice_id: "I00000001".to_string(),
    //         address: ADDR2.to_string(),
    //     };

    //     let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    //     let res: PaymentResponse = from_binary(&bin).unwrap();

    //     println!("payment: {:?}", res.payment);
    //     assert!(res.payment.is_some());

    //     let msg = QueryMsg::Payment{
    //         invoice_id: "000000111".to_string(),
    //         address: ADDR1.to_string(),
    //     };

    //     let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    //     let res: PaymentResponse = from_binary(&bin).unwrap();

    //     assert!(res.payment.is_none());

    // }

    

}
