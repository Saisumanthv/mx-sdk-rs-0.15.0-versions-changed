use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{BigUintApi, EndpointFinishApi, ErrorApi, SendApi};
use crate::io::EndpointResult;
use crate::types::{Address, BoxedBytes};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendDct<BigUint: BigUintApi> {
	pub to: Address,
	pub token_name: BoxedBytes,
	pub amount: BigUint,
	pub data: BoxedBytes,
}

impl<FA, BigUint> EndpointResult<FA> for SendDct<BigUint>
where
	BigUint: BigUintApi + 'static,
	FA: EndpointFinishApi + SendApi<BigUint> + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
		api.direct_dct_via_async_call(
			&self.to,
			&self.token_name.as_slice(),
			&self.amount,
			self.data.as_slice(),
		);
	}
}

impl<BigUint: BigUintApi> TypeAbi for SendDct<BigUint> {
	fn type_name() -> String {
		"SendDct".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
