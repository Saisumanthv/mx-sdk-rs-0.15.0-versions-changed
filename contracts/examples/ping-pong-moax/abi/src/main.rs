use dharitri_wasm_debug::*;
use ping_pong_moax::*;

fn main() {
	let contract = PingPongImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
