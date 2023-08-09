#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{config_write, Config};
use crate::execute_fns::{create_invoice, update_invoice};
use crate::query_fns::{query_all_invoices, query_one_invoice};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:invoice";
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

    //Payee's USDC address; to address
    let usdc_address = msg.usdc_address.unwrap_or("_".to_string());
    //let validated_usdc_address = deps.api.addr_validate(&usdc_address)?;

    let config = Config{
        admin: validated_admin.clone(),
        business_alias: msg.business_alias,
        usdc_address: usdc_address.clone(),
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
        .add_attribute("usdc_address", usdc_address.to_string())
        .add_attribute("bank_routing", config.bank_routing.to_string())
        .add_attribute("bank_account", config.bank_account.to_string())
    )
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

        ExecuteMsg::CreateInvoice{
            payer_addr,
            payer_alias,
            invoice_id,
            invoiced_value,
            date_due,
            pay_unit,
            receipt_unit,
        } => create_invoice(deps, env, info, payer_addr, payer_alias, invoice_id, invoiced_value, date_due, pay_unit,receipt_unit),

        ExecuteMsg::UpdateInvoice{
            invoice_id,
            invoice,
        } => update_invoice(deps, env, invoice_id, invoice),
        
        //ExecuteMsg::DeleteInvoice{}

        //ExecuteMsg::RefundPayment{}

    }
}





/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, 
    _env: Env, 
    msg: QueryMsg
) -> StdResult<Binary> {

    match msg {

        QueryMsg::AllInvoices{} => query_all_invoices(deps),

        QueryMsg::OneInvoice{invoice_id} => query_one_invoice(deps, invoice_id),

    }
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
    use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, AllInvoicessResponse, InvoiceResponse};//, PaymentResponse}; 
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

    #[test]
    fn test_execute_create_invoice_valid(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        //Instantiate the invoice
        let msg = InstantiateMsg {admin: None};
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //new execute message
        let msg = ExecuteMsg::CreateInvoice{
            payer_addr: ADDR2.to_string(),
            payer_alias: "Ditobanx".to_string(),
            invoice_id: "I00000001".to_string(),
            invoiced_value: "1000000.0".to_string(),
            date_due: "2023-07-01".to_string(),
            pay_unit: "USDC".to_string(),
            receipt_unit: "USD".to_string(),
        };
        //unwrap to assert success
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();

    }

    #[test]
    fn test_execute_create_invoice_invalid(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        //Instantiate the invoice
        let msg = InstantiateMsg {admin: None};
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        let msg = ExecuteMsg::CreateInvoice{
            payer_addr: ADDR2.to_string(),
            payer_alias: "Ditobanx".to_string(),
            invoice_id: "I00000001".to_string(),
            invoiced_value: "-1000000.0".to_string(),
            date_due: "2023-07-01".to_string(),
            pay_unit: "USDC".to_string(),
            receipt_unit: "USD".to_string(),
        };
        //unwrap to assert success
        let _res = execute(deps.as_mut(), env, info, msg).unwrap_err();

    }



    //Have a lot more tests to write but I want to get to deploy
    #[test]
    fn test_query_all_invoices(){

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        //Instantiate the invoice
        let msg = InstantiateMsg {admin: None};
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //Test Invoice #1
        let msg = ExecuteMsg::CreateInvoice{
            payer_addr: ADDR2.to_string(),
            payer_alias: "Ditobanx".to_string(),
            invoice_id: "I00000001".to_string(),
            invoiced_value: "1000000.0".to_string(),
            date_due: "2023-07-01".to_string(),
            pay_unit: "USDC".to_string(),
            receipt_unit: "USD".to_string(),
        };
        //unwrap to assert success
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //Test Invoice #2
        let msg = ExecuteMsg::CreateInvoice{
            payer_addr: ADDR2.to_string(),
            payer_alias: "Ditobanx".to_string(),
            invoice_id: "I00000002".to_string(),
            invoiced_value: "1000000.0".to_string(),
            date_due: "2023-09-01".to_string(),
            pay_unit: "USDC".to_string(),
            receipt_unit: "USD".to_string(),
        };
        //unwrap to assert success
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let msg = QueryMsg::AllInvoices {};
        //use as_ref because queries cannot change the contract state
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: AllInvoicessResponse = from_binary(&bin).unwrap();

        println!("Invoices: {:?}", res.invoices);
        assert_eq!(res.invoices.len(),2);


    }

    #[test]
    fn test_query_invoice(){

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        //Instantiate the invoice
        let msg = InstantiateMsg {admin: None};
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //Test Invoice #1
        let msg = ExecuteMsg::CreateInvoice{
            payer_addr: ADDR2.to_string(),
            payer_alias: "Ditobanx".to_string(),
            invoice_id: "I00000001".to_string(),
            invoiced_value: "1000000.0".to_string(),
            date_due: "2023-07-01".to_string(),
            pay_unit: "USDC".to_string(),
            receipt_unit: "USD".to_string(),
        };
        //unwrap to assert success
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = QueryMsg::Invoice{
            invoice_id: "I00000001".to_string(),
        };

        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: InvoiceResponse = from_binary(&bin).unwrap();

        assert!(res.invoice.is_some());

        let msg = QueryMsg::Invoice{
            invoice_id: "00000111".to_string(),
        };

        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: InvoiceResponse = from_binary(&bin).unwrap();

        assert!(res.invoice.is_none());
    }

}
