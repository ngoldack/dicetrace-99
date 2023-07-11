<script lang="ts">
	import { page } from '$app/stores';

	let error: App.Error | null = $page.error;
</script>

<div class="w-full items-center justify-center h-full content-center flex">
	{#if error}
		<div class="p-2 items-center max-w-xl">
			<h1 class="text-3xl font-bold">Error</h1>
			<p class="text-xl">Error status: {error.code}</p>
			<p class="text-xl">{error.message}</p>
			<p class="text-sm">ID: {error.id}</p>
		</div>
	{:else}
		<div class="p-2 items-center max-w-xl">
			<h1 class="text-3xl font-bold">Error</h1>
			<p class="text-xl">An unknown error eccured</p>
		</div>
	{/if}
</div>
