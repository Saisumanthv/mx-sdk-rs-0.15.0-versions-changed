use crowdfunding_moax::*;
use dharitri_wasm_debug::*;

fn main() {
	let contract = CrowdfundingImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
