<script>
	import { signIn } from '@auth/sveltekit/client';
	import { AppBar, AppShell } from '@skeletonlabs/skeleton';
</script>

<AppShell>
	<!-- (sidebarRight) -->
	<!-- (pageHeader) -->
	<!-- Router Slot -->
	<div class="w-full flex flex-col gap-10 items-center justify-center p-20">
		<div>
			<h1 class="h1 text-center">
				<span
					class="bg-gradient-to-br from-blue-500 to-cyan-300 bg-clip-text text-transparent box-decoration-clone"
				>
					dicetrace
				</span>
			</h1>
			<h2 class="h2">
				<span
					class="bg-gradient-to-br from-red-500 to-yellow-500 bg-clip-text text-transparent box-decoration-clone"
				>
					Play. Track. Analyze.
				</span>
			</h2>
		</div>
		<div class="flex flex-row gap-2">
			<button
				class="btn variant-outline-primary"
				on:click={() => {
					signIn(
						'auth0',
						{
							redirect: false,
							callbackUrl: 'http://localhost:5173/app'
						},
						{
							scope: 'api openid profile email'
						}
					);
				}}
			>
				Open App
			</button>
		</div>
	</div>
</AppShell>
