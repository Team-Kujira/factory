#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw_storage_plus::Item;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

static ADMIN: Item<Option<Addr>> = Item::new("admin");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ADMIN.save(deps.storage, &msg.admin)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::InstantiateContract {
            label,
            code_id,
            msg,
        } => {
            let admin = ADMIN.load(deps.storage)?;
            let msg = CosmosMsg::Wasm(WasmMsg::Instantiate {
                admin: admin.map(|a| a.to_string()),
                code_id,
                msg,
                funds: info.funds,
                label,
            });
            Ok(Response::default().add_message(msg))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
