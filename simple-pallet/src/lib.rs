#![cfg_attr(not(feature = "std"), no_std)]

// 1.Add required imports and dependencies
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// 2.Declare the pallet type
	// This is a placeholder to implement traits and methods.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 3.Add the runtime configuration trait
	// All types and constants go here.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// 4.Add runtime storage to declare storage items.
	#[pallet::storage]
	// #[pallet::getter(fn something)]
	pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>;

	// 5.Add runtime events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(u32, u128),
	}

	// 6.Add hooks to define some logic that should be executed
	// in a specific context, for example on_initialize.
	//  #[pallet::hooks]
	//  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	// 7.Add functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(
			origin: OriginFor<T>,
			id: u32,
			claim: u128,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			Proofs::<T>::insert(&id, &claim);

			Self::deposit_event(Event::ClaimCreated(id, claim));
			Ok(().into())
		}
	}
}
