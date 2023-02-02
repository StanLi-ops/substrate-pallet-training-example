#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn my_class)]
	// 班级
	pub type Class<T: Config> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetClass(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		SetClassError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: BlockNumberFor<T>) {
			log::info!(target: "hooks","---------- on_finalize, block number is {:?}",n);
		}
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			log::info!(target: "hooks","++++++++++ on_initialize, block number is {:?}",n);
			Weight::zero()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_class(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;
			if Class::<T>::exists() {
				return Err(Error::<T>::SetClassError.into());
			}
			Class::<T>::put(class);
			Self::deposit_event(Event::SetClass(class));
			Ok(().into())
		}
	}
}
