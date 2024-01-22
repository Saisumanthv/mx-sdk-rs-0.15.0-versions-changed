extern crate moax_dct_swap;
use moax_dct_swap::*;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/moax-dct-swap.wasm",
		Box::new(|context| Box::new(MoaxDctSwapImpl::new(context))),
	);
	contract_map
}

#[test]
fn wrap_moax_test() {
	parse_execute_denali("denali/wrap_moax.scen.json", &contract_map());
}

#[test]
fn wrap_then_unwrap_moax_test() {
	parse_execute_denali("denali/unwrap_moax.scen.json", &contract_map());
}
