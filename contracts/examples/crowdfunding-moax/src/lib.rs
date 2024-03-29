#![no_std]
#![allow(unused_attributes)]

dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy, TypeAbi)]
pub enum Status {
	FundingPeriod,
	Successful,
	Failed,
}

#[dharitri_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {
	#[init]
	fn init(&self, target: BigUint, deadline: u64) {
		let my_address: Address = self.blockchain().get_caller();
		self.set_owner(&my_address);
		self.set_target(&target);
		self.set_deadline(deadline);
	}

	#[payable("MOAX")]
	#[endpoint]
	fn fund(&self, #[payment] payment: BigUint) -> SCResult<()> {
		if self.blockchain().get_block_nonce() > self.get_deadline() {
			return sc_error!("cannot fund after deadline");
		}

		let caller = self.blockchain().get_caller();
		let mut deposit = self.get_deposit(&caller);
		deposit += payment;
		self.set_deposit(&caller, &deposit);

		Ok(())
	}

	#[view]
	fn status(&self) -> Status {
		if self.blockchain().get_block_nonce() <= self.get_deadline() {
			Status::FundingPeriod
		} else if self.blockchain().get_sc_balance() >= self.get_target() {
			Status::Successful
		} else {
			Status::Failed
		}
	}

	#[view(currentFunds)]
	fn current_funds(&self) -> SCResult<BigUint> {
		Ok(self.blockchain().get_sc_balance())
	}

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		match self.status() {
			Status::FundingPeriod => sc_error!("cannot claim before deadline"),
			Status::Successful => {
				let caller = self.blockchain().get_caller();
				if caller != self.get_owner() {
					return sc_error!("only owner can claim successful funding");
				}
				self.send().direct_moax(
					&caller,
					&self.blockchain().get_sc_balance(),
					b"funding success",
				);
				Ok(())
			},
			Status::Failed => {
				let caller = self.blockchain().get_caller();
				let deposit = self.get_deposit(&caller);
				if deposit > 0 {
					self.send()
						.direct_moax(&caller, &deposit, b"reclaim failed funding");
					self.set_deposit(&caller, &BigUint::zero());
				}
				Ok(())
			},
		}
	}

	// storage

	#[storage_set("owner")]
	fn set_owner(&self, address: &Address);

	#[view]
	#[storage_get("owner")]
	fn get_owner(&self) -> Address;

	#[storage_set("target")]
	fn set_target(&self, target: &BigUint);

	#[view]
	#[storage_get("target")]
	fn get_target(&self) -> BigUint;

	#[storage_set("deadline")]
	fn set_deadline(&self, deadline: u64);

	#[view]
	#[storage_get("deadline")]
	fn get_deadline(&self) -> u64;

	#[storage_set("deposit")]
	fn set_deposit(&self, donor: &Address, amount: &BigUint);

	#[view]
	#[storage_get("deposit")]
	fn get_deposit(&self, donor: &Address) -> BigUint;
}
