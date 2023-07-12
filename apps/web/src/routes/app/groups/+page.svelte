<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="p-2">
	<h2 class="h2 text-center">Your groups</h2>
	<div class="flex flex-row w-full gap-2">
		{#each data.groups || [] as group}
			<div class="p-2">
				<div class="card">
					<header class="card-header">
						<h2 class="h2">{group.name}</h2>
						<h6 class="text-center font-extralight text-sm">Members: {group.users.length}</h6>
					</header>
					<footer class="card-footer flex flex-col">
						<a class="btn variant-outline-primary" href="/app/groups/{group.id}">Show</a>
					</footer>
				</div>
			</div>
		{/each}
	</div>
</div>
