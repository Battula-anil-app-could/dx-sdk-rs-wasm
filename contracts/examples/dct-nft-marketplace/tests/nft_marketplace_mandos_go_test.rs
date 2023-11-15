#[test]
fn auction_end_deadline_go() {
	dharitri_wasm_debug::mandos_go("mandos/auction_end_deadline.scen.json");
}

#[test]
fn auction_end_max_bid_go() {
	dharitri_wasm_debug::mandos_go("mandos/auction_end_max_bid.scen.json");
}

#[test]
fn auction_token_go() {
	dharitri_wasm_debug::mandos_go("mandos/auction_token.scen.json");
}

#[test]
fn bid_first_go() {
	dharitri_wasm_debug::mandos_go("mandos/bid_first.scen.json");
}

#[test]
fn bid_max_go() {
	dharitri_wasm_debug::mandos_go("mandos/bid_max.scen.json");
}

#[test]
fn bid_second_go() {
	dharitri_wasm_debug::mandos_go("mandos/bid_second.scen.json");
}

#[test]
fn init_go() {
	dharitri_wasm_debug::mandos_go("mandos/init.scen.json");
}

#[test]
fn invalid_bids_go() {
	dharitri_wasm_debug::mandos_go("mandos/invalid_bids.scen.json");
}

#[test]
fn specific_token_auctioned_go() {
	dharitri_wasm_debug::mandos_go("mandos/specific_token_auctioned.scen.json");
}

#[test]
fn view_functions_go() {
	dharitri_wasm_debug::mandos_go("mandos/view_functions.scen.json");
}

#[test]
fn withdraw_go() {
	dharitri_wasm_debug::mandos_go("mandos/withdraw.scen.json");
}
