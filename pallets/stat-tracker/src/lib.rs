//** About **//
	// Keeps track of a wallet's stats and tokens. Requires registration to be
	// interacted with. If no entries exist when tokens/stats are allocated, one is
	// automatically created for the wallet in question. Most other pallets reference
	// this pallet and use it to keep track of allocated/claimable tokens. It holds
	// tokens alongside the representation of how much each pallet has allocated
	// per wallet. It is also used as an abstraction to transfer funds to/from the
	// treasury, with the amount being designated per feature.
	


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
				dispatch::DispatchResultWithPostInfo,
				pallet_prelude::*,
				PalletId,
				traits::{
					Currency,
					ReservableCurrency,
					ExistenceRequirement::{
						AllowDeath,
					},
				},
				sp_runtime::{
					traits::{
						CheckedAdd,
						CheckedSub,
						CheckedDiv,
						Saturating,
						AccountIdConversion,
					},
				}
			};
			use frame_system::pallet_prelude::*;

			use codec::{Decode, Encode, MaxEncodedLen};

		//* Config *//

			#[pallet::pallet]
			pub struct Pallet<T>(_);

			#[pallet::config]
			pub trait Config: frame_system::Config {
				type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
				type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

				type DefaultReputation: Get<u32>;
				type NameStringLimit: Get<u32>;
				
				type PalletId: Get<PalletId>;
			}


			
	//** Types **//	
	
		//* Types *//

			type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

		//* Constants *//
		//* Enums *//

			// Allows the desambiguation of feature types.
			// Particularly useful for updating tokens values 
			// related to wallets.	
			#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
			pub enum FeatureType {
				Festival,
				RankingList,
				Moderation,
				Movie,
			}
			
			// Allows the desambiguation of token types.
			// Particularly useful for updating tokens values 
			// related to wallets.		
			#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
			pub enum TokenType {
				Locked,
				Claimable,
			}

		//* Structs *//


			// Stats that are bound to a wallet. This is required by many features, to ensure safety.
			// The "..._public" boolean parameters and the name are both defined by the user upon creation.
			#[derive(Clone, Encode, Copy, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
			pub struct Stats<BoundedName> {
				pub is_name_public: bool,
				pub is_wallet_public: bool,
				pub name: BoundedName,
			}
			
			
			// The "total_..." and "claimable_..." balance parameters are each updated by the corresponding app feature.
			// To get the current locked balance, you must do "total_..." - "claimable_..." = "locked_...". 
			#[derive(Clone, Encode, Copy, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo,)]
			pub struct Tokens<Balance, TokenImbalance> {
				pub reputation_moderation: u32,
				pub locked_tokens_moderation: Balance,
				pub claimable_tokens_moderation: Balance,
				pub locked_tokens_festival: Balance,
				pub claimable_tokens_festival: Balance,
				pub locked_tokens_ranking: Balance,
				pub claimable_tokens_ranking: Balance,
				pub imbalance_tokens_ranking: TokenImbalance,
				pub locked_tokens_movie: Balance,
				pub claimable_tokens_movie: Balance,
			}





	//** Storage **//


		// Contains stats related to the identification of this address.
		// When an entery is created for WalletStats, an entry is automatically
		// created in WalletTokens.
		#[pallet::storage]
		#[pallet::getter(fn get_wallet_stats)]
		pub type WalletStats<T: Config> = 
			StorageMap<
				_, 
				Blake2_128Concat, T::AccountId,
				Stats<
					BoundedVec<u8, T::NameStringLimit>,
				>,
			>;


		// Keeps track of the amount of tokens (and reputation_moderation) a wallet has.
		// It is independent from the "WalletStats" storage, meaning an entry
		// can exist by itself without being registed in "WalletStats".
		#[pallet::storage]
		#[pallet::getter(fn get_wallet_tokens)]
		pub type WalletTokens<T: Config> = 
			StorageMap<
				_, 
				Blake2_128Concat, T::AccountId,
				Tokens<
					BalanceOf<T>,
					(BalanceOf<T>, BalanceOf<T>),
				>,
			>;
	
	
	//** Events **//

		#[pallet::event]
		#[pallet::generate_deposit(pub(super) fn deposit_event)]
		pub enum Event<T: Config> {
			AccountRegisteredAddress(T::AccountId),
			AccountRegisteredName(BoundedVec<u8, T::NameStringLimit>),

			AccountUnregisteredAddress(T::AccountId),
			AccountUnregisteredName(BoundedVec<u8, T::NameStringLimit>),

			AccountDataUpdatedAddress(T::AccountId),
			AccountDataUpdatedName(BoundedVec<u8, T::NameStringLimit>),

			TokensClaimed(T::AccountId),
		}
	


	//** Errors **//

		#[pallet::error]
		pub enum Error<T> {
			WalletAlreadyRegistered,
			WalletNotRegisteredStatTracker,
			WalletStatsNotFound,
			WalletTokensNotFound,

			DraftedModeratorNotRegistered,

			BadMetadata,
			WalletStatsRegistryRequired,
			
			TokenOverflow,
			ReputationOverflow,
			TokenUnderflow,
			ReputationUnderflow,
			NotEnoughBalance,
		}



	//** Hooks **//

		// #[pallet::hooks]
		// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	

		
	//** Extrinsics **//
		
		#[pallet::call]
		impl<T:Config> Pallet<T> {


			// Register a new wallet if previously unregistered.
			// This is required by many features in the app.
			#[pallet::call_index(0)]
			#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
			pub fn register_new_wallet(
				origin: OriginFor<T>,
				is_name_public: bool,
				is_wallet_public: bool,
				name: BoundedVec<u8, T::NameStringLimit>,
			) -> DispatchResultWithPostInfo {
				
				let who = ensure_signed(origin)?;
				ensure!(
					!WalletStats::<T>::contains_key(who.clone()), 
					Error::<T>::WalletAlreadyRegistered
				);
				
				let stats = Stats {
					is_wallet_public: is_wallet_public,
					is_name_public: is_name_public,
					name: name.clone(),
				};
				WalletStats::<T>::insert(who.clone(), stats.clone());

				if !WalletTokens::<T>::contains_key(who.clone()) {
					let zero_balance = BalanceOf::<T>::from(0u32);
					let tokens = Tokens {
						reputation_moderation: T::DefaultReputation::get(),
						locked_tokens_moderation: zero_balance.clone(),
						claimable_tokens_moderation: zero_balance.clone(),
						locked_tokens_festival: zero_balance.clone(),
						claimable_tokens_festival: zero_balance.clone(),
						locked_tokens_ranking: zero_balance.clone(),
						claimable_tokens_ranking: zero_balance.clone(),
						imbalance_tokens_ranking: (zero_balance.clone(), zero_balance.clone()),
						locked_tokens_movie: zero_balance.clone(),
						claimable_tokens_movie: zero_balance,
					};
					WalletTokens::<T>::insert(who.clone(), tokens.clone());
				};

				// check if events should be emitted, depending on the privacy settings
				if is_wallet_public {
					Self::deposit_event(Event::AccountRegisteredAddress(who));   
				}
				else if is_name_public {
					Self::deposit_event(Event::AccountRegisteredName(name));   
				};   

				Ok(().into())
			}


			// Unregister a wallet, automatically claiming any leftover tokens.
			//TODO-2
			//TODO-3
			#[pallet::call_index(1)]
			#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
			pub fn unregister_wallet(
				origin: OriginFor<T>,
				name: BoundedVec<u8, T::NameStringLimit>,
			) -> DispatchResultWithPostInfo {
				
				let who = ensure_signed(origin)?;

				let stats = WalletStats::<T>::try_get(who.clone()).unwrap();

				WalletStats::<T>::remove(who.clone());

				// check if events should be emitted, depending on the privacy settings
				if stats.is_wallet_public {
					Self::deposit_event(Event::AccountUnregisteredAddress(who));   
				}
				else if stats.is_name_public {
					Self::deposit_event(Event::AccountUnregisteredName(name));   
				}

				Ok(().into())
			}

		}
	
	
	
	//** Helpers **//
	
		impl<T:Config> Pallet<T> {
					

			// True if the wallet is registered in the "WalletStats" storage.
			// This always implies that an entry also exists e the 
			// "WalletTokens" storage.
			pub fn do_is_wallet_registered(
				who: T::AccountId,
			) -> Result<bool, DispatchError> {

				Ok(WalletStats::<T>::contains_key(who))
			}

		}
}