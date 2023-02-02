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
		// type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn my_class)]
	// 班级
	pub type Class<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn student_info)]
	// 学生信息,key=学号,value=姓名
	pub type StudentInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	// 宿舍信息,key1=宿舍号,value1(key2=床号,value2=学号)
	pub type DormInfo<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, u32, ValueQuery>;

	// #[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	// pub enum Event<T: Config> {
	// 	ClaimCreated(u32, u128),
	// }

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_class(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			Class::<T>::put(class);

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn set_student_info(
			origin: OriginFor<T>,
			student_number: u32,
			student_name: u128,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			StudentInfo::<T>::insert(&student_number, &student_name);

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn set_dorm_info(
			origin: OriginFor<T>,
			dorm_number: u32,
			bed_number: u32,
			student_name: u32,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			DormInfo::<T>::insert(&dorm_number, &bed_number, &student_name);

			Ok(().into())
		}
	}
}
