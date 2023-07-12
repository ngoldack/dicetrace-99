<script lang="ts">
	import {
		AppShell,
		AppBar,
		AppRail,
		AppRailAnchor,
		Avatar,
		type PopupSettings,
		popup
	} from '@skeletonlabs/skeleton';

	import { updated } from '$app/stores';
	import { page } from '$app/stores';

	import RefreshIcon from '~icons/mdi/refresh';
	import WarningIcon from '~icons/mdi/alert-circle-outline';
	import DashboardIcon from '~icons/mdi/view-dashboard';
	import SettingsIcon from '~icons/mdi/cog';
	import CollectionIcon from '~icons/mdi/cube-outline';
	import HistoryIcon from '~icons/mdi/history';
	import SearchIcon from '~icons/mdi/magnify';
	import GroupIcon from '~icons/mdi/account-group-outline';

	import { signOut } from '@auth/sveltekit/client';
	import { Toaster } from 'svelte-sonner';
	import { enhance } from '$app/forms';

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

<nav class="h-full" data-sveltekit-reload={$updated ? '' : 'off'}>
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
					<form method="post" action="/app/search?/search" use:enhance>
						<div
							class="input-group input-group-divider grid-cols-[auto_1fr_auto] outline-none w-max"
						>
							<input
								name="query"
								id="query"
								type="search"
								class="outline-none p-2 w-full"
								placeholder="Search..."
							/>
							<button class="variant-filled-secondary w-full" type="submit">
								<SearchIcon />
							</button>
						</div>
					</form>
				</div>

				<svelte:fragment slot="trail">
					<button
						class="flex flex-row gap-4 items-center justify-center h-fit btn btn-sm"
						use:popup={popupAccount}
					>
						<Avatar
							initials={data.userdata.name[0]}
							url={data.session?.user?.image}
							background="bg-primary-500"
							rounded="rounded"
							width="w-12"
						/>
					</button>
				</svelte:fragment>
			</AppBar>
		</svelte:fragment>

		<svelte:fragment slot="pageHeader">
			<!-- https://github.com/getsentry/sentry-javascript/discussions/5838#discussioncomment-6310009 -->
			{#if $updated}
				<div class="p-2">
					<aside class="alert variant-filled-warning">
						<!-- Icon -->
						<div>
							<WarningIcon class="text-3xl" />
						</div>
						<!-- Message -->

						<div class="alert-message">
							<h3 class="h3">App updated!</h3>
							<p>This app has been updated, please refresh.</p>
						</div>
						<!-- Actions -->
						<div class="alert-actions">
							<button class="btn variant-outline-primary" on:click={() => location.reload()}>
								<div>
									<RefreshIcon class="text-xl" />
								</div>
								Refresh
							</button>
						</div>
					</aside>
				</div>
			{/if}
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
				<AppRailAnchor href="/app/search" selected={$page.url.pathname === '/app/search'}>
					<svelte:fragment slot="lead">
						<SearchIcon class="text-xl" />
					</svelte:fragment>
					<span>Search</span>
				</AppRailAnchor>

				<AppRailAnchor href="/app/collection" selected={$page.url.pathname === '/app/collection'}>
					<svelte:fragment slot="lead">
						<CollectionIcon class="text-xl" />
					</svelte:fragment>
					<span>Collection</span>
				</AppRailAnchor>

				<AppRailAnchor href="/app/groups" selected={$page.url.pathname === '/app/groups'}>
					<svelte:fragment slot="lead">
						<GroupIcon class="text-xl" />
					</svelte:fragment>
					<span>Groups</span>
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
		<li>{data.userdata?.name}</li>
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

<Toaster richColors />
