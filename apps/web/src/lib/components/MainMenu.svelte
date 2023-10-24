<script>
	import { page } from '$app/stores';
	import { signOut } from '@auth/sveltekit/client';
	import {
		Navbar,
		NavBrand,
		NavLi,
		NavUl,
		NavHamburger,
		Avatar,
		Button,
		Dropdown,
		DropdownItem,
		DropdownDivider
	} from 'flowbite-svelte';

	$: activeUrl = $page.url.pathname;
	$: user = $page.data.session?.user;
</script>

<Navbar>
	<NavBrand href="/">
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">PenPal</span>
	</NavBrand>
	<div class="flex items-center gap-x-4">
		<div class="flex md:order-2">
			{#if user}
				<Avatar class="cursor-pointer" src={user.image ?? ''} rounded />
				<Dropdown class="w-44 z-20">
					<DropdownItem href="/me">Profile</DropdownItem>
					<DropdownDivider />
					<DropdownItem on:click={() => signOut({ callbackUrl: '/' })}>Sign out</DropdownItem>
				</Dropdown>
			{:else}
				<Button size="sm" href="/login">Sign in</Button>
			{/if}
			<NavHamburger />
		</div>
		<NavUl class="order-1" {activeUrl}>
			<NavLi href="/">Home</NavLi>
			{#if user}
				<NavLi href="/letters">Letters</NavLi>
			{/if}
		</NavUl>
	</div>
</Navbar>
