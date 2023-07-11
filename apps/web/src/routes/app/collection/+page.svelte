<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { onDestroy, onMount } from 'svelte';
	import { page } from '$app/stores';
	import { AppBar } from '@skeletonlabs/skeleton';

	export let data;

	onMount(async () => {
		// invalidate the current page so that future requests will refetch
		await invalidate($page.url.pathname);
	});
</script>

<div class="p-4 max-w-full flex flex-col gap-2">
	{#each data.collection as item}
		<a
			class="card p-2 flex flex-row items-center w-full h-full shadow-xl"
			href={`/app/item/${item.id}`}
		>
			<div class="min-w-[20%] max-h-[20%] items-center justify-center flex">
				<img src={item.thumbnail} alt={item.name} />
			</div>
			<div class="w-full h-full items-start">
				<header class="card-header flex flex-col gap-1">
					<AppBar class="w-full">
						<svelte:fragment slot="lead">
							<div class="flex flex-col">
								<span class="h3">
									{item.name !== '' ? item.name : item.alternate}
								</span>
								<span class="text-sm font-extralight">
									ID: {item.id}
								</span>
							</div>
						</svelte:fragment>

						<svelte:fragment slot="trail">
							<a
								href={`https://boardgamegeek.com/thing/${item.id}`}
								class="btn variant-outline-primary"
							>
								Open in BGG
							</a>
						</svelte:fragment>
					</AppBar>
				</header>
				<section class="flex flex-row gap-2">
					<span class="chip variant-outline-primary">
						Published: {item.year_published}
					</span>
					<span class="chip variant-outline-secondary min-w-fit">
						Status: {item.collection_status}
					</span>
				</section>
			</div>
		</a>
	{/each}
</div>
