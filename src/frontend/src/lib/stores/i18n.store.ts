import { writable } from 'svelte/store';
import en from '$lib/i18n/en.json';

// Initial state of the store
const initialState = {
	lang: 'en',
	...en
};

const createI18nStore = () => {
	const { subscribe, set } = writable(initialState);

	// Function to switch language
	const switchLang = async (lang) => {
		const translations = await import(`../i18n/${lang}.json`); // Lazy load translations
		set({ lang, ...translations });
	};

	return {
		subscribe,
		switchLang
	};
};

export const i18n = createI18nStore();
