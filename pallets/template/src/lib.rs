#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get, StorageMap,
};
use frame_system::ensure_signed;
use pallet_timestamp;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait + pallet_timestamp::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	//type Moment: Parameter + Default + AtLeast32Bit + Scale<Self::BlockNumber, Output = Self::Moment> + Copy;
	//type OnTimestampSet: OnTimestampSet<Self::Moment>;
	//type MinimumPeriod: Get<Self::Moment>;
	//type WeightInfo: WeightInfo;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		//Something get(fn something): Option<u32>;
		AccountInfo :map hasher(blake2_128_concat) T::AccountId=>(Vec<u8>,Vec<u8>);
		AccountJoin get(fn  get_test):map hasher(blake2_128_concat) T::AccountId => Vec<u8>;
		Messages :map hasher(blake2_128_concat) u8 => Vec<(T::AccountId,Vec<u8>,Vec<u8>,T::Moment)>;//Moment=u64
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
		Moment = <T as pallet_timestamp::Trait>::Moment,
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		//SomethingStored(u32, AccountId),
		UserInfo(AccountId, Vec<u8>),
		Rooms(AccountId, Vec<u8>),
		UserJoin(AccountId, u8),
		SendOK(u32),
		GetMessage(u8, Vec<(AccountId, Vec<u8>, Vec<u8>, Moment)>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		UserExist,
		NoUser,
		SendFailed,
		BeyondBound,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;



		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn new_user(origin,name:Vec<u8>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!AccountJoin::<T>::contains_key(&who),Error::<T>::UserExist);
			let null:Vec<u8>=Vec::new();
			AccountJoin::<T>::insert(who.clone(),null.clone());
			AccountInfo::<T>::insert(who.clone(),(name,null.clone()));
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads(1)]
		pub fn get_user_info(origin) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(AccountInfo::<T>::contains_key(&who),Error::<T>::NoUser);
			let (name,_) =AccountInfo::<T>::get(&who);

			Self::deposit_event(RawEvent::UserInfo(who,name));
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn get_rooms(origin) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let rooms:Vec<u8>;
			if AccountJoin::<T>::contains_key(&who){
				rooms=AccountJoin::<T>::get(&who);
				Self::deposit_event(RawEvent::Rooms(who,rooms));
			}
			else {
				//rooms=Vec::new();
				//AccountJoin::<T>::insert(who.clone(),rooms.clone());
				Err(Error::<T>::NoUser)?;
			}
			//Self::deposit_event(RawEvent::Rooms(who,rooms));

			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn join_room(origin, id: u8) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			//ensure!(AccountInfo::<T>::contains_key(&who),Error::<T>::NoUser);
			if AccountJoin::<T>::contains_key(&who){
				//let mut rooms=AccountInfo::<T>::get(&who);
				//rooms.push(id);
				AccountJoin::<T>::try_mutate(&who,|x:&mut Vec<u8>|->Result<(),dispatch::DispatchError>{ x.push(id); Ok(())})?;
				//frame_support::debug::native::debug!("{} {}",&who,AccountInfo::<T>::get(&who).clone().len());
			}
			else {
				Err(Error::<T>::NoUser)?;
			}

			// Update storage.
			//Something::put(something);

			// Emit an event.
			//Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Self::deposit_event(RawEvent::UserJoin(who,id));
			Ok(())
		}


		#[weight = 10_000 + T::DbWeight::get().reads_writes(2,1)]
		pub fn send_message(origin, room: u8, message: Vec<u8>, index: u32) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(AccountInfo::<T>::contains_key(&who),Error::<T>::NoUser);
			//ensure user join room
			let now=<pallet_timestamp::Module<T>>::get();
			if !Messages::<T>::contains_key(&room){
				let t:Vec<(T::AccountId,Vec<u8>,Vec<u8>,T::Moment)>=Vec::new();
				Messages::<T>::insert(&room,t);
			}
			let (name,_) =AccountInfo::<T>::get(&who);
			//let data=vec![(who.clone(),message,now)];
			Messages::<T>::try_mutate(&room,|x:&mut Vec<(T::AccountId,Vec<u8>,Vec<u8>,T::Moment)>| -> Result<(),dispatch::DispatchError> {x.push((who.clone(),name,message,now)); Ok(())})?;
			Self::deposit_event(RawEvent::SendOK(index));
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads(3)]
		pub fn get_message(origin, room: u8, offset_new: u32, count: u8) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(AccountInfo::<T>::contains_key(&who),Error::<T>::NoUser);
			//ensure user join room
			ensure!(Messages::<T>::contains_key(&room),Error::<T>::NoUser);
			let message=Messages::<T>::get(&room);
			let len=message.len();
			let end=offset_new + count as u32;
			ensure!(end < len as u32,Error::<T>::BeyondBound);
			let m:Vec<(T::AccountId,Vec<u8>,Vec<u8>,T::Moment)>=message[(offset_new as usize)..(end as usize)].to_vec();
			Self::deposit_event(RawEvent::GetMessage(room,m));
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			/*
			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}*/
			Ok(())
		}
	}
}
