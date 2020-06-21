#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	traits::{Get, Currency, ExistenceRequirement},
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	// 附加题答案
	type MaxClaimLength: Get<u32>;
	type MaxMemoLength: Get<u32>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, Vec<u8>);
		AccountProofs get(fn price): map hasher(blake2_128_concat) T::AccountId => Vec<Vec<u8>>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where 
			AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ProofTooLong,
		MemoTooLong,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>, memo: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
			// 附加题答案
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);
			ensure!(T::MaxMemoLength::get() >= memo.len() as u32, Error::<T>::MemoTooLong);
			
			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number(), &memo));
			AccountProofs::<T>::append(&sender, &claim);

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim, memo));
			Ok(())
		}

	}
}
