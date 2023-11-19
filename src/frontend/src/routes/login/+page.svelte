<script lang="ts">
	import { dS, login, logout } from '$lib/stores';
	import { i18n } from '$lib/stores/i18n.store';

	const handleLoginII = async () => {
		login();
	};

	const handleLogout = async () => {
		logout();
	};

</script>

<section>
	<br />
	<br />

	{#if $dS.identity}
	<p>{$i18n.core.yes}</p>
	{$i18n.core.logged_in}
		<button on:click={handleLogout}>Logout</button>
	{:else}
		<button on:click={handleLoginII}>Login with internet identity</button>
	{/if}

	<br />
	<br />
</section>