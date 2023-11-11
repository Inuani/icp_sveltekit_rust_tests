import { dS } from '$lib/stores';
import { get } from 'svelte/store';
import { Actor, HttpAgent } from '@dfinity/agent';
import { host } from '$lib/constants';
import { idlFactory } from '../../../../declarations/template/template.did';

export async function load({ params }) {
	const { society } = params;
	const society_got = await get(dS).generator.get_society(society);
	const agent = new HttpAgent({ host });
	agent.fetchRootKey().catch(console.error);
	console.log(society_got);
	const newCanisterActor = Actor.createActor(idlFactory, {
		agent,
		canisterId: society_got.Ok.canister_id
	});
	const getName = await newCanisterActor.get_name();
	console.log('soc:', getName);
}
