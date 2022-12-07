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
    CloseBidding {},
    Retract { reciever: Option<Addr> },

}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the,
    #[returns(HighestBidResponse)]
    GetHighestBid {},
    #[returns(HighestBidderResponse)]
    GetHighestBidder {},
    #[returns(GetOwnerResponse)]
    GetOwner {},
    #[returns(GetIsBiddingClosedResponse)]
    GetIsBiddingClosed {},
}



// We define a custom struct for each query response
#[cw_serde]
pub struct HighestBidResponse {
    pub highest_bid: Bidder,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct HighestBidderResponse {
    pub highest_bidder: Bidder,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: Addr,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetIsBiddingClosedResponse {
    pub closed_bidding: bool,
}