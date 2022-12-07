#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, coins};
use cosmwasm_std::{Addr, Coin, Storage, Uint128, BankMsg};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use std::ops::{Sub, Add};
use std::sync::Mutex;
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;
use crate::msg::{HighestBidResponse, HighestBidderResponse, GetOwnerResponse, GetIsBiddingClosedResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{STATE, State, Bid, BIDDER, Bidder};

use cw2::{set_contract_version};
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const COMMISSION : u128 = 10;
const BID_DENOM : &str = "uatom";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        highest_bid: Bidder {
            sender: info.sender,
            total_amount: Bid {
                fund: coins(0, "uatom"),
            },
            transfer_addr: None,
        },
        owner: info.sender,
        closed_bidding: false,
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("highest_bid", state.highest_bid.sender.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bid {} => submit_bid(deps, info),
        ExecuteMsg::CloseBidding {} => close_bidding(deps, info),
        Retract { reciever: Option::<Addr> } => retract_bid(deps, info, reciever),
    }
}

pub fn submit_bid(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
  
    if state.closed_bidding {
        return Err(ContractError::BiddingClosed {});
    }

    if info.sender == state.owner {
        return Err(ContractError::Unauthorized {});
    }


    if state.highest_bid.total_amount.fund.is_empty() {
        let mut current_bidder = info.sender;
        let mut current_bidder = BIDDER.load(deps.storage)?;
        
        state.highest_bid.total_amount.fund = current_bidder.total_amount.fund;
        state.highest_bid.sender = current_bidder.sender;
        state.highest_bid.transfer_addr = None;

        let mut total_amount = state.highest_bid.total_amount.fund;
        let mut commission = total_amount * COMMISSION / 100;
;       let mut amount = total_amount - commission;

        let mut current_bid = Bid {
            fund: coins(amount, BID_DENOM),
        };

        let mut current_bidder = Bidder {
            sender: info.sender,
            total_amount: Bid {
                fund: coins(amount, BID_DENOM),
            },
            transfer_addr: None,
        };
       

        BIDDER.save(deps.storage, &current_bidder)?;
        STATE.save(deps.storage, &state)?;

        let bank_msg = BankMsg::Send {
            to_address: state.owner.to_string(),
            amount: coins(commission.u128(), BID_DENOM),
        };

        return Ok(Response::new().add_attribute("method", "submit_bid"))
    }
    
    if info.sender == state.highest_bid.sender {
        return Err(ContractError::AlreadyHighestBidder{});
    }

    let current_bidder = info.sender;
    let current_bidder = BIDDER.may_load(deps.storage)?;

    current_bidder.iter().map(|bidder| bidder.sender == info.sender).collect();
    let new_bid = Bid {
        fund: info.funds
    };

    let mut commission = new_bid.fund[0].amount * COMMISSION / 100;
    let mut amount = new_bid.fund[0].amount - commission;
    

    let highest_bidder = amount + current_bidder.clone().unwrap().total_amount.fund[0].amount;
    if highest_bidder < state.highest_bid.total_amount.fund[0].amount {
        return Err(ContractError::BidTooLow {});
    }
    

    let mut current_bidder = Bidder {
        sender: info.sender,
        total_amount: Bid {
            fund: highest_bidder
        },
        transfer_addr: None,
    };

    let mut state = State {
        highest_bid: Bidder {
            sender: info.sender,
            total_amount: Bid {
                fund: highest_bidder
            },
            transfer_addr: None,
        },
        owner: info.sender,
        closed_bidding: false,
    };

    BIDDER.save(deps.storage, &current_bidder)?;
    STATE.save(deps.storage, &state)?;

    let bank_msg = BankMsg::Send {
        to_address: state.owner.to_string(),
        amount: coins(commission.u128(), BID_DENOM),
    };

    Ok(Response::new().add_attribute("method", "submit_bid"))
}


pub fn close_bidding(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if state.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    state.closed_bidding = true;

    let bank_msg = BankMsg::Send {
        to_address: state.owner.to_string(),
        amount: coins(, BID_DENOM),
    };

    STATE.save(deps.storage, &state)?;

   
    Ok(Response::new().add_attribute("method", "close_bidding"))
}

pub fn retract_bid(deps: DepsMut, info: MessageInfo, reciever: Option<Addr>) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    
    info.sender {
        return Err(ContractError::Unauthorized {});
    }
    let mut current_bidder = BIDDER.load(deps.storage)?;
    let mut current_bidder = Bidder {
        sender: info.sender,
        total_amount: Bid {
            fund: coins(0, BID_DENOM),
        },
        transfer_addr: None,
    };
    BIDDER.save(deps.storage, &current_bidder)?;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "retract_bid"))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

   
    use crate::{state::{Bidder, Bid}, msg::InstantiateMsg};
    fn bidding_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn submit_bid(){
        let mut app: App<Empty> = AppBuilder::new().with_contract(bidding_contract()).build();
        let owner = "owner";
        let bidder = "bidder";
        
        let owner_addr = Addr::unchecked(owner);
        let bidder_addr = Addr::unchecked(bidder);

        let init_msg = InstantiateMsg {
            higest_bid: Bidder {
                sender: Addr::unchecked("bidder"),
                total_amount: Bid {
                    fund: coins(0, "atom"),
                },
                transfer_addr: None,
            },
        };

        let init_exec = app
            .update_contract(owner, init_msg, &[], &[])
            .unwrap();

        let bid_msg = ExecuteMsg::Bid {
            sender: bidder_addr.clone(),
            total_amount: Bid {
                fund: coins(100, BID_DENOM),
            },
            transfer_addr: None,
        };

        let bid_exec = app
            .update_contract(bidder, bid_msg, &[], &[])
            .unwrap();
        
        let state = STATE.load(app.store()).unwrap();
        assert_eq!(state.highest_bid.sender, bidder_addr);
        assert_eq!(state.highest_bid.total_amount.fund, coins(100, BID_DENOM));
    }
}
