
use ic_cdk::{query, update, caller};
use crate::types::{ Denizen , StablePrincipal};
use crate::H_DENIZENS;
use candid::{ Principal };

#[update]
fn create_denizen( 
    dname: String,
    firstname: Option<String>,
    xp: u64, 
    level: u64, 
    token_balance: u64
) -> Option<Denizen> {
    let caller: Principal = caller();
    let denizen = Denizen {
        principal: caller,
        dname,
        firstname,
        xp,
        level,
        token_balance,
    };
    insert_denizen(caller, denizen)
}

fn insert_denizen(key: Principal, value: Denizen) -> Option<Denizen> {
    // let stable_key = StablePrincipal(key);
    H_DENIZENS.with(|p| p.borrow_mut().insert(key, value))
}

#[query]
fn get_all_denizens() -> Vec<Denizen> {
    H_DENIZENS.with(|map| {
        map.borrow()
            .iter() 
            .map(|(_key, value)| value.clone())
            .collect()
    })
}
