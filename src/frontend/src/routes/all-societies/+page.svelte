<script lang="ts">
	import type { fromHex } from '@dfinity/agent';
    import type { SocietyInfos } from '../../../../declarations/generator/generator.did.d';
	import { Principal } from '@dfinity/principal'; 
    import { dS } from '$lib/stores';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let societies: SocietyInfos[] = [];

    onMount(async () => {
        societies = await $dS.generator.get_societies();
    });

    function goToSociety(societyName: string) {
        goto(`/${societyName}`);
    }

</script>



{#each societies as society}
    <div class="society">
        <h2>{society.name}</h2>
        <p>Description: {society.description}</p>
        <p>Number of Denizens: {society.a_denizens}</p>
        <button on:click={() => goToSociety(society.name)}> goto this soc </button>
    </div>
{/each}
      


<style lang="scss">
  
</style>
