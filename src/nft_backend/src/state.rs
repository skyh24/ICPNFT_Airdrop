use std::cell::RefCell;
use std::collections::HashMap;

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk::*;

use crate::token::Storage;
use crate::types::*;

thread_local! {
    pub(crate) static STORAGE: RefCell<Storage> = RefCell::new(Storage::new());
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub(crate) struct StorageStable {
    pub manager: HashMap<Principal, String>,
    pub minter: Principal,
    pub registry: HashMap<TokenIndex, AccountIdentifier>,
    pub allowance: HashMap<TokenIndex, Principal>,
    pub token_metadata: HashMap<TokenIndex, Metadata>,
    pub next_token_id: TokenIndex,
    pub next_claim_id: TokenIndex,
    pub supply: Balance,
    pub supply_claim: Balance,
    pub claimed: HashMap<AccountIdentifier, TokenIndex>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    match storage::stable_save({
        STORAGE.with(|storage| {
            let s = storage.borrow();
            (StorageStable {
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
            },)
        })
    }) {
        Ok(_) => {
            ()
        }
        Err(e) => trap(format!("Failed to save state before upgrade: {:?}", e).as_str()),
    };
}

#[post_upgrade]
fn post_upgrade() {
    match storage::stable_restore::<(StorageStable,)>() {
        Ok(map_stable) => {
            let payload = map_stable.0;
            STORAGE.with(|s| {
                s.replace(Storage::from(&payload));
            });
        }
        Err(e) => trap(format!("Failed to restored state after upgrade: {:?}", e).as_str()),
    }
}