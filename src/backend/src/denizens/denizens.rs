
use ic_cdk::{query, update, caller};
use crate::types::{ Denizen , StablePrincipal};
use crate::MAP;
use candid::{ Principal };

#[update]
fn create_denizen( 
    dname: String, 
    xp: u64, 
    level: u64, 
    token_balance: u64
) -> Option<Denizen> {
    let caller: Principal = caller();
    let denizen = Denizen {
        principal: caller,
        dname,
        xp,
        level,
        token_balance,
    };
    insert_denizen(caller, denizen)
}

fn insert_denizen(key: Principal, value: Denizen) -> Option<Denizen> {
    let stable_key = StablePrincipal(key);
    MAP.with(|p| p.borrow_mut().insert(stable_key, value))
}

#[query]
fn get_all_denizens() -> Vec<Denizen> {
    MAP.with(|map| {
        map.borrow()
            .iter() 
            .map(|(_key, value)| value.clone())
            .collect()
    })
}

// no use
// #[query]
// fn get_denizen(key: Principal) -> Option<Denizen> {
//     let stable_key = StablePrincipal(key);
//     MAP.with(|p| p.borrow().get(&stable_key).cloned())
// }

