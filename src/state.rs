use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item};

// The struct for a bid
#[cw_serde]
pub struct Bid {
    pub fund: Vec<Coin>,
}

// The struct for a bidder
#[cw_serde]
pub struct Bidder {
    pub sender: Addr,
    pub total_amount: Bid,
    pub transfer_addr: Option<Addr>,
}

// The state of the contract
pub const STATE: Item<State> = Item::new("state");
pub const BIDDER: Item<Bidder> = Item::new("bidders");

#[cw_serde]
pub struct State {
    // The current highest bid
    pub highest_bid: Bidder,

    // The address of the contract owner
    pub owner: Addr,

    // closes the bidding
    pub closed_bidding: bool,
}

