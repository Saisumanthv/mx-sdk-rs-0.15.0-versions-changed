extern crate payable_features;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;
use payable_features::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/payable-features.wasm",
		Box::new(|context| Box::new(PayableFeaturesImpl::new(context))),
	);
	contract_map
}

#[test]
fn payable_any_1() {
	parse_execute_denali("denali/payable_any_1.scen.json", &contract_map());
}

#[test]
fn payable_any_2() {
	parse_execute_denali("denali/payable_any_2.scen.json", &contract_map());
}

#[test]
fn payable_any_3() {
	parse_execute_denali("denali/payable_any_3.scen.json", &contract_map());
}

#[test]
fn payable_any_4() {
	parse_execute_denali("denali/payable_any_4.scen.json", &contract_map());
}

#[test]
fn payable_moax_0() {
	parse_execute_denali("denali/payable_moax_0.scen.json", &contract_map());
}

#[test]
fn payable_moax_1() {
	parse_execute_denali("denali/payable_moax_1.scen.json", &contract_map());
}

#[test]
fn payable_moax_2() {
	parse_execute_denali("denali/payable_moax_2.scen.json", &contract_map());
}

#[test]
fn payable_moax_3() {
	parse_execute_denali("denali/payable_moax_3.scen.json", &contract_map());
}

#[test]
fn payable_moax_4() {
	parse_execute_denali("denali/payable_moax_4.scen.json", &contract_map());
}

#[test]
fn payable_token_1() {
	parse_execute_denali("denali/payable_token_1.scen.json", &contract_map());
}

#[test]
fn payable_token_2() {
	parse_execute_denali("denali/payable_token_2.scen.json", &contract_map());
}

#[test]
fn payable_token_3() {
	parse_execute_denali("denali/payable_token_3.scen.json", &contract_map());
}

#[test]
fn payable_token_4() {
	parse_execute_denali("denali/payable_token_4.scen.json", &contract_map());
}
