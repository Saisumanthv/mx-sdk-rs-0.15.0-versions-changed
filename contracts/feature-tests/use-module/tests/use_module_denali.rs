extern crate use_module;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;
use use_module::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/use-module.wasm",
		Box::new(|context| Box::new(UseModuleImpl::new(context))),
	);
	contract_map
}

#[test]
fn use_module_features() {
	parse_execute_denali("denali/use_module_features.scen.json", &contract_map());
}

#[test]
fn use_module_pause() {
	parse_execute_denali("denali/use_module_pause.scen.json", &contract_map());
}
