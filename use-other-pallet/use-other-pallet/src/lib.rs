#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use frame_support::{
		pallet_prelude::{DispatchResultWithPostInfo, *},
		sp_runtime::traits::AtLeast32BitUnsigned,
		sp_std::fmt::Debug,
	};
	use frame_system::pallet_prelude::*;
	use storage_provider::traits::StorageInterface;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Value: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ Default
			+ From<u32>
			+ Into<u32>
			+ Copy
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Debug;

		type MyStorage: StorageInterface<Value = Self::Value>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		StorageEvent,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn storage_value(origin: OriginFor<T>, value: T::Value) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			T::MyStorage::set_param(value);

			let v = T::MyStorage::get_param();
			log::info!(target: "use-other-pallet","Value get from storage is {:?}",v);

			Self::deposit_event(Event::StorageEvent);
			Ok(().into())
		}
	}
}
