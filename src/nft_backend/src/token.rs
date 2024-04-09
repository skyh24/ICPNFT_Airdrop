use std::collections::HashMap;

use ic_cdk::{caller, trap};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_ledger_types::DEFAULT_SUBACCOUNT;
use serde::Serialize;

use crate::state::StorageStable;
use crate::types::*;

#[derive(CandidType, Deserialize, Serialize)]
pub struct Storage {
    pub manager: HashMap<Principal, String>,
    pub minter: Principal,
    pub registry: HashMap<TokenIndex, AccountIdentifier>,
    pub allowance: HashMap<TokenIndex, Principal>,
    pub token_metadata: HashMap<TokenIndex, Metadata>,
    pub next_token_id: TokenIndex,
    pub next_claim_id: TokenIndex,
    pub reserve: Balance, // init before next claim id
    pub supply: Balance,
    pub supply_claim: Balance,
    pub claimed: HashMap<AccountIdentifier, TokenIndex>,
}

impl From<&StorageStable> for Storage {
    fn from(s: &StorageStable) -> Self {
        Storage {
            manager: s.manager.clone(),
            minter: s.minter.clone(),
            registry: s.registry.clone(),
            allowance: s.allowance.clone(),
            token_metadata: s.token_metadata.clone(),
            next_token_id: s.next_token_id.clone(),
            next_claim_id: s.next_claim_id.clone(),
            supply: s.supply.clone(),
            supply_claim: s.supply_claim.clone(),
            claimed: s.claimed.clone(),
            reserve: 0, // next time 0
        }
    }
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            manager: HashMap::default(),
            minter: Principal::anonymous(),
            registry:HashMap::default(),
            allowance:HashMap::default(),
            token_metadata: HashMap::default(),
            next_token_id: 0,
            next_claim_id: 0, // reserve
            supply: 0,
            supply_claim: 0,
            claimed: HashMap::default(),
            reserve: 0,
        }
    }

    pub fn init_manager(&mut self, caller: Principal) {
        self.manager.insert(caller, "init manager".to_string());
    }

    pub fn add_manager(&mut self, caller: Principal) -> u64 {
        self.manager.insert(caller, "init manager".to_string());
        self.manager.len() as u64
    }

    pub fn is_manager(&self, caller: Principal) -> bool {
        self.manager.contains_key(&caller)
    }

    pub fn mint(&mut self, _to: User, metadata: Option<Vec<u8>>) -> TokenIndex {
        // let receiver = to_aid(to);
        let token_id = self.next_token_id;
        let md = Metadata::NonFungible {metadata};
        self.registry.insert(token_id, "AstroX".to_string());
        self.token_metadata.insert(token_id, md);
        self.supply += 1;
        self.next_token_id += 1;
        token_id
    }

    pub fn is_claimable(&self, principal: Principal) -> bool {
        let receiver = from_principal(principal);
        !self.claimed.contains_key(&receiver)
    }

    pub fn claim(&mut self, principal: Principal) -> TokenIndex {
        let receiver = from_principal(principal);
        let token_id = self.next_claim_id;
        if token_id >= self.supply_claim {
            trap("exceed claim supply")
        }
        if  self.claimed.contains_key(&receiver) {
            trap("user already claimed")
        }

        self.registry.insert(token_id.clone(), receiver.clone());
        self.claimed.insert(receiver.clone(), token_id.clone());
        self.next_claim_id += 1;
        token_id
    }

    pub fn force_claim(&mut self, to: User, token_id: TokenIndex) -> TokenIndex {
        let receiver = to_aid(to);
        if token_id >= self.reserve {
            trap("exceed claim reserve")
        }

        match self.registry.get(&token_id) {
            Some(account_id) => {
                if account_id.clone().len() < 18 {
                    self.registry.insert(token_id, receiver);
                } else {
                    trap(format!("account_id {} exist", account_id).as_str())
                }
            },
            None => trap(format!("invalid token_id {}", token_id).as_str())
        }

        token_id
    }

    pub fn reserves(&self) -> Vec<TokenIndex> {
        self.registry.iter().filter_map(|(key, val)| {
            if key.clone() < self.reserve && val.clone() == "AstroX".to_string() {
                Some(key.clone())
            } else {
                None
            }
        }).collect()
    }

    // amount = 1
    // subaccount = DEFAULTs
    pub fn transfer_from(&mut self, tid: TokenIdentifier, from: User, to: User) -> TransferResponse {
        if !is_principal(tid.clone(), ic_cdk::id()) {
            return Err(TransferError::InvalidToken(tid.clone()))
        }

        let token = get_index(tid.clone());
        let spender = from_principal(caller());
        let owner = to_aid(from);
        let receiver = to_aid(to);

        match self.registry.get(&token) {
            Some(token_owner) => {
                if owner != token_owner.clone() {
                    return Err(TransferError::Unauthorized(owner))
                }
                if owner != spender {
                    match self.allowance.get(&token) {
                        Some(token_spender) => {
                            if caller() != token_spender.clone() {
                                return Err(TransferError::Unauthorized(spender))
                            }
                        },
                        None => return Err(TransferError::Unauthorized(spender))
                    }
                }
                self.allowance.remove(&token);
                self.registry.insert(token, receiver);
                Ok(1)
            },
            None => Err(TransferError::InvalidToken(tid))
        }
    }

    pub fn approve(&mut self, tid: TokenIdentifier, spender: Principal) -> bool {
        if !is_principal(tid.clone(), ic_cdk::id()) {
            return false
        }

        let token = get_index(tid);
        let owner = from_principal(caller());

        match self.registry.get(&token) {
            Some(token_owner) => {
                if owner != token_owner.clone() {
                    return false
                }
                self.allowance.insert(token, spender);
                true
            },
            None => false
        }
    }

    pub fn allowance(&self, tid: TokenIdentifier, from: User, spender: Principal) -> Result<Balance, CommonError> {
        if !is_principal(tid.clone(), ic_cdk::id()) {
            return Err(CommonError::InvalidToken(tid))
        }

        let token = get_index(tid.clone());
        let owner = to_aid(from);

        match self.registry.get(&token) {
            Some(token_owner) => {
                if owner != token_owner.clone() {
                    return Err(CommonError::Other("Invalid owner".to_string()))
                }
                match self.allowance.get(&token) {
                    Some(token_spender) => {
                        if spender == token_spender.clone() {
                            Ok(1)
                        } else {
                            Ok(0)
                        }
                    },
                    None => Ok(0)
                }
            },
            None => Err(CommonError::InvalidToken(tid))
        }
    }

    pub fn balance(&self, tid: TokenIdentifier, user: User) -> BalanceResponse {
        if !is_principal(tid.clone(), ic_cdk::id()) {
            return Err(CommonError::InvalidToken(tid))
        }

        let token = get_index(tid.clone());
        let aid = to_aid(user);

        match self.registry.get(&token) {
            Some(token_owner) => {
                if aid == token_owner.clone() {
                    Ok(1)
                } else {
                    Ok(0)
                }
            },
            None => Err(CommonError::InvalidToken(tid))
        }
    }

    pub fn owner_of(&self, tid: TokenIdentifier) -> Result<AccountIdentifier, CommonError> {
        if !is_principal(tid.clone(), ic_cdk::id()) {
            return Err(CommonError::InvalidToken(tid))
        }

        let token = get_index(tid.clone());
        match self.registry.get(&token) {
            Some(token_owner) => {
                Ok(token_owner.clone())
            },
            None => return Err(CommonError::InvalidToken(tid))
        }
    }

    pub fn metadata(&self, tid: TokenIdentifier) -> Result<Metadata, CommonError> {
        if !is_principal(tid.clone(), ic_cdk::id()) {
            return Err(CommonError::InvalidToken(tid))
        }

        let token = get_index(tid.clone());
        match self.token_metadata.get(&token) {
            Some(metadata) => Ok(metadata.clone()),
            None => return Err(CommonError::InvalidToken(tid))
        }
    }
}

pub fn to_aid(user: User) -> AccountIdentifier {
    match user {
        User::Principal(principal) => from_principal(principal),
        User::Address(address) => address
    }
}

pub fn from_principal(principal: Principal) -> AccountIdentifier {
    ic_ledger_types::AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT).to_string()
}

pub fn is_principal(tid: TokenIdentifier, p: Principal) -> bool {
    let tobj = decode_token_id(tid).unwrap();
    tobj.canister == p.as_slice()
}

pub fn get_index(tid: TokenIdentifier) -> TokenIndex {
    let tobj = decode_token_id(tid).unwrap();
    tobj.index
}

const TOKEN_ID_PREFIX: [u8; 4] = [10, 116, 105, 100];
const CANISTER_ID_HASH_LEN_IN_BYTES: usize = 10;

pub fn decode_token_id(tid: TokenIdentifier) -> Result<TokenObj, String> {
    let principal_parse_res = Principal::from_text(tid);
    match principal_parse_res {
        Ok(principal) => {
            let bytes = principal.as_slice();
            if !bytes.starts_with(&TOKEN_ID_PREFIX) {
                return Ok(TokenObj {
                    index: 0,
                    canister: bytes.into(),
                });
            }
            let canister: Vec<u8> = bytes[4..(4 + CANISTER_ID_HASH_LEN_IN_BYTES)].to_vec();
            let mut token_index: [u8; 4] = Default::default();
            token_index.copy_from_slice(&bytes[14..]);

            return Ok(TokenObj {
                index: u32::from_be_bytes(token_index),
                canister,
            });
        }
        Err(_) => Err("invalid token id".to_string()),
    }
}

pub fn encode_token_id(canister_id: Principal, token_index: TokenIndex) -> TokenIdentifier {
    let canister_blob: &[u8] = canister_id.as_slice();
    let mut data: [u8;18] = [10, 116, 105, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut count = 4;
    for b in canister_blob{
        data[count] = *b;
        count += 1;
    }
    let id_blob = token_index.to_be_bytes();
    for b in &id_blob{
        data[count] = *b;
        count += 1;
    }
    ic_cdk::println!("{:?}",data);
    Principal::from_slice(&data).to_text()
}
