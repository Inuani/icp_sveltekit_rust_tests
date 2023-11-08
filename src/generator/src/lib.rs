use std::borrow::Cow;
use std::cell::RefCell;

use ic_cdk::{caller, api, export_candid};
use ic_stable_structures::memory_manager::{ VirtualMemory};
use ic_cdk_macros::{query, update, pre_upgrade, post_upgrade};
use candid::{Decode, Encode, Principal, CandidType, Deserialize};
use ic_stable_structures::{DefaultMemoryImpl };
pub const CREATE_CANISTER_CYCLES: u128 = 100_000_000_000u128;
use crate::types::ic::WasmArg;
use crate::types::interface::SegmentArgs;
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::api::management_canister::main::{ create_canister, install_code as ic_install_code
	, CanisterInstallMode, CanisterSettings,
    CreateCanisterArgument, InstallCodeArgument,
};

use ic_stable_structures::{StableBTreeMap, Storable, storable::Bound, memory_manager::MemoryId, memory_manager::MemoryManager};
const MAX_VALUE_SIZE: u32 = 1000;

mod types;


#[derive(CandidType, Deserialize, Clone)]
pub struct Dao {
	pub name: String,
	pub id: Principal,
	pub owner: Principal,
	pub controllers: Vec<Principal>,

}
impl Storable for Dao {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
  }

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // pub static H_DENIZENS: RefCell<BTreeMap<Principal, Denizen>> = RefCell::new(BTreeMap::new());

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static DAOS: RefCell<StableBTreeMap<String, Dao, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

fn get_wasm() -> WasmArg
{
	WasmArg {
		wasm: std::include_bytes!("../../../.dfx/local/canisters/template/template.wasm").to_vec(),
		install_arg: Encode!(&SegmentArgs {
			
			controllers: vec![caller(), api::id()]
		}).unwrap()
	}
}

#[update]
async fn generate(name: String)-> Result<Principal, ()>
{
	let args = get_wasm();

	let res = create_canister_install_code(&args).await.unwrap();
	DAOS.with(|f| {
		f.borrow_mut().insert(name.clone(), Dao {
			name: name.clone(),
			id: res,
			owner: caller(),
			controllers: vec![caller(), api::id()],
		});
	});
	return Ok(res);
}

#[update]
async fn update_daos_code() -> Result< (), String> {
	// let dao = DAOS.into().bor;
	let wasm_arg = get_wasm();

	let daos = get_daos();
	for i in daos {
		let install: Result<(), (RejectionCode, String)> = install_code(i.1.id, &wasm_arg, CanisterInstallMode::Upgrade).await;

		match install {
			Err(reject) => {
				return Err(reject.1 + "MMMMMMM");
			},
			Ok(_) => (),
		}
	}
	Ok(())
}

#[update]
async fn set_dao_name(name: String, new: String) -> () {
	// let dao = DAOS.into().bor;

	let dao = DAOS.with(|p|
		{
			p.borrow().get(&name).unwrap()
		}
	);
	api::call::call::<(String,), ()>(dao.id.clone(), "set_name", (new,)).await;
}

#[update]
async fn call_dao_function(name: String, function: String) -> Result<(String,), (RejectionCode, std::string::String)>{
	let dao: Dao = DAOS.with(|p|
		{
			p.borrow().get(&name).unwrap()
		}
	);
	api::call::call::<(), (String,)>(dao.id.clone(), &function, ()).await
}

#[query]
fn get_daos() -> Vec<(String, Dao)>{
	// let dao = DAOS.into().bor;

	DAOS.with(|p|
		{
			p.borrow().iter().collect::<Vec<(String, Dao)>>()
		}
	)
}

pub async fn create_canister_install_code(wasm_arg: &WasmArg) -> Result<Principal, String> {
	let controllers = vec![caller(), api::id()];
    let record = create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(controllers.clone()),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            }),
        },
        CREATE_CANISTER_CYCLES,
    )
    .await;
    match record {
        Err((_, message)) => Err(["Failed to create canister.", &message].join(" - ")),
        Ok(record) => {
            let canister_id = record.0.canister_id;
            let install: Result<(), (RejectionCode, String)> = install_code(canister_id, wasm_arg, CanisterInstallMode::Install).await;

            match install {
                Err(reject) => Err(reject.1.to_string()),
                Ok(_) => Ok(canister_id),
            }
        }
    }
}

async fn install_code(
    canister_id: Principal,
    WasmArg { wasm, install_arg }: &WasmArg,
    mode: CanisterInstallMode,
) -> CallResult<()> {
    let arg = InstallCodeArgument {
        mode,
        canister_id,
        wasm_module: wasm.clone(),
        arg: install_arg.clone(),
    };

	// print(format!("install_arg: {:?}", install_arg).as_str());

    ic_install_code(arg).await
}

// pub async fn update_canister_controllers(
//     canister_id: Principal,
//     controllers: Vec<Principal>,
// ) -> CallResult<()> {
//     let arg = UpdateSettingsArgument {
//         canister_id,
//         settings: CanisterSettings {
//             controllers: Some(controllers),
//             compute_allocation: None,
//             memory_allocation: None,
//             freezing_threshold: None,
//         },
//     };

//     update_settings(arg).await
// }

// pub async fn segment_status(canister_id: CanisterId) -> SegmentStatusResult {
//     let status = ic_canister_status(CanisterIdRecord { canister_id }).await;

//     match status {
//         Ok((status,)) => Ok(SegmentStatus {
//             id: canister_id,
//             metadata: None,
//             status,
//             status_at: time(),
//         }),
//         Err((_, message)) => Err(["Failed to get canister status: ".to_string(), message].join("")),
//     }
// }

export_candid!();