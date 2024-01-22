use moax_dct_swap::*;
use dharitri_wasm_debug::*;

fn main() {
	let contract = MoaxDctSwapImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
