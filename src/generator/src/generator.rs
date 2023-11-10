
use ic_cdk::{caller, api};
use candid::{Principal, Encode};
use ic_cdk_macros::update;
pub const CREATE_CANISTER_CYCLES: u128 = 100_000_000_000u128;
use crate::types::{Society, SocietyArgs};
use crate::types::interface::SegmentArgs;
use crate::{SOCIETIES, get_societies, call};
use crate::types::ic::WasmArg;
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::api::management_canister::main::{ create_canister, install_code as ic_install_code
	, CanisterInstallMode, CanisterSettings,
    CreateCanisterArgument, InstallCodeArgument,
};

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
async fn create_society(args: SocietyArgs)-> Result<Principal, String>
{
	let wasm_args = get_wasm();

	if let Some(_) = SOCIETIES.with(|p| { p.borrow().get(&args.name)}) {
		return	Err("Society already exists".to_string());
	}
	let res = create_canister_install_code(&wasm_args).await;
	match res {
		Err(e) => {
			return Err(e);
		},
		Ok(res) => {
			SOCIETIES.with(|f| {
				f.borrow_mut().insert(args.name.clone(), Society {
					name: args.name.clone(),
					description: args.description.clone(),

					denizens_principals: vec![caller()],
					canister_id: res,
					owner: caller(),
					controllers: vec![caller(), api::id()],
				});
			});
			Ok(res)
		}
	}
}

#[update]
async fn update_societies() -> Result< (), (RejectionCode, String)> {
	// let dao = DAOS.into().bor;
	let wasm_arg = get_wasm();

	let societies = get_societies();
	for society in societies {
		let install: Result<(), (RejectionCode, String)> = install_code(society.canister_id, &wasm_arg, CanisterInstallMode::Upgrade).await;

		match install {
			Err(reject) => {
				return Err(reject);
			},
			Ok(_) => (),
		}
	}
	Ok(())
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
    ic_install_code(arg).await
}