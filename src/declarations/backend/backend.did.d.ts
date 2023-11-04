import type { ActorMethod } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';

export interface Denizen {
	xp: bigint;
	principal: Principal;
	level: bigint;
	token_balance: bigint;
	dname: string;
}
export interface _SERVICE {
	create_denizen: ActorMethod<[string, bigint, bigint, bigint], [] | [Denizen]>;
	get: ActorMethod<[Principal], [] | [Denizen]>;
	get_all_denizens: ActorMethod<[], Array<Denizen>>;
	insert: ActorMethod<[Principal, Denizen], [] | [Denizen]>;
}
