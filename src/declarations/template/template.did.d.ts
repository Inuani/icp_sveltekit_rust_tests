import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
	get_name: ActorMethod<[], string>;
	set_name: ActorMethod<[string], undefined>;
}
