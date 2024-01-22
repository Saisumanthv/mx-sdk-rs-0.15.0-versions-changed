extern crate str_repeat;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;
use str_repeat::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/str-repeat.wasm",
		Box::new(|context| Box::new(StrRepeatImpl::new(context))),
	);
	contract_map
}

#[test]
fn test_str_repeat_denali() {
	parse_execute_denali("denali/str_repeat.scen.json", &contract_map());
}
