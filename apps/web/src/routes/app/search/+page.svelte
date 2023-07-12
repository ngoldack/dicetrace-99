<script lang="ts">
	export let data;
</script>

<div class="p-4">
	{#if data.query === undefined}
		<h2 class="h2 text-center mb-2">
			Use the search bar to find an item by
			<span class="italic font-light"> Name </span>
			or{' '}
			<span class="italic font-light">ID </span>
		</h2>
	{:else}
		<h2 class="h2 text-center mb-2">
			<span class="font-extrabold">{data.results.length}</span>
			<span class="font-extralight">search results for</span>
			<span class="font-extrabold">{data.query}</span>
			<span class="font-extralight">:</span>
		</h2>

		{#each data.results || [] as item}
			<div class="card p-2">
				<a href={`/app/item/${item.ID}`}>
					<header class="card-header">
						<span class="h3">
							{item.Name !== '' ? item.Name : item.AlternateNames ?? item.ID}
						</span>
					</header>
					<section class="p-4">{JSON.stringify(item)}</section>
					<footer class="card-footer" />
				</a>
			</div>
			<div />
		{/each}
	{/if}
</div>
