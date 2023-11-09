use ic_cdk::api::{call::RejectionCode, self};
use ic_cdk_macros::update;

use crate::{types::Society, SOCIETIES};

#[update]
async fn set_society_name(name: String, new: String) -> Result<(), (RejectionCode, String)> {
	// let Society = SocietyS.into().bor;

	let society: Society = SOCIETIES.with(|p|
		{
			p.borrow().get(&name).unwrap()
		}
	);
	api::call::call::<(String,), ()>(society.canister_id.clone(), "set_name", (new,)).await
}

#[update]
async fn call_society_function(name: String, function: String) -> Result<(String,), (RejectionCode, std::string::String)>{
	let society: Society = SOCIETIES.with(|p|
		{
			p.borrow().get(&name).unwrap()
		}
	);
	api::call::call::<(), (String,)>(society.canister_id.clone(), &function, ()).await
}