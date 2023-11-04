import type { ActorSubclass, Identity } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import type { _SERVICE } from '../../../declarations/backend/backend.did';

export type All = {
	backend: ActorSubclass<_SERVICE> | null;
	isAuth: boolean | null;
	principal: Principal | null;
	// authClient: AuthClient | null;
	delegIdentity: Identity | null;
};

export let all: All = {
	backend: null,
	isAuth: null,
	principal: null,
	delegIdentity: null
};
