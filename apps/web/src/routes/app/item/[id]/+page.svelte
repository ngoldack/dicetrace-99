<script>
	import { AppBar } from '@skeletonlabs/skeleton';

	export let data;
	let item = data.item;
</script>

<div class="p-4">
	<div class="card p-2">
		<header class="card-header w-full">
			<AppBar>
				<svelte:fragment slot="lead">
					<h2 class="h2 font-bold">{item.name}</h2>
				</svelte:fragment>

				<svelte:fragment slot="headline">
					<div class="flex flex-col gap-1">
						<div class="flex flex-row items-center gap-4">
							<h5 class="h5">Categories:</h5>
							<div class="flex flex-row gap-1">
								{#each item.links.boardgamecategory || [] as category}
									<a
										class="chip variant-soft-secondary"
										href={`https://boardgamegeek.com/boardgamecategory/${category.id}`}
										>{category.name}</a
									>
								{/each}
							</div>
						</div>
						<div class="flex flex-row items-center gap-4">
							<h5 class="h5">Expansions:</h5>
							<div class="flex flex-row gap-1">
								{#each item.links.boardgameexpansion || [] as expansions}
									<a
										class="chip variant-soft-secondary"
										href={`https://boardgamegeek.com/boardgameexpansion/${expansions.id}`}
										>{expansions.name}</a
									>
								{/each}
							</div>
						</div>
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
		<section class="p-4">
			<img src={item.image} alt="Game Thumbnail" class="mb-4" />
			<h3 class="text-lg font-semibold">Alternate Names</h3>
			{#if !item.alternate_names || item.alternate_names.length === 0}
				<p class="italic font-light">No alternate names found.</p>
			{/if}
			{#each item.alternate_names || [] as name}
				<p>{name}</p>
			{/each}
			<p class="mt-4">{item.description}</p>
		</section>
		<footer class="card-footer">
			<p>Min Players: {item.min_players}</p>
			<p>Max Players: {item.max_players}</p>
		</footer>
	</div>
</div>
