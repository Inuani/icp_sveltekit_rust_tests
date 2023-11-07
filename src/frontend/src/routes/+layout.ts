export const prerender = true;
export const ssr: boolean = false;
export const trailingSlash = 'always';

import { goto } from '$app/navigation';
import { dS, syncAuth } from '$lib/stores';
import { get } from 'svelte/store';

export async function load({ url }) {
	await syncAuth();
	if (!get(dS).identity) {
		if (url.pathname !== '/login') {
			goto('/login');
		}
	}
}
