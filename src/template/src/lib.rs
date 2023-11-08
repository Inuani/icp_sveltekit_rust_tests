// use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
// use ic_cdk_macros::{query, update, export_candid, pre_upgrade, post_upgrade};
// use candid::{Decode, Encode, Principal, CandidType, Deserialize};
// use ic_stable_structures::{ storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable };
use std::{cell::RefCell};

use ic_cdk::{export_candid, query, update};
// use ic_cdk_macros::{update, export_candid, query};

mod types;
// mod denizens { mod denizens; }

// type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static NAME: RefCell<String> = RefCell::new(String::new());
}

#[query]
fn get_name() -> String
{
	NAME.with(|n| n.borrow().clone())
}

#[update]
fn set_name(name: String) -> ()
{
	NAME.with(|n| *n.borrow_mut() = name);
}

export_candid!();