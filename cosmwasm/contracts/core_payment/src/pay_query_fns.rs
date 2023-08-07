use crate::state::{PAYMENTS};
use crate::msg::{PaymentResponse};
use cosmwasm_std::{
    Binary, Deps, Env, StdResult, to_binary,
};

pub fn query_payment(deps: Deps, _env: Env, address: String, invoice_id: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    let payment = PAYMENTS.may_load(deps.storage, (validated_address, invoice_id.to_string()))?;

    to_binary(&PaymentResponse { payment })
}
