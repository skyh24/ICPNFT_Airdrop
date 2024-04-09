use candid::{candid_method, Principal};
use ic_cdk::*;

use crate::service::{manager_guard, Service};
use crate::state::STORAGE;
use crate::token::{encode_token_id, from_principal};
use crate::types::*;

#[init]
#[candid_method(init)]
fn init(manager: Principal) {
    STORAGE.with(|ext| {
        let mut ext_borrow = ext.borrow_mut();
        ext_borrow.init_manager(manager);
    });
}

// ## Manager
#[update(name = "add_manager", guard = "manager_guard")]
#[candid_method(update, rename = "add_manager")]
fn add_manager(manager: Principal) -> u64 {
    STORAGE.with(|ext| {
        ext.borrow_mut().add_manager(manager)
    })
}

#[query(name = "is_manager")]
#[candid_method(query, rename = "is_manager")]
fn is_manager(principal: Principal) -> bool {
    STORAGE.with(|ext| {
        ext.borrow().is_manager(principal)
    })
}

// ## Minter
#[update(name = "setMinter", guard = "manager_guard")]
#[candid_method(update, rename = "setMinter")]
fn set_minter(minter: Principal)  {
    STORAGE.with(|ext| {
        ext.borrow_mut().minter = minter;
    });
}

#[query(name = "getMinter")]
#[candid_method(query, rename = "getMinter")]
fn get_minter() -> Principal{
    STORAGE.with(|ext| {
        ext.borrow().minter.clone()
    })
}

// ## Reserve
#[update(name = "init_reserve", guard = "manager_guard")]
#[candid_method(update, rename = "init_reserve")]
fn init_reserve(rsv: Balance)  {
    STORAGE.with(|ext| {
        let mut ext_borrow = ext.borrow_mut();
        if rsv < ext_borrow.supply && ext_borrow.next_claim_id < 2 { // in case test claimed
            ext_borrow.reserve = rsv;
            ext_borrow.next_claim_id = rsv; // free claim id
        } else {
            trap(format!("init_reserve fail, claimed too big: {}", ext_borrow.next_claim_id).as_str())
        }
    });
}

#[update(name = "set_claim_supply", guard = "manager_guard")]
#[candid_method(update, rename = "set_claim_supply")]
fn set_claim_supply(claim: Balance)  {
    STORAGE.with(|ext| {
        let mut ext_borrow = ext.borrow_mut();
        if claim == 0 {
            ext_borrow.supply_claim = ext_borrow.supply;
            return
        }

        if claim <= ext_borrow.reserve {
            trap(format!("claim {} below reserve {}", claim, ext_borrow.reserve).as_str());
        }

        if claim > ext_borrow.supply {
            trap(format!("claim {} exceed supply {}", claim, ext_borrow.supply).as_str());
        }

        ext_borrow.supply_claim = claim;

    });
}

// ## Mint && claim
#[update(name = "mintNFT", guard = "manager_guard")]
#[candid_method(update, rename = "mintNFT")]
fn mint_nft(req: MintRequest) -> TokenIndex {
    Service::mint(req)
}

#[update(name = "batchMintNFT", guard = "manager_guard")]
#[candid_method(update, rename = "batchMintNFT")]
fn batch_mint_nft(reqs: Vec<MintRequest>) -> Vec<TokenIndex> {
    let mut res = Vec::new();
    for req in reqs {
        let token_index = Service::mint(req);
        res.push(token_index)
    }
    res
}

#[update(name = "is_claimable")]
#[candid_method(update, rename = "is_claimable")]
fn is_claimable(principal: Principal) -> bool {
    Service::is_claimable(principal)
}

#[update(name = "claimNFT", guard = "manager_guard")]
#[candid_method(update, rename = "claimNFT")]
fn claim_nft(principal: Principal) -> TokenIndex {
    Service::claim(principal)
}

#[update(name = "force_claim_reserve", guard = "manager_guard")]
#[candid_method(update, rename = "force_claim_reserve")]
fn force_claim_reserve(req: ClaimRequest) -> TokenIndex {
    Service::claim_reserve(req)
}

#[update(name = "transfer")]
#[candid_method(update, rename = "transfer")]
fn transfer(req: TransferRequest) -> TransferResponse {
    Service::transfer_from(req)
}

#[update(name = "approve")]
#[candid_method(update, rename = "approve")]
fn approve(req: ApproveRequest) -> bool {
    Service::approve(req)
}

#[update(name = "approveAll")]
#[candid_method(update, rename = "approveAll")]
fn approve_all(reqs: Vec<ApproveRequest>) -> Vec<TokenIndex> {
    let mut resp =  Vec::new();
    for req in reqs {
        let success = Service::approve(req.clone());
        if success {
            let token_index = req.token.parse::<u32>().unwrap();
            resp.push(token_index as TokenIndex)
        }
    }
    resp
}

#[query(name = "allowance")]
#[candid_method(query, rename = "allowance")]
fn allowance(req: AllowanceRequest) -> Result<Balance, CommonError> {
    Service::allowance(req)
}

#[query(name = "balance")]
#[candid_method(query, rename = "balance")]
fn balance(req: BalanceRequest) -> BalanceResponse {
    Service::balance(req)
}

#[query(name = "supply")]
#[candid_method(query, rename = "supply")]
fn supply(_token_identifier: TokenIdentifier) -> Result<Balance, CommonError> {
    STORAGE.with(|ext| {
        Ok(ext.borrow().supply.clone())
    })
}

#[query(name = "bearer")]
#[candid_method(query, rename = "bearer")]
fn bearer(token_identifier: TokenIdentifier) -> Result<AccountIdentifier, CommonError> {
    Service::owner_of(token_identifier)
}

#[query(name = "metadata")]
#[candid_method(query, rename = "metadata")]
fn metadata(token_identifier: TokenIdentifier) -> Result<Metadata, CommonError> {
    Service::metadata(token_identifier)
}

#[query(name = "extensions")]
#[candid_method(query, rename = "extensions")]
fn extensions() -> Vec<Extension> {
    vec![
        "@ext/common".to_string(),
        "@ext/allowance".to_string(),
        "@ext/nonfungible".to_string()
    ]
}

#[query(name = "getTokensByIds")]
#[candid_method(query, rename = "getTokensByIds")]
fn get_tokens_by_ids(token_ids: Vec<TokenIndex>) -> Vec<(TokenIndex, Metadata)> {
    let mut res = Vec::new();
    STORAGE.with(|ext| {
        let ext_borrow  = ext.borrow();
        for token_id in token_ids {
            match ext_borrow.token_metadata.get(&token_id) {
                Some(metadata) => res.push((token_id, metadata.clone())),

                None => {}
            }
        }
    });
    res
}

#[query(name = "tokens")]
#[candid_method(query, rename = "tokens")]
fn tokens(owner: AccountIdentifier) -> Result<Vec<TokenIndex>, CommonError> {
    STORAGE.with(|ext| {
        let result = ext.borrow().registry.iter().filter_map(|(key, val)| {
            if val.clone() == owner {
                Some(key.clone())
            } else {
                None
            }
        }).collect();
        Ok(result)
    })
}

#[query(name = "getRegistry")]
#[candid_method(query, rename = "getRegistry")]
fn get_registry() -> Vec<(TokenIndex, AccountIdentifier)> {
    STORAGE.with(|ext| {
        ext.borrow().registry.clone().into_iter().collect()
    })
}

#[query(name = "getAllowances")]
#[candid_method(query, rename = "getAllowances")]
fn get_allowances() -> Vec<(TokenIndex, Principal)> {
    STORAGE.with(|ext| {
        ext.borrow().allowance.clone().into_iter().collect()
    })
}

#[query(name = "getTokens")]
#[candid_method(query, rename = "getTokens")]
fn get_tokens(page: u32) -> Vec<(TokenIndex, Metadata)> {
    let result = STORAGE.with(|ext| {
        ext.borrow().token_metadata.iter().filter_map(|(key, val)| {
            if key.clone() >= 10 * page && key.clone() < 10 * (page + 1) {
                Some((key.clone(), val.clone()))
            } else {
                None
            }
        }).collect()
    });
    result
}

#[query(name = "next_claim_id")]
#[candid_method(query, rename = "next_claim_id")]
fn next_claim_id() -> Result<Balance, CommonError> {
    STORAGE.with(|ext| {
        Ok(ext.borrow().next_claim_id.clone())
    })
}

#[query(name = "claim_supply")]
#[candid_method(query, rename = "claim_supply")]
fn claim_supply() -> Result<Balance, CommonError> {
    STORAGE.with(|ext| {
        Ok(ext.borrow().supply_claim.clone())
    })
}

#[query(name = "pop_status")]
#[candid_method(query, rename = "pop_status")]
pub fn pop_status() -> Info {
    STORAGE.with(|ext| {
        let ext_borrow = ext.borrow();
        Info {
            reserve: ext_borrow.reserve.clone(),
            claimed: ext_borrow.next_claim_id.clone(),
            available: ext_borrow.supply_claim.clone(),
            supply: ext_borrow.supply.clone(),
        }
    })
}

#[query(name = "reserve_tokens")]
#[candid_method(query, rename = "reserve_tokens")]
fn reserve_tokens() -> Vec<TokenIndex> {
    STORAGE.with(|ext| {
        ext.borrow().reserves()
    })
}

#[query(name = "account_id")]
#[candid_method(query, rename = "account_id")]
fn account_id(principal: Principal) -> AccountIdentifier {
    from_principal(principal)
}

#[query(name = "token_id")]
#[candid_method(query, rename = "token_id")]
fn token_id(token_index: TokenIndex) -> TokenIdentifier {
    encode_token_id(ic_cdk::id(), token_index)
}

#[query(name = "decode_id", guard = "manager_guard")]
#[candid_method(query, rename = "decode_id")]
fn decode_id(tid: TokenIdentifier) -> Result<TokenObj, String> {
    crate::token::decode_token_id(tid)
}

#[query(name = "canister_id", guard = "manager_guard")]
#[candid_method(query, rename = "canister_id")]
fn canister_id() -> Principal {
    ic_cdk::id()
}

#[query(name = "is_principal", guard = "manager_guard")]
#[candid_method(query, rename = "is_principal")]
fn is_principal(tid: TokenIdentifier) -> bool {
    crate::token::is_principal(tid, ic_cdk::id())
}


#[query(name = "test", guard = "manager_guard")]
#[candid_method(query, rename = "test")]
fn test() -> User {
    User::Address("0".to_string())
}

// #[update(name = "acceptCycles")]
// #[candid_method(update, rename = "acceptCycles")]
// fn acceptCycles() {
// }
//
// #[update(name = "availableCycles")]
// #[candid_method(update, rename = "availableCycles")]
// fn available_cycles() -> Nat{
//     api::canister_balance().into()
// }
//
// ### no list
// #[update(name = "tokens_ext")]
// #[candid_method(update, rename = "tokens_ext")]
// fn tokens_ext(owner: Principal) -> Result<
//     (TokenIndex, Option<Listing>, Option<Vec<u8>>), CommonError> {
// }

//
// #[update(name = "http_request")]
// #[candid_method(update, rename = "http_request")]
// fn http_request(request: HttpRequest) -> HttpResponse {
// }

// *************************************************************************************************
//
// // ### no property
// #[update(name = "initproperties")]
// #[candid_method(update, rename = "initproperties")]
// fn initproperties() {
// }
//
// #[update(name = "getMinter")]
// #[candid_method(update, rename = "getMinter")]
// fn getProperties() -> Vec<String, Vec<(String, Nat)>> {
// }
//
// #[update(name = "getTokensByProperties")]
// #[candid_method(update, rename = "getTokensByProperties")]
// fn getTokensByProperties(properties: Vec<(String, Vec<String>)>) -> Vec<(TokenIndex, AccountIdentifier)> {
// }