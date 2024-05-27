//** About **//
	// This is a template pallet, built around the structure used for other pallets in
	// this project. Use it as a style reference while there is no rustfmt file.
	// It also includes examples of how to interact with different features substrate
	// has to offer. 



	#![cfg_attr(not(feature = "std"), no_std)]

	pub use pallet::*;
	
	#[cfg(test)]
	mod mock;
	
	#[cfg(test)]
	mod tests;
	
	#[cfg(feature = "runtime-benchmarks")]
	mod benchmarking;
	
	
	
	#[frame_support::pallet]
	pub mod pallet {
		
		//** Config **//
	
			//* Imports *// 
	
				use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
				use frame_system::pallet_prelude::*;
	
			//* Config *//
			
				#[pallet::pallet]
				pub struct Pallet<T>(_);
	
				#[pallet::config]
				pub trait Config: frame_system::Config {
					type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
				}
	

	
		//** Types **//	
		
			//* Types *//
			//* Constants *//
			//* Enums *//
			//* Structs *//
	
	
	
		//** Genesis **//
			
			// #[pallet::genesis_config]
			// pub struct GenesisConfig<T: Config> {
			// 	pub category_to_tag_map: Vec<(
			// 		BoundedVec<u8, T::CategoryStringLimit>,
			// 		BoundedVec<BoundedVec<u8, T::TagStringLimit>, T::MaxTags>
			// 	)>
			// }
	
	
			// #[cfg(feature = "std")]
			// impl<T: Config> Default for GenesisConfig<T> {
			// 	fn default() -> Self {
			// 		Self { 
			// 			category_to_tag_map: Default::default() 
			// 		}
			// 	}
			// }
	
	
			// #[pallet::genesis_build]
			// impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
			// 	fn build(&self) {
			// 		for (category_id, bounded_tag_id_list) in &self.category_to_tag_map {
						
			// 			// initialize the category
			// 			let category = Category {
			// 				tag_list: bounded_tag_id_list.clone(),
			// 			};
	
			// 			<Categories<T>>::insert(category_id.clone(), category);
	
	
			// 			// create an entry for each of the category's tags and bind them together
			// 			let tag_description: BoundedVec<u8, T::DescStringLimit>
			// 				= TryInto::try_into(Vec::new()).unwrap();
	
			// 			for tag_id in bounded_tag_id_list {
			// 				let tag = Tag {
			// 					parent_category: category_id.clone(),
			// 					description: tag_description.clone(),
			// 				};
	
			// 				<Tags<T>>::insert(tag_id, tag);
			// 			}
			// 		}
			// 	}
			// }
	
	

		//** Storage **//
	
			#[pallet::storage]
			#[pallet::getter(fn something)]
			pub type Something<T> = StorageValue<_, u32>;
	
	

		//** Events **//
	
			#[pallet::event]
			#[pallet::generate_deposit(pub(super) fn deposit_event)]
			pub enum Event<T: Config> {
				SomethingStored(u32, T::AccountId),
			}
	


		//** Errors **//
			
			#[pallet::error]
			pub enum Error<T> {
				NoneValue,
				StorageOverflow,
			}
	

		//** Hooks **//
		#[pallet::hooks]
		impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		}



		//** Extrinsics **//
	
			#[pallet::call]
			impl<T: Config> Pallet<T> {

				#[pallet::call_index(0)]
				#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
				pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
					let who = ensure_signed(origin)?;

					<Something<T>>::put(something);

					Self::deposit_event(Event::SomethingStored(something, who));
					Ok(().into())
				}


				#[pallet::call_index(1)]
				#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
				pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
					let _who = ensure_signed(origin)?;

					match <Something<T>>::get() {
						None => Err(Error::<T>::NoneValue)?,
				
						Some(old) => {
							let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
							<Something<T>>::put(new);
							Ok(().into())
						},
					}
				}
			
			
			}
	
	
			
		//** Helpers **//
	
			impl<T: Config> Pallet<T> {
	
			}
	}
	