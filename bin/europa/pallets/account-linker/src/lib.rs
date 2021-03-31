#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;
	use frame_support::pallet_prelude::*;
	use weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId")]
	pub enum Event<T: Config> {
		EthAddressLinked(T::AccountId, Vec<u8>),
		BtcAddressLinked(T::AccountId, Vec<u8>),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn eth_addresses)]
	pub(super) type EthereumLink<T: Config> =  StorageMap<_, Blake2_128Concat, T::AccountId, Vec<[u8; 20]>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn btc_addresses)]
	pub(super) type BitcoinLink<T: Config> =  StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		
	}
}


