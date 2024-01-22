use dharitri_wasm_debug::*;
use dct_nft_marketplace::*;

fn main() {
	let contract = DctNftMarketplaceImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
