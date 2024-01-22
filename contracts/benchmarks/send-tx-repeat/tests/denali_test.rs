extern crate send_tx_repeat;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;
use send_tx_repeat::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/send-tx-repeat.wasm",
		Box::new(|context| Box::new(SendTxRepeatImpl::new(context))),
	);
	contract_map
}

#[test]
fn test_send_tx_repeat_without_data_denali() {
	parse_execute_denali(
		"denali/send_tx_repeat_without_data.scen.json",
		&contract_map(),
	);
}

#[test]
fn test_send_tx_repeat_with_data_denali() {
	parse_execute_denali("denali/send_tx_repeat_with_data.scen.json", &contract_map());
}
