extern crate crypto_bubbles;
use crypto_bubbles::*;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/crypto-bubbles.wasm",
		Box::new(|context| Box::new(CryptoBubblesImpl::new(context))),
	);
	contract_map
}

#[test]
fn balanceof() {
	parse_execute_denali("denali/balanceOf.scen.json", &contract_map());
}

#[test]
fn create() {
	parse_execute_denali("denali/create.scen.json", &contract_map());
}

#[test]
fn exceptions() {
	parse_execute_denali("denali/exceptions.scen.json", &contract_map());
}

#[test]
fn joingame() {
	parse_execute_denali("denali/joinGame.scen.json", &contract_map());
}

#[test]
fn rewardandsendtowallet() {
	parse_execute_denali("denali/rewardAndSendToWallet.scen.json", &contract_map());
}

#[test]
fn rewardwinner_last() {
	parse_execute_denali("denali/rewardWinner_Last.scen.json", &contract_map());
}

#[test]
fn rewardwinner_simple() {
	parse_execute_denali("denali/rewardWinner.scen.json", &contract_map());
}

#[test]
fn topup_ok() {
	parse_execute_denali("denali/topUp_ok.scen.json", &contract_map());
}

#[test]
fn topup_withdraw() {
	parse_execute_denali("denali/topUp_withdraw.scen.json", &contract_map());
}

#[test]
fn withdraw_ok() {
	parse_execute_denali("denali/withdraw_Ok.scen.json", &contract_map());
}

#[test]
fn withdraw_toomuch() {
	parse_execute_denali("denali/withdraw_TooMuch.scen.json", &contract_map());
}
