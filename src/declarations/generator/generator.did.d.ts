import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type RejectionCode =
	| { NoError: null }
	| { CanisterError: null }
	| { SysTransient: null }
	| { DestinationInvalid: null }
	| { Unknown: null }
	| { SysFatal: null }
	| { CanisterReject: null };
export type Result = { Ok: [string] } | { Err: [RejectionCode, string] };
export type Result_1 = { Ok: Principal } | { Err: string };
export type Result_2 = { Ok: Society } | { Err: string };
export type Result_3 = { Ok: null } | { Err: [RejectionCode, string] };
export type Result_4 = { Ok: null } | { Err: string };
export interface Society {
	denizens_principals: Array<Principal>;
	controllers: Array<Principal>;
	owner: Principal;
	name: string;
	canister_id: Principal;
	description: string;
}
export interface SocietyArgs {
	name: string;
	description: string;
}
export interface SocietyInfos {
	is_denisen: boolean;
	name: string;
	canister_id: Principal;
	description: string;
	a_denizens: bigint;
}
export interface _SERVICE {
	call_society_function: ActorMethod<[string, string], Result>;
	create_society: ActorMethod<[SocietyArgs], Result_1>;
	get_my_societies: ActorMethod<[], Array<SocietyInfos>>;
	get_societies: ActorMethod<[], Array<SocietyInfos>>;
	get_society: ActorMethod<[string], Result_2>;
	set_society_name: ActorMethod<[string, string], Result_3>;
	update_societies_wasm_code: ActorMethod<[], Result_3>;
	update_society: ActorMethod<[string, SocietyArgs], Result_4>;
}
