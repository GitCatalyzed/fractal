use crate::state::{INVOICES};
use crate::msg::{AllInvoicesResponse, OneInvoiceResponse};
use cosmwasm_std::{
    Binary, Deps, StdResult, Order, to_binary,
};

pub fn query_all_invoices(deps:Deps) -> StdResult<Binary>{
   // need to retrieve all values from our storage map
    let invoices = INVOICES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllInvoicesResponse{invoices})

}

pub fn query_one_invoice(deps: Deps, invoice_id: String) -> StdResult<Binary> {

    let invoice = INVOICES.may_load(deps.storage, invoice_id.to_string())?;
    to_binary(&OneInvoiceResponse { invoice })

}