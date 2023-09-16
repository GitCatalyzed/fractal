use crate::state::{PAYMENTS};
use crate::msg::{PaymentResponse, AllPaymentsResponse};
use cosmwasm_std::{
    Binary, Deps, Env, StdResult, to_binary, Order
};

pub fn query_payment(deps: Deps, _env: Env, payment_id: String) -> StdResult<Binary> {
    //let validated_address = deps.api.addr_validate(&address).unwrap();
    let payments = PAYMENTS.may_load(deps.storage, payment_id.to_string())?;

    to_binary(&PaymentResponse { payments })
}

pub fn query_all_payments(deps:Deps) -> StdResult<Binary>{
    // need to retrieve all values from our storage map
     let payments = PAYMENTS
         .range(deps.storage, None, None, Order::Ascending)
         .map(|p| Ok(p?.1))
         .collect::<StdResult<Vec<_>>>()?;
 
     to_binary(&AllPaymentsResponse{payments})
 
 }