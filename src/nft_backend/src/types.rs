use std::fmt::Debug;

use candid::Int;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

pub type Balance = u32; // Nat
pub type TokenIndex = u32;
pub type SubAccount = Vec<u8>;
pub type TokenIdentifier=String;
pub type AccountIdentifier=String;
pub type Extension=String;
pub type BalanceResponse = Result<Balance, CommonError>;
pub type TransferResponse = Result<Balance, TransferError>;

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct TokenObj {
    pub index: TokenIndex,
    pub canister: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum User {
    #[serde(rename = "principal")]
    Principal(Principal),
    #[serde(rename = "address")]
    Address(AccountIdentifier),
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum CommonError {
    InvalidToken(TokenIdentifier),
    Other(String),
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum TransferError {
    CannotNotify(AccountIdentifier),
    InsufficientBalance(Balance),
    InvalidToken(TokenIdentifier),
    Rejected,
    Unauthorized(AccountIdentifier),
    Other(String),
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct AllowanceRequest {
    pub token: TokenIdentifier,
    pub owner: User,
    pub spender: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ApproveRequest {
    pub token: TokenIdentifier,
    pub subaccount: Option<SubAccount>,
    pub allowance: Balance,
    pub spender: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct BalanceRequest {
    pub token: TokenIdentifier,
    pub user: User,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct MintRequest {
    pub to: User,
    pub metadata: Option<Vec<u8>>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ClaimRequest {
    pub to: User,
    pub index: TokenIndex,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize,  Eq, PartialEq)]
pub enum Metadata {
    #[serde(rename = "fungible")]
    Fungible {
        decimals: u8,
        metadata: Option<Vec<u8>>,
        name: String,
        symbol: String,
    },
    #[serde(rename = "nonfungible")]
    NonFungible {
        metadata: Option<Vec<u8>>,
    },
}

pub type Time = Int;
#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct Listing {
    pub locked: Option<Time>,
    pub seller: Principal,
    pub price: u64,
}

pub type Memo = Vec<u8>;
#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct TransferRequest {
    pub from: User,
    pub to: User,
    pub token: TokenIdentifier,
    pub memo: Memo,
    pub subaccount: Option<SubAccount>, // omit
    pub amount: Balance,
}



#[derive(CandidType, Deserialize, Debug)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Info{
    pub reserve: u32,
    pub claimed: u32,
    pub available: u32,
    pub supply: u32,
}


