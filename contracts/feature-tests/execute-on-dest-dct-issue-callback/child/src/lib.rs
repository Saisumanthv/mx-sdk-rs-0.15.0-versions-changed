#![no_std]
#![allow(unused_attributes)]

dharitri_wasm::imports!();

const MOAX_DECIMALS: usize = 18;

#[dharitri_wasm_derive::contract(ChildImpl)]
pub trait Child {
	#[init]
	fn init(&self) {}

	#[payable("MOAX")]
	#[endpoint(issueWrappedMoax)]
	fn issue_wrapped_moax(
		&self,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: BigUint,
		#[payment] issue_cost: BigUint,
	) -> AsyncCall<BigUint> {
		DCTSystemSmartContractProxy::new()
			.issue_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				&initial_supply,
				FungibleTokenProperties {
					num_decimals: MOAX_DECIMALS,
					can_freeze: false,
					can_wipe: false,
					can_pause: false,
					can_mint: true,
					can_burn: false,
					can_change_owner: false,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().dct_issue_callback())
	}

	// callbacks

	#[callback]
	fn dct_issue_callback(
		&self,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] _amount: BigUint,
		#[call_result] _result: AsyncCallResult<()>,
	) {
		self.wrapped_moax_token_identifier().set(&token_identifier);
	}

	// storage

	#[view(getWrappedMoaxTokenIdentifier)]
	#[storage_mapper("wrappedMoaxTokenIdentifier")]
	fn wrapped_moax_token_identifier(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
}
