import { dS } from '$lib/stores';
import { get } from 'svelte/store';

export async function load({ params }) {
	const { society } = params;
	const society_got = await get(dS).generator.get_society(society);
	console.log(society_got);
}
