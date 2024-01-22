extern crate erc1155;
use erc1155::*;

extern crate erc1155_user_mock;
use erc1155_user_mock::*;

use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/erc1155.wasm",
		Box::new(|context| Box::new(Erc1155Impl::new(context))),
	);
	contract_map.register_contract(
		"file:../../erc1155-user-mock/output/erc1155-user-mock.wasm",
		Box::new(|context| Box::new(Erc1155UserMockImpl::new(context))),
	);

	contract_map
}

#[test]
fn deploy_test() {
	parse_execute_denali("denali/deploy.scen.json", &contract_map());
}

// Create token tests

#[test]
fn create_token_fungible_test() {
	parse_execute_denali("denali/create_token_fungible.scen.json", &contract_map());
}

#[test]
fn create_token_non_fungible_test() {
	parse_execute_denali(
		"denali/create_token_non_fungible.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_two_fungible_same_creator_test() {
	parse_execute_denali(
		"denali/create_two_tokens_both_fungible_same_creator.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_two_fungible_different_creator_test() {
	parse_execute_denali(
		"denali/create_two_tokens_both_fungible_different_creator.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_two_non_fungible_same_creator_test() {
	parse_execute_denali(
		"denali/create_two_tokens_both_non_fungible_same_creator.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_one_fungible_one_non_fungible_test() {
	parse_execute_denali(
		"denali/create_one_fungible_one_non_fungible.scen.json",
		&contract_map(),
	);
}

/* is_smart_contract not yet implemented - uncomment later

// transfer tests -  to account
#[test]
fn transfer_fungible_ok_test() {
	parse_execute_denali("denali/transfer_fungible_ok.scen.json", &contract_map());
}

#[test]
fn transfer_fungible_not_enough_balance_test() {
	parse_execute_denali(
		"denali/transfer_fungible_not_enough_balance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_non_fungible_ok_test() {
	parse_execute_denali("denali/transfer_non_fungible_ok.scen.json", &contract_map());
}

#[test]
fn batch_transfer_fungible_test() {
	parse_execute_denali("denali/batch_transfer_fungible.scen.json", &contract_map());
}

#[test]
fn batch_transfer_non_fungible_test() {
	parse_execute_denali(
		"denali/batch_transfer_non_fungible.scen.json",
		&contract_map(),
	);
}

#[test]
fn batch_transfer_both_types_test() {
	parse_execute_denali(
		"denali/batch_transfer_both_types.scen.json",
		&contract_map(),
	);
}

// transfer tests -  to smart contract
// TODO: Uncomment once IsSmartContractAddress is added to framework

/*#[test]
fn transfer_fungible_ok_to_sc_test() {
	parse_execute_denali("denali/transfer_fungible_ok_to_sc.scen.json", &contract_map());
}

#[test]
fn transfer_non_fungible_ok_to_sc_test() {
	parse_execute_denali("denali/transfer_non_fungible_ok_to_sc.scen.json", &contract_map());
}


#[test]
fn batch_transfer_fungible_to_sc_test() {
	parse_execute_denali("denali/batch_transfer_fungible_to_sc.scen.json", &contract_map());
}

#[test]
fn batch_transfer_non_fungible_to_sc_test() {
	parse_execute_denali("denali/batch_transfer_non_fungible_to_sc.scen.json", &contract_map());
}

#[test]
fn batch_transfer_both_types_to_sc_test() {
	parse_execute_denali("denali/batch_transfer_both_types_to_sc.scen.json", &contract_map());
}
*/

*/
// mint tests

#[test]
fn mint_fungible_test() {
	parse_execute_denali("denali/mint_fungible.scen.json", &contract_map());
}

#[test]
fn mint_non_fungible_test() {
	parse_execute_denali("denali/mint_non_fungible.scen.json", &contract_map());
}

#[test]
fn mint_not_creator_test() {
	parse_execute_denali("denali/mint_not_creator.scen.json", &contract_map());
}

// burn tests

#[test]
fn burn_fungible_test() {
	parse_execute_denali("denali/burn_fungible.scen.json", &contract_map());
}

#[test]
fn burn_non_fungible_test() {
	parse_execute_denali("denali/burn_non_fungible.scen.json", &contract_map());
}
