# 0 manager
dfx canister call nft_backend is_manager '(principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae")'

dfx canister call nft_backend setMinter '(principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae")'
dfx canister call nft_backend getMinter

dfx canister call nft_backend add_manager '(principal "renrk-eyaaa-aaaaa-aaada-cai")'

# 1.5 after mint > 5
dfx canister call nft_backend init_reserve '(3: nat32)'
dfx canister call nft_backend set_claim_supply '(5: nat32)'

# 1. mint
dfx canister call nft_backend mintNFT '(record {
to = variant{"principal" = principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae"};
opt vec {(1)}})'

# to no use
dfx canister call nft_backend batchMintNFT '(vec {
record { metadata = opt vec {1; 2; 3};
to = variant{"principal" = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"}};
record { metadata = opt vec {4; 5; 6};
to = variant{"principal" = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"}};
record { metadata = opt vec {7; 8; 9};
to = variant{"principal" = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"}};
})'
# *n times
dfx canister call nft_backend set_claim_supply '(0: nat32)'
# then claim

# 1.99 get token_id and account_id
dfx canister call nft_backend account_id '(principal "r2a3n-etwya-gh4cb-4xhax-rcndg-guats-vbwis-w3q6v-4svgz-uf53c-mae")'
# 3a8516788fe6c5fdd68fc224ac8199f7b57c74152ac9d4161561b001aaf70ab2
dfx canister call nft_backend token_id '(4: nat32)'
# dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a

# 2. claim
dfx canister call nft_backend set_claim_supply '(55: nat32)'
dfx canister call nft_backend claimNFT '(principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae")'
dfx canister call nft_backend tokens '("dc1db5358d5ae1dad0247666c43e3125ec675f93c427a2c6663ba931e88c646d")'

# after init_reserve
dfx canister call nft_backend force_claim_reserve '(record {
to = variant {"principal" = principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae"};
index = (1: nat32) })'

dfx canister call nft_backend tokens '("3a8516788fe6c5fdd68fc224ac8199f7b57c74152ac9d4161561b001aaf70ab2")'
dfx canister call nft_backend balance '(record { token = "dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a";
user = variant {"principal" = principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae"}})'

# 3. approve
dfx canister call nft_backend approve '(record { token = "dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a";
spender = principal "rrkah-fqaaa-aaaaa-aaaaq-cai";
subaccount = null;
allowance = (1: nat32)})'
# after approve get allowance
dfx canister call nft_backend allowance '(record { token = "dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a";
owner = variant {"principal" = principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae"};
spender = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"})'

# 4. transfer
dfx canister call nft_backend bearer '("dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a")'

dfx canister call nft_backend transfer '(record {
from = variant{"principal" = principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae"};
to = variant{"principal" = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"};
token = "dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a";
memo = vec {};
subaccount = null;
amount = (1: nat32)})'

# 5. query
dfx canister call nft_backend bearer '("dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a")'
dfx canister call nft_backend metadata '("dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a")'
dfx canister call nft_backend tokens '("3a8516788fe6c5fdd68fc224ac8199f7b57c74152ac9d4161561b001aaf70ab2")'
dfx canister call nft_backend getTokensByIds '(vec {(0: nat32); (1: nat32)})'

# 6. helper query
dfx canister call nft_backend extensions

dfx canister call nft_backend claim_count  # next_claim_id
dfx canister call nft_backend claim_supply # claim_supply
dfx canister call nft_backend supply '("test")'
dfx canister call  nft_backend pop_info # claim_supply
dfx canister call nft_backend reserve_tokens # vec<token>

# 7 get all
dfx canister call nft_backend getRegistry
dfx canister call nft_backend getAllowances
dfx canister call nft_backend getTokens


#dfx canister call nft_backend decode_id '("dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a")'
#dfx canister call nft_backend is_principal '("dyi3c-bqkor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-a")'
#dfx canister call nft_backend canister_id


dfx deploy --argument '(principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae")'

dfx canister call nft_backend add_manager '(principal "rkp4c-7iaaa-aaaaa-aaaca-cai")'

dfx canister call nft_backend init_reserve '(3: nat32)'
dfx canister call nft_backend set_claim_supply '(5: nat32)'

dfx canister call nft_backend force_claim_reserve '(record {
to = variant {"principal" = principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae"};
index = (1: nat32) })'
dfx canister call nft_backend claimNFT '(principal "2jlx3-drmhh-yw4yn-ltb5s-gp36o-xvosf-s6dqt-xeq62-3x4fc-s7ozz-iqe")'





