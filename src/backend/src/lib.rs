use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_cdk_macros::{query, update, export_candid, pre_upgrade, post_upgrade};
use candid::{Decode, Encode, Principal, CandidType, Deserialize};
use ic_stable_structures::{ storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable };
use std::{borrow::Cow, cell::RefCell, collections::BTreeMap};

use crate::types::{Denizen, StablePrincipal};

mod types;
mod denizens { mod denizens; }

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static H_DENIZENS: RefCell<BTreeMap<Principal, Denizen>> = RefCell::new(BTreeMap::new());

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static S_DENIZENS: RefCell<StableBTreeMap<Principal, Denizen, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

#[pre_upgrade]
fn pre_upgrade() {
    // Move data from HEAP_DENIZENS to STABLE_DENIZENS
    H_DENIZENS.with(|heap| {
        let heap_map = heap.borrow();
        S_DENIZENS.with(|stable| {
            let mut stable_map = stable.borrow_mut();
            for (key, value) in heap_map.iter() {
                stable_map.insert(StablePrincipal(*key), value.clone());
            }
        });
    });
}

#[post_upgrade]
fn post_upgrade() {
    // Move data back from STABLE_DENIZENS to HEAP_DENIZENS
    S_DENIZENS.with(|stable| {
        let stable_map = stable.borrow();
        H_DENIZENS.with(|heap| {
            let mut heap_map = heap.borrow_mut();
            // Iterate over the stable map and insert the data back into the heap map
            for (stable_key, denizen) in stable_map.iter() {
                heap_map.insert(stable_key.into_inner(), denizen.clone());
            }
        });
    });
}




#[query]
fn get(key: Principal) -> Option<Denizen> {
    // let stable_key = StablePrincipal(key);
    S_DENIZENS.with(|p| p.borrow().get(&key))
}

#[update]
fn insert(key: Principal, value: Denizen) -> Option<Denizen> {
    let stable_key = StablePrincipal(key);
    S_DENIZENS.with(|p| p.borrow_mut().insert(stable_key, value))
}

export_candid!();