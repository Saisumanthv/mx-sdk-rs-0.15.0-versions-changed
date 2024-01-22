use dharitri_codec::TopEncode;

use super::{BigUintApi, ErrorApi};
use crate::hex_call_data::HexCallDataSerializer;
use crate::types::{
	Address, ArgBuffer, AsyncCall, BoxedBytes, CodeMetadata, TokenIdentifier, Vec, H256,
};

pub const DCT_TRANSFER_STRING: &[u8] = b"DCTTransfer";
pub const DCT_NFT_TRANSFER_STRING: &[u8] = b"DCTNFTTransfer";

/// API that groups methods that either send MOAX or DCT, or that call other contracts.
#[allow(clippy::too_many_arguments)] // TODO: some arguments should be grouped though
pub trait SendApi<BigUint>: ErrorApi + Clone + Sized
where
	BigUint: BigUintApi + 'static,
{
	/// Sends MOAX to a given address, directly.
	/// Used especially for sending MOAX to regular accounts.
	fn direct_moax(&self, to: &Address, amount: &BigUint, data: &[u8]);

	/// Sends MOAX to an address (optionally) and executes like an async call, but without callback.
	fn direct_moax_execute(
		&self,
		to: &Address,
		amount: &BigUint,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends an DCT token to a given address, directly.
	/// Used especially for sending DCT to regular accounts.
	///
	/// Unlike sending DCT via async call, this method can be called multiple times per transaction.
	fn direct_dct_via_transf_exec(
		&self,
		to: &Address,
		token: &[u8],
		amount: &BigUint,
		data: &[u8],
	) -> Result<(), &'static [u8]> {
		self.direct_dct_execute(to, token, amount, 0, data, &ArgBuffer::new())
	}

	/// Sends DCT to an address and executes like an async call, but without callback.
	fn direct_dct_execute(
		&self,
		to: &Address,
		token: &[u8],
		amount: &BigUint,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends DCT NFT to an address and executes like an async call, but without callback.
	fn direct_dct_nft_execute(
		&self,
		to: &Address,
		token: &[u8],
		nonce: u64,
		amount: &BigUint,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends either MOAX or an DCT token to the target address,
	/// depending on what token identifier was specified.
	fn direct(&self, to: &Address, token: &TokenIdentifier, amount: &BigUint, data: &[u8]) {
		if token.is_moax() {
			self.direct_moax(to, amount, data);
		} else {
			let _ = self.direct_dct_via_transf_exec(to, token.as_dct_identifier(), amount, data);
		}
	}

	/// Performs a simple DCT transfer, but via async call.
	/// This is the preferred way to send DCT.
	fn direct_dct_via_async_call(
		&self,
		to: &Address,
		dct_token_name: &[u8],
		amount: &BigUint,
		data: &[u8],
	) -> ! {
		let mut serializer = HexCallDataSerializer::new(DCT_TRANSFER_STRING);
		serializer.push_argument_bytes(dct_token_name);
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
		if !data.is_empty() {
			serializer.push_argument_bytes(data);
		}
		self.async_call_raw(&to, &BigUint::zero(), serializer.as_slice())
	}

	/// Sends either MOAX or an DCT token to the target address,
	/// depending on what token identifier was specified.
	/// In case of DCT it performs an async call.
	fn direct_via_async_call(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		amount: &BigUint,
		data: &[u8],
	) {
		if token.is_moax() {
			let _ = self.direct_moax(to, amount, data);
		} else {
			self.direct_dct_via_async_call(to, token.as_dct_identifier(), amount, data);
		}
	}

	/// Sends an asynchronous call to another contract.
	/// Calling this method immediately terminates tx execution.
	/// Using it directly is generally discouraged.
	///
	/// The data is expected to be of the form `functionName@<arg1-hex>@<arg2-hex>@...`.
	/// Use a `HexCallDataSerializer` to prepare this field.
	fn async_call_raw(&self, to: &Address, amount: &BigUint, data: &[u8]) -> !;

	/// Sends an asynchronous call to another contract, with either MOAX or DCT value.
	/// The `token` argument decides which one it will be.
	/// Calling this method immediately terminates tx execution.
	fn async_call(&self, async_call: AsyncCall<BigUint>) -> ! {
		self.async_call_raw(
			&async_call.to,
			&async_call.moax_payment,
			async_call.hex_data.as_slice(),
		)
	}

	/// Deploys a new contract in the same shard.
	/// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
	/// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
	fn deploy_contract(
		&self,
		gas: u64,
		amount: &BigUint,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	) -> Address;

	fn execute_on_dest_context_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &BigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes>;

	fn execute_on_dest_context_by_caller_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &BigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes>;

	fn execute_on_same_context_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &BigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	);

	/// Used to store data between async call and callback.
	fn storage_store_tx_hash_key(&self, data: &[u8]);

	/// Used to store data between async call and callback.
	fn storage_load_tx_hash_key(&self) -> BoxedBytes;

	/// Allows synchronously calling a local function by name. Execution is resumed afterwards.
	/// You should never have to call this function directly.
	/// Use the other specific methods instead.
	fn call_local_dct_built_in_function(&self, gas: u64, function: &[u8], arg_buffer: &ArgBuffer);

	/// Allows synchronous minting of DCT tokens. Execution is resumed afterwards.
	fn dct_local_mint(&self, gas: u64, token: &[u8], amount: &BigUint) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.call_local_dct_built_in_function(gas, b"DCTLocalMint", &arg_buffer);
	}

	/// Allows synchronous burning of DCT tokens. Execution is resumed afterwards.
	fn dct_local_burn(&self, gas: u64, token: &[u8], amount: &BigUint) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.call_local_dct_built_in_function(gas, b"DCTLocalBurn", &arg_buffer);
	}

	/// Creates a new NFT token of a certain type (determined by `token_identifier`).  
	/// `attributes` can be any serializable custom struct.  
	/// This is a built-in function, so the smart contract execution is resumed after.
	fn dct_nft_create<T: dharitri_codec::TopEncode>(
		&self,
		gas: u64,
		token: &[u8],
		amount: &BigUint,
		name: &BoxedBytes,
		royalties: &BigUint,
		hash: &H256,
		attributes: &T,
		uris: &[BoxedBytes],
	) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());
		arg_buffer.push_argument_bytes(name.as_slice());
		arg_buffer.push_argument_bytes(royalties.to_bytes_be().as_slice());
		arg_buffer.push_argument_bytes(hash.as_bytes());

		let mut top_encoded_attributes = Vec::new();
		let _ = attributes.top_encode(&mut top_encoded_attributes);
		arg_buffer.push_argument_bytes(top_encoded_attributes.as_slice());

		// The API function has the last argument as variadic,
		// so we top-encode each and send as separate argument
		for uri in uris {
			let mut top_encoded_uri = Vec::new();
			let _ = uri.top_encode(&mut top_encoded_uri);

			arg_buffer.push_argument_bytes(top_encoded_uri.as_slice());
		}

		self.call_local_dct_built_in_function(gas, b"DCTNFTCreate", &arg_buffer);
	}

	/// Adds quantity for an Non-Fungible Token. (which makes it a Semi-Fungible Token by definition)  
	/// This is a built-in function, so the smart contract execution is resumed after.
	fn dct_nft_add_quantity(&self, gas: u64, token: &[u8], nonce: u64, amount: &BigUint) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(&nonce.to_be_bytes()[..]);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.call_local_dct_built_in_function(gas, b"DCTNFTAddQuantity", &arg_buffer);
	}

	/// The reverse operation of `dct_nft_add_quantity`, this locally decreases
	/// This is a built-in function, so the smart contract execution is resumed after.
	fn dct_nft_burn(&self, gas: u64, token: &[u8], nonce: u64, amount: &BigUint) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(&nonce.to_be_bytes()[..]);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.call_local_dct_built_in_function(gas, b"DCTNFTBurn", &arg_buffer);
	}

	/// Performs a simple DCT NFT transfer, but via async call.
	/// This is the preferred way to send DCT.
	/// Note: call is done to the SC itself, so `from` should be the SCs own address
	fn direct_dct_nft_via_async_call(
		&self,
		from: &Address,
		to: &Address,
		token: &[u8],
		nonce: u64,
		amount: &BigUint,
		data: &[u8],
	) {
		let mut serializer = HexCallDataSerializer::new(DCT_NFT_TRANSFER_STRING);
		serializer.push_argument_bytes(token);
		serializer.push_argument_bytes(&nonce.to_be_bytes()[..]);
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
		serializer.push_argument_bytes(to.as_bytes());
		if !data.is_empty() {
			serializer.push_argument_bytes(data);
		}
		self.async_call_raw(&from, &BigUint::zero(), serializer.as_slice());
	}

	/// Sends an DCT NFT to a given address, directly.
	/// Used especially for sending DCT to regular accounts.
	///
	/// Unlike sending DCT via async call, this method can be called multiple times per transaction.
	fn direct_dct_nft_via_transfer_exec(
		&self,
		to: &Address,
		token: &[u8],
		nonce: u64,
		amount: &BigUint,
		data: &[u8],
	) -> Result<(), &'static [u8]> {
		self.direct_dct_nft_execute(to, token, nonce, amount, 0, data, &ArgBuffer::new())
	}
}
