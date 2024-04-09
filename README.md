# CMD

dfx deploy --argument '(principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae")'

dfx canister call nft_backend add_manager '(principal "rkp4c-7iaaa-aaaaa-aaaca-cai")'

dfx canister call nft_backend init_reserve '(3: nat32)'
dfx canister call nft_backend set_claim_supply '(5: nat32)'

dfx canister call nft_backend force_claim_reserve '(record {
to = variant {"principal" = principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae"};
index = (1: nat32) })'
dfx canister call nft_backend claimNFT '(principal "2jlx3-drmhh-yw4yn-ltb5s-gp36o-xvosf-s6dqt-xeq62-3x4fc-s7ozz-iqe")'


see call.sh
