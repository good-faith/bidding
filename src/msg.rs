use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    NewOwner { new_owner: Addr },
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(CountResponse)]
    GetCount {},
    #[returns(OwnerResponse)]
    GetOwner {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct CountResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}
