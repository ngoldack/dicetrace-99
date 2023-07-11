<script lang="ts">
	export let data;
</script>

<div class="p-4">
	{#each data.collection as item}
		<div class="card p-2">
			<a href={`/app/item/${item.id}`}>
				<header class="card-header">
					<span class="h3">
						{item.name !== '' ? item.name : item.alternate ?? item.ID}
					</span>
				</header>
				<section class="p-4">{JSON.stringify(item)}</section>
				<footer class="card-footer">Test</footer>
			</a>
		</div>
	{/each}
</div>
