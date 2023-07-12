<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="p-2 justify-center">
	<div class="card p-2 max-w-xl">
		<header class="card-header">
			<h2 class="h2 text-center">{data.group.name}</h2>
		</header>
		<hr class="!border-t-2" />
		<section class="p-2">
			<h4 class="h4">Members:</h4>
			<ul class="list w-full">
				{#each data.members as member}
					<li class="justify-center">
						<span>{member.name}</span>
						<a
							href="/app/collection?username={member.bgg_username}"
							class="btn variant-outline-primary">View collection</a
						>
					</li>
				{/each}
			</ul>
		</section>
		<hr class="!border-t-2" />
	</div>
</div>
