extern crate adder;
use adder::*;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/adder.wasm",
		Box::new(|context| Box::new(AdderImpl::new(context))),
	);
	contract_map
}

#[test]
fn test_denali() {
	parse_execute_denali("denali/adder.scen.json", &contract_map());
}
