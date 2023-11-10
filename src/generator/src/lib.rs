use std::borrow::Cow;
use std::cell::RefCell;

use ic_cdk::{caller, export_candid};
use ic_stable_structures::memory_manager::{ VirtualMemory};
use ic_cdk_macros::{query, update};
use candid::{Principal};
use ic_stable_structures::{DefaultMemoryImpl };
use types::{Society, SocietyArgs, SocietyInfos};
pub const CREATE_CANISTER_CYCLES: u128 = 100_000_000_000u128;
use ic_cdk::api::call::{RejectionCode};
use ic_stable_structures::{StableBTreeMap, memory_manager::MemoryId, memory_manager::MemoryManager};

mod call;
mod types;
mod generator;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // pub static H_DENIZENS: RefCell<BTreeMap<Principal, Denizen>> = RefCell::new(BTreeMap::new());

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static SOCIETIES: RefCell<StableBTreeMap<String, Society, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

#[query]
fn get_societies() -> Vec<SocietyInfos>{

	SOCIETIES.with(|societies|
		{
			societies.borrow().iter().collect::<Vec<(String, Society)>>().iter().map(|(name, society)| {
				SocietyInfos {
					name: name.clone(),
					description: society.description.clone(),
					canister_id: society.canister_id.clone(),
					is_denisen: society.denizens_principals.contains(&caller()),
					a_denizens: society.denizens_principals.len() as u64,
				}
			}).collect::<Vec<SocietyInfos>>()
		}
	)
}

#[query]
fn get_my_societies() -> Vec<SocietyInfos>{

	SOCIETIES.with(|societies|
		{
			societies.borrow().iter().collect::<Vec<(String, Society)>>().iter().filter(|(name, society)| {
				society.denizens_principals.contains(&caller())
			}).map(|(name, society)| SocietyInfos {
				name: name.clone(),
				description: society.description.clone(),
				canister_id: society.canister_id.clone(),
				is_denisen: society.denizens_principals.contains(&caller()),
				a_denizens: society.denizens_principals.len() as u64,
			}).collect::<Vec<SocietyInfos>>()
		}
	)
}

#[update]
fn update_society(name: String, society: SocietyArgs) -> Result<(), String>
{
	SOCIETIES.with(|societies|
	{
		let oldsociety: Option<Society> = societies.borrow_mut().get(&name);
		match oldsociety {
			Some(mut oldsociety) => {
				if oldsociety.owner != caller() {
					return Err("Not owner".to_string());
				}
				societies.borrow_mut().remove(&name);
				oldsociety.name = society.name.clone();
				oldsociety.description = society.description;
				societies.borrow_mut().insert(society.name.clone(), oldsociety);
				Ok(())
			},
			None => {
				Err("Society not found".to_string())
			}
		}
	}
	)
}

#[query]
fn get_society(name: String) -> Result<Society, String> {
    SOCIETIES.with(|societies| {
        let societies_borrowed = societies.borrow();
        match societies_borrowed.get(&name) {
            Some(society) => Ok(society.clone()),
            None => Err("Society not found".to_string()),
        }
    })
}


export_candid!();