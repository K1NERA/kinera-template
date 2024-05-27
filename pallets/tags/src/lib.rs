//** About **//
	// This pallet handles information regarding Categories/Tags. 
	// These classifications act as a way to classify existing content,
	// providing a framework to feed other systems with information.


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

			use frame_support::{
				pallet_prelude::*,
			};
			use frame_system::pallet_prelude::*;

			use scale_info::prelude::vec::Vec;

			

		//* Config *//
		
			#[pallet::pallet]
			pub struct Pallet<T>(_);

			#[pallet::config]
			pub trait Config: frame_system::Config {
				type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
				
				type MaxTags: Get<u32>;
				type MaxContentWithTag: Get<u32>; 
				type ContentStringLimit: Get<u32>; //TODO-1

                type CategoryStringLimit: Get<u32>;
				type TagStringLimit: Get<u32>;
			}



	//** Types **//	
	
		//* Types *//

			pub type CategoryType<T> = BoundedVec<u8, <T as Config>::CategoryStringLimit>;
			pub type CategoryId<T> = BoundedVec<u8, <T as Config>::CategoryStringLimit>;
			pub type TagId<T> = BoundedVec<u8, <T as Config>::TagStringLimit>;
		
        //* Constants *//
		//* Enums *//
		//* Structs *//

			#[derive(Clone, Encode, Copy, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
			#[scale_info(skip_type_params(T))]
			pub struct TagIdList<BoundedTagList> {
				pub tag_list: BoundedTagList,
			}

			#[derive(Clone, Encode, Copy, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
			#[scale_info(skip_type_params(T))]
			pub struct TagData<BoundedTagList> {
				pub content_with_tag: BoundedTagList,
			}





	//** Storage **//

        //* Category *// 

			// Matches a tuple of CategoryType (ex: Moderation) 
			// and a CategoryId (ex: Offensive)
			// to the list of all the respective TagIds.
			// ex: (Moderation, Offensive) -> [Racism, Hate Speech, ...]. 
			#[pallet::storage]
			#[pallet::getter(fn get_category)]
			pub type Categories <T: Config> =
				StorageMap<
					_, 
					Blake2_128Concat, (CategoryType<T>, CategoryId<T>), 
					TagIdList<
						BoundedVec<TagId<T>, T::MaxTags>, 
					>,
					OptionQuery
				>;

				
			// Matches a tuple of CategoryType (ex: Moderation) 
			// and a CategoryId (ex: Offensive) with a secondary TagId
			// (ex: Hate Speech) to the respective tag data.
			// ex: (Moderation, Offensive), Hate Speech 
			// -> Moderation / Offensive / Hate Speech Tag Data. 
			#[pallet::storage]
			#[pallet::getter(fn get_tag)]
			pub type Tags <T: Config> =
				StorageDoubleMap<
					_, 
					Blake2_128Concat, (CategoryType<T>, CategoryId<T>),
					Blake2_128Concat, TagId<T>,
					TagData<
						BoundedVec<
							BoundedVec<u8, T::ContentStringLimit>,
							T::MaxContentWithTag,
						>
						
					>,
					OptionQuery
				>;



				


	//** Events **//

		#[pallet::event]
		#[pallet::generate_deposit(pub(super) fn deposit_event)]
		pub enum Event<T: Config> {
            CategoryCreated(T::AccountId, CategoryId<T>),
            TagCreated(T::AccountId, TagId<T>, CategoryId<T>),
		}



	//** Errors **//
		
		#[pallet::error]
		pub enum Error<T> {
			NoneValue,
			StorageOverflow,
			BadMetadata,

			CategoryAlreadyExists,
			NonexistentCategory,
			
			TagAlreadyExists,
			NonexistentTag,
		}

		

	//** Extrinsics **//

		#[pallet::call]
		impl<T: Config> Pallet<T> {
		

	
		}


		
	//** Helpers **//

		impl<T: Config> Pallet<T> {


				pub fn do_validate_tag_data (
					category_type: CategoryType<T>,
					category_tag_list: BoundedVec<(CategoryId<T>, TagId<T>), T::MaxTags>,
				)-> Result<(), DispatchError> {
						
					for (category_id, tag_id) in category_tag_list {
						let tag_list = Categories::<T>::try_get((category_type.clone(), category_id.clone()));
						ensure!(tag_list.is_ok(), Error::<T>::NonexistentCategory);
						
						let tag_data = Tags::<T>::try_get((category_type.clone(), category_id), tag_id);
						ensure!(tag_data.is_ok(), Error::<T>::NonexistentTag);
					}

					Ok(())
				}

				pub fn do_update_tag_data (
					category_type: CategoryType<T>,
					category_tag_list: BoundedVec<(CategoryId<T>, TagId<T>), T::MaxTags>,
					content_id: BoundedVec<u8, T::ContentStringLimit>,
				)-> Result<(), DispatchError> {
						
					for (category_id, tag_id) in category_tag_list {

						Tags::<T>::try_mutate_exists(
						(category_type.clone(), category_id), 
						tag_id, |content_with_tag| -> DispatchResult {
							
							let tag_content_data = content_with_tag.as_mut().ok_or(Error::<T>::BadMetadata)?;
							tag_content_data.content_with_tag.try_push(content_id.clone()).unwrap();

							Ok(())
						})?;



					}

					Ok(())
				}




				
			
		}


}
