<script lang="ts">
	import { goto } from '$app/navigation';
	import { env } from '$env/dynamic/public';
	import { Step, Stepper } from '@skeletonlabs/skeleton';
	import { toast } from 'svelte-sonner';

	export let data;

	let userdata = {
		name: '',
		bgg_username: '',
		email: data.session?.user?.email
	};

	async function submit() {
		try {
			const response = await fetch(`${env.PUBLIC_USERDATA_URL}`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(userdata)
			});

			if (response.ok) {
				toast.success('Profile saved');

				await goto('/app');
			} else {
				toast.error("Couldn't save profile");
			}
		} catch (err) {
			toast.error("Couldn't save profile");
			console.error(err);
		}
	}
</script>

<div class="w-full items-center justify-center h-full content-center flex">
	<div class="p-2 items-center max-w-xl">
		<Stepper on:complete={submit} class="w-full">
			<Step class="w-full">
				<svelte:fragment slot="header">Profile Setup</svelte:fragment>

				<p>
					Before using the app, we need to know a little bit about you. Please complete the
					following steps to get started.
				</p>
			</Step>
			<Step class="w-full">
				<svelte:fragment slot="header">Profile Setup</svelte:fragment>

				<label class="label">
					<span>Name</span>
					<input class="input" title="Name" type="text" bind:value={userdata.name} />
				</label>

				<label class="label">
					<span>Boardgamegeek Username</span>
					<input
						class="input"
						title="Boardgamegeek Username"
						type="text"
						bind:value={userdata.bgg_username}
					/>
				</label>
			</Step>
		</Stepper>
	</div>
</div>
