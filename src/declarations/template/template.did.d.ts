import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface SegmentArgs {
	controllers: Array<Principal>;
	name: string;
}
export interface _SERVICE {
	get_name: ActorMethod<[], string>;
	set_name: ActorMethod<[string], undefined>;
}
