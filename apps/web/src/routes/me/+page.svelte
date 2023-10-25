<script lang="ts">
	import { Avatar, Button, Card, Heading, Helper, Input, Label } from 'flowbite-svelte';
	import { page } from '$app/stores';
	import type { ActionData } from './$types';
	import { onMount } from 'svelte';

	export let form: ActionData;

	$: user = $page.data.session?.user;

	let profile_image: string | undefined;

	onMount(() => {
		profile_image = (form?.profile_image as string | undefined) ?? user?.profile_image ?? undefined;
	});
</script>

<div class="mx-auto container">
	<form method="post">
		<Card class="flex flex-col mx-auto max-w-lg gap-y-4">
			<Heading>Settings</Heading>
			{#if form?.reason && 'general' in form.reason}
				<Helper color="red">{form.reason.general}</Helper>
			{/if}
			{#if form?.success}
				<Helper color="green">Profile saved</Helper>
			{/if}
			<Label class="block space-y-2">
				<span>Profile image</span>
				<Avatar class="mx-auto" rounded size="lg" src={profile_image} />
				<Input
					label="Profile image"
					id="profile_image"
					name="profile_image"
					bind:value={profile_image}
					required
					placeholder="Profile image"
				/>
				{#if form?.reason && 'profile_image' in form.reason}
					<Helper color="red">{form.reason.profile_image}</Helper>
				{/if}
			</Label>
			<Label class="block space-y-2">
				<span>Username</span>
				<Input
					label="Username"
					id="username"
					name="username"
					value={form?.username ?? user?.username}
					required
					placeholder="Username"
				/>
				{#if form?.reason && 'username' in form.reason}
					<Helper color="red">{form.reason.username}</Helper>
				{/if}
			</Label>
			<Label class="block space-y-2">
				<span>Email</span>
				<Input
					label="Email"
					id="email"
					name="email"
					value={user?.email}
					required
					placeholder="Email"
					disabled
				/>
			</Label>
			<Label class="block space-y-2">
				<span>Display name</span>
				<Input
					label="Display name"
					id="display_name"
					name="display_name"
					value={form?.display_name ?? user?.display_name}
					required
					placeholder="Display name"
				/>
				{#if form?.reason && 'display_name' in form.reason}
					<Helper color="red">{form.reason.display_name}</Helper>
				{/if}
			</Label>
			<Button class="self-end" type="submit">Save changes</Button>
		</Card>
	</form>
</div>
