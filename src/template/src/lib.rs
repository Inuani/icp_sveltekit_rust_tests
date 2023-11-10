use std::{cell::RefCell};
use ic_cdk::{export_candid, query, update, init};
use candid::{Principal, CandidType, Deserialize};

mod types;

thread_local! {
    pub static NAME: RefCell<String> = RefCell::new(String::new());
}
pub type ControllerId = Principal;

#[derive(CandidType, Deserialize)]
    pub struct SegmentArgs {
        pub controllers: Vec<ControllerId>,
        pub name: String,
    }

#[init]
fn init(args: SegmentArgs) {
    set_name(args.name);
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