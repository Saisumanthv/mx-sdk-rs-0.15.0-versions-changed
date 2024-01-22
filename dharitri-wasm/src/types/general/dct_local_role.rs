use crate::abi::TypeAbi;
use alloc::string::String;
use dharitri_codec::*;

const DCT_ROLE_NONE: &[u8] = &[];
const DCT_ROLE_LOCAL_MINT: &[u8] = b"DCTRoleLocalMint";
const DCT_ROLE_LOCAL_BURN: &[u8] = b"DCTRoleLocalBurn";
const DCT_ROLE_NFT_CREATE: &[u8] = b"DCTRoleNFTCreate";
const DCT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"DCTRoleNFTAddQuantity";
const DCT_ROLE_NFT_BURN: &[u8] = b"DCTRoleNFTBurn";

#[derive(Clone, PartialEq, Debug)]
pub enum DctLocalRole {
	None,
	Mint,
	Burn,
	NftCreate,
	NftAddQuantity,
	NftBurn,
}

impl DctLocalRole {
	pub fn as_u8(&self) -> u8 {
		match self {
			Self::None => 0,
			Self::Mint => 1,
			Self::Burn => 2,
			Self::NftCreate => 3,
			Self::NftAddQuantity => 4,
			Self::NftBurn => 5,
		}
	}

	pub fn as_role_name(&self) -> &'static [u8] {
		match self {
			Self::None => DCT_ROLE_NONE,
			Self::Mint => DCT_ROLE_LOCAL_MINT,
			Self::Burn => DCT_ROLE_LOCAL_BURN,
			Self::NftCreate => DCT_ROLE_NFT_CREATE,
			Self::NftAddQuantity => DCT_ROLE_NFT_ADD_QUANTITY,
			Self::NftBurn => DCT_ROLE_NFT_BURN,
		}
	}
}

impl From<u8> for DctLocalRole {
	#[inline]
	fn from(value: u8) -> Self {
		match value {
			1 => Self::Mint,
			2 => Self::Burn,
			3 => Self::NftCreate,
			4 => Self::NftAddQuantity,
			5 => Self::NftBurn,
			_ => Self::None,
		}
	}
}

impl<'a> From<&'a [u8]> for DctLocalRole {
	#[inline]
	fn from(byte_slice: &'a [u8]) -> Self {
		if byte_slice == DCT_ROLE_LOCAL_MINT {
			Self::Mint
		} else if byte_slice == DCT_ROLE_LOCAL_BURN {
			Self::Burn
		} else if byte_slice == DCT_ROLE_NFT_CREATE {
			Self::NftCreate
		} else if byte_slice == DCT_ROLE_NFT_ADD_QUANTITY {
			Self::NftAddQuantity
		} else if byte_slice == DCT_ROLE_NFT_BURN {
			Self::NftBurn
		} else {
			Self::None
		}
	}
}

impl NestedEncode for DctLocalRole {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_u8().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_u8().dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for DctLocalRole {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_u8().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_u8().top_encode_or_exit(output, c, exit);
	}
}

impl NestedDecode for DctLocalRole {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Self::from(u8::dep_decode(input)?))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Self::from(u8::dep_decode_or_exit(input, c, exit))
	}
}

impl TopDecode for DctLocalRole {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(Self::from(u8::top_decode(input)?))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Self::from(u8::top_decode_or_exit(input, c, exit))
	}
}

impl TypeAbi for DctLocalRole {
	fn type_name() -> String {
		"DctLocalRole".into()
	}
}
