use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<Addr>,
}

#[cw_serde]
pub enum ExecuteMsg {
    InstantiateContract {
        label: String,
        code_id: u64,
        msg: Binary,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Addr)]
    Admin {},
}
