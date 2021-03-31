//! # Offchain Worker
//! The pallet is responsible for get the external assets claim from the extrinsic and then query and aggregate the
//! balance (btc and eth) according to linked external accounts in account linker pallet. Offchain worker get the data
//! from most popular websire like etherscan, infura and blockinfo. After get the balance, Offchain worker emit the event
//! with balance info and store them on chain for on-chain query.
//!
//! ## API token
//! The offchain worker need the API token to query data from third party data provider. Currently, offchain worker get
//! the API tokens from a local server. Then store the API tokens in offchain worder local storage.
//!

#![cfg_attr(not(feature = "std"), no_std)]

// everything define in pallet mod must be public
pub use pallet::*;
use sp_core::crypto::KeyTypeId;

pub mod weights;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocw!");

#[frame_support::pallet]
pub mod pallet {
	pub mod crypto {
		use super::KEY_TYPE;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			traits::Verify, MultiSignature, MultiSigner,
		};
		use sp_core::sr25519::Signature as Sr25519Signature;
		app_crypto!(sr25519, KEY_TYPE);
	
		pub struct TestAuthId;
		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	
		impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	}

	use crate::*;
	use frame_system::pallet_prelude::*;
	use frame_support::pallet_prelude::*;
	use weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
				type WeightInfo: weights::WeightInfo;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId", T::BlockNumber = "BlockNumber")]
	pub enum Event<T: Config> {
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn account_balance)]
	pub(super) type AccountBalance<T: Config> =  StorageMap<_, Blake2_128Concat, T::AccountId, (Option<u128>, Option<u128>), ValueQuery>;

	#[pallet::call]
	impl<T:Config> Pallet<T> {
	}

	impl<T: Config> Pallet<T> {
	}
}