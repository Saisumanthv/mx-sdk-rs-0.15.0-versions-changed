use async_alice::*;
use async_bob::*;
use forwarder::*;
use forwarder_raw::*;
use vault::*;

use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../async-alice/output/async-alice.wasm",
		Box::new(|context| Box::new(AliceImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../async-bob/output/async-bob.wasm",
		Box::new(|context| Box::new(BobImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../forwarder/output/forwarder.wasm",
		Box::new(|context| Box::new(ForwarderImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../forwarder-raw/output/forwarder-raw.wasm",
		Box::new(|context| Box::new(ForwarderRawImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../vault/output/vault.wasm",
		Box::new(|context| Box::new(VaultImpl::new(context))),
	);

	contract_map
}

#[test]
fn forwarder_async_accept_moax() {
	parse_execute_denali(
		"denali/forwarder_async_accept_moax.scen.json",
		&contract_map(),
	);
}

#[test]
fn forwarder_async_accept_dct() {
	parse_execute_denali(
		"denali/forwarder_async_accept_dct.scen.json",
		&contract_map(),
	);
}

#[test]
fn forwarder_raw_async_accept_moax() {
	parse_execute_denali(
		"denali/forwarder_raw_async_accept_moax.scen.json",
		&contract_map(),
	);
}

#[test]
fn forwarder_raw_async_accept_dct() {
	parse_execute_denali(
		"denali/forwarder_raw_async_accept_dct.scen.json",
		&contract_map(),
	);
}

#[test]
fn forwarder_raw_async_echo() {
	parse_execute_denali("denali/forwarder_raw_async_echo.scen.json", &contract_map());
}

#[test]
fn forwarder_raw_direct_moax() {
	parse_execute_denali(
		"denali/forwarder_raw_direct_moax.scen.json",
		&contract_map(),
	);
}

#[test]
fn forwarder_raw_direct_dct() {
	parse_execute_denali(
		"denali/forwarder_raw_direct_dct.scen.json",
		&contract_map(),
	);
}

// #[test]
// fn forwarder_raw_sync_echo() {
// 	parse_execute_denali("denali/forwarder_raw_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_raw_sync_moax() {
// 	parse_execute_denali("denali/forwarder_raw_sync_moax.scen.json", &contract_map());
// }

// TODO: successive asyncs currently not supported
// #[test]
// fn forwarder_send_twice_moax() {
// 	parse_execute_denali(
// 		"denali/forwarder_send_twice_moax.scen.json",
// 		&contract_map(),
// 	);
// }

// TODO: successive asyncs currently not supported
// #[test]
// fn forwarder_send_twice_dct() {
// 	parse_execute_denali(
// 		"denali/forwarder_send_twice_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_accept_moax() {
// 	parse_execute_denali(
// 		"denali/forwarder_sync_accept_moax.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_accept_dct() {
// 	parse_execute_denali(
// 		"denali/forwarder_sync_accept_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_echo() {
// 	parse_execute_denali("denali/forwarder_sync_echo.scen.json", &contract_map());
// }

#[test]
fn message_othershard() {
	parse_execute_denali("denali/message_otherShard.scen.json", &contract_map());
}

#[test]
fn message_othershard_callback() {
	parse_execute_denali(
		"denali/message_otherShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn message_sameshard() {
	parse_execute_denali("denali/message_sameShard.scen.json", &contract_map());
}

#[test]
fn message_sameshard_callback() {
	parse_execute_denali(
		"denali/message_sameShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn payment_othershard() {
	parse_execute_denali("denali/payment_otherShard.scen.json", &contract_map());
}

#[test]
fn payment_othershard_callback() {
	parse_execute_denali(
		"denali/payment_otherShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn payment_sameshard() {
	parse_execute_denali("denali/payment_sameShard.scen.json", &contract_map());
}

#[test]
fn payment_sameshard_callback() {
	parse_execute_denali(
		"denali/payment_sameShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn send_moax() {
	parse_execute_denali("denali/send_moax.scen.json", &contract_map());
}

#[test]
fn send_dct() {
	parse_execute_denali("denali/send_dct.scen.json", &contract_map());
}
