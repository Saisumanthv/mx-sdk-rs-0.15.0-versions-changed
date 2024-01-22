extern crate kitty_genetic_alg;
use kitty_genetic_alg::*;

use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/kitty-genetic-alg.wasm",
		Box::new(|context| Box::new(KittyGeneticAlgImpl::new(context))),
	);
	contract_map
}

#[test]
fn init() {
	parse_execute_denali("denali/init.scen.json", &contract_map());
}

#[test]
fn generate_kitty_genes_test() {
	parse_execute_denali("denali/generate-kitty-genes.scen.json", &contract_map());
}
