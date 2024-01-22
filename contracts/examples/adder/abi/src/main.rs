use adder::*;
use dharitri_wasm_debug::*;

fn main() {
	let contract = AdderImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
