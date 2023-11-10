<script lang="ts">
	import { Actor, HttpAgent } from '@dfinity/agent';
    import { createActor as createGeneratorActor } from '../../../declarations/generator';
    import type { SocietyArgs } from '../../../declarations/generator/generator.did.d';
	import { idlFactory as newCanisterIdl } from '../../../declarations/template/template.did'; 
	import { Principal } from '@dfinity/principal';

    let societyName = '';
    let societyDescription = '';
    let disabled = false;
    let creationResult = '';

    const handleCreateSociety = async () => {
        disabled = true;
        creationResult = '';

        try {
            // Canister IDs are automatically expanded to .env config - see vite.config.ts
            const canisterId = import.meta.env.VITE_GENERATOR_CANISTER_ID;

            // We pass the host instead of using a proxy to support NodeJS >= v17 (ViteJS issue: https://github.com/vitejs/vite/issues/4794)
            const host = import.meta.env.VITE_HOST;

            // Create an actor to interact with the IC for a particular canister ID
            const actor = createGeneratorActor(canisterId, { agentOptions: { host } });

            // Construct the arguments for the `create_society` call
            const args: SocietyArgs = {
                name: societyName,
                description: societyDescription
            };

            // Call the `create_society` method on the IC
            const result = await actor.create_society(args);
			console.log(result.Ok);
			console.log(result);
			if ('Ok' in result) {
				const principalNewCanister = result.Ok;
				const newCanisterActor = createActorForNewCanister(principalNewCanister, host);

				const nameOfSoc = await newCanisterActor.set_name('oudaa');
				const getName = await newCanisterActor.get_name();
				console.log('soc:', getName);
			}
			
            creationResult = `Society created with ID: ${result}`;
        } catch (err: unknown) {
            console.error(err);
            creationResult = 'Failed to create society. See console for details.';
        }

        disabled = false;
    };

	function createActorForNewCanister(principal, host) {
        const agent = new HttpAgent({ host });
        
            agent.fetchRootKey().catch(console.error);
    
        return Actor.createActor(newCanisterIdl, { agent, canisterId: principal });
    };
</script>

<main>
    <!-- ... other HTML ... -->

    <form on:submit|preventDefault={handleCreateSociety}>
        <label for="society-name">Society Name: &nbsp;</label>
        <input id="society-name" type="text" bind:value={societyName} {disabled} />

        <label for="society-description">Description: &nbsp;</label>
        <input id="society-description" type="text" bind:value={societyDescription} {disabled} />

        <button type="submit" {disabled}>Create Society</button>
    </form>

    <section id="creation-result">
        {creationResult}
    </section>
</main>

<!-- ... styles ... -->
