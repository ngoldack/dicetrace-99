<script lang="ts">
	import {
		AppShell,
		AppBar,
		AppRail,
		AppRailAnchor,
		AppRailTile,
		Avatar,
		type PopupSettings,
		popup
	} from '@skeletonlabs/skeleton';

	import { page } from '$app/stores';

	import DashboardIcon from '~icons/mdi/view-dashboard';
	import SettingsIcon from '~icons/mdi/cog';
	import CollectionIcon from '~icons/mdi/cube-outline';
	import HistoryIcon from '~icons/mdi/history';
	import SearchIcon from '~icons/mdi/magnify';

	import { signOut } from '@auth/sveltekit/client';

	const popupAccount: PopupSettings = {
		// Represents the type of event that opens/closed the popup
		event: 'click',
		// Matches the data-popup value on your popup element
		target: 'popupAccount',
		// Defines which side of your trigger the popup will appear
		placement: 'bottom'
	};

	export let data;
</script>

<nav>

	<AppShell>
		<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
				<h1 class="h1">
					<span
						class="bg-gradient-to-br from-blue-500 to-cyan-300 bg-clip-text text-transparent box-decoration-clone"
					>
						dicetrace
					</span>
				</h1>
			</svelte:fragment>

			<div class="w-[50%]">
				<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] outline-none w-max">
					<input type="search" class="outline-none p-2 w-full" placeholder="Search..." />
					<button class="variant-filled-secondary w-full">
						<SearchIcon />
					</button>
				</div>
			</div>

			<svelte:fragment slot="trail">
				<div class="flex flex-row gap-4 items-center justify-center h-fit" use:popup={popupAccount}>
					<Avatar
						initials={data.session?.user?.name
							?.split(' ')
							.map((n) => n[0])
							.join('')}
						url={data.session?.user?.image}
						background="bg-primary-500"
						rounded="rounded"
						width="w-12"
					/>
				</div>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<AppRail>
			<svelte:fragment slot="lead">
				<AppRailAnchor href="/app" selected={$page.url.pathname === '/app'}>
					<svelte:fragment slot="lead">
						<DashboardIcon class="text-xl" />
					</svelte:fragment>
					<span>Dashboard</span>
				</AppRailAnchor>
			</svelte:fragment>
			<!-- --- -->
			<AppRailAnchor href="/app/collection" selected={$page.url.pathname === '/app/collection'}>
				<svelte:fragment slot="lead">
					<CollectionIcon class="text-xl" />
				</svelte:fragment>
				<span>Collection</span>
			</AppRailAnchor>

			<AppRailAnchor href="/app/history" selected={$page.url.pathname === '/app/history'}>
				<svelte:fragment slot="lead">
					<HistoryIcon class="text-xl" />
				</svelte:fragment>
				<span>History</span>
			</AppRailAnchor>
			<!-- --- -->
			<svelte:fragment slot="trail">
				<AppRailAnchor href="/app/settings" selected={$page.url.pathname === '/app/settings'}>
					<svelte:fragment slot="lead">
						<SettingsIcon class="text-xl" />
					</svelte:fragment>
					<span>Settings</span>
				</AppRailAnchor>
			</svelte:fragment>
		</AppRail>
	</svelte:fragment>
	<slot />
</AppShell>
</nav>

<div class="card p-4 w-72 shadow-xl" data-popup="popupAccount">
	<ul>
		<li>{data.session?.user?.name}</li>
		<li>
			<button
				class="btn variant-outline-primary"
				on:click={() => {
					signOut({
						callbackUrl: '/'
					});
				}}
			>
				Sign Out
			</button>
		</li>
	</ul>
</div>
