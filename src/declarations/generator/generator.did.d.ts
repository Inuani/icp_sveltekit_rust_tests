import type { ActorMethod } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';

export interface Dao {
	id: Principal;
	controllers: Array<Principal>;
	owner: Principal;
	name: string;
}
export type RejectionCode =
	| { NoError: null }
	| { CanisterError: null }
	| { SysTransient: null }
	| { DestinationInvalid: null }
	| { Unknown: null }
	| { SysFatal: null }
	| { CanisterReject: null };
export type Result = { Ok: [string] } | { Err: [RejectionCode, string] };
export type Result_1 = { Ok: Principal } | { Err: null };
export type Result_2 = { Ok: null } | { Err: string };
export interface _SERVICE {
	call_dao_function: ActorMethod<[string, string], Result>;
	generate: ActorMethod<[string], Result_1>;
	get_daos: ActorMethod<[], Array<[string, Dao]>>;
	set_dao_name: ActorMethod<[string, string], undefined>;
	update_daos_code: ActorMethod<[], Result_2>;
}
