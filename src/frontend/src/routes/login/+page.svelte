<script lang="ts">
	import { dS, login, logout } from '$lib/stores';

	const handleLoginII = async () => {
		login();
	};

	const handleLogout = async () => {
		logout();
	};

	let dname = '';
	let xp = 0n;
	let level = 0n;
	let tokenBalance = 0n;

	const createDenizen = async () => {
		if ($dS.backend) {
			try {
				const denizen = await $dS.backend.create_denizen(dname, xp, level, tokenBalance);
				console.log('Denizen created:', denizen);
			} catch (error) {
				console.error('Error creating denizen:', error);
			}
		}
	};

	let denizens: any = [];

	const fetchDenizens = async () => {
		if ($dS.backend) {
			try {
				denizens = await $dS.backend.get_all_denizens();
				console.log('Fetched denizens:', denizens);
			} catch (error) {
				console.error('Error fetching denizens:', error);
			}
		}
	};

	$: if ($dS.backend) {
		fetchDenizens();
	}
</script>

<section>
	<br />
	<br />

	{#if $dS.identity}
		logged in with
		{$dS.principal}
		<br />

		<form on:submit|preventDefault={createDenizen}>
			<input type="text" bind:value={dname} placeholder="Name" />
			<input type="number" bind:value={xp} placeholder="XP" />
			<input type="number" bind:value={level} placeholder="Level" />
			<input type="number" bind:value={tokenBalance} placeholder="Token Balance" />
			<button type="submit">Create Denizen</button>
		</form>

		<br />

		{#if $dS.backend && denizens}
			{#each denizens as denizen (denizen.principal)}
				<div>
					Principal: {denizen.principal}
					Name: {denizen.dname}
					XP: {denizen.xp}
					Level: {denizen.level}
					Token Balance: {denizen.token_balance}
				</div>
			{/each}
		{/if}

		<button on:click={handleLogout}>Logout</button>
	{:else}
		<button on:click={handleLoginII}>Login with internet identity</button>
	{/if}

	<br />
	<br />
</section>

<style lang="scss">
	button[type='submit'] {
		padding: 5px 20px;
		margin: 10px auto;
		float: right;
	}
</style>
