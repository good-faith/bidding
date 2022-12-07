use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Already Owner")]
    AlreadyOwner {},

    #[error("Address Invalid")]
    AddressInvalid {},

    #[error("Bid Insufficient")]
    BidInsufficient {},

    #[error("Invalid Amount")]
    InvalidAmount {},

    #[error("Invalid Denom")]
    InvalidDenom {},

    #[error("Bid Too Low")]
    BidTooLow {},

    #[error("Bidding Closed")]
    BiddingClosed {},

    #[error("Already Highest Bidder")]
    AlreadyHighestBidder {},


    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
