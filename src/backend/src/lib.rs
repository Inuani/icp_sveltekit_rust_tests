use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_cdk_macros::{query, update, export_candid};
use candid::{Decode, Encode, Principal, CandidType, Deserialize};
use ic_stable_structures::{ storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable };
use std::{borrow::Cow, cell::RefCell};

use crate::types::{Denizen, StablePrincipal};

mod types;
mod denizens { mod denizens; }

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static MAP: RefCell<StableBTreeMap<StablePrincipal, Denizen, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

#[query]
fn get(key: Principal) -> Option<Denizen> {
    let stable_key = StablePrincipal(key);
    MAP.with(|p| p.borrow().get(&stable_key))
}

#[update]
fn insert(key: Principal, value: Denizen) -> Option<Denizen> {
    let stable_key = StablePrincipal(key);
    MAP.with(|p| p.borrow_mut().insert(stable_key, value))
}

export_candid!();