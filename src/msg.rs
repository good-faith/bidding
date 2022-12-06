use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;

use crate::state::{Bidder};

#[cw_serde]
pub struct InstantiateMsg {
    pub higest_bid: Bidder,
}

#[cw_serde]
pub enum ExecuteMsg {
    Bid {},
    AddTranserAddr {},
    Retract { new_owner: Addr },
    CloseBidding { count: i32 },

}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(HighestBidResponse)]
    GetHighestBid {},
    #[returns(HighestBidderResponse)]
    GetHighestBidder {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct HighestBidResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct HighestBidderResponse {
    pub owner: Addr,
}
