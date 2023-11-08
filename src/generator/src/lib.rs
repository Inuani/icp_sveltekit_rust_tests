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


mod types;


type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // pub static H_DENIZENS: RefCell<BTreeMap<Principal, Denizen>> = RefCell::new(BTreeMap::new());

    // static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    //     RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // pub static S_DENIZENS: RefCell<StableBTreeMap<Principal, Denizen, Memory>> = RefCell::new(
    //     StableBTreeMap::init(
    //         MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
    //     )
    // );
}

#[update]
async fn generate(name: String)-> Result<Principal, ()>
{
	let args = WasmArg {
		wasm: std::include_bytes!("../../../.dfx/local/canisters/template/template.wasm").to_vec(),
		install_arg: Encode!(&SegmentArgs {
			
			controllers: vec![caller(), api::id()]
		}).unwrap()
	};

	let res = create_canister_install_code(&args).await.unwrap();
	// intercanister call, objectif call a init method with all the arg needed
	// api::call::call::<(String,), ()>(res.clone(), "set_name", (name,)).await;
	return Ok(res);
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
            let install = install_code(canister_id, wasm_arg, CanisterInstallMode::Install).await;

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