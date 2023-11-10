import type { Principal } from '@dfinity/principal';

export const generatorCanisterId: string | Principal = import.meta.env.VITE_GENERATOR_CANISTER_ID;

export const FrontendCanisterId: string | Principal = import.meta.env.VITE_FRONTEND_CANISTER_ID;

export const host = import.meta.env.VITE_HOST;
