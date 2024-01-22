use crypto_bubbles::*;
use dharitri_wasm_debug::*;

fn main() {
	let contract = CryptoBubblesImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
