import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
	get_jesus: ActorMethod<[], string>;
	get_name: ActorMethod<[], string>;
	get_pancreas: ActorMethod<[], string>;
	set_name: ActorMethod<[string], undefined>;
}
