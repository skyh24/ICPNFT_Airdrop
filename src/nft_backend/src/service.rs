use candid::Principal;
use crate::state::STORAGE;

use crate::types::*;

pub struct Service {}

impl Service {
    pub fn mint(req: MintRequest) -> TokenIndex {
        STORAGE.with(|ext| {
            ext.borrow_mut().mint(req.to, req.metadata)
        })
    }
    pub fn is_claimable(principal: Principal) -> bool {
        STORAGE.with(|ext| {
            ext.borrow_mut().is_claimable(principal)
        })
    }

    pub fn claim(principal: Principal) -> TokenIndex {
        STORAGE.with(|ext| {
            ext.borrow_mut().claim(principal)
        })
    }

    pub fn claim_reserve(req: ClaimRequest) -> TokenIndex {
        STORAGE.with(|ext| {
            ext.borrow_mut().force_claim(req.to, req.index)
        })
    }

    pub fn transfer_from(req: TransferRequest) -> TransferResponse {
        STORAGE.with(|ext| {
            ext.borrow_mut().transfer_from(req.token, req.from, req.to)
        })
    }

    pub fn approve(req: ApproveRequest) -> bool {
        STORAGE.with(|ext| {
            ext.borrow_mut().approve(req.token, req.spender)
        })
    }

    pub fn allowance(req: AllowanceRequest) -> Result<Balance, CommonError> {
        STORAGE.with(|ext| {
            ext.borrow().allowance(req.token, req.owner, req.spender)
        })
    }

    pub fn balance(req: BalanceRequest) -> BalanceResponse{
        STORAGE.with(|ext| {
            ext.borrow().balance(req.token, req.user)
        })
    }

    pub fn owner_of(token: TokenIdentifier) -> Result<AccountIdentifier, CommonError> {
        STORAGE.with(|ext| {
            ext.borrow().owner_of(token)
        })
    }

    pub fn metadata(token: TokenIdentifier) -> Result<Metadata, CommonError> {
        STORAGE.with(|ext| {
            ext.borrow().metadata(token)
        })
    }
}

#[inline(always)]
pub fn manager_guard() -> Result<(), String> {
    if STORAGE.with(|ext| ext.borrow().is_manager(ic_cdk::caller())) {
        Ok(())
    } else {
        Err("Not manager".to_string())
    }
}
