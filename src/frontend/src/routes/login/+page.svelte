<script lang="ts">
	import { dS, login, logout } from '$lib/stores';

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
		logged in!
		<button on:click={handleLogout}>Logout</button>
	{:else}
		<button on:click={handleLoginII}>Login with internet identity</button>
	{/if}

	<br />
	<br />
</section>